use std::fs::File;

pub fn get_logfile(path: String) -> std::io::Result<File> {
    std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .append(true)
        .open(path)
}
