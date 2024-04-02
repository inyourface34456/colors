#![allow(unused)]
use wai_bindgen_rust::Handle;

wai_bindgen_rust::export!("colors.wai");

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl crate::colors::Color for Color {
    fn new(r: u8, g: u8, b: u8) -> Handle<Color> {
        Self {r, g, b}.into()
    }
}

struct Colors;
impl crate::colors::Colors for Colors {}
