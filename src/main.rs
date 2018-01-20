extern crate csv;
use std::time::Duration;
use std::thread;

fn main() {
    let mut cnt = 0;
    let mut rdr = csv::Reader::from_file("sf_censusTracts.csv").unwrap().delimiter(b';');

    for _row in rdr.records() {
        cnt+=1;
    }
    
    println!("{} rows found",cnt);
    println!("program ending in 10 seconds..");
    use std::thread;
    thread::sleep(Duration::from_millis(5000));
}