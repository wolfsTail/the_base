use std::fmt::{self, Display};
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
struct Out {
    bytes: u64,
    lines: u64,
    words: u64,
}

impl Out {
    fn new(tpl: (u64, u64, u64)) -> Out {
        Out {
            bytes: tpl.0,
            lines: tpl.1,
            words: tpl.2,
        }
    }
}

impl Display for Out {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.lines, self.words, self.bytes)
    }
}

fn main() {
    println!("{}", get_bytes_lines_words_count(io::stdin()));
}

fn get_bytes_lines_words_count<R>(mut src: R) -> Out
where
    R: Read,
{
    let mut buf: [u8; 1 << 16] = [0; 1 << 16];
    let mut bytes: u64 = 0;
    let mut lines: u64 = 0;
    let mut words: u64 = 0;
    let mut in_word: bool = false;
    loop {
        let c = match src.read(&mut buf) {
            Ok(n) => n,
            Err(e) => panic!("Во время чтения ввода произошла ошибка {}", e),
        };
        if c == 0 {
            break;
        }
        for b in &buf[..c] {
            if b.is_ascii_whitespace() {
                in_word = false;
                if *b == b'\n' {
                    lines += 1;
                }
            } else if !in_word {
                words += 1;
                in_word = true;
            }
        }
        bytes += c as u64;
    }
    Out::new((bytes, lines, words))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multy_cases() {
        let big_input = vec![0u8; 1 << 20];
        let big_input_1 = b"Abcd ".repeat(1 << 10);
        let big_input_2 = b"Abcd\n".repeat(1 << 10);
        let cases = vec![
            (&b""[..], Out::new((0, 0, 0))),
            (&b"abcd"[..], Out::new((4, 0, 1))),
            (&b"abcd\r"[..], Out::new((5, 0, 1))),
            (&b"abcd/r"[..], Out::new((6, 0, 1))),
            (&b"abcd\n"[..], Out::new((5, 1, 1))),
            (&b"abcd, abcd\n"[..], Out::new((11, 1, 2))),
            (&big_input, Out::new((1 << 20, 0, 1))),
            (&big_input_1, Out::new((5 * (1 << 10), 0, 1 << 10))),
            (&big_input_2, Out::new((5 * (1 << 10), 1 << 10, 1 << 10))),
            ("🦀".as_bytes(), Out::new((4, 0, 1))),
        ];
        for (input, output) in cases {
            assert_eq!(get_bytes_lines_words_count(input), output)
        }
    }

    #[test]
    #[should_panic]
    fn panic_on_read() {
        struct DummyReader;
        impl Read for DummyReader {
            fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("Ошибка!"))
            }
        }
        get_bytes_lines_words_count(DummyReader);
    }
}
