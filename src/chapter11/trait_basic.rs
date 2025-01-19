use std::io::Write;


trait Write {
    fn write(&mut self, but: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}

fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}
