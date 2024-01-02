fn main() {
    println!("Hello, world!");

    let mut x = 10;
    println!("X is {}", x);
    x = 20;
    println!("X is now {}", x);

    let y: u8 = 255;
    println!("Y is unsigned 8bit {}", y);


    let x_f: f32 = 0.1324242;
    println!("X is 32bit float {}", x_f);


    let x_1 = 25;
    let x_2 = 54;
    let x_1_2 = x_1 + x_2;
    println!("Addition of {} and {} is {}", x_1, x_2, x_1_2);

    let x_1_d = 2.5;
    let x_2_d = 54;
    let x_1_2_d = x_1_d / x_2_d as f32;
    println!("Division of {} by {} is {}", x_1_d, x_2_d, x_1_2_d);

    let x_1_d_2 = 2.5;
    let x_2_d_2 = 54;
    let x_1_2_d_2 = x_1_d_2 / x_2_d_2 as f32;
    println!("Division of {} by {} is {:.3}", x_1_d_2, x_2_d_2, x_1_2_d_2);
}
