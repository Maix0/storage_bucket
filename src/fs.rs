use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
/// A ReadBucket that will read the file from disk with the given path as a base
pub struct FsBucket {
    base_path: std::path::PathBuf,
}

impl FsBucket {
    /// Create a new `FsBucket` with the given base path
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    // Transform the given path to a relative path and create a new path with the base path
    fn transform_path(&self, path: &Path) -> PathBuf {
        let mut base_path = self.base_path.clone();
        if path.starts_with("/") {
            base_path.push(path.strip_prefix("/").unwrap())
        } else {
            base_path.push(path)
        }
        base_path
    }
}

impl super::ReadBucket for FsBucket {
    fn has_file(&self, path: &Path) -> bool {
        let new_path = self.transform_path(path);
        return new_path.exists() && new_path.is_file();
    }

    fn get_file(
        &mut self,
        path: &Path,
    ) -> Result<Option<Box<dyn std::io::Read + '_>>, Box<dyn std::error::Error>> {
        if self.has_file(path) {
            let path = self.transform_path(path);
            let file = std::fs::File::open(&path)?;
            return Ok(Some(Box::new(file)));
        } else {
            return Ok(None);
        }
    }
}
