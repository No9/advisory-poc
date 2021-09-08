use std::fs::File;
use std::{thread, time};
use advisory_lock::{AdvisoryFileLock, FileLockMode, FileLockError};

fn main() -> Result<(), std::io::Error>{
    println!("Hello, world!");
    let shared_file = File::open("../locker/foo.txt")?;
    shared_file.unlock().expect("unlock failed");
    shared_file.try_lock(FileLockMode::Shared).expect("Works, because the exclusive lock was released");
    Ok(())
}
