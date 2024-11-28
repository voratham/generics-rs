use num_traits::{Float, ToPrimitive};

// First version - we can pass in BOTH f32 or f64
//  Second -- we can pass in any type of numbers
// fn solve(a: f64, b: f64) -> f64 {
//     let a_f64 = a.to_f64().unwrap();
//     let b_f64 = b.to_f64().unwrap();

//     (a_f64.powi(2) + b_f64.powi(2)).sqrt()
// }


//  Generic Type - Like an argument list, but for types.
fn solve<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    // TODO: version manual
    // let a: f32 = 3.0;
    // let b: f64 = 4.0;

    // How to casting manual
    // let a_f64 = a as f64;

    //  incase import num_traits::ToPrimitive
    // let a_f64 = a.to_f64().unwrap();

    // println!("{} ", solve(a_f64, b));

    // TODO: version generic with  num traits

    let a: f32 = 3.0;
    let b: f32 = 4.0;

    println!("{} ", solve::<f32>(a, b));
}
