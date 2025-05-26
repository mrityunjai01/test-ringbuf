use ringbuf::{traits::*, HeapRb};
use std::thread;
use std::time::{Duration, SystemTime};

fn test_crate_ringbuf() {
    let rb = HeapRb::<usize>::new(1000);
    let (mut prod, mut cons) = rb.split();

    let h = thread::spawn(move || {
        let start = SystemTime::now();
        let mut blocks = 0;
        for i in 0..1000000 {
            loop {
                match cons.try_pop() {
                    None => {
                        thread::sleep(Duration::from_micros(1));
                        blocks += 1;
                    }
                    Some(x) => {
                        assert_eq!(x, i);
                        break;
                    }
                }
            }
        }
        println!("consumer end! {:?} {blocks}", start.elapsed());
    });

    let start = SystemTime::now();
    let mut blocks = 0;
    for i in 0..1000000 {
        loop {
            match prod.try_push(i) {
                Ok(_) => break,
                Err(_) => {
                    //println!("producer retry: {i}");
                    blocks += 1;
                    thread::sleep(Duration::from_micros(1));
                }
            }
        }
    }
    println!("producer end! {:?} {blocks}", start.elapsed());

    h.join().unwrap();
}

fn test_my_ringbuf() {
    let (mut prod, mut cons) = lib::my_ringbuf::<usize>(1000);

    let h = thread::spawn(move || {
        let start = SystemTime::now();
        let mut blocks = 0;
        for i in 0..1000000 {
            loop {
                match cons.try_pop() {
                    None => {
                        thread::sleep(Duration::from_micros(1));
                        blocks += 1;
                    }
                    Some(x) => {
                        assert_eq!(x, i);
                        break;
                    }
                }
            }
        }
        println!("consumer end! {:?} {blocks}", start.elapsed());
    });

    let start = SystemTime::now();
    let mut blocks = 0;
    for i in 0..1000000 {
        loop {
            match prod.try_push(i) {
                None => break,
                Some(_) => {
                    //println!("producer retry: {i}");
                    blocks += 1;
                    thread::sleep(Duration::from_micros(1));
                }
            }
        }
    }
    println!("producer end! {:?} {blocks}", start.elapsed());

    h.join().unwrap();
}
