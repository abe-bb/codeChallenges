fn main() {
    // testing mutable borrows inside of colsures
    let mut counter = 0;

    let mut closure_test = || counter += 1;

    closure_test();
    closure_test();

    println!("{}", counter);
}
