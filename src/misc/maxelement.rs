fn main() {
    let numbers = vec![1, 2, 3, 10, 5];
    let max_element = numbers.iter().max();
    match max_element {
        Some(&max) => println!("Maximum element: {}", max),
        None => println!("The vector is empty."),
    }
}
