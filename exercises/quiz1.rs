// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!


// Put your function here!
// fn ..... {



fn calculate_apple_price(amount:i32) -> i32 {

    let apple_cost: i32 = 2;
    let apple_cost_reduced: i32 = 1;   

    if amount <= 40 {
        apple_cost * amount
    } else {
        apple_cost_reduced * amount
    }


}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
