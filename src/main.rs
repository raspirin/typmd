use std::fs::File;
use std::io;
use std::io::Read;
use pulldown_cmark::escape::StrWrite;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};

fn fix_lang(raw: &str) -> String {
    let mut new = raw.to_lowercase();

    if new == "c++".to_string() {
        new = "cxx".to_string();
    }

    new
}

struct Typ<I, W> {
    iter: I,
    writer: W,
    end_newline: bool,
}

impl<'a, I, W> Typ<I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: StrWrite
{
    pub fn new(iter: I, writer: W) -> Self {
        Self {
            iter, writer,
            end_newline: true,
        }
    }

    #[inline]
    fn write(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_str(s)?;

        if !s.is_empty() {
            self.end_newline = s.ends_with('\n');
        }

        Ok(())
    }

    #[inline]
    fn open_newline(&mut self) -> io::Result<()> {
        self.end_newline = true;
        self.writer.write_str("\n")
    }

    pub fn run(&mut self) -> io::Result<()> {
        while let Some(event) = self.iter.next() {
            match event {
                Event::Start(tag) => self.start_tag(tag)?,
                Event::End(tag) => self.end_tag(tag)?,
                Event::Text(text) => self.write(&text)?,
                Event::Code(text) => {
                    self.write("`")?;
                    self.write(&text)?;
                    self.write("`")?;
                }
                Event::Html(text) => {
                    // TODO: fix html
                    self.write(&text)?;
                }
                Event::FootnoteReference(name) => {
                    // TODO: fix footnote reference
                    self.write(&format!("footnote_ref:{}", &name))?;
                }
                Event::SoftBreak => self.open_newline()?,
                Event::HardBreak => {
                    if self.end_newline {
                        self.write("parbreak()")?;
                    } else {
                        self.write("\nparbreak()")?;
                    }
                }
                Event::Rule => {
                    if self.end_newline {
                        self.write("pagebreak()")?;
                    } else {
                        self.write("\npagebreak()")?;
                    }
                }
                Event::TaskListMarker(yes) => {
                    // TODO: fix task list marker
                    self.write(&format!("checkbox:{}", yes))?;
                }
            }
        }

        Ok(())
    }

    fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                if !self.end_newline {
                    self.write("\n")?;
                }
            }
            Tag::Heading(level, _, _) => {
                if self.end_newline {
                    self.end_newline = false;
                } else {
                    self.write("\n")?;
                }

                for _ in 1..=(level as i32) {
                    self.write("=")?;
                }
                self.write(" ")?;
            }
            Tag::BlockQuote => {}
            Tag::CodeBlock(kind) => {
                if !self.end_newline {
                    self.open_newline()?;
                }

                match kind {
                    CodeBlockKind::Indented => {
                        self.write("```\n")?;
                    }
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap();
                        let lang = fix_lang(lang);

                        self.write("```")?;
                        if lang.is_empty() {
                            self.write("\n")?;
                        } else {
                            self.write(&format!("{}\n", lang))?;
                        }
                    }
                }
            }
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

    fn end_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                self.write("\n")?;
            }
            Tag::Heading(_, _, _) => {
                self.write("\n")?;
            }
            Tag::BlockQuote => {}
            Tag::CodeBlock(_) => {
                if !self.end_newline {
                    self.open_newline()?;
                }
                self.write("```\n")?;
            }
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
    let mut file = File::open("test/in.md").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let parser = Parser::new(&input);

    let mut output = String::new();
    let mut tpy = Typ::new(parser, &mut output);
    tpy.run().unwrap();

    print!("{output}");
}
