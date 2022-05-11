fn main() {
    let arr = [10, 10, 7, 3, 4, 5, 6, 4, 7, 0, 10, 6, 0, 10, 10, 10];
    let v = arr.to_vec(); // v: Vec<i32>

    println!("{}", bowling_score(&arr, 0));
    println!("{}", bowling_score(&v, 0));
}

fn bowling_score(throws: &[i32], score: i32) -> i32 {
    if throws.len() <= 3 {
        let score_last_throws: i32 = throws.iter().sum();
        return score + score_last_throws;
    }

    match throws {
        [10, rest @ ..] => bowling_score(rest, score + 10 + rest[0] + rest[1]), // Strike
        [t1, t2, rest @ ..] if t1 + t2 == 10 => bowling_score(rest, score + 10 + rest[0]), // Spare
        [t1, t2, rest @ ..] => bowling_score(rest, score + t1 + t2),            // Open frame
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::bowling_score;
    use rstest::rstest;

    // Using rstest only due to avoiding rustfmt making ugly formatting (max 100 chars per line).
    #[rstest]
    #[case(0, &[0; 20])]
    #[case(45, &[0, 1, 7, 2, 1, 2, 1, 2, 1, 0, 1, 2, 4, 3, 5, 4, 3, 2, 1, 3])]
    #[case(81, &[1, 5, 3, 6, 7, 2, 3, 6, 4, 4, 5, 3, 3, 3, 4, 5, 8, 1, 2, 6])]
    #[case(94, &[10, 3, 6, 7, 2, 3, 6, 4, 4, 5, 3, 3, 3, 4, 5, 8, 1, 2, 6])]
    #[case(88, &[1, 9, 3, 6, 7, 2, 3, 6, 4, 4, 5, 3, 3, 3, 4, 5, 8, 1, 2, 6])]
    #[case(103, &[10, 4, 6, 7, 2, 3, 6, 4, 4, 5, 3, 3, 3, 4, 5, 8, 1, 2, 6])]
    #[case(300, &[10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10])]
    fn bowling_score_test(#[case] expected: i32, #[case] input: &[i32]) {
        assert_eq!(expected, bowling_score(input, 0))
    }
}
