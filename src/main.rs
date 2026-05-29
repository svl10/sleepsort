use sleepsort;
use std::env;
fn main() {
    let thevec: Vec<String> = env::args().collect();

    let mut newvec: Vec<i32> = vec![];
    for i in &thevec[1..] {
        let value: i32 = i.parse().expect("shet");
        newvec.push(value);
    }
    sleepsort::sleepsort(&newvec);
    // println!("{:?}", newvec);
}
