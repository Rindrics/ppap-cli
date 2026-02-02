use anyhow::{Result, Context};
use std::fs;
use std::io::Write;
use zip::write::FileOptions;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

struct ZipBuilder {
    temp_path: String,
    password: String,
}

impl ZipBuilder {
    fn new(original_path: &str) -> Self {
        let temp_path = format!("{}.zip", original_path);
        let password: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        Self {
            temp_path,
            password,
        }
    }

    fn create_archive(&self, file_path: &str) -> Result<()> {
        let file_content = fs::read(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path))?;

        let zip_file = fs::File::create(&self.temp_path)
            .with_context(|| "Failed to create zip file")?;

        let mut zip = zip::ZipWriter::new(zip_file);
        let options: FileOptions<'_, ()> = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .with_aes_encryption(zip::AesMode::Aes256, &self.password)
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

    fn get_password(&self) -> &str {
        &self.password
    }
}

pub fn compress_file(file_path: &str) -> Result<(String, String)> {
    let builder = ZipBuilder::new(file_path);
    builder.create_archive(file_path)?;
    Ok((builder.get_path().to_string(), builder.get_password().to_string()))
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

        let (zip_path, password) = compress_file(&test_file)?;
        println!("Generated password: {}", password);

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
    fn test_unique_passwords() -> Result<()> {
        let temp_dir = tempdir()?;
        let test_file = create_test_file(
            temp_dir.path(),
            "test.txt",
            b"test content"
        )?;

        let mut passwords = vec![];
        for _ in 0..5 {
            let (zip_path, password) = compress_file(&test_file)?;
            passwords.push(password);
            cleanup_temp_file(&zip_path)?;
        }

        let unique_passwords: std::collections::HashSet<_> = passwords.iter().collect();
        assert_eq!(unique_passwords.len(), passwords.len(), "Some passwords were not unique");

        Ok(())
    }
}
