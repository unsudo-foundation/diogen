pub trait Measurable 
where
    Self: ::std::fmt::Display {}
impl Measurable for Absolute {}
impl Measurable for Relative {}
impl Measurable for Viewport {}
impl Measurable for Angle {}
impl Measurable for Time {}
impl Measurable for Frequency {}
impl Measurable for Resolution {}


#[derive(Clone)]
#[derive(PartialEq)]
pub struct Unit<T> 
where
    T: Measurable {
    pub measurement: T,
    pub n: f64
}

impl<A, B> From<(B, f64)> for Unit<A> 
where
    A: Measurable,
    B: Into<A> {
    fn from(value: (B, f64)) -> Self {
        Self {
            measurement: value.0.into(),
            n: value.1
        }
    }
}

impl<T> ::std::fmt::Display for Unit<T>
where
    T: Measurable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.n, self.measurement)
    }
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Absolute {
    Px,
    Cm,
    Mm,
    Q,
    In,
    Pc,
    Pt
}

impl Absolute {
    pub fn into_unit(self, n: f64) -> Unit<Self> {
        Unit {
            measurement: self,
            n
        }
    }
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Relative {
    Em,
    Ex,
    Ch,
    Rem,
    Lh,
    Rlh,
    Percentage
}

impl Relative {
    pub fn into_unit(self, n: f64) -> Unit<Self> {
        Unit {
            measurement: self,
            n
        }
    }
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Viewport {
    Vw,
    Vh,
    Vmin,
    Vmax,
    Vb,
    Vi,
    Svw,
    Svh,
    Lvw,
    Lvh,
    Dvw,
    Dvh
}

impl Viewport {
    pub fn into_unit(self, n: f64) -> Unit<Self> {
        Unit {
            measurement: self,
            n
        }
    }
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Angle {
    Grad,
    Turn,
    Deg,
    Rad
}

impl Angle {
    pub fn into_unit(self, n: f64) -> Unit<Self> {
        Unit {
            measurement: self,
            n
        }
    }
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Time {
    S,
    Ms
}

impl Time {
    pub fn into_unit(self, n: f64) -> Unit<Self> {
        Unit {
            measurement: self,
            n
        }
    }
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Frequency {
    Hz,
    Khz
}

impl Frequency {
    pub fn into_unit(self, n: f64) -> Unit<Self> {
        Unit {
            measurement: self,
            n
        }
    }
}


#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(::strum_macros::EnumString)]
#[derive(::strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Resolution {
    Dpi,
    Dpcm,
    Dppx
}

impl Resolution {
    pub fn into_unit(self, n: f64) -> Unit<Self> {
        Unit {
            measurement: self,
            n
        }
    }
}