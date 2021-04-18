use nus3audio::Nus3audioFile;
use humansize::{FileSize, file_size_opts as options};

macro_rules! fmt_lit {
    () => {
r#"Namco Audio Container

Files:
{}"#
    };
}

pub fn info(contents: Vec<u8>) -> String {
    format!(
        fmt_lit!(),
        file_list(contents)
    )
}

fn file_list(contents: Vec<u8>) -> String {
    Nus3audioFile::from_bytes(&contents)
        .files
        .into_iter()
        .map(|file| format!(
            "[{:2}] {} ({})\n",
            file.id,
            file.filename(),
            file.data.len().file_size(options::BINARY).unwrap()
        ))
        .collect()
}
