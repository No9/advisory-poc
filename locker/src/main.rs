use std::fs::File;
use std::{thread, time};
use advisory_lock::{AdvisoryFileLock, FileLockMode, FileLockError};

fn main() -> Result<(), FileLockError>{
    println!("Hello, world!");
    let exclusive_file = File::create("foo.txt").unwrap();
    exclusive_file.lock(FileLockMode::Exclusive)?;
    
    let shared_file = File::open("foo.txt").unwrap();
    shared_file.unlock().expect("unlock failed");
    thread::sleep(time::Duration::from_secs(30));
    Ok(())
}
