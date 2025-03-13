use crossterm::cursor;
use crossterm::style::{style, PrintStyledContent, Stylize};
use crossterm::QueueableCommand;
use std::io::{Result, Stdout, Write};

use crate::pixel::Pixel;
use crossterm::style::Color;

pub struct FrameBuffer {
    width: usize,
    height: usize,
    f_buf: Vec<Pixel>,
    b_buf: Vec<Pixel>,
    changes: Vec<i32>,
}

impl FrameBuffer {
    pub fn new(height: usize, width: usize) -> Self {
        let size = height * width;
        Self {
            width,
            height,
            f_buf: vec![Pixel::new(' ', Color::Black, Color::DarkRed); size],
            b_buf: vec![Pixel::new(' ', Color::Black, Color::DarkRed); size],
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
    }

    pub fn flush(&self, stdout: &mut Stdout) -> std::io::Result<()> {
        for (i, val) in self.f_buf.iter().enumerate() {
            let x = (i % self.width) as u16;
            let y = (i / self.width) as u16;
            stdout
                .queue(cursor::MoveTo(x, y))?
                .queue(PrintStyledContent(style(
                    val.ch().with(val.fg()).on(val.bg()),
                )))?;
        }
        stdout.flush()?;
        Ok(())
    }
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let dx: i32 = x1 as i32 - x0 as i32;
        let dy: i32 = y1 as i32 - y0 as i32;
        let mut d: i32 = 2 * dy - dx;
        let mut y = y0 as i32;

        for x in x0..=x1 {
            self.draw_point(x, y as usize, Pixel::default());
            if d > 0 {
                y += 1;
                d -= 2 * dx;
            } else {
                d += 2 * dy
            }
        }
    }

    fn draw_point(&mut self, x: usize, y: usize, px: Pixel) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = y * self.width + x;
        self.changes.push(index as i32);
        self.b_buf[index] = px;
    }
}
