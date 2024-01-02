fn main() {
    variables_arithmetic_printing();
    bitwise_operations();
    bool_operations();
    bool_operations_2();
    char_operations();
    find_average();
    arrays();
}

fn variables_arithmetic_printing() {
    println!("Basic variable syntax\n");

    // Variable
    let mut x = 10;
    println!("X is {}", x);
    x = 20;
    println!("X is now {}", x);

    // Unsigned ints
    let y: u8 = 255;
    println!("Y is unsigned 8bit {}", y);

    // Floats
    let x_f: f32 = 0.1324242;
    println!("X is 32bit float {}", x_f);

    // Addition
    let x_1 = 25;
    let x_2 = 54;
    let x_1_2 = x_1 + x_2;
    println!("Addition of {} and {} is {}", x_1, x_2, x_1_2);

    // Division with casting
    let x_1_d = 2.5;
    let x_2_d = 54;
    let x_1_2_d = x_1_d / x_2_d as f32;
    println!("Division of {} by {} is {}", x_1_d, x_2_d, x_1_2_d);

    // Division with casting and out to .000 decimal places
    let x_1_d_2 = 2.5;
    let x_2_d_2 = 54;
    let x_1_2_d_2 = x_1_d_2 / x_2_d_2 as f32;
    println!("Division of {} by {} is {:.3}", x_1_d_2, x_2_d_2, x_1_2_d_2);

    // Formatting
    let x_1_d_2_opt = 2.5;
    let x_2_d_2_opt = 54;
    let x_1_2_d_2_opt = x_1_d_2_opt / x_2_d_2_opt as f32;
    println!("Division of {0} by {1} is {2:.3}", x_1_d_2_opt, x_2_d_2_opt, x_1_2_d_2_opt);
    println!();
}

fn bitwise_operations() {
    println!("Bitwise syntax\n");

    let binary = 0b1111_0101u8;
    // display as int
    println!("Binary value is {}", binary);
    // display as binary
    println!("Binary value is {:08b}", binary);


    // NOT - inverts bits.
    let inverted_bits = !binary;
    println!("Inverted bits: {}", inverted_bits);
    println!("Inverted bits: {:08b}", inverted_bits);

    // AND - If both bits mask + value are 1, set 1 otherwise 0.
    let and_bits =  binary & 0b0000_0101u8;
    println!("AND'ed bits: {}", and_bits);
    println!("AND'ed bits: {:08b}", and_bits);

    // AND - Can be used for checking the value of a specific bit.
    let is_last_bit_active = (and_bits & 0b0000_0001u8) == 0b0000_0001;
    println!("Is the last bit inverted: {}", is_last_bit_active);

    // OR - If either mask or input is 1, result is 1 otherwise 0.
    // Can be used to set a value on a specific bit index.
    let add_one_to_ten = inverted_bits | 0b0000_0001u8;
    let should_be_eleven = add_one_to_ten == 11;
    println!("Add 1 to 10: {}", add_one_to_ten);
    println!("Add 1 to 10: {:08b}", add_one_to_ten);
    println!("Add 1 to 10 should be 11: {}", should_be_eleven);

    // XOR - if either mask or input is equal, result is 0 otherwise 1.
    let mask_should_validate_binary_to_zero = add_one_to_ten ^ 0b0000_01011;
    println!("Validate binary as zero: {}", mask_should_validate_binary_to_zero);
    println!("Validate binary as zero: {:08b}", mask_should_validate_binary_to_zero);

    // Left Shift operator, shift bits to the left by specified number.
    let should_shift_our_bits_left_2 = add_one_to_ten << 2;
    let bits_should_be_shifted_left = should_shift_our_bits_left_2 == 0b0010_1100;
    println!("Shifted bits over left: {:08b}", should_shift_our_bits_left_2);
    println!("Shifted bits over left: {}", bits_should_be_shifted_left);

    let should_shift_our_bits_right_2 = should_shift_our_bits_left_2 >> 2;
    let bits_should_be_shifted_right = should_shift_our_bits_right_2 == 0b0000_01011;
    println!("Shifted bits over right: {:08b}", should_shift_our_bits_right_2);
    println!("Shifted bits over right: {}", bits_should_be_shifted_right);
    println!();
}

fn bool_operations() {
    println!("Boolean syntax\n");

    // Bitwise operators work on bools, 1 = true, 0 = false.
    let a = true;
    let b = false;
    println!("a is {}, and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    // XOR a,b then AND a,b, OR the result
    // which should be 1.
    let c = (a ^ b) | (a & b);
    println!("c is: {}", c);

    // Short circuiting && or ||.
    // Won't evaluate right side after determining left.
    let c_short_circuited = (a ^ b) || panic!();
    println!("c short circuited is: {}", c_short_circuited);
    println!();
}

fn bool_operations_2() {
    println!("Boolean syntax expressions\n");

    // Bitwise operators work on bools, 1 = true, 0 = false.
    let a = 1;
    let b = 2;
    println!("a is {}, and b is {}", a, b);
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);
    println!();
}

fn char_operations() {
    println!("Character syntax\n");

    // Unicode by default.
    // Stored using 4 bytes, differing from cpp's and java's 1|2 bytes respectively.
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}'; // ☝
    println!("{}\n{}\n{}", letter, number, finger);
    println!();
}

fn find_average() {
    println!("Find average\n");

    let number_1 = 15;
    let number_2 = 2.4;
    let number_3 = 234;
    let sum:f64 = number_1 as f64 + number_2 + number_3 as f64;
    let average = sum / 3.0;

    const expected:f64 = 83.8;
    assert_eq!(average, expected);
    println!("Average is {} as expected {}", average, average);
    println!();
}

fn arrays() {
    println!("Arrays syntax\n");

    // Can't dynamically resize array.
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';

    let get_first_val = letters[0];
    println!("First value is {}", get_first_val);

    // Initialize array of 32bit ints with size of 5.
    let numbers: [i32; 5];
    // Can use repeat expression to initialize large arrays easily.
    numbers = [1000; 5];

    // Usize is a type which informs the compiler to alloc memory
    // that is the specific size of the return data type.
    // eg. int32 = 4 bytes, float64 = 8 bytes etc.
    let final_index: usize = numbers.len() - 1;
    println!("Last number is {}", numbers[final_index]);
}
