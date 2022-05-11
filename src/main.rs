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
        [t1, t2, rest @ ..] => bowling_score(rest, score + t1 + t2), // Open frame
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::bowling_score;

    #[test]
    fn bowling_score_sums() {
        assert_eq!(0, bowling_score(&[0; 20], 0));
        assert_eq!(45, bowling_score(&[0, 1, 7, 2, 1, 2, 1, 2, 1, 0, 1, 2, 4, 3, 5, 4, 3, 2, 1, 3], 0));
        assert_eq!(
            81,
            bowling_score(
                &[1, 5, 3, 6, 7, 2, 3, 6, 4, 4, 5, 3, 3, 3, 4, 5, 8, 1, 2, 6],
                0
            )
        );
        assert_eq!(
            94,
            bowling_score(
                &[10, 3, 6, 7, 2, 3, 6, 4, 4, 5, 3, 3, 3, 4, 5, 8, 1, 2, 6],
                0
            )
        );
        assert_eq!(
            88,
            bowling_score(
                &[1, 9, 3, 6, 7, 2, 3, 6, 4, 4, 5, 3, 3, 3, 4, 5, 8, 1, 2, 6],
                0
            )
        );
        assert_eq!(
            103,
            bowling_score(
                &[10, 4, 6, 7, 2, 3, 6, 4, 4, 5, 3, 3, 3, 4, 5, 8, 1, 2, 6],
                0
            )
        );
        assert_eq!(
            300,
            bowling_score(&[10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10], 0)
        );
    }
}
