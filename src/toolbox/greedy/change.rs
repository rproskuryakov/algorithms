fn get_change( m: i32) -> i32 {
    let n_coins = 0;
    while (m > 0) {
        if (m - 10 >= 0) {
            m = m - 10;
        } else if (m - 5 >= 0) {
            m = m - 5;
        } else {
            m = m - 1;
        }
        n_coins+=1;
    }
    return n_coins;
}

fn local_main() -> i32 {
    let m: i32;
    io::stdin()
        .read_line(&mut m) // Read line into `n`
        .expect("Failed to read line");
    println!(get_change(m));
    return 0;
}
