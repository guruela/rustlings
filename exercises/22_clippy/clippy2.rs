fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    if let Some(num) = option {
        res += num
    };
    println!("{res}");
}
