/**
 * Going through the Rust programming language basics on linkin learning.
 * Searching for that syntactic sugar.
 */
fn main() {
    variables_arithmetic_printing();
    bitwise_operations();
    bool_operations();
    bool_operations_2();
    char_operations();
    find_average();
    arrays();
    two_d_arrays();
    tuples();
    use_input_function(2, 3);
    square_input();
    return_early();
    return_tuple();
    convert_to_fahrenheit();
    condition_if();
    conditional_assignment();
    loops();
    while_loop();
    for_loop();
    nested_loop();
    calculate_mean_min_max([1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3]);
}

fn output_section_separator() {
    println!("====================================");
    println!();
} 

fn variables_arithmetic_printing() {
    println!("Basic variable syntax:\n");

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
    output_section_separator();
}

fn bitwise_operations() {
    println!("Bitwise syntax:\n");

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
    output_section_separator();
}

fn bool_operations() {
    println!("Boolean syntax:\n");

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
    output_section_separator();
}

fn bool_operations_2() {
    println!("Boolean syntax expressions:\n");

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
    output_section_separator();
}

fn char_operations() {
    println!("Character syntax:\n");

    // Unicode by default.
    // Stored using 4 bytes, differing from cpp's and java's 1|2 bytes respectively.
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}'; // ☝
    println!("{}\n{}\n{}", letter, number, finger);
    output_section_separator();
}

fn find_average() {
    println!("Find average:\n");

    let number_1 = 15;
    let number_2 = 2.4;
    let number_3 = 234;
    let sum:f64 = number_1 as f64 + number_2 + number_3 as f64;
    let average = sum / 3.0;

    const EXPECTED:f64 = 83.8;
    assert_eq!(average, EXPECTED);
    println!("Average is {} as expected {}", average, EXPECTED);
    output_section_separator();
}

fn arrays() {
    println!("Arrays syntax:\n");

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
    output_section_separator();
}

fn two_d_arrays() {
    println!("2D Arrays syntax:\n");

    // 2d array, group of groups.
    // Inner dimensions must match or else error.
    let numbers = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    let get_first_val_of_first_array = numbers[0][0];
    println!("First value of the first array is {}", get_first_val_of_first_array);

    let garage: [[[i32; 100]; 20]; 5];
    // Initialize each group with zeros
    garage = [[[0; 100]; 20]; 5];

    println!("Initialized 3D array: {}", garage[0][0][0]);
    output_section_separator();
}

fn tuples() {
    println!("Tuples syntax:\n");

    // Can use a mix of data types.
    let things = (21, 3.14, 'e');
    let first_thing = things.0;

    println!("First thing in tuple: {}", first_thing);

    // Destructuring tuple
    let (_destructured_tuple_0, destructured_tuple_1, _destructured_tuple_2) = things;

    println!("Second thing in destructure tuple {}", destructured_tuple_1);
    output_section_separator();
}

// Rust doesn't care where you declare your functions/
fn use_input_function(input: i32, input_2: i32) {
    println!("Function input syntax:\n");

    println!("Input provided was summed to: {}", input + input_2);
    output_section_separator();
}

fn square_input() {
    println!("Function return syntax:\n");

    println!("Squared input: {}", square_input_value(2));
    output_section_separator();
}

fn square_input_value(x: i32) -> i32 {

    // Returns are the last line of a rust function
    x * x
}

fn return_early() {
    println!("Function return early syntax:\n");

    println!("Squared input: {}", return_early_function(2));
    output_section_separator();
}

fn return_early_function(x: i32) -> i32 {

    if x < 5 {
        return x;
    }

    // Returns are the last line of a rust function
    x * x
}

fn return_tuple() {
    println!("Function return tuple syntax:\n");

    // {:?} pattern will display tuple.
    println!("Squared input: {:?}", return_tuple_function(10));
    output_section_separator();
}

fn return_tuple_function(x: i32) -> (i32, i32) {
    if x > 5 {
        return (x, x * x);
    }

    // Returns are the last line of a rust function
    (0, 0)
}

fn convert_to_fahrenheit() {
    println!("Challenge convert to fahrenheit:\n");

    let converted_celsius = convert_from_celsius_to_fahrenheit(1.0);

    const EXPECTED: f64 = 33.8;
    assert_eq!(converted_celsius, EXPECTED);
    println!("Converted °C to °F: {}", converted_celsius);

    output_section_separator();
}


fn convert_from_celsius_to_fahrenheit(celsius: f64) -> f64 {


    // alternative formula
    //celsius * (9.0 / 5.0 + 32.0)
    celsius * (1.8 + 32.0)
}

fn condition_if() {
    println!("If syntax:\n");

    condition_if_return(3);
    condition_if_return(10);

    output_section_separator();
}

fn condition_if_return(x: i32) -> () {
    if x == 3 {
        println!("x is 3!");
    } else {
        println!("x is not 3!");
    }
}

fn conditional_assignment() {
    println!("Conditional assignment syntax:\n");

    let make_x_odd = true;
    // If is an expression in Rust so it can return a value.
    let x = if make_x_odd {1.0} else {2.0};
    println!("Squared input: {}", x);

    output_section_separator();
}

fn loops() {
    println!("Loops syntax:\n");

    let mut end_loop_at_end_limit = 0;

    // Loops in rust are expressions so they can return values.
    let loop_result = loop {



        if end_loop_at_end_limit == 10 {
            break 10 * 10;
        } else {
            end_loop_at_end_limit += 1;
            println!("Working: {}", end_loop_at_end_limit);
        }
    };

    println!("Ended loop: {}, {}", end_loop_at_end_limit, loop_result);

    output_section_separator();
}

fn while_loop() {
    println!("While loops syntax:\n");

    let mut end_loop_at_end_limit = 0;

    // Loops in rust are expressions so they can return values.
    while end_loop_at_end_limit < 10 {

        end_loop_at_end_limit += 1;
        println!("Working: {}", end_loop_at_end_limit);
    };

    println!("Ended loop: {}", end_loop_at_end_limit);

    end_loop_at_end_limit = 0;
    let a_word = ['M', 'o', 'n', 'd', 'a', 'y'];
    while end_loop_at_end_limit < a_word.len() {
        print!("{}", a_word[end_loop_at_end_limit]);
        end_loop_at_end_limit += 1;
    }
    println!();
    output_section_separator();
}

fn for_loop() {
    println!("For loops syntax:\n");

    // Enhanced for loop.
    let a_word = ['M', 'o', 'n', 'd', 'a', 'y'];
    for letter in a_word {
        print!("{}", letter);
    }
    println!();

    // Create tuple from enumerated iterator.
    let a_word = ['M', 'o', 'n', 'd', 'a', 'y'];
    for (index, &item) in a_word.iter().enumerate() {
        println!("{}, {}", index, item);

        if item == 'a' {
            break;
        }

    }
    println!();

    // Iterates over 0 to 4.
    for number in 0..5 {
        println!("{}", number);

    }
    println!();

    output_section_separator();
}

fn nested_loop() {
    println!("Nested loops syntax:\n");

    let mut matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    // Multi dimensional array
    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
    }


    println!();

    output_section_separator();
}

//
fn calculate_mean_min_max(array: [i32; 14]) {
    println!("Challenge calculate mean/max/min:\n");

    // Mean = sum / length
    let mut sum = 0.0;
    for number in array {
        sum += number as f64;
    }

    let mean_average = sum / array.len() as f64;
    const EXPECTED_MEAN: f64 = 12.5;
    assert_eq!(EXPECTED_MEAN, mean_average);
    println!("Mean for the inputted array is: {}", mean_average);

    // Min
    let mut min = 0;
    for number in array {
        if number < min {
            min = number;
        }
    }

    const EXPECTED_MIN: i32 = -18;
    assert_eq!(EXPECTED_MIN, min);
    println!("Min for the inputted array is: {}", min);

    // Max
    let mut max = 0;
    for number in array {
        if number > max {
            max = number;
        }
    }

    const EXPECTED_MAX: i32 = 56;
    assert_eq!(EXPECTED_MAX, max);
    println!("Max for the inputted array is: {}", max);

    println!();
    output_section_separator();
}