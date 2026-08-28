pub fn parse_bitmap_8x8(lines: [&str; 8]) -> [u8; 8] {
    let mut out: [u8; 8] = [0; 8];
    for (i, line) in lines.iter().enumerate() {
        let mut b: u8 = 0b0000_0000;
        let len = line.len();
        if len != 8 {
            panic!("Ошибка парсинга картинки!")
        }
        for (j, c) in line.bytes().enumerate() {
            let n = len - j - 1;
            if c == b'#' {
                b = b | (1 << n);
            }
        }
        out[i] = b;
    }
    out
}

pub fn render_bitmap_8x8(bytes: [u8; 8]) -> [String; 8] {
    let mut out: [String; 8] = std::array::from_fn(|_| String::new());
    for (i, byte) in bytes.iter().enumerate() {
        let mut n: i32 = 7;
        let mut tmp: String = String::new();
        while n >= 0 {
            if *byte & (1 << n) != 0 {
                tmp.push('#');
            } else {
                tmp.push('.');
            }
            n -= 1;
        }
        out[i] = tmp;
    }
    out
}

pub fn invert_bitmap_8x8(bytes: [u8; 8]) -> [u8; 8] {
    bytes.map(|x| !x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bitmap() {
        let image = [
            "..####..", ".#....#.", "#.#..#.#", "#..##..#", "#......#", "#.#..#.#", ".#....#.",
            "#.......",
        ];
        let bytes = parse_bitmap_8x8(image);
        let expected = [
            0b0011_1100,
            0b0100_0010,
            0b1010_0101,
            0b1001_1001,
            0b1000_0001,
            0b1010_0101,
            0b0100_0010,
            0b1000_0000,
        ];
        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_render_bitmap() {
        let bytes = [
            0b0011_1100,
            0b0100_0010,
            0b1010_0101,
            0b1001_1001,
            0b1000_0001,
            0b1010_0101,
            0b0100_0010,
            0b1000_0000,
        ];
        let image = render_bitmap_8x8(bytes);
        let expected = [
            "..####..", ".#....#.", "#.#..#.#", "#..##..#", "#......#", "#.#..#.#", ".#....#.",
            "#.......",
        ];
        assert_eq!(image, expected.map(|x: &str| x.to_string()));
    }
}
