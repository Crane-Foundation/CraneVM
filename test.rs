fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Use `map` to square each number, then `filter` to keep only even squares
    let even_squares: Vec<i32> = numbers
        .iter()
        .map(|&x| x * x)
        .filter(|&x| x % 2 == 0)
        .collect();
    let f: f32 = 1.0;
    println!("Even Squares: {:?}", even_squares);
    println!("{}", std::mem::size_of::<isize>())
}

