use std::fs::File as StdFile;
use std::io::copy;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

pub fn create_zip_from_paths(paths: &[impl AsRef<Path>], output_path: &Path) -> anyhow::Result<()> {
    let file = StdFile::create(output_path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    for path in paths {
        let path = path.as_ref();
        if path.is_file() {
            let name = path.file_name().unwrap().to_string_lossy();
            zip.start_file(name.as_ref(), options)?;
            let mut f = StdFile::open(path)?;
            copy(&mut f, &mut zip)?;
        } else if path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    let relative_path = entry_path.strip_prefix(path)?;
                    // make zip path (dir name + rel path)
                    let zip_path = PathBuf::from(path.file_name().unwrap()).join(relative_path);
                    zip.start_file(zip_path.to_string_lossy().as_ref(), options)?;
                    let mut f = StdFile::open(entry_path)?;
                    copy(&mut f, &mut zip)?;
                }
            }
        }
    }
    zip.finish()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_zip() {
        let paths = [std::path::PathBuf::from("dummy.txt")];
        let tmp = std::env::temp_dir().join("dummy.zip");
        let _ = create_zip_from_paths(&paths, &tmp);
    }
}
