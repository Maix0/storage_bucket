pub mod fs;
#[cfg(feature = "zip")]
extern crate zip as zip_crate;
#[cfg(feature = "zip")]
pub mod zip;

use std::path::Path;

pub trait ReadBucket {
    fn has_file(&self, path: &Path) -> bool;
    fn get_file(
        &mut self,
        path: &Path,
    ) -> Result<Option<Box<dyn std::io::Read + '_>>, Box<dyn std::error::Error>>;
}

pub struct StorageBuckets {
    inner_read: Vec<Box<dyn ReadBucket>>,
}

impl StorageBuckets {
    pub fn new() -> Self {
        Self {
            inner_read: Vec::new(),
        }
    }

    pub fn push_read_bucket(&mut self, bucket: Box<dyn ReadBucket>) {
        self.inner_read.push(bucket)
    }

    pub fn has_file<A: AsRef<Path>>(&self, path: A) -> bool {
        for bucket in &self.inner_read {
            if bucket.has_file(path.as_ref()) {
                return true;
            }
        }
        return false;
    }
    pub fn get_file(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<Option<Box<dyn std::io::Read + '_>>, Box<dyn std::error::Error>> {
        for bucket in &mut self.inner_read {
            let maybe_file = bucket.get_file(path.as_ref())?;
            if maybe_file.is_some() {
                return Ok(maybe_file);
            }
        }
        return Ok(None);
    }
}
