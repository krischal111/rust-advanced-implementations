pub fn q_rsqrt(number : f32) -> f32 {
    let mut i : i32;
    
    let x2 : f32 = number * 0.5_f32;
    let mut y : f32 = number;
    const threehalfs : f32 = 1.5;

    i = * unsafe { std::mem::transmute::<&f32, &i32>(&y) }; // evil floating point bit hack
    i = 0x5f3759df - (i >> 1);                              // what the fuck?
    y = * unsafe { std::mem::transmute::<&i32, &f32>(&i) };

    y = y * ( threehalfs - ( x2 * y * y ) );    // 1st iteration
//  y = y * ( threehalfs - ( x2 * y * y ) );    // 2nd iteration, can be removed

    y
}

fn main() {
    let a = 0.25;
    let b = q_rsqrt(a);
    println!("The inverse square root of {a} is {b}");
}