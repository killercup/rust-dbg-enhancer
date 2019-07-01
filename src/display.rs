use self::pad::PadAdapter;
use crate::ast::*;
use std::fmt::{self, Write};

impl<'src> fmt::Display for Source<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.item)
    }
}

impl<'src> fmt::Display for Item<'src> {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Array(Array { items }) => {
                if items.is_empty() {
                    write!(f, "[]")?;
                    return Ok(());
                }
                write!(f, "[\n")?;
                let mut writer = PadAdapter::wrap(&mut f);
                for item in items {
                    writer.write_fmt(format_args!("{},\n", item))?;
                }
                write!(f, "]")?;
                Ok(())
            }
            Item::Structure(Structure { name, fields }) => {
                if fields.is_empty() {
                    write!(f, "{{}}")?;
                    return Ok(());
                }

                write!(f, "{} {{\n", name)?;
                let mut writer = PadAdapter::wrap(&mut f);
                for StructureField { name, value } in fields {
                    writer.write_fmt(format_args!("{}: {},\n", name, value))?;
                }
                write!(f, "}}")?;
                Ok(())
            }
            Item::String(s) => write!(f, "\"{}'\"", s.content),
            Item::Ident(s) => write!(f, "{}", s.content),
        }
    }
}

impl<'src> fmt::Display for Ident<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

// copied from libcore's `fmt/builders.rs`
pub(crate) mod pad {
    use std::fmt;

    pub(crate) struct PadAdapter<'a> {
        buf: &'a mut (dyn fmt::Write + 'a),
        on_newline: bool,
    }

    impl<'a> PadAdapter<'a> {
        pub(crate) fn wrap<'b, 'c: 'a + 'b>(buf: &'c mut fmt::Formatter<'_>) -> Self {
            PadAdapter { buf, on_newline: true }
        }
    }

    impl fmt::Write for PadAdapter<'_> {
        fn write_str(&mut self, mut s: &str) -> fmt::Result {
            while !s.is_empty() {
                if self.on_newline {
                    self.buf.write_str("    ")?;
                }

                let split = match s.find('\n') {
                    Some(pos) => {
                        self.on_newline = true;
                        pos + 1
                    }
                    None => {
                        self.on_newline = false;
                        s.len()
                    }
                };
                self.buf.write_str(&s[..split])?;
                s = &s[split..];
            }

            Ok(())
        }
    }
}
