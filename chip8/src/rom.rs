pub struct Rom {
    content: Vec<u8>,
}

impl Rom {
    pub fn new(file_path: String) -> Rom {
        // TODO: add exception instead of panic
        let content = std::fs::read(file_path).unwrap();
        Rom { content }
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }
}
