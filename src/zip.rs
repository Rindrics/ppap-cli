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
            .unix_permissions(0o755);

        let file_name = std::path::Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");

        zip.start_file(file_name, options)?;
        zip.write_all(&file_content)?;

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
