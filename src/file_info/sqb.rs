pub fn info(data: Vec<u8>) -> String {
    sqb::from_stream(&mut std::io::Cursor::new(data))
        .map(|sqb| {
            format!("Sound Sequence Data File\n{}", serde_yaml::to_string(&sqb).unwrap())
        })
        .unwrap_or_else(|_| String::from("Sound Sequence Data File"))
}
