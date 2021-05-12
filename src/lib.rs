extern crate crossterm;
extern crate futures;
extern crate unicode_segmentation;
extern crate unicode_width;

use std::{
    io::{stdout, Write},
    pin::Pin,
    task::{Context, Poll},
};

use crossterm::{
    event::{Event, EventStream},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::Stream;
use unicode_segmentation::UnicodeSegmentation;

pub use crossterm::Result;

pub struct ReadLines {
    stream: EventStream,
    buffer: String,
}

impl ReadLines {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        Ok(Self {
            stream: EventStream::new(),
            buffer: String::new(),
        })
    }
}

impl Drop for ReadLines {
    fn drop(&mut self) {
        disable_raw_mode().ok();
    }
}

fn print_flush<S: AsRef<str>>(string: S) -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout.write(string.as_ref().as_bytes())?;
    stdout.flush()
}

macro_rules! print_flush {
    ($($arg:tt)*) => (print_flush(format!($($arg)*)).unwrap());
}

fn delete_string<S: AsRef<str>>(string: S) -> std::io::Result<()> {
    let len = unicode_width::UnicodeWidthStr::width(string.as_ref());
    let bs = "\x08".repeat(len);
    let sp = " ".repeat(len);
    print_flush(format!("{}{}{}", bs, sp, bs))
}

fn pop_grapheme(s: &mut String) -> Option<String> {
    match s.graphemes(true).last().map(std::borrow::ToOwned::to_owned) {
        None => None,
        Some(g) => {
            s.replace_range((s.len() - g.len()).., "");
            Some(g)
        }
    }
}

impl Stream for ReadLines {
    type Item = Result<String>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<<Self as futures::Stream>::Item>> {
        let this = Pin::get_mut(self);

        loop {
            match Pin::new(&mut this.stream).poll_next(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => {
                    if this.buffer.is_empty() {
                        return Poll::Ready(None);
                    } else {
                        let res = this.buffer.clone();
                        this.buffer.clear();
                        return Poll::Ready(Some(Ok(res)));
                    }
                }
                Poll::Ready(Some(Err(err))) => return Poll::Ready(Some(Err(err))),
                Poll::Ready(Some(Ok(Event::Key(key)))) => match key.code {
                    crossterm::event::KeyCode::Backspace => {
                        pop_grapheme(&mut this.buffer).map(|x| delete_string(x).unwrap());
                    }
                    crossterm::event::KeyCode::Enter => {
                        let res = this.buffer.clone();
                        this.buffer.clear();
                        print_flush!("\r\n");
                        return Poll::Ready(Some(Ok(res)));
                    }
                    crossterm::event::KeyCode::Tab => {
                        this.buffer.push('\t');
                        print_flush!("\t");
                    }
                    crossterm::event::KeyCode::Char(c) => {
                        this.buffer.push(c);
                        print_flush!("{}", c);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
