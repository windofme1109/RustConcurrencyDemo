use std::thread;

mod basic_thread;

mod thread_builder;

mod current_thread;

mod concurrency_count;

mod thread_sleep_and_thread_park;

mod scoped_thread;

mod thread_local;

mod thread_move;


use basic_thread::*;

use thread_builder::*;

use current_thread::*;

use concurrency_count::*;

use thread_sleep_and_thread_park::*;

use scoped_thread::*;

use thread_local::*;

use thread_move::*;

fn main() {
    // println!("Hello, world!");
    // start_one_thread();
    // start_two_thread();

    // start_n_threads();

    // start_one_thread_by_builder();

    // current_thread();

    // unpark_thread()

    // test_max_concurrency_count();

    // start_thread_with_sleep();
    // start_thread_with_yield_now();

    // thread_park2();
    // thread_park3();
    // thread_park4();

    // start_scoped_threads();

    // start_threads_with_threadlocal();
    start_one_thread_with_move();
}


