use crate::char_reader::CharReader;
use crate::encoded::Encoded;

use std::io::Read;

pub struct Utf8;

impl<T: Read> CharReader for Encoded<T, Utf8> {
    fn read_char(&mut self) -> std::io::Result<Option<char>> {
        let mut bytes = [0u8; 4];
        let mut codepoint: u32;

        let n = self.read(&mut bytes[0..1])?;
        if n == 0 {
            return Ok(None); // EOF
        }
        assert_eq!(n, 1);

        let first = bytes[0];
        let width = match first {
            0x00..0x7f => 1,
            0xC0..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF7 => 4,

            _ => return Ok(Some('\u{FFFD}')),
        };

        if width == 1 {
            return Ok(Some(first as char));
        }

        self.read_exact(&mut bytes[1..width])?;

        codepoint = (first & (0x7f >> width)) as u32;

        for &byte in &bytes[1..width] {
            if byte & 0xC0 != 0x80 {
                return Ok(Some('\u{FFFD}'));
            }

            codepoint = (codepoint << 6) | (byte & 0x3F) as u32;
        }

        Ok(std::char::from_u32(codepoint))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read<R: CharReader>(mut r: R) -> Vec<char> {
        let mut out = Vec::new();
        while let Some(c) = r.read_char().unwrap() {
            out.push(c);
        }
        out
    }

    fn reader(bytes: &[u8]) -> Encoded<&[u8], Utf8> {
        Encoded::<_, Utf8>::new(bytes)
    }

    #[test]
    fn ascii() {
        let r = reader(b"ABC");
        assert_eq!(read(r), vec!['A', 'B', 'C']);
    }

    #[test]
    fn two_byte() {
        let r = reader(&[0xC2, 0xA2]);
        assert_eq!(read(r), vec!['¢']);
    }

    #[test]
    fn three_byte() {
        let r = reader(&[0xE2, 0x82, 0xAC]);
        assert_eq!(read(r), vec!['€']);
    }

    #[test]
    fn four_byte() {
        let r = reader(&[0xF0, 0x9F, 0x98, 0x80]);
        assert_eq!(read(r), vec!['😀']);
    }
}
