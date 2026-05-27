use std::{thread, time::Duration};

pub fn sleepsort(list: &Vec<i32>){

    for &i in list{
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(i as u64));
            print!("{} ", i);
    });
    }
    let highest = list.iter().max().unwrap();
    thread::sleep(Duration::from_millis(*highest as u64));
}