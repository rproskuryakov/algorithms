#include <iostream>

#include "fibonacci.h"

int fibonacci(int number) {
    int n_1 = 1;
    int n_2 = 1;

    if (number == 1 || number == 2) {
        return 1;
    }
    int acc = 0;
    for (int i = 2; i < number; ++i) {
        acc = n_1 + n_2;
        n_1 = n_2;
        n_2 = acc;
    }
    return acc;
}

fn local_main() {
    let n: i32;
    io::stdin()
        .read_line(&mut n) // Read line into `n`
        .expect("Failed to read line");
    let result = fibonacci(n);
    println!(result);
    return 0;
}

