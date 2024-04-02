#![allow(unused)]
mod utils;

use utils::*;
use wai_bindgen_rust::Handle;

wai_bindgen_rust::export!("colors.wai");

struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl crate::colors::Color for Color {
    fn new(r: f32, g: f32, b: f32) -> Handle<Color> {
        Self { r, g, b }.into()
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

        k = 1.-max(r,g,b);
        c = (1.-r-k)/(1.-k);
        m = (1.-g-k)/(1.-k);
        y = (1.-b-k)/(1.-k);

        (c, m, y, k)
    }
}

struct Colors;
impl crate::colors::Colors for Colors {}
