fn maxPairwiseProduct(array: vec<i32>) -> i32 {
    let first_max = 0;
    let second_max = 0;

    if (array[1] > array[0]) {
        first_max = array[1];
        second_max = array[0];
    } else {
        first_max = array[0];
        second_max = array[1];
    }

    for (int i = 2; i < array.size(); ++i) {
        let current_element = array[i];
        if (current_element > second_max && current_element < first_max)
        {
            second_max = current_element;
        } else if (current_element >= first_max){
            second_max = first_max;
            first_max = current_element;
        }
    }
    return first_max* second_max;
}


fn local_main() {
    let n: i32;
    io::stdin()
        .read_line(&mut n) // Read line into `n`
        .expect("Failed to read line");
    let numbers(n): vec<i32>;
    for (int i = 0; i < n; ++i) {
        io::stdin()
            .read_line(&mut numbers[i]) // Read line into `n`
            .expect("Failed to read line");
    }

    let result = maxPairwiseProduct(numbers);
    println!(result);
    return 0;
}
