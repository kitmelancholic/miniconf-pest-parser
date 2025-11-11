use std::io::Write;

use tempfile::NamedTempFile;

/// Returns a representative MiniConf document used throughout the tests.
pub fn sample_document() -> &'static str {
    r#"
    title = Example
    enabled = true

    [database]
    host = localhost
    tags = [primary, read]
    "#
}

/// Writes `contents` to a temporary file and returns the handle.
pub fn write_temp_file(contents: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("temp file");
    file.write_all(contents.as_bytes())
        .expect("write temp file");
    file.flush().expect("flush");
    file
}
