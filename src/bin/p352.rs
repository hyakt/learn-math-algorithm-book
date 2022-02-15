use rand::prelude::*;

fn main() {
    let n = 1_000_000;

    let mut count = 0;

    for _ in 0..n {
        let mut rng = thread_rng();
        let x = rng.gen_range(0f64, 6f64);
        let y = rng.gen_range(0f64, 9f64);

        let d1 = ((x - 3.0).powf(2.0) + (y - 3.0).powf(2.0)).sqrt();
        if d1 <= 3.0 {
            count += 1;
            continue;
        }

        let d2 = ((x - 3.0).powf(2.0) + (y - 7.0).powf(2.0)).sqrt();
        if d2 <= 2.0 {
            count += 1;
        }
    }

    println!("{}", count);
}
