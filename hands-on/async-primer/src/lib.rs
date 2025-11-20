use std::fs;
use std::io;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

#[cfg(feature = "async-tokio")]
pub async fn read_file_async<P: AsRef<Path>>(path: P) -> io::Result<String> {
    tokio::fs::read_to_string(path).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn read_file_reads_contents() {
        let mut path: PathBuf = std::env::temp_dir();
        path.push("test_sync_read.txt");
        fs::write(&path, "hello sync").expect("write tmp");
        let got = read_file(&path).expect("read file");
        assert_eq!(got, "hello sync");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn read_file_missing_error() {
        let mut path: PathBuf = std::env::temp_dir();
        path.push("test_12345.txt");
        let res = read_file(&path);
        assert!(res.is_err());
    }
}

// Async unit tests: only compiled when running `cargo test --features async-tokio`
#[cfg(all(test, feature = "async-tokio"))]
mod async_tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[tokio::test]
    async fn read_file_async_reads_contents() {
        let mut path: PathBuf = std::env::temp_dir();
        path.push("test_async_read.txt");
        fs::write(&path, "hello async").expect("write tmp");
        let got = read_file_async(&path).await.expect("read file async");
        assert_eq!(got, "hello async");
        let _ = fs::remove_file(path);
    }
}