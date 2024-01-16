use std::{io, mem};
use std::io::{stdin, Write};
use rand::prelude::*;
use std::env;
use std::fs;
use std::cmp::PartialOrd;

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
    variable_shadowing();
    stack_info();
    heap_info();
    heap_string();
    variable_ownership();
    heap_arguments();
    heap_arguments_borrowing_references();
    slice();
    slice_as_function();
    trim_spaces_tests();
    //standard_io();
    //standard_io_and_cast();
    using_creates_rand();
    //higher_lower_game();
    //using_stdin_args();
    using_file_system_read_file();
    using_file_system_write_file();
    does_name_exist_in_txt_file();
    defining_structures();
    defining_methods();
    defining_associated_function();
    using_tuple_structs();
    create_rectangle_with_methods_and_new_ass_fn();
    generic_struct_definitions();
    generic_method_definitions();
    generic_function_definitions();
    using_box_data_type();
    using_boxed_objects();
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

fn variable_shadowing() -> () {
    println!("Variable shadowing:\n");

    // basically you can reassign the same variables
    // within scope without using mut.
    let shadow_variable = "Earth";
    println!("Planet is {}", shadow_variable);
    let shadow_variable = "Mars";
    println!("Planet is {}", shadow_variable);
    println!();
    output_section_separator();
}

fn stack_info() -> () {
    println!("Stack:\n");

    println!("Push and pop data very quickly.");
    println!("Access data very quickly.");
    println!("Small memory size.");
    println!("All data must have a known fixed size.");

    println!();
    output_section_separator();
}

fn heap_info() -> () {
    println!("Heap:\n");

    println!("Once memory is assigned, a pointer is returned.");
    println!("A pointer is a data type used to access the memory location.");
    println!("It is slower than the stack.");
    println!("Ability to dynamically add and remove data.");

    println!();
    output_section_separator();
}

fn heap_string() -> () {
    println!("Heap String:\n");

    let mut heap_string = String::from("Earth");
    println!("Message is {}", heap_string);
    heap_string.push_str(" is home.");
    println!("Message is {}", heap_string);

    println!();
    output_section_separator();
}

fn variable_ownership() -> () {
    println!("Ownership:\n");

    println!("Variables are responsible for freeing their own resources.");
    println!("Rules:");
    println!("1. Every value is owned by one and only one variable at a time.");
    println!("2. When the owning variable goes out of scope, the value is dropped(mem freed).");
    println!("Advantage:");
    println!("1. Safe, no memory leaks or invalid memory access bugs.");
    println!("2. Efficient because the compiler knows when to deallocate at compile time.");
    println!("Disadvantage:");
    println!("1. Requires understanding of ownership.");
    println!("2. Longer to pick up because of new language paradigm.");
    println!();

    // Example of ownership pointer move. (shadow copy)
    println!("Ownership example, move.");
    let outer_planet: String;
    {
        let inner_planet = String::from("Mercury");
        println!("Inner planet is {}", inner_planet);
        outer_planet = inner_planet;
    }
    println!("Outer planet is {}", outer_planet);
    println!();

    // Example of ownership pointer deep copy.
    println!("Ownership example, copy.");
    let outer_planet: String;
    {
        let inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone();
        println!("Inner planet is {}", inner_planet);
    }
    println!("Outer planet is {}", outer_planet);
    println!();

    println!();
    output_section_separator();
}

fn heap_arguments() -> () {
    println!("Heap Argument passing:\n");

    let mut heap_string = String::from("1st String");
    // Copy the data is one solution.
    // Changes won't be reflected.
    process_heap_argu(heap_string.clone());

    // Can't use the same pointer after this function
    // because the ownership has been transferred to
    // the function.
    // Returning another string will transfer the ownership
    // back.
    heap_string = process_heap_argu(heap_string);

    println!("Message is {}", heap_string);

    println!();
    output_section_separator();
}

fn process_heap_argu(str: String) -> String {
    println!("Passed Argument is {}", str);
    let new_return = String::from("2nd String");
    new_return
}

fn heap_arguments_borrowing_references() -> () {
    println!("Heap Argument passing w/ references(borrowing):\n");

    let mut heap_string = String::from("1st String");

    // Borrowing i.e References
    //
    // Access/modify data without taking ownership of it.
    // Create references using the borrow operator &.
    // Essentially a reference is a pointer to a pointer.
    //
    // RESTRICTION:
    // Once you create a mutable reference, you cannot create
    // any other references to it, within that scope.
    // Done for safety, prevents data races.
    let length = process_heap_argu_borrowing_references(&mut heap_string);

    println!("Message is {} with length of {}", heap_string, length);

    println!();
    output_section_separator();
}

fn process_heap_argu_borrowing_references(str: &mut String) -> usize {
    println!("Passed Argument is {}", str);
    str.push_str("(of many)");
    str.len()
}

// This is called a dangling reference.
// It will fail to compile because this heap String
// will be freed from memory before the reference
// can be used.
// fn return_reference_func() -> &String {
//     & String::from("Returning Str Reference")
// }

fn slice() {
    println!("Slice :\n");

    let heap_string = String::from("Greetings from Saturn!");

    // Reference to a contiguous section of a collection.
    // Commonly encountered as the string slice data type: &str
    // String literals are slices.
    // Length is in bytes.
    // Range indices must occur at valid UTF-8 character boundaries.
    let sliced_reference_of_message = &heap_string[15 ..heap_string.len()-1];
    println!("Last word is {}", sliced_reference_of_message);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets : &[i32] = &planets[..4];
    println!("Inner planets are {:?}", inner_planets);

    println!();
    output_section_separator();
}

fn slice_as_function() {
    println!("Slice :\n");

    let heap_string = String::from("Greetings from Saturn!");

    let sliced_reference_of_message = get_first_word(&heap_string[10..]);
    println!("First word is {}", sliced_reference_of_message);

    println!();
    output_section_separator();
}

fn get_first_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' { // Found a space at this index.
            return &str[..index];
        }
    }

    &str // If we got to here no space was found.
}

fn trim_spaces_tests() {
    println!("Challenge trim str reference :\n");

    println!("testing strings...");

    let test1 = "We need more space.";
    assert_eq!(trim_spaces(&test1), "We need more space.");

    let test2 = "    There's a space in front.";
    assert_eq!(trim_spaces(&test2), "There's a space in front.");

    let test3 = "There's a space in the rear.   ";
    assert_eq!(trim_spaces(&test3), "There's a space in the rear.");

    let test4 = "    We're surrounded by space!     ";
    assert_eq!(trim_spaces(&test4), "We're surrounded by space!");

    let test5 = "       ";
    assert_eq!(trim_spaces(&test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(&test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(&test7), "🚀");

    println!("All passed");

    println!();
    output_section_separator();
}

fn trim_spaces(str: &str) -> &str {

    let enumerated_bytes = str.as_bytes().iter().enumerate();
    let enumerated_bytes_in_rev = str.as_bytes().iter().enumerate().rev();


    let mut found_left:bool = false;
    let mut left_index: usize = 0;
    let mut found_right:bool = false;
    let mut right_index: usize = str.len();

    // two pointer algo, scan left and right each iteration, searching for spaces.
    for ( (x1, &x2), (y1, &y2)) in enumerated_bytes.zip(enumerated_bytes_in_rev) {

        if x2 != b' ' && !found_left {
            if x1 != 0 {
                left_index = x1;
            }
            found_left = true;
        }

        if y2 != b' ' && !found_right {
            if y1 != str.len() {
                right_index = y1 + 1;
            }

            found_right = true;
        }

        // If we found the point which there is no more spaces on each side return
        // new sliced reference of our &str argument.
        if found_left && found_right {
            return &str[left_index .. right_index];
        }

    }

    ""
}

fn standard_io() {
    println!("Standard I/O library :\n");

    println!("Please supply a message");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    println!("Entered message is: {}", buffer);

    println!();
    output_section_separator();
}

fn standard_io_and_cast() {
    println!("Standard I/O library casting :\n");

    println!("Please supply an integer to cast");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);

    let casted_number: i32 = buffer.trim().parse::<i32>().unwrap();

    println!("Entered integer is: {}", casted_number);

    println!();
    output_section_separator();
}

fn using_creates_rand() {
    println!("Using crates, Rand create :\n");

    let number = random::<f64>();

    println!("Random number is {}", number);

    let thread_local_rng = thread_rng().gen_range(1..11);
    println!("Random number between 1 and 10 is {}", thread_local_rng);

    println!();
    output_section_separator();
}

fn higher_lower_game() {
    println!("Challenge crates, Rand create :\n");

    let prize_number = thread_rng().gen_range(1..101);


    loop {
        println!("Guess a number between 1 and 100");
        let mut user_guess = String::new();
        stdin().read_line(&mut user_guess);

        let casted_number:i8 = user_guess.trim().parse().unwrap();

        if casted_number == prize_number {
            println!("Woo, you got got it!");
            break;
        }
        else if casted_number < prize_number {
            println!("Higher...");
        }
        else if casted_number > prize_number {
            println!("Lower...");
        }

    }

    println!();
    output_section_separator();
}

fn using_stdin_args() {
    println!("Using std command line args :\n");

    for (index, argument ) in env::args().enumerate() {
        println!("Argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    print!("Arg2 is {}",arg2);

    println!();
    output_section_separator();
}

fn using_file_system_read_file() {
    println!("Using fs crate to read file :\n");

    let file_contents = fs::read_to_string("planets.txt").unwrap();
    println!("Contents of txt file: {}", file_contents);

    for line in file_contents.lines() {
        println!("line is {}", line);
    }

    let file_contents_as_byte_vector = fs::read("planets.txt").unwrap();

    println!("Bytes contents are {:?}", file_contents_as_byte_vector);
    println!();
    output_section_separator();
}

fn using_file_system_write_file() {
    println!("Using fs crate to write file :\n");

    let mut moon_speech = String::new();
    moon_speech.push_str("We choose to go to the Moon in this decade\n");
    moon_speech.push_str("and do the other things,\n");
    moon_speech.push_str("not because they are easy,\n");
    moon_speech.push_str("but because they are hard.\n");

    const MOON_SPEECH_FILE_NAME: &str = "speech.txt";
    // Will replace existing files.
    // Writes all at once.
    fs::write(MOON_SPEECH_FILE_NAME, moon_speech);
    println!("Moon speech was written to speech.txt");

    const PLANET_NAMES_FILE_NAME: &str = "planets.txt";
    // OpenOptions allows us to configure how the
    // file will be opened.
    // In this case, append.
    let mut file = fs::OpenOptions::new().append(true).open(PLANET_NAMES_FILE_NAME).unwrap();

    // Reinstate Pluto
    file.write(b"\nPluto");

    println!();
    output_section_separator();
}

fn does_name_exist_in_txt_file() {

    // Needs to be less than 3 if this is being run from cargo
    if env::args().len() < 3 {
        eprintln!("This program does_name_exist_in_txt_file() requires two arguments: <file path> <search name>");
        println!();
        output_section_separator();
        return;
    }


    let file:String = env::args().nth(1).unwrap();
    let name_to_find:String = env::args().nth(2).unwrap();


    println!("Challenge: Does name {} exist in file {}:", name_to_find, file);

    let lines = fs::read_to_string(file).unwrap();

    let mut found_name = false;
    let mut line_counter = 0;
    for line in lines.lines() {
        line_counter += 1;

        if line == name_to_find {
            found_name = true;
            println!("Name {} exists on line {}", name_to_find, line_counter);
            break;
        }
    }

    if !found_name {
        println!("Failed to find {} within {}", name_to_find, found_name)
    }

    println!();
    output_section_separator();
}

#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn defining_structures() {
    println!("Creating and using structs");

    // Group multiple items of mixed data types.
    // Elements are named.

    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0f64
    };

    let mut vehicle_2 = Shuttle {
        name: String::from("Discovery"),
        crew_size: 6,
        ..vehicle
    };

    vehicle.name = String::from("Atlantis");
    println!("Shuttle values {:?}", vehicle);

    println!("Shuttle 2 values {:?}", vehicle_2);

    println!();
    output_section_separator();
}

impl Shuttle {
    fn get_name(&self) -> &str{
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str, crew_size: u8, propellant: f64) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: crew_size,
            propellant: propellant
        }
    }
}

fn defining_methods() {
    println!("Creating and using methods");

    // Subroutine associated with a struct.
    // Can have input parameters and a return value.
    // Declared using the fn keyword.
    // First parameter is a reference to the struct instance.

    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0f64
    };

    let name = vehicle.get_name();

    println!("Shuttle name is {}", name);

    vehicle.add_fuel(1000f64);
    println!("Shuttle propellant is {}", vehicle.propellant);

    println!();
    output_section_separator();
}

fn defining_associated_function() {
    println!("Creating and using methods");

    // Function associated with a struct data type.
    // Does not have a &self parameter.

    let mut vehicle = Shuttle::new(
        "Instance",
        7,
        835958.0f64
    );

    let name = vehicle.get_name();

    println!("Shuttle name is {}", name);

    println!();
    output_section_separator();
}

struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

fn get_y (p: Point) -> u8 {
    p.1
}

fn using_tuple_structs() {
    println!("Using tuple structs");

    // Store a collection of mixed data without named fields.
    // Distinguishable as a unique data type.

    let red = Color(255, 0, 0);

    println!("RED in RGB is {}", red.0);

    let point = Point(23, 55, 0);
    println!("Y in XYZ point is {}", point.1);

    println!();
    output_section_separator();
}

struct Rect {
    width: f64,
    height: f64
}

impl Rect {
    fn get_width(&self) -> f64 {
        self.width
    }

    fn get_height(&self) -> f64 {
        self.height
    }

    fn get_area(&self) -> f64 {
        self.height * self.width
    }

    fn scale(&mut self, scaler: f64) {
        self.width = self.width * scaler;
        self.height = self.height * scaler;
    }

    fn new(width: f64, height: f64) -> Rect {
        Rect {
            width: width,
            height: height
        }
    }
}

fn create_rectangle_with_methods_and_new_ass_fn() {
    println!("Challenge: Create rect with methods and associated fn");

    // Define a struct to represent a Rectangle.
    // Methods:
    // get_area() returns width * height.
    // scale() scales width and height by an input f64 value.

    let mut rect = Rect::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");

    println!();
    output_section_separator();
}

#[derive(Debug)]
struct RectOf<T, U> {
    width: T,
    height: U
}

fn generic_struct_definitions() {
    println!("Generic struct definitions");

    // Generics are a zero-cost abstraction.
    // Make programming easier without reducing runtime
    // performance.
    // Compiler uses a process called 'Monomorphization'
    // which replaces the generic placeholders with concrete
    // data types at compile time.

    let u32_rect = RectOf {
        width: 4,
        height: 34.2
    };

    println!("Rect vals are {:?}", u32_rect);

    println!();
    output_section_separator();
}

impl<T, U> RectOf<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }

    fn get_height(&self) -> &U {
        &self.height
    }
}

// Create a specific concrete type of RectOf
impl RectOf<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn generic_method_definitions() {
    println!("Generic method definitions");

    // Generics are a zero-cost abstraction.
    // Make programming easier without reducing runtime
    // performance.
    // Compiler uses a process called 'Monomorphization'
    // which replaces the generic placeholders with concrete
    // data types at compile time.

    let u32_rect = RectOf {
        width: 4u8,
        height: 3u8
    };

    println!("Rect vals are {:?}", u32_rect);
    println!("Rect width is {}", u32_rect.get_width());
    println!("Rect perimeter is {}", u32_rect.get_perimeter());

    println!();
    output_section_separator();
}

fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    }
    else {
        b
    }
}

fn generic_function_definitions() {
    println!("Generic functions definitions");

    // Generics are a zero-cost abstraction.
    // Make programming easier without reducing runtime
    // performance.
    // Compiler uses a process called 'Monomorphization'
    // which replaces the generic placeholders with concrete
    // data types at compile time.

    println!("Biggest val are {:?}", get_biggest(1.3, 2.6));

    println!();
    output_section_separator();
}

fn using_box_data_type() {
    println!("Box data types");

    // Store data on the heap.
    // Considered a smart pointer because it
    // adds additional functionality beyond references.
    // Box<T> has ownership of the data it points to.
    // When Box<T> goes out of scope it deallocates the
    // heap memory.

    // Use cases for Box<T>
    // Store a type whose size cannot be known at compile time.
    // Example: Recursive types.
    // Struct containing struct.
    // Transfer ownership of data rather than copy it on the stack.
    // Avoid copying large amounts of stack data.

    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 4,
        propellant: 2340f64
    };

    println!("Vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    // Is now only 8 bytes, data has been moved to the heap.
    // Pointer on stack 8 bytes.
    println!("Boxed Vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));
    // Data on heap 40 bytes.
    println!("Boxed Vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle));

    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("Unboxed Vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle));

    println!();
    output_section_separator();
}

fn sum_boxes<T : std::ops::Add<Output = T>>(box_1: Box<T>, box_2: Box<T>) -> Box<T>
{
    let unboxed_1 = *box_1;
    let unboxed_2 = *box_2;
    let summed = unboxed_1 + unboxed_2;
    Box::new(summed)
}

fn using_boxed_objects() {
    println!("Challenge: write a function to add 2 numbers stored within boxed objects.");

    // function name: sum_boxes.
    // input two Box<T> objects where T is a numeric type

    assert_eq!(*sum_boxes(Box::new(1), Box::new(2)), 3);
    assert_eq!(*sum_boxes(Box::new(3.14159), Box::new(2.71828)), 5.85987);
    println!("Tests passed!");

    println!();
    output_section_separator();
}