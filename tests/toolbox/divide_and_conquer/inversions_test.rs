mod inversions;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_description_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> main_array = {2, 3, 9, 2, 9};
        // std::vector<int> additional_array(main_array.size());
        // long long result = get_number_of_inversions(main_array, additional_array, 0, main_array.size());
        // EXPECT_EQ(result, 2);
    }

    #[test]
    fn no_inversions_array_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> main_array = {1, 2, 3, 4, 5};
        // std::vector<int> additional_array(main_array.size());
        // long long result = get_number_of_inversions(main_array, additional_array, 0, main_array.size());
        // EXPECT_EQ(result, 0);
    }

    #[test]
    fn one_element_array_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> main_array = {1};
        // std::vector<int> additional_array(main_array.size());
        // long long result = get_number_of_inversions(main_array, additional_array, 0, main_array.size());
        // EXPECT_EQ(result, 0);
    }

    #[test]
    fn all_inversions_array_test() {
        let a = vec![5, 4, 3, 2, 1];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> main_array = {5, 4, 3, 2, 1};
        // std::vector<int> additional_array(main_array.size());
        // long long result = get_number_of_inversions(main_array, additional_array, 0, main_array.size());
        // EXPECT_EQ(result, (int)(5 * 4 / 2));
    }

    #[test]
    fn overflow_array_test() {
        let a = vec![5, 4, 3, 2, 1];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> main_array = {1000000000, 5000, 30000};
        // std::vector<int> additional_array(main_array.size());
        // long long result = get_number_of_inversions(main_array, additional_array, 0, main_array.size());
        // EXPECT_EQ(result, 2);
    }

    #[test]
    fn maximum_elements_in_array_test() {
        let a = vec![5, 4, 3, 2, 1];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> main_array(100000);
        // std::vector<int> additional_array(100000);
        // for (int i = 0; i < 50000; ++i) {
        //     main_array[i] = 10;
        // }
        // for (int i = 50000; i < 100000; ++i) {
        //     main_array[i] = 5;
        // }
        // long long result = get_number_of_inversions(main_array, additional_array, 0, main_array.size());
        // EXPECT_EQ(result, (long long)50000 * 50000);
    }
}
