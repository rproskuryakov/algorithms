mod majority_element;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_description_first_example_contains_majority_element_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {2, 3, 9, 2, 2};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 1);
    }

    #[test]
    fn task_description_second_example_doesnt_contain_majority_element_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {1, 2, 3, 4};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 0);
    }

    #[test]
    fn task_description_third_example_doesnt_contain_majority_element_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {1, 2, 3, 1};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 0);
    }

    #[test]
    fn array_with_one_element_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {1};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 1);
    }

    #[test]
    fn two_elements_array_with_majority_element_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {2000, 2000};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 1);
    }

    #[test]
    fn two_elements_array_without_majority_element_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {500000, 200000};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 0);
    }

    #[test]
    fn no_majority_element_even_number_of_elements_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {6, 1, 4, 2, 0, 10};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 0);
    }

    #[test]
    fn no_majority_element_odd_number_of_elements_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {5, 4, 2, 3, 1};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 0);
    }

    #[test]
    fn no_majority_element_half_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {0, 10, 1, 5, 10, 10};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 0);
    }

    #[test]
    fn maximum_value_array_with_majority_element_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {1000000000, 0, 1000000000};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 1);
    }

    #[test]
    fn maximum_elements_array_without_majority_element_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array(100000);
        // for (size_t i = 0; i < 50000; ++i) {
        //     array[i] = 5;
        // }
        // for (size_t i = 50000; i < 100000; ++i) {
        //     array[i] = 10;
        // }
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 0);
    }

    #[test]
    fn maximum_elements_array_with_majority_element_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array(100000);
        // for (size_t i = 0; i < 50001; ++i) {
        //     array[i] = 5;
        // }
        // for (size_t i = 50001; i < 100000; ++i) {
        //     array[i] = 10;
        // }
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 1);
    }

    #[test]
    fn second_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> array = {512766168, 717383758, 5, 126144732, 5, 573799007, 5, 5, 5, 405079772};
        // EXPECT_EQ(get_majority_element(array, 0, array.size()), 0);
    }
}
