use std::ptr::{read_volatile, write_volatile};
use std::sync::atomic::{fence, Ordering};
use std::thread;

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100000;

// volatile用マクロ
macro_rules! read_mem {
    ($addr: expr) => {
        unsafe { read_volatile($addr) }
    };
}
macro_rules! write_mem {
    ($addr: expr, $val: expr) => {
        unsafe { write_volatile($addr, $val) }
    };
}

struct BakeryLock {
    entering: [bool; NUM_THREADS],
    tickets: [Option<u64>; NUM_THREADS],
}

impl BakeryLock {
    fn lock(&mut self, idx: usize) -> LockGuard {
        // チケット取得処理
        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], true);
        fence(Ordering::SeqCst);

        // チケットの最大値を取得
        let mut max = 0;
        for i in 0..NUM_THREADS {
            if let Some(t) = read_mem!(&self.tickets[i]) {
                max = max.max(t);
            }
        }

        // 取得した最大値 + 1を自分のチケット番号とする
        // この番号は別のスレッドとかぶっていても良い
        let ticket = max + 1;
        write_mem!(&mut self.tickets[idx], Some(ticket));

        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], false);
        fence(Ordering::SeqCst);

        //ここから待機処理
        for i in 0..NUM_THREADS {
            if i == idx {
                continue;
            }

            // 調査対象がチケット取得集の場合待つ
            while read_mem!(&self.entering[i]) {}

            loop {
                match read_mem!(&self.tickets[i]) {
                    Some(t) => {
                        // スレッドiのチケット番号より
                        // 自分のチケット番号のほうが若いか、
                        // チケット番号が同じでかつ、自分のスレッド番号のほうが若い場合に終了
                        if ticket < t || ((ticket == t) && idx < i) {
                            break;
                        }
                    }
                    None => {
                        // スレッドiが処理中ではない場合。
                        break;
                    }
                }
            }
        }

        fence(Ordering::SeqCst);
        LockGuard { idx }
    }
}

struct LockGuard {
    idx: usize,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.idx], None);
    }
}

static mut LOCK: BakeryLock = BakeryLock {
    entering: [false; NUM_THREADS],
    tickets: [None; NUM_THREADS],
};

static mut COUNT: u64 = 0;

fn main() {
    let mut v = Vec::new();
    for i in 0..NUM_THREADS {
        let th = thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let _lock = unsafe { LOCK.lock(i) };
                let c = read_mem!(&COUNT);
                write_mem!(&mut COUNT, c + 1);
            }
        });
        v.push(th);
    }

    for th in v {
        th.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {}",
        unsafe { COUNT },
        NUM_LOOP * NUM_THREADS
    );
}
