use std::thread;

mod basic_thread;

mod thread_builder;

mod current_thread;

mod concurrency_count;

mod thread_sleep_and_thread_park;

mod scoped_thread;

mod thread_local;

mod thread_move;
mod control_thread;
mod thread_priority;
mod thread_affinity;
mod thread_panic;
mod crossbeam_scoped_thread;
mod rayon_scoped_thread;
mod send_wrapper;
mod go_spawn;

use basic_thread::*;

use thread_builder::*;

use current_thread::*;

use concurrency_count::*;

use thread_sleep_and_thread_park::*;

use scoped_thread::*;

use thread_local::*;

use thread_move::*;

use control_thread::*;

use thread_priority::*;

use thread_affinity::*;

use thread_panic::*;

use crossbeam_scoped_thread::*;


use rayon_scoped_thread::*;

use send_wrapper::*;
use go_spawn::*;

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

    // start_one_thread_with_move();

    // control_thread();

    // start_thread_with_priority();

    // thread_builder();

    // bind_even_cores();

    // panic_example();

    // panic_caught_example()

    // thread_scope_panic_example();

    // crossbeam_scope();

    // rayon_scope();

    // rayon_fifo_example();

    // wrong_send();

    // send_wrapper()
    go_thread();
}





