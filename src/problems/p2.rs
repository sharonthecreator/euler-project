pub fn solution() {
    let mut old = 1;
    let mut new = 2;
    let mut sum = 0;

    while new <= 4000000 {
        new += old;
        old = new - old;

        // checking before we override value.
        if old % 2 == 0 {
            sum += old;
        }
    }

    println!("The sum of the even-valued Fibonacci values: {sum}");
}