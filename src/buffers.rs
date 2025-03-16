use crossterm::cursor;
use crossterm::style::{style, Color, PrintStyledContent, Stylize};
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};

use crate::pixel::Pixel;

pub struct FrameBuffer {
    width: u16,
    height: u16,
    f_buf: Vec<Pixel>,
    b_buf: Vec<Pixel>,
    changes: Vec<i32>,
}

impl FrameBuffer {
    pub fn new(height: u16, width: u16) -> Self {
        let size = (height * width) as usize;
        Self {
            width,
            height,
            f_buf: vec![Pixel::new(' ', Color::Reset, Color::Reset); size],
            b_buf: vec![Pixel::new(' ', Color::Reset, Color::Reset); size],
            changes: Vec::new(),
        }
    }
    pub fn swapbuf(&mut self) {
        for &i in self.changes.iter() {
            let index = i as usize;
            if self.f_buf[index] != self.b_buf[index] {
                self.f_buf[index] = self.b_buf[index];
            }
        }
        self.changes.clear();
        self.clear();
    }

    pub fn flush(&self, stdout: &mut Stdout) -> std::io::Result<()> {
        for (i, val) in self.f_buf.iter().enumerate() {
            let x = (i % self.width as usize) as u16;
            let y = (i / self.width as usize) as u16;
            stdout
                .queue(cursor::MoveTo(x, y))?
                .queue(PrintStyledContent(style(
                    val.ch().with(val.fg()).on(val.bg()),
                )))?;
        }
        stdout.flush()?;
        Ok(())
    }
    pub fn draw_line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        let dx: i16 = x1 as i16 - x0 as i16;
        let dy: i16 = y1 as i16 - y0 as i16;
        let mut d: i16 = 2 * dy - dx;
        let mut y = y0;

        for x in x0..=x1 {
            self.draw_point(x, y, Pixel::default());
            if d > 0 {
                y += 1;
                d -= 2 * dx;
            } else {
                d += 2 * dy
            }
        }
    }
    pub fn clear(&mut self) {
        for i in 0..self.b_buf.len() {
            self.b_buf[i] = Pixel::new(' ', Color::Reset, Color::Reset);
            self.changes.push(i as i32);
        }
    }

    fn draw_point(&mut self, x: u16, y: u16, px: Pixel) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = y * self.width + x;
        self.changes.push(index as i32);
        self.b_buf[index as usize] = px;
    }
}
