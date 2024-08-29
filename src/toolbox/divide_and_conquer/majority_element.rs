use std::io;

fn count_element(a: vec<i32>, elem: i32, left: i32, right: i32) -> i32 {
    let count: i32 = 0;
    for (i: usize = left; i < right; i+=1) {
        if (a[i] == elem) {
            count+=1;
        }
    }
    return count;
}

fn recursive_majority_element(a: vec<i32>, left: i32, right: i32) -> i32 {
    if (left == right - 1) {
        return a[left];
    }

    let middle: i32 = (int)((right + left) / 2);
    let left_majority: i32 = recursive_majority_element(a, left, middle);
    let right_majority: i32 = recursive_majority_element(a, middle, right);
    if (left_majority == right_majority) {
        return left_majority;
    }
    let left_count: i32 = count_element(a, left_majority, left, right);
    let right_count: i32 = count_element(a, right_majority, left, right);
    return if left_count > right_count {left_majority} else {right_majority};
}

fn get_majority_element(a: vec<i32>, left: i32, right: i32) {
    if (left == right - 1) {
        return 1;
    }

    let middle = ((right + left) / 2);
    let left_majority = recursive_majority_element(a, left, middle);
    let right_majority = recursive_majority_element(a, middle, right);
    let left_count = count_element(a, left_majority, left, right);
    let right_count = count_element(a, right_majority, left, right);
    if (left_count > ((right - left) / 2)) {
        return 1;
    }
    if (right_count > ((right - left) / 2)) {
        return 1;
    }
    return 0;
}

fn majority_element_main() -> i32{
    let n: i32;
    io::stdin()
        .read_line(&mut n) // Read line into `n`
        .expect("Failed to read line");
    let a(n): vec<i32>;
    for (i: i32 : a) {
        io::stdin()
        .read_line(&mut i) // Read line into `n`
        .expect("Failed to read line");
    }
    println!(get_majority_element(a, 0, a.size()));
    return 0;
}
