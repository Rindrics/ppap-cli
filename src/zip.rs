use anyhow::{Result, Context};
use std::fs;
use std::io::Write;
use zip::write::FileOptions;

struct ZipBuilder {
    temp_path: String,
}

impl ZipBuilder {
    fn new(original_path: &str) -> Self {
        let temp_path = format!("{}.zip", original_path);
        Self { temp_path }
    }

    fn create_archive(&self, file_path: &str) -> Result<()> {
        let file_content = fs::read(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path))?;

        let zip_file = fs::File::create(&self.temp_path)
            .with_context(|| "Failed to create zip file")?;

        let mut zip = zip::ZipWriter::new(zip_file);
        let options: FileOptions<'_, ()> = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .with_aes_encryption(zip::AesMode::Aes256, "password")
            .unix_permissions(0o755);

        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");

        zip.start_file(file_name, options)?;
        zip.write_all(&file_content)?;
        zip.finish()?;

        Ok(())
    }

    fn get_path(&self) -> &str {
        &self.temp_path
    }
}

pub fn compress_file(file_path: &str) -> Result<String> {
    let builder = ZipBuilder::new(file_path);
    builder.create_archive(file_path)?;
    Ok(builder.get_path().to_string())
}

pub fn cleanup_temp_file(temp_path: &str) -> Result<()> {
    fs::remove_file(temp_path)
        .with_context(|| "Failed to remove temporary zip file")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    fn create_test_file(dir: &std::path::Path, filename: &str, content: &[u8]) -> Result<String> {
        let file_path = dir.join(filename);
        let mut file = File::create(&file_path)
            .with_context(|| "Failed to create test file")?;
        file.write_all(content)?;
        Ok(file_path.to_string_lossy().into_owned())
    }

    #[test]
    fn test_encrypted_compression() -> Result<()> {
        let temp_dir = tempdir()?;

        let test_content = b"This is a test content that will be encrypted in the ZIP file.";
        let test_file = create_test_file(
            temp_dir.path(),
            "secure.txt",
            test_content
        )?;

        let zip_path = compress_file(&test_file)?;

        assert!(std::path::Path::new(&zip_path).exists());

        let zip_metadata = fs::metadata(&zip_path)?;
        assert!(zip_metadata.len() > test_content.len() as u64);

        cleanup_temp_file(&zip_path)?;

        Ok(())
    }

    #[test]
    fn test_file_not_found() {
        let result = compress_file("nonexistent.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_cleanup_nonexistent_file() {
        let result = cleanup_temp_file("nonexistent.zip");
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_compressions() -> Result<()> {
        let temp_dir = tempdir()?;

        let files = vec![
            ("file1.txt", b"Content 1"),
            ("file2.txt", b"Content 2"),
            ("file3.txt", b"Content 3"),
        ];

        let mut zip_paths = Vec::new();

        for (filename, content) in files {
            let file_path = create_test_file(temp_dir.path(), filename, content)?;
            let zip_path = compress_file(&file_path)?;
            assert!(std::path::Path::new(&zip_path).exists());
            zip_paths.push(zip_path);
        }

        for zip_path in zip_paths {
            cleanup_temp_file(&zip_path)?;
        }

        Ok(())
    }
}
