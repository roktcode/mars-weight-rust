use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();

    // unwrap will exit the program if any operation fails
    // and will continue and return the value when success
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars is: {}kg", mars_weight)
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    // rust returns the last statement, (without a ;)
    (weight / 9.81) * 3.711
}
