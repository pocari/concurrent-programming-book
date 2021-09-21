mod semaphore;

use std::{sync::{Arc, atomic::{AtomicUsize, Ordering}}};

use semaphore::Semaphore;

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 8;
const SEM_NUM: isize = 4;

static mut CNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut v = Vec::new();
    let sem = Arc::new(Semaphore::new(SEM_NUM));

    for i in 0..NUM_THREADS {
        let s = Arc::clone(&sem);
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                s.wait();

                unsafe { CNT.fetch_add(1, Ordering::SeqCst) };
                let n = unsafe { CNT.load(Ordering::SeqCst) };
                println!("current thread: {}, CNT = {}", i, n);
                assert!((n as isize) <= SEM_NUM);
                unsafe { CNT.fetch_sub(1, Ordering::SeqCst) };

                s.post();
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}
