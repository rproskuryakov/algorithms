mod edit_distance;


#[cfg(test)]
mod edit_distance_tests {
    use super::*;

    #[test]
    fn task_description_first_example_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::string first_string = "ab";
        // std::string second_string = "ab";
        // EXPECT_EQ(edit_distance(first_string, second_string), 0);
    }

    #[test]
    fn task_description_second_example_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::string first_string = "short";
        // std::string second_string = "ports";
        // EXPECT_EQ(edit_distance(first_string, second_string), 3);
    }

    #[test]
    fn task_description_third_example_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // std::string first_string = "editing";
        // std::string second_string = "distance";
        // EXPECT_EQ(edit_distance(first_string, second_string), 5);
    }
}
