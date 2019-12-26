pub fn file_clear(path: &str) -> Result<(), std::io::Error> {
    std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .map(|_| ())
}

pub fn open_or_create_file(path: &str) -> Result<std::fs::File, std::io::Error> {
    std::fs::OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open(path)
}
