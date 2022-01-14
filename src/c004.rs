use proconio::input;

fn main() {
    input! {
        a1: usize,
        a2: usize,
        a3: usize,
    }

    println!("{}", a1 * a2 * a3)
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c004";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin("7 7 25")
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "1225\n");
        assert!(output.stderr_str().is_empty());
    }
}
