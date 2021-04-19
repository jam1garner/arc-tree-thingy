use csb::CsbFile;
use std::io::Cursor;

pub fn info(contents: Vec<u8>) -> String {
    "Common Sound Table File\n".to_string() +
        &CsbFile::read(&mut Cursor::new(contents))
            .map(|sli| serde_yaml::to_string(&sli).unwrap_or_else(|_| String::new()))
            .unwrap_or_else(|_| String::new())
}
