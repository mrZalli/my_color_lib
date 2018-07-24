use std::str;
use std::fmt;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// The basic colors of the rainbow
pub enum BaseColor {
    Black,
    Grey,
    White,
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
}

impl fmt::Display for BaseColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::BaseColor::*;

        write!(f, "{}",
            match *self {
                Black   => "black",
                Grey    => "grey",
                White   => "white",
                Red     => "red",
                Yellow  => "yellow",
                Green   => "green",
                Cyan    => "cyan",
                Blue    => "blue",
                Magenta => "magenta",
            }
        )
    }
}

pub trait Color {
    /// Return the RGB representation
    fn rgb(&self) -> ColorRGB;
    fn r(&self) -> u8 { self.rgb().r }
    fn g(&self) -> u8 { self.rgb().g }
    fn b(&self) -> u8 { self.rgb().b }

    /// Return the HSV representation
    fn hsv(&self) -> ColorHSV;
    fn h(&self) -> f32 { self.hsv().h }
    fn s(&self) -> f32 { self.hsv().s }
    fn v(&self) -> f32 { self.hsv().v }

    /// Categorize this color's most prominent shades
    fn shades(&self) -> Vec<(f32, BaseColor)> {
        use self::BaseColor::*;
        // this whole function is horrible code. TODO: fix

        const COLORS: [BaseColor; 9] =
            [Black, Grey, White, Red, Yellow, Green, Cyan, Blue, Magenta];

        let (r, g, b) = self.rgb().to_tuple();

        let mut shade_weights = [0f32; 9];
        let mut min_weight = 0f32;
        let mut max_weight = 0f32;

        for (weight, color) in shade_weights.iter_mut().zip(COLORS.iter()) {
            let (r2, g2, b2) = color.rgb().to_tuple();

            let dist2 =
                (r2 as i32 - r as i32).pow(2) +
                (g2 as i32 - g as i32).pow(2) +
                (b2 as i32 - b as i32).pow(2);

            let dist = (dist2 as f32).sqrt();

            if dist < min_weight {
                min_weight = dist;
            }
            if dist > max_weight {
                max_weight = dist;
            }

            *weight = dist;
        }

        let mut shades = Vec::with_capacity(3);

        // rescale weight to go from range 0% to 100%
        for (dist, color) in shade_weights.iter().zip(COLORS.iter()) {
            let percentage = (max_weight - dist - 2.0 * min_weight) / (max_weight - min_weight);

            shades.push((percentage, *color))
        }

        return shades;
    }
}

impl Color for BaseColor {
    fn rgb(&self) -> ColorRGB {
        use self::BaseColor::*;

        let f = &ColorRGB::new;
        match self {
            Black   => f(  0,   0,   0),
            Grey    => f(128, 128, 128),
            White   => f(255, 255, 255),
            Red     => f(255,   0,   0),
            Yellow  => f(255, 255,   0),
            Green   => f(  0, 255,   0),
            Cyan    => f(  0, 255, 255),
            Blue    => f(  0,   0, 255),
            Magenta => f(255,   0, 255),
        }
    }

    fn hsv(&self) -> ColorHSV {
        use self::BaseColor::*;

        let f = &ColorHSV::new;
        match self {
            Black   => f(  0.0, 0.0, 0.0),
            Grey    => f(  0.0, 0.0, 0.5),
            White   => f(  0.0, 0.0, 1.0),
            Red     => f(  0.0, 1.0, 1.0),
            Yellow  => f( 60.0, 1.0, 1.0),
            Green   => f(120.0, 1.0, 1.0),
            Cyan    => f(180.0, 1.0, 1.0),
            Blue    => f(240.0, 1.0, 1.0),
            Magenta => f(300.0, 1.0, 1.0),
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// A 24-bit color with red, green and blue channels.
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB {
    /// Create a new RGB color.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ColorRGB {r, g, b}
    }

    /// Create `ColorRGB` from a hexcode.
    ///
    /// # Safety
    /// If `hex_str` is not a valid utf-8 string then this function will result in undefined
    /// behaviour.
    ///
    /// If `hex_str` doesn't consist only of the characters `[0-9a-fA-F]` then this function will
    /// result in a panic.
    pub unsafe fn from_hex_unchecked(hex_str: Box<str>) -> Self {
        let f = |h1: u8, h2: u8|
            u8::from_str_radix(str::from_utf8_unchecked(&[h1, h2]), 16).unwrap();

        let mut hex_str = hex_str;
        let h = hex_str.as_bytes_mut();
        h.make_ascii_lowercase();

        ColorRGB {
            r: f(h[0], h[1]),
            g: f(h[2], h[3]),
            b: f(h[4], h[5]),
        }
    }

    pub fn to_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl Color for ColorRGB {
    fn rgb(&self) -> ColorRGB { *self }

    fn hsv(&self) -> ColorHSV {
        let (r, g, b) =
            (self.r as f32 / 255.0,
             self.g as f32 / 255.0,
             self.b as f32 / 255.0);

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let chroma = max - min;

        let value = (r + g + b) / 3.0;

        let saturation =
            if value == 0.0 {
                0.0
            } else {
                chroma / value
            };

        let hue = 60.0 *
            if chroma == 0.0 {
                0.0
            } else if max == r {
                ((g - b) / chroma) % 6.0
            } else if max == g {
                (b - r) / chroma + 2.0
            } else { // max == b
                (r - g) / chroma + 4.0
            };

        ColorHSV::new(hue, saturation, value)
    }
}

impl fmt::Display for ColorRGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>3}, {:>3}, {:>3}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct ColorHSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl ColorHSV {
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        ColorHSV {h, s, v}
    }

    pub fn to_tuple(&self) -> (f32, f32, f32) {
        (self.h, self.s, self.v)
    }
}

impl Color for ColorHSV {
    fn rgb(&self) -> ColorRGB {
        let (h, s, v) = self.to_tuple();
        let h = h / 60.0;

        // chroma, largest component
        let c = s * v;

        // second largest component
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());

        // smallest component
        let min = v - c;

        let (r, g, b) =
            match h as u8 {
                0   => (  c,   x, 0.0),
                1   => (  x,   c, 0.0),
                2   => (0.0,   c,   x),
                3   => (0.0,   x,   c),
                4   => (  x, 0.0,   c),
                5|6 => (  c, 0.0,   x),
                _   => panic!("Invalid hue value: {}", self.h)
            };

        let (r, g, b) =
            ((r+min) as u8,
             (g+min) as u8,
             (b+min) as u8);

        ColorRGB{ r, g, b }
    }

    fn hsv(&self) -> ColorHSV { *self }

}
impl fmt::Display for ColorHSV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>width$.precision$}°, {:>width$.precision$}%, {:>width$.precision$}%",
               self.h, self.s * 100.0, self.v * 100.0, width=5, precision=1)
    }
}