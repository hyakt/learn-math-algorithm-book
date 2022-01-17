use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    println!("{}", a.iter().copied().sum::<u32>())
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c003";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"5
3 1 4 1 5
"#,
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), "14\n");
        assert!(output.stderr_str().is_empty());
    }
}
