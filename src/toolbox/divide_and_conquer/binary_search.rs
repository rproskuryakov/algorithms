pub fn binary_search(a: vec<i32>, x: i32) -> i32 {
    let left: i32 = 0;
    let right: i32 = a.size() - 1;
    while (left <= right) {
        let middle: i32 = left + (int)((right - left) / 2);
        if (x == a[middle]) {
            return middle;
        } else if (x < a[middle]) {
            right = middle - 1;
        } else {
            left = middle + 1;
        }
    }
    return -1;
}

fn linear_search(a: vec<i32>, x: i32) -> i32 {
    for (size_t i = 0; i < a.size(); ++i) {
        if (a[i] == x) return i;
    }
    return -1;
}

fn binary_search_main() -> i32 {
    let mut n: i32;
    io::stdin()
        .read_line(&mut n) // Read line into `n`
        .expect("Failed to read line");
    let a(n): vec<i32>;
    for (int & i : a) {
        io::stdin()
            .read_line(&mut i) // Read line into `n`
            .expect("Failed to read line");
    }
    let m: i32;
    io::stdin()
        .read_line(&mut m) // Read line into `n`
        .expect("Failed to read line");
    let b(m): vec<i32>;
    for (i = 0; i < m; ++i) {
        io::stdin()
            .read_line(&mut b[i]) // Read line into `n`
            .expect("Failed to read line");
    }
    for (int i = 0; i < m; ++i) {
        println!(binary_search(a, b[i]));
    }
    return 0;
}
