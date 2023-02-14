/// Implementation of RFC 4648 Base 32 encoding/decoding

use bytes::{Bytes, BytesMut, Buf, BufMut};

pub fn decode(value: &str) -> Option<BytesMut> {
    let values = value
                      .chars()
                      .map( |x| x.to_ascii_uppercase());
    let mut buf = BytesMut::with_capacity(0);
    buf.reserve(5 * value.len() / 8);
    let mut next = 0u8;
    let mut i = 0u8;
    for ele in values {
        if ele == '=' {
            break;
        }
        match decode_char(ele) {
            None => {
                return None;
            }
            Some(v) => {
                i = match i {
                    0 => {
                        next = next | (v << 3);
                        5
                    }
                    1 => {
                        next = next | (v << 2);
                        6
                    }
                    2 => {
                        next = next | (v << 1);
                        7
                    }
                    3 => {
                        next = next | v;
                        buf.put_u8(next);
                        next = 0;
                        0
                    }
                    4 => {
                        next = next | (v >> 1);
                        buf.put_u8(next);
                        next = v << 7;
                        1
                    }
                    5 => {
                        next = next | (v >> 2);
                        buf.put_u8(next);
                        next = v << 6;
                        2
                    }
                    6 => {
                        next = next | (v >> 3);
                        buf.put_u8(next);
                        next = v << 5;
                        3
                    }
                    7 => {
                        next = next | (v >> 4);
                        buf.put_u8(next);
                        next = v << 4;
                        4
                    }
                    _ => {0}
                }

            }
        }
    }
    Some(buf)
}

fn decode_char(v: char) -> Option<u8> {
    match v {
        'A' => Some(0u8),
        'B' => Some(1u8),
        'C' => Some(2u8),
        'D' => Some(3u8),
        'E' => Some(4u8),
        'F' => Some(5u8),
        'G' => Some(6u8),
        'H' => Some(7u8),
        'I' => Some(8u8),
        'J' => Some(9u8),
        'K' => Some(10u8),
        'L' => Some(11u8),
        'M' => Some(12u8),
        'N' => Some(13u8),
        'O' => Some(14u8),
        'P' => Some(15u8),
        'Q' => Some(16u8),
        'R' => Some(17u8),
        'S' => Some(18u8),
        'T' => Some(19u8),
        'U' => Some(20u8),
        'V' => Some(21u8),
        'W' => Some(22u8),
        'X' => Some(23u8),
        'Y' => Some(24u8),
        'Z' => Some(25u8),
        '2' => Some(26u8),
        '3' => Some(27u8),
        '4' => Some(28u8),
        '5' => Some(29u8), 
        '6' => Some(30u8),
        '7' => Some(31u8),
        '=' => None,
        _ => None
    }
}

mod test {
    use crate::base32::decode;
    #[test]
    fn test_normal_decoding() {
        let value = decode("JBSWY3DPEHPK3PXP").unwrap();
        assert_eq!(value.len(), 10);
        assert_eq!(value.get(0).unwrap().to_owned(), 0x48u8);        
        assert_eq!(value.get(1).unwrap().to_owned(), 0x65u8);        
        assert_eq!(value.get(2).unwrap().to_owned(), 0x6cu8);        
        assert_eq!(value.get(3).unwrap().to_owned(), 0x6cu8);        
        assert_eq!(value.get(4).unwrap().to_owned(), 0x6fu8);        
        assert_eq!(value.get(5).unwrap().to_owned(), 0x21u8);        
        assert_eq!(value.get(6).unwrap().to_owned(), 0xdeu8);        
        assert_eq!(value.get(7).unwrap().to_owned(), 0xadu8);        
        assert_eq!(value.get(8).unwrap().to_owned(), 0xbeu8);        
        assert_eq!(value.get(9).unwrap().to_owned(), 0xefu8);        
    }
    #[test]
    fn test_decode_padding() {
        let value = decode("32W353Y====").unwrap();
        assert_eq!(value.len(), 4);
        assert_eq!(value.get(0).unwrap().to_owned(), 0xdeu8);        
        assert_eq!(value.get(1).unwrap().to_owned(), 0xadu8);        
        assert_eq!(value.get(2).unwrap().to_owned(), 0xbeu8);        
        assert_eq!(value.get(3).unwrap().to_owned(), 0xefu8);        
    }

    #[test]
    fn test_invalud_decode_input() {
        let value = decode ("32W39");
        assert!(value.is_none());
    }
}