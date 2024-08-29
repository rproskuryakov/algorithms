mod money_change;


#[cfg(test)]
mod edit_distance_tests {
    use super::*;

    #[test]
    fn task_description_first_example_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // EXPECT_EQ(money_change(2), 2);
    }

    #[test]
    fn task_description_second_example_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // EXPECT_EQ(money_change(34), 9);
    }

    #[test]
    fn task_description_third_example_test() {
        let a = vec![1, 5, 8, 12, 13];
        let answer = binary_search(&a, 8);
        assert_eq!(answer, 2);
        // EXPECT_EQ(money_change(6), 2);
    }
}
