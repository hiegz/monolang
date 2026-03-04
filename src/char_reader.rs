pub trait CharReader {
    fn read_char(&mut self) -> std::io::Result<Option<char>>;
}
