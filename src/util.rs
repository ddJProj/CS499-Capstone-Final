// util.rs
//
// Created by Edward Johnson 07/11/24
// SNHU - CS499 - Final Project
//

//! This module provides reusable utility functions to the application.
//! These take the form of input validation/sanitization methods.

use std::io::{self, Write};

use crate::errors::ApplicationError;

//
// ********************************************
// util.rs module definitions begin here:
// ********************************************
//

/// Gets a valid integer input from the user
///
/// Continues looping until the user provides a valid
/// 32-bit integer.
///
///# Returns
///
///* 'input_string / num' - user input value as an i32 integer
///
///# Behavior
///
/// 1. accept user [input](https://doc.rust-lang.org/std/io/struct.Stdin.html)
/// 2. attempt to parse input to 64-bit integer
/// 3. if succeed, check if 64-bit integer is in valid range of 32-bit integers
/// 4. if valid, return the input as i32 integer
/// 5. [wildcard/_ ](https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern) to catch any cases no explicitly handled.
///
///# Notes
/// - gracefully handles errors that may arise during input process
///
pub fn get_integer_input() -> Result<i32, ApplicationError> {
    // https://doc.rust-lang.org/std/keyword.loop.html
    // simple loop avoids need to manage condition variable
    loop {
        let mut input_string = String::new();

        print!("Enter your choice as an integer: ");
        // https://doc.rust-lang.org/std/macro.print.html
        std::io::stdout()
            .flush()
            .map_err(|e| ApplicationError::IoError(e))?;

        // begins by reading line into mut& input_string for matching
        match std::io::stdin().read_line(&mut input_string) {
            // input successful, assign it place of Ok(wildcard)
            // attempt parse/trim input to 64bit integer
            Ok(_) => match input_string.trim().parse::<i64>() {
                // valid i32 int is greater or equal to -2147483648 AND less or = to 2147483648,
                Ok(num) if num >= std::i32::MIN as i64 && num <= std::i32::MAX as i64 => {
                    return Ok(num as i32);
                }

                _ => {
                    println!("\nYou must enter a valid integer.");
                }
            },
            Err(e) => {
                return Err(ApplicationError::IoError(e));
            }
        }
    }
}

/**
* Checks to see if the provided input string contains
* invalid characters
*@return: boolean - input contains invalid chars - true or false
*/

/// invalid character detection function
///
/// compares each character in provided input string to the
/// list of invalid characters, if any match, returns true.
///
///# Arguments
///
///* 'input' - reference to the string input
///
///# Returns
///
///* 'boolean' - true / false result of .contains() comparison
///
fn invalid_input_chars(input: &str) -> bool {
    let invalid_characters = r#"\!$@()#%^&*<>/"\,.|;~`:' "#;
    // handled with any() into closure(arg c).contains(c) to validate
    input.chars().any(|c| invalid_characters.contains(c))
}

/// Gets a valid string input from the user
///
/// Continues looping until the user provides a valid,
/// non-empty string.
///
///# Returns
///
///* 'String' - user input string value
///
///# Behavior
///
/// 1. prompts for a user input
/// 2. reads in user input str
/// 3. remove / trim any unused whitespace chars
/// 4. perform non-empty validation step
/// 5. perform invalid chars validation step
/// 6. so long as input valid, return input as string
/// 7. else invalid input, give msg/reason, & restart loop
///
///# Notes
/// - gracefully handles errors that may arise during string input process
///
pub fn get_string_input() -> Result<String, ApplicationError> {
    loop {
        let mut user_input = String::new();
        print!("Enter your password input: ");
        // flushes console out to make sure request displays
        io::stdout()
            .flush()
            .map_err(|e| ApplicationError::IoError(e))?;
        // read input into user_input
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                // read is ok, using wildcard placeholder
                let trimmed = user_input.trim();
                if trimmed.is_empty() {
                    println!("\nInput cannot be empty. Please try again.");
                } else if invalid_input_chars(trimmed) {
                    println!("\nInput contains invalid characters. Please try again.");
                } else {
                    return Ok(trimmed.to_string());
                }
            }
            Err(e) => {
                return Err(ApplicationError::IoError(e));
            }
        }
    }
}
