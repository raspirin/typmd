use std::io;
use pulldown_cmark::escape::StrWrite;
use pulldown_cmark::{Event, Tag};

struct Typ<'a, I, W> {
    iter: I,
    writer: W
}

impl<'a, I, W> Typ<'a, I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: StrWrite
{
    pub fn new(iter: I, writer: W) -> Self {
        Self {
            iter, writer,
        }
    }

    pub fn run(&mut self, s: &str) -> io::Result<()> {
        while let Some(event) = self.iter.next() {
            match event {
                Event::Start(_) => {}
                Event::End(_) => {}
                Event::Text(_) => {}
                Event::Code(_) => {}
                Event::Html(_) => {}
                Event::FootnoteReference(_) => {}
                Event::SoftBreak => {}
                Event::HardBreak => {}
                Event::Rule => {}
                Event::TaskListMarker(_) => {}
            }
        }

        Ok(())
    }

    fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {}
            Tag::Heading(_, _, _) => {}
            Tag::BlockQuote => {}
            Tag::CodeBlock(_) => {}
            Tag::List(_) => {}
            Tag::Item => {}
            Tag::FootnoteDefinition(_) => {}
            Tag::Table(_) => {}
            Tag::TableHead => {}
            Tag::TableRow => {}
            Tag::TableCell => {}
            Tag::Emphasis => {}
            Tag::Strong => {}
            Tag::Strikethrough => {}
            Tag::Link(_, _, _) => {}
            Tag::Image(_, _, _) => {}
        }

        Ok(())
    }
}


fn main() {
    println!("Hello, world!");
}
