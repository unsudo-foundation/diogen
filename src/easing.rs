pub fn ease_in(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c *
    (p / d) *
    (p / d) + s
}

pub fn ease_in_cubic(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c *
    (p / d) *
    (p / d) *
    (p / d) + s
}

pub fn ease_in_quart(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c *
    (p / d) *
    (p / d) *
    (p / d) * 
    (p / d) + s
}

pub fn ease_in_quint(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c * 
    (p / d) *
    (p / d) *
    (p / d) *
    (p / d) *
    (p / d) + s
}


pub fn ease_out(p: f32, s: f32, c: f32, d: f32) -> f32 {
    -c * (p / d) * ((p / d) - 2.0f32) + s
}

pub fn ease_out_expo(p: f32, s: f32, c: f32, d: f32) -> f32 {
    if p == d {
        return s + c
    }
    let k: f32 = -2f32.powf(-10.0f32 * p / d) + 1.0f32;
    c * k + s
}

pub fn ease_out_cubic(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c * (
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) + 1.0f32
    ) + s
}

pub fn ease_out_quart(p: f32, s: f32, c: f32, d: f32) -> f32 {
    -c * (
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) - 1.0f32
    ) + s
}

pub fn ease_out_quint(p: f32, s: f32, c: f32, d: f32) -> f32 {
    c * (
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) *
        (p / d - 1.0f32) + 1.0f32
    ) + s
}