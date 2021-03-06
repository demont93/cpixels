use std::collections::HashMap;
use std::iter::{Iterator};
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Cpixel(pub char);

impl Cpixel {
    pub fn from_brightness(brightness: u8) -> Self {
        static CHARS: &'static [char] = &[
            ' ', '`', '.', '\'', '_', '~', '"', '^', 'r', '!', '/', '(', ')',
            '?', 'i', 't', 'c', 'j', '=', '7', 'Y', 'J', '}', '1', 'o', '%',
            'e', 'V', 'S', 'F', '4', 'k', '5', 'O', 'q', 'd', 'p', 'Q', 'E',
            '9', 'H', 'g', 'R', 'm', 'W', '@', 'B', 'N'
        ];
        static BRIGHTNESS: &'static [u8] = &[
            0_u8, 5_u8, 14_u8, 23_u8, 32_u8, 45_u8, 58_u8, 64_u8, 69_u8, 75_u8,
            82_u8, 92_u8, 97_u8, 101_u8, 108_u8, 116_u8, 119_u8, 123_u8, 127_u8,
            131_u8, 134_u8, 138_u8, 142_u8, 145_u8, 151_u8, 156_u8, 160_u8,
            164_u8, 168_u8, 171_u8, 177_u8, 184_u8, 190_u8, 193_u8, 197_u8,
            201_u8, 204_u8, 208_u8, 212_u8, 216_u8, 219_u8, 223_u8, 227_u8,
            230_u8, 236_u8, 243_u8, 249_u8, 253_u8
        ];
        lazy_static::lazy_static! {
            static ref MAP: HashMap<u8, char> = {
                let mut map = HashMap::new();
                BRIGHTNESS.iter().zip(CHARS.iter()).for_each(|(&b,&c)| {
                    map.insert(b,c);
                });
                map
            };
        }
        Cpixel(*MAP.get(&brightness).unwrap())
    }
}

impl Display for Cpixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Cpixel(c) = self;
        write!(f, "{}", *c)
    }
}
