
fn money_change(money: i32) -> i32 {
    let n_coins: vec<i32> = {0};
    let coins: vec<i32> = {1, 3, 4};
    for (int i = 1; i <= money; ++i) {
        let bool is_inf = true;
        for (auto &element: coins) {
            if (i >= element) {
                int num_coins = n_coins.at(i - element) + 1;
                if (is_inf) {
                    n_coins.push_back(num_coins);
                    is_inf = false;
                } else if (num_coins < n_coins.at(i)) {
                    n_coins.at(i) = num_coins;
                }
            }
        }
    }
    return n_coins.at(money);
}