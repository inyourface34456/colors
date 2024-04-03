#![allow(unused)]
mod utils;

use std::fmt::Display;
use utils::*;
use wai_bindgen_rust::Handle;
use std::i32;

wai_bindgen_rust::export!("colors.wai");

struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl crate::colors::Color for Color {
    fn new(r: f32, g: f32, b: f32) -> Handle<Color> {
        Self::fromrgb(r, g, b)
    }

    fn fromrgb(r: f32, g: f32, b: f32) -> Handle<Color> {
        Self { r, g, b }.into()
    }

    fn fromcmyk(cyan: f32, magenta: f32, yellow: f32, black: f32) -> Handle<Color> {
        Self {
            r: 255.*(1.-cyan)*(1.-black),
            g: 255.*(1.-magenta)*(1.-black),
            b: 255.*(1.-yellow)*(1.-black),
        }
        .into()
    }

    fn fromhex(value: String) -> Handle<Color> {
        assert!(value.len() == 6 || value.len() == 7);
        let values: [u8; 4];
        
        if value.starts_with('#') {
            let value = &value[1..value.len()-1];
            values = i32::from_str_radix(value, 16).expect("invalid syntax").to_le_bytes();
        } else {
            values = i32::from_str_radix(value.as_str(), 16).expect("invalid syntax").to_le_bytes();
        }

        assert!(values[3] == 0);
        
        Self {
            r: values[0] as f32,
            g: values[1] as f32,
            b: values[2] as f32,
        }
        .into()
    }

    fn fromhsl(hue: f32, sateration: f32, lightness: f32) -> Handle<Color> {
        let c = (1.-(2.*lightness-1.).abs())*sateration;
        let x = c*(1.-((hue/60.)%2.-1.).abs());
        let m = lightness-c/2.;
        
        let (mut r, mut g, mut b) = 
        if hue >= 0.   && hue < 60.  {(c, x, 0.)} else 
        if hue >= 60.  && hue < 120. {(x, c, 0.)} else 
        if hue >= 120. && hue < 180. {(0., c, x)} else 
        if hue >= 180. && hue < 240. {(0., x, c)} else 
        if hue >= 240. && hue < 300. {(x, 0., c)} else 
        if hue >= 300. && hue < 360. {(c, 0., x)} else
        {panic!("invalid hue")};

        (r, g, b) = ((r+m)*255., (g+m)*255., (b+m)*255.,);

        Self {
            r,
            g,
            b,
        }
        .into()
    }

    fn fromhsv(hue: f32, sateration: f32, value: f32) -> Handle<Color> {
        let c = value*sateration;
        let x = c*(1.-((hue/60.)%2.-1.).abs());
        let m = value-c;
        
        let (mut r, mut g, mut b) = 
        if hue >= 0.   && hue < 60.  {(c, x, 0.)} else 
        if hue >= 60.  && hue < 120. {(x, c, 0.)} else 
        if hue >= 120. && hue < 180. {(0., c, x)} else 
        if hue >= 180. && hue < 240. {(0., x, c)} else 
        if hue >= 240. && hue < 300. {(x, 0., c)} else 
        if hue >= 300. && hue < 360. {(c, 0., x)} else
        {panic!("invalid hue")};

        (r, g, b) = ((r+m)*255., (g+m)*255., (b+m)*255.,);

        Self {
            r,
            g,
            b,
        }
        .into()
    }

    fn tohex(&self) -> String {
        format!("{:x}{:x}{:x}", self.r as u8, self.g as u8, self.b as u8)
    }

    fn tohsl(&self) -> (f32, f32, f32) {
        let (mut h, s, l);
        let r = self.r / 255.;
        let g = self.g / 255.;
        let b = self.b / 255.;

        let cmin = min(r, g, b);
        let cmax = max(r, g, b);

        let delta = cmax - cmin;

        if delta == 0. {
            h = 0.
        } else if cmax == r {
            h = (60. * (((g - b) / delta) % 6.)) % 360.;

            if h < 0. {
                h += 360.
            }
        } else if cmax == g {
            h = (60. * (((b - r) / delta) + 2.)) % 360.;
        } else {
            h = (60. * (((r - g) / delta) + 4.)) % 360.;
        }

        l = (cmin + cmax) / 2.;

        if delta == 0. {
            s = 0.
        } else {
            s = delta / (1. - 2. * l - 1.).abs()
        }

        (h, s, l)
    }

    fn tohsv(&self) -> (f32, f32, f32) {
        let (mut h, s);
        let r = self.r / 255.;
        let g = self.g / 255.;
        let b = self.b / 255.;

        let cmin = min(r, g, b);
        let cmax = max(r, g, b);

        let delta = cmax - cmin;

        if delta == 0. {
            h = 0.
        } else if cmax == r {
            h = (60. * (((g - b) / delta) % 6.)) % 360.;

            if h < 0. {
                h += 360.
            }
        } else if cmax == g {
            h = (60. * (((b - r) / delta) + 2.)) % 360.;
        } else {
            h = (60. * (((r - g) / delta) + 4.)) % 360.;
        }

        if cmax != 0. {
            s = delta / cmax
        } else {
            s = 0.
        }

        let delta = cmax - cmin;
        (h, s, cmax)
    }

    fn tocmyk(&self) -> (f32, f32, f32, f32) {
        let (c, m, y, k);
        let r = self.r / 255.;
        let g = self.g / 255.;
        let b = self.b / 255.;

        k = 1. - max(r, g, b);
        c = (1. - r - k) / (1. - k);
        m = (1. - g - k) / (1. - k);
        y = (1. - b - k) / (1. - k);

        (c, m, y, k)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Into<(f32, f32, f32)> for Color {
    fn into(self) -> (f32, f32, f32) {
        (self.r, self.b, self.g)
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(value: (f32, f32, f32)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

struct Colors;
impl crate::colors::Colors for Colors {}
