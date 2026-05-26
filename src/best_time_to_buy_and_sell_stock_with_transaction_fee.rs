fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let mut min_price = prices[0];
    let mut profit = 0;

    for i in 1..prices.len() {
        if prices[i] < min_price {
            min_price = prices[i];
        }

        if prices[i] > min_price + fee {
            profit += prices[i] - min_price - fee;
            min_price = prices[i] - fee;
        }
    }

    profit
}

pub fn main() {
    let prices = [1,3,7,5,10,3].to_vec();
    let fee = 3;
    println!("{}", max_profit(prices, fee));
}
