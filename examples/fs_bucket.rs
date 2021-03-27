extern crate storage_bucket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut storage = storage_bucket::BucketsList::new();

    storage.push_read_bucket(Box::new(storage_bucket::fs::FsBucket::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/bucket1"
    ))));
    storage.push_read_bucket(Box::new(storage_bucket::fs::FsBucket::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/bucket2"
    ))));

    assert!(storage.has_file("read1"));
    assert!(storage.has_file("read2"));
    assert!(storage.has_file("read/read3"));

    let mut read1 = storage.get_file("read1")?.unwrap();

    let mut buffer = [0; 5];

    read1.read(&mut buffer).unwrap();
    assert_eq!(&buffer[..], b"READ1");
    std::mem::drop(read1);

    let mut read2 = storage.get_file("read2")?.unwrap();
    read2.read(&mut buffer).unwrap();
    assert_eq!(&buffer[..], b"READ2");
    std::mem::drop(read2);

    let mut read3 = storage.get_file("read/read3")?.unwrap();
    read3.read(&mut buffer).unwrap();
    assert_eq!(&buffer[..], b"READ3");
    std::mem::drop(read3);

    Ok(())
}
