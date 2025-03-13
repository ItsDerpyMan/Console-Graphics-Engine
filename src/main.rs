use std::io::stdout;
use std::io::Stdout;

use buffers::FrameBuffer;
use crossterm::terminal::size;

mod buffers;
mod pixel;

fn main() {
    let mut stdout: Stdout = stdout();

    let (width, height) = size().unwrap();
    let mut screen = FrameBuffer::new(height as usize, width as usize);

    loop {
        screen.draw_line(0, 0, 5, 5);
        screen.swapbuf();
        screen.flush(&mut stdout).unwrap();
    }
}
