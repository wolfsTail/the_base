use std::env::args;
use std::io::{self, Write};

use ba03::merge_sort;

fn main() {
    write_sorted_args(
        args().skip(1).collect(),
        &mut io::BufWriter::new(io::stdout()),
    );
}

fn write_sorted_args<W>(args: Vec<String>, out: &mut W)
where
    W: Write,
{
    let args = merge_sort(args);
    for arg in args {
        if let Err(e) = writeln!(out, "{arg}") {
            panic!("Во время записи ответа произошла ошибка {}", e)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut buf: Vec<u8> = Vec::new();
        let input = vec!["b".to_string(), "a".to_string()];
        write_sorted_args(input, &mut buf);
        let haved = String::from_utf8(buf).unwrap();
        assert_eq!(haved, "a\nb\n");
    }
    #[test]
    fn test_2() {
        let mut buf: Vec<u8> = Vec::new();
        let input = vec!["B".to_string(), "A".to_string()];
        write_sorted_args(input, &mut buf);
        let haved = String::from_utf8(buf).unwrap();
        assert_eq!(haved, "A\nB\n");
    }
    #[test]
    fn test_3() {
        let mut buf: Vec<u8> = Vec::new();
        let input = vec!["a".to_string(), "A".to_string()];
        write_sorted_args(input, &mut buf);
        let haved = String::from_utf8(buf).unwrap();
        assert_eq!(haved, "A\na\n");
    }
    #[test]
    fn test_4() {
        let mut buf: Vec<u8> = Vec::new();
        let input = vec![
            "hello,".to_string(),
            "world,".to_string(),
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "program".to_string(),
        ];
        write_sorted_args(input, &mut buf);
        let haved = String::from_utf8(buf).unwrap();
        assert_eq!(haved, "a\nhello,\nis\nprogram\nthis\nworld,\n");
    }
}
