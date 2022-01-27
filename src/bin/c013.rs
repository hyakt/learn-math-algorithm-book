use proconio::input;

// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_m

fn generate_divisors(num: f64) -> Vec<usize> {
    let mut divisors: Vec<usize> = Vec::new();
    let r = num.sqrt().floor() as usize;

    for i in 1..=r {
        if num as usize % i == 0 {
            divisors.push(i);
            divisors.push((num as usize) / i);
        }
    }

    divisors
}

fn main() {
    input! {
        n: f64,
    };

    // eprintln!("generate_divisors(n) = {:?}", generate_divisors(n));
    generate_divisors(n).iter().for_each(|x| println!("{}", x));
}

#[cfg(test)]
mod tests {
    use cli_test_dir::*;
    const BIN: &'static str = "./c013";

    #[test]
    fn sample1() {
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r"12")
            .tee_output()
            .expect_success();
        assert_eq!(
            output.stdout_str(),
            "1
12
2
6
3
4\n"
        );
        assert!(output.stderr_str().is_empty());
    }
}
