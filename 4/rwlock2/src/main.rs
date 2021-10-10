use std::{
    sync::{Arc, RwLock},
    thread,
};

fn main() {
    let val = Arc::new(RwLock::new(true));

    let th = thread::spawn(move || {
        // _flagにするとこのscopeがある間read lockをとるのでwriteロックをとるところでdead lockにな
        // る
        // let _flag = val.read().unwrap();

        // _ にすると、rustがこの変数は使われない、と判断し即座に破棄 == ロック解除 となり、
        // deadlockしない
        let _ = val.read().unwrap();
        *val.write().unwrap() = false;
        println!("*val ... {:?}", *val);
    });

    th.join().unwrap();
}
