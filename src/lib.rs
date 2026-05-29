use std::{thread, time::Duration};

pub fn sleepsort(list: &Vec<i32>) {
    let mut handles = Vec::new();
    for &i in list {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(i as u64));
            print!("{} ", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
