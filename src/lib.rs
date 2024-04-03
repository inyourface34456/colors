#![allow(unused)]
mod utils;

use std::fmt::Display;
use std::error::Error;
use utils::*;
use wai_bindgen_rust::Handle;
use std::i32;
use crate::colors::Exeptions;

wai_bindgen_rust::export!("colors.wai");

struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl crate::colors::Color for Color {
    fn new(r: f64, g: f64, b: f64) -> Result<Handle<Color>, Exeptions> {
        Self::fromrgb(r, g, b)
    }

    fn fromrgb(r: f64, g: f64, b: f64) -> Result<Handle<Color>, Exeptions> {
        if r>255.||r<0. {
            return Err(Exeptions::Redoutofrange(r))
        } else if g>255.||g<0. {
            return Err(Exeptions::Greenoutofrange(g))
        } else if b>255.||b<0. {
            return Err(Exeptions::Blueoutofrange(b))
        }
        
        Ok(Self { r, g, b }.into())
    }

    fn fromcmyk(cyan: f64, magenta: f64, yellow: f64, black: f64) -> Result<Handle<Color>, Exeptions> {
        if cyan>1.||cyan<0. {
            return Err(Exeptions::Cyanoutofrange(cyan));
        } else if magenta>1.||magenta<0. {
            return Err(Exeptions::Magentaoutofrange(magenta));
        } else if yellow>1.||yellow<0. {
            return Err(Exeptions::Yellowoutofrange(yellow));
        } else if black>1.||black<0. {
            return Err(Exeptions::Blackoutofrange(black));
        }
        
        Ok(Self {
            r: 255.*(1.-cyan)*(1.-black),
            g: 255.*(1.-magenta)*(1.-black),
            b: 255.*(1.-yellow)*(1.-black),
        }
        .into())
    }

    fn fromhex(value: String) -> Result<Handle<Color>, Exeptions> {
        if value.len() != 6 && value.len() != 7 {
            return Err(Exeptions::Incorrectlength(value.len().try_into().expect("length to large")))
        }
        let values: [u8; 4];
        
        if value.starts_with('#') {
            let value = &value[1..value.len()-1];
            values = i32::from_str_radix(value, 16).expect("invalid syntax").to_le_bytes();
        } else {
            values = i32::from_str_radix(value.as_str(), 16).expect("invalid syntax").to_le_bytes();
        }

        assert!(values[3] == 0);
        
        Ok(Self {
            r: values[0] as f64,
            g: values[1] as f64,
            b: values[2] as f64,
        }
        .into())
    }

    fn fromhsl(hue: f64, sateration: f64, lightness: f64) -> Result<Handle<Color>, Exeptions> {
        if hue>360.||hue<0. {
            return Err(Exeptions::Hueoutofrange(hue));
        } else if sateration>1.||sateration<0. {
            return Err(Exeptions::Saterationoutofrange(sateration));
        } else if lightness>1.||lightness<0. {
            return Err(Exeptions::Lightnessoutofrange(lightness));
        }
        
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

        Ok(Self {
            r,
            g,
            b,
        }
        .into())
    }

    fn fromhsv(hue: f64, sateration: f64, value: f64) -> Result<Handle<Color>, Exeptions> {
        if hue>360.||hue<0. {
            return Err(Exeptions::Hueoutofrange(hue));
        } else if sateration>1.||sateration<0. {
            return Err(Exeptions::Saterationoutofrange(sateration));
        } else if value>1.||value<0. {
            return Err(Exeptions::Valueoutofrange(value));
        }
        
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

        Ok(Self {
            r,
            g,
            b,
        }
        .into())
    }

    fn tohex(&self) -> String {
        format!("{:x}{:x}{:x}", self.r as u8, self.g as u8, self.b as u8)
    }

    fn tohsl(&self) -> (f64, f64, f64) {
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

    fn tohsv(&self) -> (f64, f64, f64) {
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

    fn tocmyk(&self) -> (f64, f64, f64, f64) {
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

    fn greyscale(&self) -> (f64, f64, f64) {
        let average = (self.r+self.g+self.b)/3.;
        (average, average, average)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Into<(f64, f64, f64)> for Color {
    fn into(self) -> (f64, f64, f64) {
        (self.r, self.b, self.g)
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(value: (f64, f64, f64)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

impl Error for Exeptions {}

impl Display for Exeptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Incorrectlength(len) => write!(f, "Expctd a length between 6 and 7 (inclusive), but got length {}", len),
            Self::Hueoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 360 but got {}", value),
            Self::Saterationoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 1 but got {}", value),
            Self::Lightnessoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 1 but got {}", value),
            Self::Valueoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 1 but got {}", value),
            Self::Cyanoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 1 but got {}", value),
            Self::Magentaoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 1 but got {}", value),
            Self::Yellowoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 1 but got {}", value),
            Self::Blackoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 1 but got {}", value),
            Self::Redoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 255 but got {}", value),
            Self::Greenoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 255 but got {}", value),
            Self::Blueoutofrange(value) => write!(f, "Expectd a value inbetween 0 and 255 but got {}", value),
        }
    }
}

struct Colors;
impl crate::colors::Colors for Colors {}
