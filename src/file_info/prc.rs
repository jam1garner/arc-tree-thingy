use std::fmt::{Display, Formatter};
use std::io::Cursor;

use prc::{ParamKind, ParamList, ParamStruct};

pub fn info(contents: Vec<u8>) -> String {
    format!("Smash Ultimate Parameter File\n\n{}", prc_to_yaml(contents))
}

fn prc_to_yaml(contents: Vec<u8>) -> String {
    prc::read_stream(&mut Cursor::new(contents))
        .map(|prc| format!("{}", YamlPrc(prc)))
        .unwrap_or_else(|_| String::new())
}

struct YamlPrc(ParamStruct);

impl Display for YamlPrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_struct(&self.0, f, 0)
    }
}

fn fmt_struct(this: &ParamStruct, f: &mut Formatter<'_>, indents: usize) -> std::fmt::Result {
    this.0.iter().try_for_each(|(hash, param)| fmt_item(Some(hash), param, f, indents))
}

fn fmt_list(this: &ParamList, f: &mut Formatter<'_>, indents: usize) -> std::fmt::Result {
    this.0.iter().enumerate().try_for_each(|(i, item)| {
        let label = match item {
            ParamKind::Struct(_) | ParamKind::List(_) => Some(i),
            _ => None
        };

        fmt_item(label, item, f, indents)
    })
}

fn fmt_item(
    label: Option<impl Display>,
    param: &ParamKind,
    f: &mut Formatter<'_>,
    indents: usize,
) -> std::fmt::Result {
    let label = label.map(|label| format!("{}:", label)).unwrap_or_else(String::new);
    match param {
        ParamKind::Struct(s) => {
            writeln!(f, "{}- {}", "    ".repeat(indents), label)?;
            fmt_struct(s, f, indents + 1)
        }
        ParamKind::List(list) => {
            writeln!(f, "{}- {}", "    ".repeat(indents), label)?;
            fmt_list(list, f, indents + 1)
        }
        ParamKind::U8(x) => writeln!(f, "{}- {} {}", "    ".repeat(indents), label, x),
        ParamKind::I8(x) => writeln!(f, "{}- {} {}", "    ".repeat(indents), label, x),
        ParamKind::U16(x) => writeln!(f, "{}- {} {}", "    ".repeat(indents), label, x),
        ParamKind::I16(x) => writeln!(f, "{}- {} {}", "    ".repeat(indents), label, x),
        ParamKind::U32(x) => writeln!(f, "{}- {} {}", "    ".repeat(indents), label, x),
        ParamKind::I32(x) => writeln!(f, "{}- {} {}", "    ".repeat(indents), label, x),
        ParamKind::Float(x) => writeln!(f, "{}- {} {}", "    ".repeat(indents), label, x),
        ParamKind::Bool(x) => writeln!(f, "{}- {} {}", "    ".repeat(indents), label, x),
        ParamKind::Hash(x) => writeln!(f, "{}- {} {}", "    ".repeat(indents), label, x),
        ParamKind::Str(x) => writeln!(f, "{}- {} {}", "    ".repeat(indents), label, x),
    }
}
