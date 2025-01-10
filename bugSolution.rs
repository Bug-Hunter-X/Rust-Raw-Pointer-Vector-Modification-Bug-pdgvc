fn main() {
    let mut v = vec![1, 2, 3];
    // Safe way to modify vector elements using indexing
    v[0] = 10;
    println!("The first element is: {}", v[0]);
}