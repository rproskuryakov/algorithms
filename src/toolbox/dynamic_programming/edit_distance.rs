use std::io;

fn edit_distance(first_string: string, second_string: string) -> i32 {
    let n_rows: usize = (1 + first_string.size());
    let n_columns: usize = (1 + second_string.size());
    int* distances = new int[n_rows * n_columns];
    for (int i = 0; i < n_columns; ++i) {
        *(distances + i) = i;
    }
    for (int i = 0; i < n_rows; ++i) {
        *(distances + i * n_columns) = i;
    }

    for (int j = 1; j < n_columns; ++j) {
        for (int i = 1; i < n_rows; ++i) {
            int insertion = *(distances + i * n_columns + j - 1) + 1;
            int deletion = *(distances + (i - 1) * n_columns + j) + 1;
            int match = *(distances + (i - 1) * n_columns + j - 1);
            int mismatch = *(distances + (i - 1) * n_columns + j - 1) + 1;
            if (first_string[i - 1] == second_string[j - 1]) {
                *(distances + i * n_columns + j) = std::min(std::min(insertion, deletion), match);
            } else {
              *(distances + i * n_columns + j) = std::min(std::min(insertion, deletion), mismatch);
            }
        }
    }

    return *(distances + n_rows * n_columns - 1);
}
