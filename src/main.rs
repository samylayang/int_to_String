fn main() {
    let num = 42;
    let num_as_string = num.to_string();
    println!("{}", num_as_string); // Outputs: 42

    let num = 42;
    let num_as_string = format!("Number: {}", num);
    println!("{}", num_as_string); // Outputs: Number: 42
}
