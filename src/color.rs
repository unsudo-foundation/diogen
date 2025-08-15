pub type Hex = u32;
pub type Rgb = (u8, u8, u8);
pub type Rgba = (u8, u8, u8, f32);

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
    ValueOutOf24BitRGBRange
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Color {
    Hex(Hex),
    Rgb(Rgb),
    Rgba(Rgba)
}

impl Color {
    pub const fn from_rgb(raw: Rgb) -> Self {
        let r = raw.0;
        let g = raw.1;
        let b = raw.2;
        Self::Rgb((r, g, b))
    }

    pub const fn from_rgba(raw: Rgba) -> Self {
        let r = raw.0;
        let g = raw.1;
        let b = raw.2;
        let a = raw.3.clamp(0.0, 1.0);
        Self::Rgba((r, g, b, a))
    }

    pub const fn from_hex(raw: Hex) -> Self {
        if raw > 0xffffff {
            panic!("[ABORT] Value out of 24-bit RGB range");
        }
        Self::Hex(raw)
    }

    pub fn from_hex_rep(rep: &str) -> Result<Self> {
        let s: &str = rep.trim_start_matches('#');
        if s.len() != 6 {
            return Err(if s.len() < 6 {
                Error::StringTooShortOnConversion
            } else {
                Error::StringTooLongOnConversion
            })
        }
        for c in s.chars() {
            Self::only_hex_char(c)?;
        }
        let ret: u32 = u32::from_str_radix(s, 16).unwrap();
        Ok(Self::Hex(ret))
    }

    fn only_hex_char(c: char) -> Result<char> {
        if !c.is_ascii_hexdigit() {
            return Err(Error::IllegalCharOnConversion)
        }
        Ok(c)
    }

    pub const fn hex(&self) -> u32 {
        match self {
            Self::Hex(code) => *code,
            Self::Rgb((r, g, b)) | Self::Rgba((r, g, b, _)) => ((*r as u32) << 16) | ((*g as u32) << 8) | (*b as u32)
        }
    }

    pub fn hex_rep(&self) -> String {
        let hex: u32 = self.hex();
        format!("#{:06X}", hex)
    }

    pub const fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Self::Hex(code) => (
                ((*code >> 16) & 0xff) as u8,
                ((*code >> 8) & 0xff) as u8,
                (*code & 0xff) as u8
            ),
            Self::Rgb((r, g, b)) | Self::Rgba((r, g, b, _)) => (*r, *g, *b)
        }
    }

    pub fn rgb_rep(&self) -> String {
        let (r, g, b) = self.rgb();
        format!("rgb({}, {}, {})", r, g, b)
    }

    pub fn rgba(&self) -> (u8, u8, u8, f32) {
        match self {
            Self::Hex(code) => (
                ((*code >> 16) & 0xff) as u8,
                ((*code >> 8) & 0xff) as u8,
                (*code & 0xff) as u8,
                1.0
            ),
            Self::Rgb((r, g, b)) => (*r, *g, *b, 1.0),
            Self::Rgba((r, g, b, a)) => (*r, *g, *b, *a)
        }
    }

    pub fn rgba_rep(&self) -> String {
        let (r, g, b, a) = self.rgba();
        format!("rgba({}, {}, {}, {})", r, g, b, a)
    }

    pub fn interpolate(&self, rhs: Self, t: f32) -> Self {
        let (rx, gx, bx, ax) = self.rgba();
        let (ry, gy, by, ay) = rhs.rgba();
        let r: u8 = (rx as f32 + (ry as f32 - rx as f32) * t).round() as u8;
        let g: u8 = (gx as f32 + (gy as f32 - gx as f32) * t).round() as u8;
        let b: u8 = (bx as f32 + (by as f32 - bx as f32) * t).round() as u8;
        let a: f32 = ax + (ay - ax) * t;
        Self::Rgba((r, g, b, a))
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
        let ret: &str = &value;
        let ret: Self = ret.try_into()?;
        Ok(ret)
    }
}

impl TryFrom<&str> for Color {
    type Error = Error;

    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        let s = value.trim_start_matches('#');
        if s.len() != 6 {
            return Err(if s.len() < 6 {
                Error::StringTooShortOnConversion
            } else {
                Error::StringTooLongOnConversion
            })
        }
        for c in s.chars() {
            Self::only_hex_char(c)?;
        }
        let ret = u32::from_str_radix(s, 16).unwrap();
        Ok(Self::Hex(ret))
    }
}

impl TryFrom<Hex> for Color {
    type Error = Error;

    fn try_from(value: Hex) -> ::std::result::Result<Self, Self::Error> {
        if value > 0xffffff {
            return Err(Error::ValueOutOf24BitRGBRange)
        }
        Ok(Self::Hex(value))
    }
}

impl From<Rgb> for Color {
    fn from(value: Rgb) -> Self {
        Self::from_rgb(value)
    }
}

impl From<Rgba> for Color {
    fn from(value: Rgba) -> Self {
        Self::from_rgba(value)
    }
}

impl ::std::fmt::Display for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Hex(code) => write!(f, "#{:06X}", code),
            Self::Rgb((r, g, b)) => write!(f, "rgb({}, {}, {})", r, g, b),
            Self::Rgba((r, g, b, a)) => write!(f, "rgba({}, {}, {}, {:.2})", r, g, b, a)
        }
    }
}