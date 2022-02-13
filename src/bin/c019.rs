use std::collections::HashMap;

use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_s

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut hash: HashMap<usize, usize> = HashMap::new();

    a.iter().for_each(|i| {
        *hash.entry(*i).or_insert(0) += 1;
    });

    let red = hash.get(&1).unwrap_or(&1);
    let blue = hash.get(&2).unwrap_or(&1);
    let yellow = hash.get(&3).unwrap_or(&1);

    let ans = (red * (red - 1) / 2) + (blue * (blue - 1) / 2) + (yellow * (yellow - 1) / 2);

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c019";

    #[test]
    fn c0191() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"6
1 3 2 1 1 2
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"4
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
