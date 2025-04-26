// Constraints:
// 1 <= prices.length <= 105
// 0 <= prices[i] <= 104

use std::i32;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price: i32 = i32::MAX;
    let mut max_profit: i32 = 0;
    
    for price in prices {
        if price < min_price {
            min_price = price;
        }

        let current_profit = price - min_price;
        if current_profit > max_profit {
            max_profit = current_profit;
        }
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        // based on constraints, nothing extra
        let cases = vec![
            (vec![7, 1, 5, 3, 6, 4], 5),
            (vec![7, 6, 4, 3, 1], 0),
            (vec![7, 2, 7, 1, 5, 3, 6, 4], 5),
        ];

        for (input, expected) in cases {
            assert_eq!(max_profit(input), expected);
        }
    }
}
