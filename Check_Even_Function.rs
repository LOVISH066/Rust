fn main() {
    println!("{}", is_even(40));
}

fn is_even(num: u64) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
