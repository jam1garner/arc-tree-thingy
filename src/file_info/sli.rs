use sli::SliFile;
use std::io::Cursor;

use prc::hash40::Hash40;

pub fn info(contents: Vec<u8>) -> String {
    "Sound Label Info File\n\n".to_string() +
        &SliFile::read(&mut Cursor::new(contents))
            .map(|sli| sli.entries().iter().map(|entry| {
                format!(
                    "- {}\n    - nus3bank_id: {:#x?}\n    - tone_id: {:#x?}\n",
                    Hash40(entry.tone_name),
                    entry.nus3bank_id,
                    entry.tone_id,
                )
            }).collect())
            .unwrap_or_else(|_| String::new())
}
