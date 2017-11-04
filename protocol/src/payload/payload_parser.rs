pub trait PayloadParser {
    fn new() -> Self;
    fn parse(bytes: Vec<&[u8]>) -> Self;
    fn as_bytes(self) -> Vec<u8>;
    fn length(&self) -> u16;
}