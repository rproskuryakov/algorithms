pub use algorithms::src::toolbox::divide_and_conquer::binary_search;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_description_first_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search::binary_search(&a, 8);
        assert_eq!(answer, 2);
    }

    #[test]
    fn task_description_second_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(a, 1);
        assert_eq!(answer, 0)
    }

    #[test]
    fn task_description_third_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(a, 23);
        assert_eq!(answer, -1)
    }

    #[test]
    fn task_description_forth_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(a, 11);
        assert_eq!(answer, -1)
    }

    #[test]
    fn no_value_in_vector() {
        let a = vec![1, 2, 3, 4, 5];
        let answer = binary_search(a, 10);
        assert_eq!(answer, -1)
    }

    #[test]
    fn maximum_elements_array_test() {
        let a: vec<i32>;
        for (i: usize = 0;
        i < 30000;
        + +i) {
            array[i] = i * i;
        }
        assert_eq!(binary_search(a, 500 * 500), 500);
        assert_eq!(binary_search(a, 29999 * 29999), 29999);
        assert_eq!(binary_search(a, 50000000), -1);
        assert_eq!(binary_search(a, 1000000000), -1);
        assert_eq!(binary_search(a, 999999999), -1);
    }

    #[test]
    fn one_element_array_test() {
        let a = vec![1, 2, 3];
        assert_eq!(binary_search(a, 3), 2);
        assert_eq!(binary_search(a, 4), -1);
    }
}
