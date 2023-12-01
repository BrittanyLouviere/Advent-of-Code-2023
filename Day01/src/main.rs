use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    println!("{}", solve(input));
}

#[test]
fn example_input_test() {
    assert_eq!(
        solve(
            "1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet"
                .to_string()
        ),
        142
    );
}

fn solve(input: String) -> u32 {
    let mut first_num = ' ';
    let mut last_num = ' ';
    let mut result = 0;
    for (_, a) in input.chars().enumerate() {
        if a == '\n' {
            let number = format!("{}{}", first_num, last_num);
            result += number.parse::<u32>().unwrap();
            first_num = ' ';
            last_num = ' ';
        }
        if a.is_numeric() {
            last_num = a;
            if first_num == ' ' {
                first_num = a;
            }
        }
    }

    let number = format!("{}{}", first_num, last_num);
    result += number.parse::<u32>().unwrap();

    result
}
