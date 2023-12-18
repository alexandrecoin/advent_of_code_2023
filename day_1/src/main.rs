use std::fs::read_to_string;

fn main() {
    println!("{}",answer_for_problem_1("./problem-1.txt"));
}

fn answer_for_problem_1(path: &str) -> u16 {
    let input = read_input_file(path);
    let mut result = vec![];
    for line in input.lines() {
        result.push(get_first_and_last_digit_from_line(&line));
    }
    result.iter().sum()
}

fn read_input_file(path: &str) -> String {
    read_to_string(path).expect("Failed to read document")
}

fn get_first_and_last_digit_from_line(line: &str) -> u16 {
    let digits = get_digits_from_line(line);
    match digits.len() {
        1 => digits[0].parse::<u16>().unwrap() * 11,
        2 => digits.join("").parse::<u16>().unwrap(),
        _ => {
            let mut result = vec![];
            result.push(String::from(&digits[0]));
            result.push(String::from(&digits[&digits.len() - 1]));
            result.join("").parse::<u16>().unwrap()
        }
    }
}

fn get_digits_from_line(line: &str) -> Vec<String> {
    let mut line_number = vec![];
    for character in line.chars() {
        match character.is_digit(10) {
            true => line_number.push(String::from(character)),
            false => continue,
        }
    }
    line_number
}

#[cfg(test)]
mod tests {
    mod answer_for_problem_1 {
        use crate::answer_for_problem_1;

        #[test]
        fn it_returns_the_correct_result() {
            let result = answer_for_problem_1("./test-input.txt");
            assert_eq!(result, 142);
        }
    }

    mod read_input_file {
        use crate::read_input_file;

        #[test]
        fn it_reads_the_input_file() {
            let file_path = "./test-input.txt";
            let result = read_input_file(file_path);
            assert_eq!(result, "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        }
    }

    mod get_digits_from_line {
        use crate::get_digits_from_line;

        #[test]
        fn it_returns_the_number_if_there_is_only_one_number_per_line() {
            let input = "treb7uchet";
            let result = get_digits_from_line(input);
            assert_eq!(result, ["7"]);
        }

        #[test]
        fn it_returns_all_numbers_if_there_are_multiple_numbers_per_line() {
            let input = "a1b2c3d4e5f";
            let result = get_digits_from_line(input);
            assert_eq!(result, ["1", "2", "3", "4", "5"]);
        }
    }

    mod get_first_and_last_digit_from_line {
        use crate::get_first_and_last_digit_from_line;

        #[test]
        fn it_returns_a_doubled_digit_if_only_one_digit_per_line() {
            let line = "treb7uchet";
            let result = get_first_and_last_digit_from_line(line);
            assert_eq!(result, 77);
        }

        #[test]
        fn it_returns_the_digits_if_two_digits_per_line() {
            let line = "pqr3stu8vwx";
            let result = get_first_and_last_digit_from_line(line);
            assert_eq!(result, 38);
        }

        #[test]
        fn it_returns_the_first_and_last_digits_if_more_than_two_digits_per_line() {
            let line = "a1b2c3d4e5f";
            let result = get_first_and_last_digit_from_line(line);
            assert_eq!(result, 15);
        }
    }
}
