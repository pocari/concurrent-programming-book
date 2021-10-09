use std::{
    sync::{Arc, RwLock},
    thread,
};

fn main() {
    let val = Arc::new(RwLock::new(true));

    let th = thread::spawn(move || {
        let flag = *val.read().unwrap();
        if flag {
            *val.write().unwrap() = false;
            println!("flag is true");
        }
    });

    th.join().unwrap();
}