#![allow(unused)]
use std::{
    env,
    sync::{Arc, Mutex},
    thread,
    time::SystemTime,
};

use rand::Rng;

type UIntType = u64;

const NUMBERS_VALUE: usize = 900000000;
const MAX_VAL: UIntType = 255;

fn sum_threaded(num_threads: usize) -> UIntType {
    let mut rng = rand::thread_rng();
    let chunk_size = (NUMBERS_VALUE + num_threads - 1) / num_threads;
    let mut sum = Arc::new(Mutex::new(UIntType::default()));
    let mut numbers = Vec::new();
    numbers.resize_with(NUMBERS_VALUE, || rng.gen_range(0..MAX_VAL));

    let time_start = SystemTime::now();

    for ch in numbers.chunks(chunk_size) {
        let ch = ch.to_vec();
        let sum_clone = Arc::clone(&sum);

        thread::spawn(move || {
            let s: UIntType = ch.iter().cloned().sum();
            *sum_clone.lock().unwrap() += s;
        });
    }

    log::debug!(
        "Execution for {} threads done. Time: {:?}",
        num_threads,
        time_start.elapsed().unwrap(),
        // &calculated_sum
    );

    *sum.clone().lock().unwrap()
}

fn main() {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    for i in 10..20 {
        sum_threaded(i);
    }
}
