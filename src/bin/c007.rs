use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    };

    let mut count: u32 = 0;

    for i in 1..=n {
        if i % x == 0 || i % y == 0 {
            count += 1
        };
    }

    println!("{}", count)
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c007";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin("15 3 5")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "7\n");
        assert!(output.stderr_str().is_empty());
    }

    #[test]
    fn sample2() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin("1000000 11 13")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "160839\n");
        assert!(output.stderr_str().is_empty());
    }
}
