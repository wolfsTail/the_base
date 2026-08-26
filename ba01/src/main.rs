use std::io::Read;
use std::io::stdin;

fn main() {
    println!("{}", get_bytes_count(stdin()));
}

fn get_bytes_count<R>(mut src: R) -> u64
where
    R: Read,
{
    let mut b: [u8; 1 << 16] = [0; 1 << 16];
    let mut r: u64 = 0;
    loop {
        let c = match src.read(&mut b) {
            Ok(n) => n,
            Err(e) => panic!("Во время чтения ввода произошла ошибка {}", e),
        };
        if c == 0 {
            break;
        }
        r += c as u64;
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multy_cases() {
        let big_input = vec![0u8; 1 << 20];
        let cases = vec![
            (&b""[..], 0),
            (&b"abcd"[..], 4),
            (&b"abcd\r"[..], 5),
            (&b"abcd/r"[..], 6),
            (&big_input, 1 << 20),
        ];
        for (input, output) in cases {
            assert_eq!(get_bytes_count(input), output)
        }
    }

    #[test]
    #[should_panic]
    fn panic_on_read() {
        struct DummyReader;
        impl Read for DummyReader {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("Ошибка!"))
            }
        }
        get_bytes_count(DummyReader);
    }
}
