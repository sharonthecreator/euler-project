use std::time;

mod problems;

fn print_and_reset_timer() {
    unsafe {
        static mut PROBLEM_ID: u8 = 1;
        static mut TIMER: Option<time::Instant> = None;

        match TIMER {
            Some(_) => {
                println!("Time elapsed in Problem {} is: {:?}\n", PROBLEM_ID, TIMER.unwrap().elapsed());
                PROBLEM_ID += 1;
            }
            None => {}
        }
        TIMER = Some(time::Instant::now());
    }
}

fn main() {
    // init timer.
    print_and_reset_timer();
    println!("Hello, thus my solutions to the Euler Project!\n");

    problems::p1::solution();
    print_and_reset_timer();
    problems::p2::solution();
    print_and_reset_timer();
    problems::p3::solution();
    print_and_reset_timer();
}
