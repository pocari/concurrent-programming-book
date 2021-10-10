use std::{
    sync::{Arc, RwLock},
    thread,
};

fn main() {
    let val = Arc::new(RwLock::new(true));

    let th = thread::spawn(move || {
        //  これだとデッドロックになる
        // let flag = val.read().unwrap();
        // if *flag {

        let flag = *val.read().unwrap();
        if flag {
            *val.write().unwrap() = false;
            println!("flag is true");
        }
    });

    let hoge = {
        let a = 1;
        a + 2
    };

    println!("hoge is {}", hoge);
    th.join().unwrap();
}
