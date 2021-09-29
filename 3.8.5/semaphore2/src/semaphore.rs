use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    mutex: Mutex<isize>,
    cond: Condvar,
    max: isize,
}

impl Semaphore {
    pub fn new(max: isize) -> Self {
        Semaphore {
            mutex: Mutex::new(0),
            cond: Condvar::new(),
            max,
        }
    }

    pub fn wait(&self) {
        // カウントが最大値以上なら待機
        let mut cnt = self.mutex.lock().unwrap();
        while *cnt >= self.max {
            cnt = self.cond.wait(cnt).unwrap()
        }
        // cntを変更しないのであれば、 
        // self.cond.wait_while(cnt, |c| *c >= self.max);
        // のようにもかけるらしい

        *cnt += 1;
    }

    pub fn post(&self) {
        let mut cnt = self.mutex.lock().unwrap();
        *cnt -= 1;
        if *cnt < self.max {
            // self.max以上になることないんやから、*cnt -= 1 のあとはの条件は
            // *cnt < self.max になるべき？？
            self.cond.notify_one();
        }
    }
}
