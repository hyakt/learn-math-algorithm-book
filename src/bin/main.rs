// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        mut plan: [(i32, i32, i32); n],  // Vec<(i32, i32, i32)>
    }
    plan.insert(0, (0, 0, 0));
    let yes = plan.windows(2).all(|w| {
        let (t1, x1, y1) = w[1];
        let (t0, x0, y0) = w[0];
        let time = t1 - t0;
        let dist = (x1 - x0).abs() + (y1 - y0).abs();
        dist <= time && time % 2 == dist % 2
    });
    println!("{}", if yes { "Yes" } else { "No" });
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;

    const BIN: &'static str = "./main";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"2
3 1 2
6 1 1
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "Yes\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"1
2 100 100
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "No\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample3() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"2
5 1 1
100 1 1
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "No\n");
        assert!(output.stderr_str().is_empty());
    }
}
