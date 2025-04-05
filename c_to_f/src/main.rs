/*
Author: David Castellucci
Project Source: The Rust Programming Language (2021 Edition)
Last Updated: 4/5/2025
Description: Receives fixed Celsius value, passes to Fahrenheit conversion fn(), outputs Fahrenheit equivalency.
*/

fn main() {
    let celsius_temp = -32f32;
    println!("{}",convert_c_to_f(celsius_temp));
}

fn convert_c_to_f(c: f32) -> f32 {
    (c * 1.8) + 32f32
}
