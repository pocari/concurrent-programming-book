use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(10);

    {
        let v1 = lock.read().unwrap();
        let v2 = lock.read().unwrap();

        println!("v1 = {:?}", v1);
        println!("*v1 = {:?}", *v1);
        println!("v2 = {:?}", v2);
    }

    {
        let mut v = lock.write().unwrap();
        *v = 100;

        println!("v = {}", v);
    }

    {
        let v1_after = lock.read().unwrap();

        println!("v1_after = {:?}", v1_after);
        println!("*v1_after = {:?}", *v1_after);
    }
}
