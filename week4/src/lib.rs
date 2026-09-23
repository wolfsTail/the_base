use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub const BUFFER_SIZE: usize = 64 * 1024;

// -----------------------------------------------------------------------------
// MyBufReader
// -----------------------------------------------------------------------------

pub struct MyBufReader {
    buf: Vec<u8>,
    current_size: usize,
    out: usize,
    f: File,
}

impl MyBufReader {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let f = File::open(path)?;
        Ok(Self {
            buf: vec![0_u8; BUFFER_SIZE],
            current_size: 0,
            out: 0,
            f: f,
        })
    }

    pub fn read_byte(&mut self) -> io::Result<Option<u8>> {
        if self.current_size == self.out {
            self.out = 0;
            self.current_size = 0; // сохраняем состояние структуры, есди следующий вызов упадет
            self.current_size = self.f.read(&mut self.buf)?;
            if self.current_size == 0 {
                return Ok(None);
            }
        }
        let b: u8 = self.buf[self.out];
        self.out += 1;
        Ok(Some(b))
    }
}

// -----------------------------------------------------------------------------
// MyBufWriter
// -----------------------------------------------------------------------------

pub struct MyBufWriter {
    buf: Vec<u8>,
    f: File,
}

impl MyBufWriter {
    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let f = File::create(path)?;
        Ok(Self {
            buf: Vec::with_capacity(BUFFER_SIZE),
            f: f,
        })
    }

    pub fn write_buffered(&mut self, data: &[u8]) -> io::Result<()> {
        let mut done: usize = 0;
        while done < data.len() {
            let mut curr = BUFFER_SIZE - self.buf.len();
            if curr == 0 {
                self.flush()?;
                curr = BUFFER_SIZE;
            }
            let will_copy = curr.min(data.len() - done);
            self.buf.extend_from_slice(&data[done..(done + will_copy)]);
            done += will_copy;
        }
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.f.write_all(&self.buf)?;
        self.buf.clear();
        Ok(())
    }

    pub fn close(mut self) -> io::Result<()> {
        self.flush()
    }
}

impl Drop for MyBufWriter {
    fn drop(&mut self) {
        // Ошибку из Drop вернуть нельзя.
        // Поэтому в реальном коде лучше явно вызывать close() или flush().
        let _ = self.flush();
    }
}

// -----------------------------------------------------------------------------
// Медленная версия
// -----------------------------------------------------------------------------

pub fn copy_slow(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut input = File::open(input)?;
    let mut output = File::create(output)?;

    let mut copied = 0;
    let mut byte = [0u8; 1];

    loop {
        let n = input.read(&mut byte)?;
        if n == 0 {
            break;
        }

        output.write_all(&byte[..n])?;
        copied += n as u64;
    }

    output.flush()?;

    Ok(copied)
}

// -----------------------------------------------------------------------------
// Быстрая версия
// -----------------------------------------------------------------------------
// copy_fast специально тоже использует побайтный API.
// Разница должна быть не в коде копирования, а в реализации MyBufReader и MyBufWriter
// эту функцию не нужно менять, она должна работать с любыми реализациями MyBufReader и MyBufWriter,
// которые вы сделаете
pub fn copy_fast(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut reader = MyBufReader::open(input)?;
    let mut writer = MyBufWriter::create(output)?;

    let mut copied = 0;

    while let Some(byte) = reader.read_byte()? {
        writer.write_buffered(&[byte])?;
        copied += 1;
    }

    writer.close()?;

    Ok(copied)
}

pub const RECORD_SIZE: usize = 10;

pub fn make_record(index: usize) -> [u8; RECORD_SIZE] {
    let mut record = [0u8; RECORD_SIZE];

    (0..RECORD_SIZE).for_each(|i| {
        record[i] = ((index + i) % 251) as u8;
    });

    record
}

pub fn generate_input_file(path: impl AsRef<Path>, records: usize) -> io::Result<()> {
    let mut file = File::create(path)?;

    for i in 0..records {
        let record = make_record(i);
        file.write_all(&record)?;
    }

    file.flush()?;

    Ok(())
}
