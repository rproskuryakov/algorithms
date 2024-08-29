fn max_dot_product(a: vec<i32>, b: vec<32>) -> i32 {
    let result = 0;
    sort(a.begin(), a.end(), std::greater<int>());
    sort(b.begin(), b.end(), std::greater<int>());
    for (size_t i = 0; i < a.size(); i++) {
        result += ((long long) a[i]) * b[i];
    }
    return result;
}

fn local_main() -> i32 {
    // a_i is profit per click & b_i is avg n of clicks per day
    let n: usize;
    io::stdin()
        .read_line(&mut n) // Read line into `n`
        .expect("Failed to read line");
    let a(n): vec<i32>; b(n): vec<i32>;
    for (size_t i = 0; i < n; i++) {
        std::cin >> a[i];
    }
    for (size_t i = 0; i < n; i++) {
        std::cin >> b[i];
    }
    println!(max_dot_product(a, b));
    return 0;
}
