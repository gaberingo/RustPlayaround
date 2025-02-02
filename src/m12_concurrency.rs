#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use std::fs::{OpenOptions, File};
    use std::io::prelude::*;
    use std::thread;


    #[test]
    fn test_concurrency(){
        
        let file:Arc<Mutex<File>> = Arc::new(Mutex::new(OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("increment.txt")
        .unwrap()
        ));

        let mut handels: Vec<thread::JoinHandle<()>> = vec![];

        for i in 0..1000 {
            let file_mutex = Arc::clone(&file);
            let handle = thread::spawn(move || {
                let mut file = file_mutex.lock().unwrap();
                writeln!(file, "{}", i).unwrap();
            });
            handels.push(handle);

        }

        for handle in handels {
            handle.join().unwrap();
        }

    }
}