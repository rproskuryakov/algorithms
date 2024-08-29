fn fibonacci_digit_sum(number: i32) -> i64{
    let n_1 = 0;
    let n_2 = 1;

    if (number == 0 || number == 1) {
        return number;
    }
    let acc = 0;
    for (int i = 1; i < number; ++i) {
        acc = n_1 + n_2;
        n_1 = n_2;
        n_2 = acc;
    }
    return acc;
}

fn local_main() -> i32{
    let n;
    io::stdin()
        .read_line(&mut n) // Read line into `n`
        .expect("Failed to read line");
    let m = (n + 2) % 60;
    //std::cout << m << std::endl;
    let fib = fibonacci_digit_sum(m) % 10;
    if (fib == 0) {
        println!(9);
    } else {
        println!(fib - 1);
    }
    return 0;
}