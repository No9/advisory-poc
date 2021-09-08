use std::fs::File;
use std::{thread, time};
use advisory_lock::{AdvisoryFileLock, FileLockMode, FileLockError};

fn main() -> Result<(), FileLockError>{
    println!("Hello, world!");
    let exclusive_file = File::create("/tmp/share/foo.txt").unwrap();
    exclusive_file.lock(FileLockMode::Exclusive)?;
    thread::sleep(time::Duration::from_secs(300));
    Ok(())
}
