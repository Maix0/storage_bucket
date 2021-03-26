extern crate storage_bucket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zip_bucket = storage_bucket::zip::ZipBucket::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/zip_bucket.zip"
    ))?;
    let mut storage = storage_bucket::StorageBuckets::new();
    storage.push_read_bucket(Box::new(zip_bucket));

    assert!(storage.has_file("zip1"));

    let mut buffer = [0; 5];
    let mut res = storage.get_file("/zip1")?.unwrap();

    res.read(&mut buffer)?;

    assert_eq!(&buffer[..], b"_ZIP1");

    Ok(())
}
