use crossterm::cursor;
use crossterm::style::{style, Color, PrintStyledContent, Stylize};
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};
use std::mem::MaybeUninit;

use crate::pixel::Pixel;

pub struct FrameBuffer {
    width: usize,
    height: usize,
    f_buf: Box<[MaybeUninit<Pixel>]>,
    b_buf: Box<[MaybeUninit<Pixel>]>,
    f_changes: Vec<usize>,
    b_changes: Vec<usize>,
}

impl FrameBuffer {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            width,
            height,
            f_buf: Self::new_uninit_buffer(width, height),
            b_buf: Self::new_uninit_buffer(width, height),
            f_changes: Vec::new(),
            b_changes: Vec::new(),
        }
    }

    fn new_uninit_buffer(width: usize, height: usize) -> Box<[MaybeUninit<Pixel>]> {
        let size = width * height;
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
            buf.into_boxed_slice().assume_init()
        }
    }

    pub fn swapbuf(&mut self) {
        for &index in self.b_changes.iter() {
            // Alloncating from the b_buf[index] if we can assume it's initialized.
            // unsafe
            self.f_buf[index].write(unsafe { self.b_buf[index].assume_init() });
            // clearing the b_buf Pixel
            self.b_buf[index].write(Pixel::default());
            // pushing the b_changes to f_changes before clearing it.
            self.f_changes.push(index);
        }
        // clearing the b_changes.
        self.b_changes.clear();
    }

    pub fn flush(&mut self, stdout: &mut Stdout) -> std::io::Result<()> {
        // Assuming that all the f_buf changes are properly initialized.
        for &i in self.f_changes.iter() {
            let x = (i % self.width) as u16;
            let y = (i / self.width) as u16;

            // Safely access Pixel and its fields
            let pixel = unsafe { self.f_buf[i].assume_init_ref() };

            // Ensure the `char` is wrapped into a `StyledContent<char>`
            let styled_char = style(pixel.ch()).with(pixel.fg()).on(pixel.bg());

            stdout
                .queue(cursor::MoveTo(x, y))?
                .queue(PrintStyledContent(styled_char))?;
        }
        stdout.flush()?;
        self.f_changes.clear();

        Ok(())
    }

    pub fn draw_line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        let dx: i16 = x1 as i16 - x0 as i16;

        let dy: i16 = y1 as i16 - y0 as i16;
        let mut d: i16 = 2 * dy - dx;
        let mut y = y0;

        for x in x0..=x1 {
            self.draw_point(
                x as usize,
                y as usize,
                Pixel::new(' ', Color::White, Color::White),
            );
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
            self.b_buf[i].write(Pixel::default());
            unsafe { self.b_buf[i].assume_init() };
            self.b_changes.push(i);
        }
    }

    fn draw_point(&mut self, x: usize, y: usize, px: Pixel) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = y * self.width + x;
        self.b_changes.push(index);
        self.b_buf[index].write(px);
        unsafe { self.b_buf[index].assume_init() };
    }
}
