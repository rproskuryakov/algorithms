mod longest_common_subsequence;


#[cfg(test)]
mod edit_distance_tests {
    use super::*;

    #[test]
    fn task_description_first_example_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> first_seq = {2, 7, 5};
        // std::vector<int> second_seq = {2, 5};
        // EXPECT_EQ(length_of_lcs(first_seq, second_seq), 2);
    }

    #[test]
    fn task_description_second_example_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> first_seq = {7};
        // std::vector<int> second_seq = {1, 2, 3, 4};
        // EXPECT_EQ(length_of_lcs(first_seq, second_seq), 0);

    }

    #[test]
    fn task_description_third_example_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::vector<int> first_seq = {2, 7, 8, 3};
        // std::vector<int> second_seq = {5, 2, 8, 7};
        // EXPECT_EQ(length_of_lcs(first_seq, second_seq), 2);
    }
}
