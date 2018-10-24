fn main() {
    let parsed:i32="3".parse().unwrap();
    let turbo_parsed="32.4".parse::<f64>().unwrap();

    let sum=parsed as f64 +turbo_parsed;
    println!("Sum: {:?}",sum);
}
