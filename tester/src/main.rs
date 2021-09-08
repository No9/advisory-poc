use std::fs::File;
use std::{thread, time};
use advisory_lock::{AdvisoryFileLock, FileLockMode};

fn main() -> Result<(), std::io::Error>{
    println!("Hello, world!");
    
    loop {
        let shared_file = File::open("/tmp/share/foo.txt")?;
        match shared_file.try_lock(FileLockMode::Shared) {
            Ok(_) => { println!("no file lock! Maybe next time");},
            Err(e) => { println!("Great we have a file lock across containers :) {}", e);}
        }
        thread::sleep(time::Duration::from_secs(300));
    }
}
