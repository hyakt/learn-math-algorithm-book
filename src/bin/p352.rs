use rand::prelude::*;

fn main() {
    let n = 1_000_000;

    let mut count = 0;
    const X: f64 = 6.0;
    const Y: f64 = 9.0;

    for _ in 0..n {
        let mut rng = thread_rng();
        let x = rng.gen_range(0f64, X);
        let y = rng.gen_range(0f64, Y);

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

    println!("x, y: {}, {}", X, Y);
    println!("丸の中の数: {}", count);
    println!("丸の割合: {}", count as f64 / n as f64);
    println!("丸の面積: {}", X * Y * (count as f64 / n as f64));
}
