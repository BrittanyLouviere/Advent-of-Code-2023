fn main() {
    println!("Hello, world!");
}

#[test]
fn example_input_test() {

    assert_eq!(
        solve(r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#), 142);
}

fn solve(input: &str) -> u32 {
    return 0
}