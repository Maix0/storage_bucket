use std::{fs::File, path::Path};
use zip_crate::{result::ZipResult, ZipArchive};

use super::{zip_crate, ReadBucket};

#[derive(Debug)]
pub struct ZipBucket {
    zip_file: ZipArchive<File>,
}

impl ZipBucket {
    pub fn new(zip_file: impl AsRef<Path>) -> ZipResult<Self> {
        let path = zip_file.as_ref();
        let zip_file = ZipArchive::new(File::open(path)?)?;
        Ok(Self { zip_file })
    }

    fn transform_path<'a>(path: &'a Path) -> &'a Path {
        if path.starts_with("/") {
            path.strip_prefix("/").unwrap()
        } else {
            path
        }
    }
}

impl ReadBucket for ZipBucket {
    fn has_file(&self, path: &Path) -> bool {
        let new_path = Self::transform_path(path);
        let filenames = self
            .zip_file
            .file_names()
            .find(|&p| Some(p) == new_path.to_str());
        filenames.is_some()
    }

    fn get_file(
        &mut self,
        path: &Path,
    ) -> Result<Option<Box<dyn std::io::Read + '_>>, Box<dyn std::error::Error>> {
        let new_path = Self::transform_path(path);

        match &new_path.to_str() {
            Some(name) => {
                let res = self.zip_file.by_name(name);
                match res {
                    Err(zip_crate::result::ZipError::FileNotFound) => return Ok(None),
                    Ok(e) => return Ok(Some(Box::new(e))),
                    Err(e) => return Err(Box::new(e)),
                }
            }
            None => Ok(None),
        }
    }
}
