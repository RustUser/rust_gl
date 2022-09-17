use crate::math::{m_max, m_min};

#[derive(Debug, Clone)]
pub enum Color {
    RGBA([f32; 4]),
    HSLA([f32; 4]),
    HSVA([f32; 4]),
}

#[macro_export]
macro_rules! rgba {
    ($color:expr) => { Color::RGBA($color) }
}

#[macro_export]
macro_rules! hsla {
    ($color:expr) => { Color::HSLA($color) }
}

#[macro_export]
macro_rules! hsva {
    ($color:expr) => { Color::HSVA($color) }
}

#[macro_export]
macro_rules! color_components {
    ($color:expr) => {
        ($color[0], $color[1], $color[2], $color[3])
    }
}

impl Color {
    pub const WHITE: Color = rgba!([1.0; 4]);
    pub const BLACK: Color = rgba!([0.0, 0.0, 0.0, 1.0]);

    pub const RED: Color = rgba!([1.0, 0.0, 0.0, 1.0]);
    pub const GREEN: Color = rgba!([0.0, 1.0, 0.0, 1.0]);
    pub const BLUE: Color = rgba!([0.0, 0.0, 1.0, 1.0]);

    pub fn to_rgba(self) -> Color {
        match self {
            Color::RGBA(rgba) => {
                rgba!(rgba)
            }
            Color::HSLA(hsla) => {
                let (h, s, l, a) = (hsla[0], hsla[1], hsla[2], hsla[3]);

                let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                let x = c * (1.0 - (h / 60.0).abs() % 2.0 - 1.0);
                let m = l - c / 2.0;

                let (r_prime, g_prime, b_prime) = Self::map_xch(c, x, h);

                rgba!([r_prime + m, g_prime + m, b_prime + m, a])
            }
            Color::HSVA(hsva) => {
                let (h, s, v, a) = (hsva[0], hsva[1], hsva[2], hsva[3]);

                let c = v * s;
                let x = c * (1.0 - (h / 60.0).abs() % 2.0 - 1.0);
                let m = v - c;

                let (r_prime, g_prime, b_prime) = Self::map_xch(c, x, h);

                rgba!([r_prime + m, g_prime + m, b_prime + m, a])
            }
        }
    }
    pub fn to_hsla(self) -> Color {
        match self {
            Color::RGBA(rgba) => {
                let (r_prime, g_prime, b_prime, a) = color_components!(rgba);

                let max = m_max(&rgba).unwrap();
                let min = m_min(&rgba).unwrap();
                let delta = max - min;

                let h = {
                    let a = 60.0;
                    let mut b = 0.0;
                    if max == r_prime {
                        let quotient = (g_prime - b_prime) / delta;
                        b = quotient % 6.0;
                    } else if max == g_prime {
                        let quotient = (b_prime - r_prime) / delta;
                        b = quotient + 2.0;
                    } else if max == b_prime {
                        let quotient = (r_prime - g_prime) / delta;
                        b = quotient + 4.0;
                    }
                    a * b
                };
                let l = (max + min) / 2.0;
                let s = match delta == 0.0 {
                    true => 0.0,
                    false => {
                        delta / (1.0 - (2.0 * l - 1.0).abs())
                    }
                };
                Color::HSLA([h, s, l, a])
            }
            Color::HSLA(_) => {
                self
            }
            Color::HSVA(_) => {
                panic!("HSVA to HSLA not currently supported.")
            }
        }
    }

    pub fn values(&self) -> &[f32; 4] {
        match self {
            Color::RGBA(values) => values,
            Color::HSLA(values) => values,
            Color::HSVA(values) => values,
        }
    }
    pub fn values_mut(&mut self) -> &mut [f32; 4] {
        match self {
            Color::RGBA(values) => values,
            Color::HSLA(values) => values,
            Color::HSVA(values) => values,
        }
    }

    fn map_xch(c: f32, x: f32, h: f32) -> (f32, f32, f32) {
        let ranges = [
            0.0..60.0,
            60.0..120.0,
            120.0..180.0,
            180.0..240.0,
            240.0..300.0,
            300.0..360.0
        ];
        let mut rgb = [0.0; 3];
        for i in 0..ranges.len() {
            let range = &ranges[i];
            if range.contains(&h) {
                rgb = match i {
                    0 => [c, x, 0.0],
                    1 => [x, c, 0.0],
                    2 => [0.0, c, x],
                    3 => [0.0, x, c],
                    4 => [x, 0.0, c],
                    5 => [c, 0.0, x],
                    _ => [0.0; 3]
                };
                break;
            }
        }
        (rgb[0], rgb[1], rgb[2])
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        *self.to_rgba().values()
    }
}