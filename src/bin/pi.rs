use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let sqrt_n = 1000;
    let mut inside_circle = 0;
    let mut inside_circle_stratified = 0;
    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);
            if x * x + y * y < 1.0 {
                inside_circle += 1;
            }
            let x: f64 = 2.0 * ((i as f64 + rng.gen::<f64>()) / sqrt_n as f64) - 1.0;
            let y: f64 = 2.0 * ((j as f64 + rng.gen::<f64>()) / sqrt_n as f64) - 1.0;
            if x * x + y * y < 1.0 {
                inside_circle_stratified += 1;
            }
        }
    }
    println!(
        "   Regular estimate of pi = {:.12}\nStratified estimate of pi = {:.12}",
        (4.0 * inside_circle as f64) / ((sqrt_n * sqrt_n) as f64),
        (4.0 * inside_circle_stratified as f64) / ((sqrt_n * sqrt_n) as f64)
    );
}
