use std::thread::spawn;

fn main() {
    let v = 10;
    let f = move || v * 2;

    let result = spawn(f).join();
    match &result {
        Ok(val) => println!("val is {:?}", val),
        Err(e) => println!("e is {:?}", e),
    }

    match spawn(|| panic!("hoooooooooogeeeeeeeeeeee")).join() {
        Ok(_) => println!("normal end"),
        Err(e) => println!("Err, {:?}", e),
    }
}
