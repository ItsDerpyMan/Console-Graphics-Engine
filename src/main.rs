use std::io::stdout;
use std::io::Stdout;

use buffers::FrameBuffer;
use crossterm::cursor;
use crossterm::cursor::Hide;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, size};
use crossterm::QueueableCommand;

mod buffers;
mod pixel;

fn main() -> std::io::Result<()> {
    let mut stdout: Stdout = stdout();

    let (width, height) = size().unwrap();
    let mut screen = FrameBuffer::new(height, width);

    disable_raw_mode().unwrap();
    let _ = stdout.queue(Hide);

    loop {
        for val in 0..150 {
            screen.draw_line(0, 0, 150, val as u16);
            screen.swapbuf();
            stdout.queue(cursor::MoveTo(0, 100)).unwrap();
            match stdout.queue(Print(val)) {
                Ok(_) => {}
                Err(e) => eprintln!("Error: {}", e),
            }
            screen.flush(&mut stdout).unwrap();
        }
        screen.clear();
        screen.swapbuf();
        screen.flush(&mut stdout).unwrap()
    }
}
