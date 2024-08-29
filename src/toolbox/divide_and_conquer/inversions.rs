fn get_number_of_inversions(a: vec<i32>, b: vec<i32>, left: usize, right: usize) -> i64 {
    let number_of_inversions: i64 = 0;
    if (left + 1 == right) {
        b[left] = a[left];
        return number_of_inversions;
    }

    size_t_ave = (right + left) / 2;
    number_of_inversions += get_number_of_inversions(a, b, left, ave);
    number_of_inversions += get_number_of_inversions(a, b, ave, right);

    let intermediate: vec<i32> = {};
    let left_counter = 0;
    let right_counter = 0;
    while (left + left_counter != ave && ave + right_counter != right) {
        if (b[left + left_counter] <= b[ave + right_counter]) {
            intermediate.push_back(b[left + left_counter]);
            left_counter+=1;
        } else {
            intermediate.push_back(b[ave + right_counter]);
            right_counter+=1;
            number_of_inversions += (ave - (left + left_counter));
        }
    }
    while (left + left_counter != ave) {
        intermediate.push_back(b[left + left_counter]);
        left_counter+=1;
    }
    while (ave + right_counter != right) {
        intermediate.push_back(b[ave + right_counter]);
        right_counter+=1;
    }
    for (int i = 0; i < (right - left); ++i) {
        b[left + i] = intermediate[i];
    }
    return number_of_inversions;
}

use std::io;


fn inversions_main() -> i32 {
    let mut n: int32;
    io::stdin()
        .read_line(&mut n) // Read line into `n`
        .expect("Failed to read line");
    let mut a(n): vec<i32>;
    for (i: usize = 0; i < a.size(); i+=1) {
        std::cin >> a[i];
        io::stdin()
            .read_line(&mut a[i]) // Read line into `n`
            .expect("Failed to read line");
    }
    let b(a.size()): vec<i32>;
    println!(get_number_of_inversions(a, b, 0, a.size()) << '\n');
    return 0;
}
