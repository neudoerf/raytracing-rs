use rand::Rng;

fn f(d: f64) -> f64 {
    2.0 * d
}

fn pdf(_x: f64) -> f64 {
    0.5
}

fn main() {
    let n = 1000000;
    let mut sum = 0.0;
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let x = f(rng.gen());
        sum += x * x / pdf(x);
    }

    println!("I = {:.12}", sum / n as f64);
}
