use std::{
    sync::{Arc, Mutex},
    thread::{self},
    time::SystemTime,
};

const NUM_COUNT: u64 = 100000;

fn main() {
    let c0 = Arc::new(Mutex::new("chopstick0"));
    let c1 = Arc::new(Mutex::new("chopstick1"));

    let c0_for_p1 = c0.clone();
    let c1_for_p1 = c1.clone();
    let p0 = thread::spawn(move || {
        for _ in 0..NUM_COUNT {
            // p0からみた左の箸をとる
            let _left1 = c0_for_p1.lock().unwrap();
            // p0からみた右の箸をとる
            let _right1 = c1_for_p1.lock().unwrap();
            println!("0: eating time: {:?}", SystemTime::now());
        }
    });

    let p1 = thread::spawn(move || {
        for _ in 0..NUM_COUNT {
            // p1からみた左の箸をとる
            let _left1 = c1.lock().unwrap();
            // p0からみた右の箸をとる
            let _right1 = c0.lock().unwrap();
            println!("1: eating time: {:?}", SystemTime::now());
        }
    });

    p0.join().unwrap();
    p1.join().unwrap();

    println!("Hello, world!");
}
