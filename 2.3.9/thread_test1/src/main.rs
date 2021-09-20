use std::thread::spawn;

fn hello() {
    println!("Hello, World")
}

fn main() {
    spawn(hello).join().unwrap();
    let h = || println!("Hello, world2");
    spawn(h).join().unwrap();
}
