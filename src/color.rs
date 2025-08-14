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

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        if value > 0xffffff {
            panic!("[ABORT] Value out of 24-bit RGB range: 0x{:X}", value);
        }
        Self::Hex(value)
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