use nutexb::parser::NutexbFile;
use std::io::Cursor;

macro_rules! fmt_lit {
    () => {
r#"Namco Texture v{}.{}

Internal name: {:?}

Size: {}x{}
Depth: {}
Mips: {}
"#
    };
}

pub fn info(contents: Vec<u8>) -> String {
    let mut contents = Cursor::new(contents);
    
    match NutexbFile::parse(&mut contents) {
        Ok(tex) => {
            let tex = tex.footer;
            format!(
                fmt_lit!(),
                tex.version.0,
                tex.version.1,
                tex.string.strip_prefix(" XNT").unwrap_or(&tex.string),
                tex.width,
                tex.height,
                tex.depth,
                tex.mip_count,
            )
        }
        _ => format!("Namco Texture")
    }
}
