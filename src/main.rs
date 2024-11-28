use num_traits::ToPrimitive;

fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    // How to casting manual
    // let a_f64 = a as f64;

    //  incase import num_traits::ToPrimitive
    let a_f64 = a.to_f64().unwrap();

    println!("{} ", solve(a_f64, b));
}
