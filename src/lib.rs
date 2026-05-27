use std::{thread, time::Duration};

fn sleepsort(list: &Vec<i32>){

    for &i in list{
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(i as u64));
            print!("{} ", i);
    });
    }
}