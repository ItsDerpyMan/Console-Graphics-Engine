use crossterm::style::Color;
//use rand::prelude::*;

#[derive(Clone, Eq, PartialEq, Copy)]
pub struct Pixel {
    ch: char,
    bg: Color,
    fg: Color,
}
impl Pixel {
    pub fn new(ch: char, bg: Color, fg: Color) -> Self {
        Self { ch, bg, fg }
    }
    // pub fn random(rng: &mut impl Rng) -> Self {
    //     Self {
    //         ch: rng.sample(Alphanumeric) as char,
    //         bg: Color::Rgb(rng.gen(), rng.gen(), rng.gen()),
    //         fg: Color::Rgb(rng.gen(), rng.gen(), rng.gen()),
    //     }
    // }
}
impl Pixel {
    pub fn ch(&self) -> char {
        self.ch
    }

    pub fn fg(&self) -> Color {
        self.fg
    }

    pub fn bg(&self) -> Color {
        self.bg
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            ch: ' ',
            bg: Color::Reset,
            fg: Color::Reset,
        }
    }
}
