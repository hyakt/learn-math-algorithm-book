use std::collections::HashMap;

use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_r

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut map: HashMap<usize, usize> = HashMap::new();

    a.iter().for_each(|i| {
        *map.entry(*i).or_insert(0) += 1;
    });

    let ans = map.get(&100).unwrap_or(&0) * map.get(&400).unwrap_or(&0)
        + map.get(&200).unwrap_or(&0) * map.get(&300).unwrap_or(&0);

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c018";

    #[test]
    fn c0181() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r"5
100 300 400 400 200
",
            )
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            r"3
"
        );
        assert!(output.stderr_str().is_empty());
    }
}
