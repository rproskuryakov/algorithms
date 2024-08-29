fn local_main() {
    let a: i32;
    let b: i32;
    io::stdin()
        .read_line(&mut a) // Read line into `n`
        .expect("Failed to read line");
    io::stdin()
        .read_line(&mut b) // Read line into `n`
        .expect("Failed to read line");

    while (a != 0 && b != 0) {
        if (a > b) {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    if (a == 0) {
        println!(b);
    } else {
        println!(a);
    }
    return 0;
}
