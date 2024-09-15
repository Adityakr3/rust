fn main() {
    let ans = is_even(16);
    println!("{}", ans);
}

fn is_even(num: i64) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

