use std::io::Cursor;

pub fn info(contents: Vec<u8>) -> String {
    format!("Smash Ultimate Parameter File\n\n{}", prc_to_yaml(contents))
}

fn prc_to_yaml(contents: Vec<u8>) -> String {
    prc::read_stream(&mut Cursor::new(contents))
        .map(|prc| serde_yaml::to_string(&prc).unwrap_or_else(|_| String::new()))
        .unwrap_or_else(|_| String::new())
}
