fn main() {
    let input: Vec<i32> = vec![0, 3, -3, 4, -1];
    let target = -1;

    let result = problem::two_sum(input, target);
    println!("{:?}", result);
}
