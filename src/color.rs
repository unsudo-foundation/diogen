static HEX_CHAR: [char; 22] = [
    '0', '1', '2',
    '3', '4', '5',
    '6', '7', '8',
    '9',
    'A', 'B', 'C',
    'D', 'E', 'F',
    'a', 'b', 'c',
    'd', 'e', 'f'
];

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
#[derive(::thiserror::Error)]
pub enum Error {
    #[error("")]
    TryFromIntError(#[from] ::std::num::TryFromIntError),
    #[error("")]
    EmptyStringOnConversion,
    #[error("")]
    StringTooShortOnConversion,
    #[error("")]
    StringTooLongOnConversion,
    #[error("")]
    IllegalCharOnConversion,
    #[error("")]
    HexOutOfRange
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Color {
    Hex(u32)
}

impl Color {
    pub const fn from_hex(code: u32) -> Self {
        if code > 0xffffff {
            panic!("[ABORT] Value out of 24-bit RGB range");
        }
        Self::Hex(code)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::from_hex(0xffffff)
    }
}

impl TryFrom<String> for Color {
    type Error = Error;

    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        let mut s: String = String::new();
        if value.is_empty() {
            return Err(Error::EmptyStringOnConversion)
        }
        if !value.starts_with("#") {
            if value.len() < 6 {
                return Err(Error::StringTooShortOnConversion)
            }
            if value.len() > 6 {
                return Err(Error::StringTooLongOnConversion)
            }
            for char in value.chars() {
                for hex_char in HEX_CHAR {
                    if char != hex_char {
                        return Err(Error::IllegalCharOnConversion)
                    }
                }
                s.push(char);
            }
        } else {
            if value.len() < 7 {
                return Err(Error::StringTooShortOnConversion)
            }
            if value.len() > 7 {
                return Err(Error::StringTooLongOnConversion)
            }
            for char in value.chars() {
                for hex_char in HEX_CHAR {
                    if char != hex_char {
                        return Err(Error::IllegalCharOnConversion)
                    }
                }
                s.push(char);
            }
        }
        let ret = u32::from_str_radix(&s, 16).unwrap();
        Ok(Self::Hex(ret))
    }
}

impl TryFrom<&str> for Color {
    type Error = Error;

    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::EmptyStringOnConversion)
        }
        if value.starts_with("#") {
            if value.len() < 7 {
                return Err(Error::StringTooShortOnConversion)
            }
            if value.len() > 7 {
                return Err(Error::StringTooLongOnConversion)
            }
            for char in value.chars() {
                for hex_char in HEX_CHAR {
                    if char != hex_char {
                        return Err(Error::IllegalCharOnConversion)
                    }
                }
            }
            let ret = u32::from_str_radix(value, 16).unwrap();
            Ok(Self::Hex(ret))  
        } else {
            if value.len() < 6 {
                return Err(Error::StringTooShortOnConversion)
            }
            if value.len() > 6 {
                return Err(Error::StringTooLongOnConversion)
            }
            for char in value.chars() {
                for hex_char in HEX_CHAR {
                    if char != hex_char {
                        return Err(Error::IllegalCharOnConversion)
                    }
                } 
            }
            let ret = u32::from_str_radix(value, 16).unwrap();
            Ok(Self::Hex(ret))
        }
    }
}

impl TryFrom<u8> for Color {
    type Error = Error;

    fn try_from(value: u8) -> ::std::result::Result<Self, Self::Error> {
        let ret: u32 = value.into();
        let ret: Self = ret.try_into()?;
        Ok(ret)
    }
}

impl TryFrom<u16> for Color {
    type Error = Error;

    fn try_from(value: u16) -> ::std::result::Result<Self, Self::Error> {
        let ret: u32 = value.into();
        let ret: Self = ret.try_into()?;
        Ok(ret)
    }
}

impl TryFrom<u32> for Color {
    type Error = Error;

    fn try_from(value: u32) -> ::std::result::Result<Self, Self::Error> {
        if value > 0xffffff {
            return Err(Error::HexOutOfRange)
        }
        Ok(Self::Hex(value))
    }
}

impl TryFrom<u64> for Color {
    type Error = Error;

    fn try_from(value: u64) -> ::std::result::Result<Self, Self::Error> {
        let ret: u32 = value.try_into()?;
        let ret: Self = ret.try_into()?;
        Ok(ret)
    }
}

impl TryFrom<u128> for Color {
    type Error = Error;

    fn try_from(value: u128) -> ::std::result::Result<Self, Self::Error> {
        let ret: u32 = value.try_into()?;
        let ret: Self = ret.try_into()?;
        Ok(ret)
    }
}

impl TryFrom<usize> for Color {
    type Error = Error;

    fn try_from(value: usize) -> ::std::result::Result<Self, Self::Error> {
        let ret: u32 = value.try_into()?;
        let ret: Self = ret.try_into()?;
        Ok(ret)
    }
}

impl ::std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Hex(code) => format!("#{}", code)
        })
    }
}





fn interpolate(range: (&str, &str), t: f32) -> String {
    let (rx, gx, bx) = hex_to_rgb(range.0);
    let (ry, gy, by) = hex_to_rgb(range.1);
    let r = rx as f32 + (ry as f32 - rx as f32) * t;
    let r = r.round() as u8;
    let g = gx as f32 + (gy as f32 - gx as f32) * t;
    let g = g.round() as u8;
    let b = bx as f32 + (by as f32 - bx as f32) * t;
    let b = b.round() as u8;
    rgb_to_hex(r, g, b)
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}