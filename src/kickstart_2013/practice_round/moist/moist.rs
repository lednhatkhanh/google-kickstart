pub fn moist(input: String) -> String {
    let mut tests = parse(input);
    let mut result = String::new();

    for (index, test) in tests.iter_mut().enumerate() {
        result.push_str(&solve(test, index));
        result.push_str("\n");
    }

    result
}

fn parse(input: String) -> Vec<Vec<String>> {
    let mut lines = input.lines();
    let tests = lines.next().unwrap().trim().parse::<usize>().unwrap();
    let mut result: Vec<Vec<String>> = vec![];

    for _ in 0..tests {
        let n = lines.next().unwrap().trim().parse::<usize>().unwrap();
        let mut test: Vec<String> = vec![];

        for _ in 0..n {
            test.push(lines.next().unwrap().trim().parse::<String>().unwrap());
        }

        result.push(test);
    }

    result
}

fn solve<T: PartialOrd>(arr: &mut Vec<T>, case_index: usize) -> String {
    let mut count = 0;

    for i in 1..arr.len() {
        let num_i = match arr.get(i) {
            None => {
                continue;
            }
            Some(num_i) => num_i,
        };
        let num_j = match arr.get(i - 1) {
            None => {
                continue;
            }
            Some(num_j) => num_j,
        };

        if num_i < num_j {
            let mut l = i;
            count += 1;

            while l > 0 {
                if arr[l - 1] > arr[l] {
                    arr.swap(l - 1, l);
                    l -= 1;
                } else {
                    break;
                }
            }
        }
    }

    format!("Case #{}: {}", case_index + 1, count)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::moist;

    fn read_test_data(file_name: &str) -> String {
        let file_path = format!(
            "src/kickstart_2013/practice_round/moist/test_data{}",
            file_name
        );
        fs::read_to_string(file_path).expect("Should have been able to read the file")
    }

    #[test]
    fn sample_test_set_1() {
        assert_eq!(
            moist(read_test_data("/sample_test_set_1/sample_ts1_input.txt")),
            read_test_data("/sample_test_set_1/sample_ts1_output.txt")
        );
    }

    #[test]
    fn test_set_1() {
        assert_eq!(
            moist(read_test_data("/test_set_1/ts1_input.txt")),
            read_test_data("/test_set_1/ts1_output.txt")
        );
    }

    #[test]
    fn test_set_2() {
        assert_eq!(
            moist(read_test_data("/test_set_2/ts2_input.txt")),
            read_test_data("/test_set_2/ts2_output.txt")
        );
    }
}
