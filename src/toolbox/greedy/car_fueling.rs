fn compute_min_refills(int dist, int tank, std::vector<int> & stops) -> i32 {
    let num_refills = 0;
    let current_refill = 0;
    let last_refill;
    let n = stops.size();
    let new_stops(n + 2): vec<i32>;
    new_stops[0] = 0;
    new_stops[n + 1] = dist;
    for (size_t i = 1; i <= n; ++i) {
        new_stops[i] = stops[i - 1];
    }
    while (current_refill <= n) {
        last_refill = current_refill;
        while (current_refill <= n && new_stops[current_refill + 1] - new_stops[last_refill] <= tank) {
            current_refill+=1;
        }
        if (current_refill == last_refill) {
            return -1;
        }
        if (current_refill <= n) {
            num_refills = num_refills + 1;
        }
    }
    return num_refills;
}


fn local_main() -> i32 {
    let d: i32 = 0;
    io::stdin()
        .read_line(&mut d) // Read line into `n`
        .expect("Failed to read line");
    let m: i32 = 0;
    io::stdin()
        .read_line(&mut m) // Read line into `n`
        .expect("Failed to read line");
    let n: i32 = 0;
    io::stdin()
        .read_line(&mut n) // Read line into `n`
        .expect("Failed to read line");

    let stops(n): vec<i32>;
    for (i: usize = 0; i < n; ++i)
        std::cin >> stops.at(i);

    println!(compute_min_refills(d, m, stops));

    return 0;
}
