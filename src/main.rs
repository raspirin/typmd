use std::fs::File;
use std::io;
use std::io::Read;
use pulldown_cmark::escape::StrWrite;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

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
                    self.write(&format!("html:{}", text))?;
                }
                Event::FootnoteReference(_name) => {
                    // TODO: fix footnote reference
                }
                Event::SoftBreak => self.write(" ")?,
                Event::HardBreak => {
                    if !self.end_newline {
                        self.open_newline()?;
                    }
                    self.open_newline()?;
                }
                Event::Rule => {
                    if self.end_newline {
                        self.write("pagebreak()")?;
                    } else {
                        self.write("\npagebreak()")?;
                    }
                }
                Event::TaskListMarker(_) => {
                    // TODO: fix task list marker
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
                self.open_newline()?;

                for _ in 1..=(level as i32) {
                    self.write("=")?;
                }
                self.write(" ")?;
            }
            Tag::BlockQuote => {
                if self.end_newline {
                    self.write("#rect(fill: gray)[")?;
                } else {
                    self.write("\n#rect(fill: gray)[")?;
                }
            }
            Tag::CodeBlock(kind) => {
                if !self.end_newline {
                    self.open_newline()?;
                }
                self.open_newline()?;

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
            Tag::List(Some(start)) => {
                if !self.end_newline {
                    self.open_newline()?;
                }
                self.open_newline()?;

                self.write(&format!("#enum(start: {})", start))?;
            }
            Tag::List(None) => {
                if !self.end_newline {
                    self.open_newline()?;
                }
                self.open_newline()?;

                self.write("#list")?;
            }
            Tag::Item => {
                self.write("[")?;
            }
            Tag::FootnoteDefinition(_) => {}
            Tag::Table(_) => {}
            Tag::TableHead => {}
            Tag::TableRow => {}
            Tag::TableCell => {}
            Tag::Emphasis => self.write("_")?,
            Tag::Strong => self.write("*")?,
            Tag::Strikethrough => self.write("#strike[")?,
            Tag::Link(_, dest, _) => self.write(&format!("#link(\"{dest}\")["))?,
            Tag::Image(_, dest, title) => {
                self.write("#figure[")?;
                self.write(&format!("#image(\"{dest}\", alt: \""))?;
            }
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
            Tag::BlockQuote => {
                self.write("]\n")?;
            }
            Tag::CodeBlock(_) => {
                if !self.end_newline {
                    self.open_newline()?;
                }
                self.write("```\n")?;
            }
            Tag::List(_) => {
                self.open_newline()?;
            }
            Tag::Item => {
                self.write("]")?;
            }
            Tag::FootnoteDefinition(_) => {}
            Tag::Table(_) => {}
            Tag::TableHead => {}
            Tag::TableRow => {}
            Tag::TableCell => {}
            Tag::Emphasis => self.write("_")?,
            Tag::Strong => self.write("*")?,
            Tag::Strikethrough => self.write("]")?,
            Tag::Link(_, _, _) => self.write("]")?,
            Tag::Image(_, _, title) => {
                self.write("\")")?;

                if !title.is_empty() {
                    self.write(&format!(", caption: [{title}]"))?;
                }

                self.write("]")?;
            }
        }

        Ok(())
    }
}


fn main() {
    let mut file = File::open("test/in.md").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let opt = Options::all();
    let parser = Parser::new_ext(&input, opt);

    let mut output = String::new();
    let mut tpy = Typ::new(parser, &mut output);
    tpy.run().unwrap();

    print!("{output}");
}
