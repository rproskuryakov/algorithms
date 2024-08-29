fn local_main() {
    let a: i32;
    let b: i32;
    let gcd_result: i32;
    io::stdin()
        .read_line(&mut a) // Read line into `n`
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut b) // Read line into `n`
        .expect("Failed to read line");    long long product = ((long long)a) * b;
    while (a != 0 && b != 0) {
        if (a > b) {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    if (a == 0) {
        gcd_result =  b;
    } else {
        gcd_result = a;
    }
    let lcm = product / gcd_result;
    println!(lcm);
    return 0;
}