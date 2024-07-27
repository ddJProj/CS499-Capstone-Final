// auth.rs
//
// Created by Edward Johnson 07/11/24
// SNHU - CS499 - Final Project
//

//! This module implements authentication related function, specifically
//! functions that implement the argon2 crate. This is used to provide
//! the validation of login credentials.

// imports the Config struct from the argon2 crate for hashing config
use argon2::Config;
// imports the Rng trait from rand crate to use in salt generation
use rand::Rng;
// imports process module from std library
use std::process;

//imports all public items from the operation_handlers module
use crate::operation_handlers::EmployeeHandler;
// imports necessary errors from errors module
use crate::errors::ApplicationError;
// imports all public items from the database module
use crate::database::DatabaseManager;
// imports all public items from the util module
use crate::util::{get_integer_input, get_string_input};

//
// ********************************************
// auth.rs module definitions begin here:
// ********************************************
//

/// struct represents the authentication process of the system
///
/// contains the fields necessary to process authentication attempts
///
///# Fields
///
///* `current_attempts` - i32 integer value, amount of authorization attempts used
///* `max_attempts` - i32 integer value, max allowed auth attempts
///
pub struct Authenticator {
    current_attempts: i32,
    pub max_attempts: i32,
}

impl Authenticator {
    /// Authenticator constructor function
    ///
    /// creates a base new instance of the Authenticator
    /// struct. Sets auth attempts to 0, and
    /// max attempts to 5
    ///
    ///# Returns
    ///
    ///* 'Self' - new instance of Authenticator implementation
    ///
    pub fn new() -> Self {
        Authenticator {
            current_attempts: 0,
            max_attempts: 5,
        }
    }

    /// Executes the authentication function for validating an
    /// employee's login attempt
    ///
    /// accepts employee's details, & attempts to validate their login attempt.
    /// also tracks their qty of login attempts, will exit if max value reached.
    ///
    ///# Arguments
    ///
    ///* '&mut self' - Reference to mutable self
    ///* 'employee_handler' - mutable reference to an implementation of EmployeeHandler
    ///* 'employee_id' - i32 integer value, employee_id
    ///* 'password' - reference to input password string
    ///
    ///# Returns
    ///
    ///* 'Result<Ok(true)>' -  Authentication succeeded in validating login attempt
    ///* 'Result<Ok(false)>' - login attempt failed (bad pass / id value)
    ///* 'Result<DatabaseError>' - an error occurred attempting to access database
    ///
    ///# Behavior
    /// 1. checks that attempts has not reached maximum allowed
    ///     if max reached, immediately terminates then application (exits)
    /// 2. increments attempt count
    /// 3. attempts to retrieve stored hash for provided id number
    /// 4. hash found: validates stored hash against hashed input password
    ///     hashes match: return Ok(true)
    ///     hashes dont match: return Ok(false)
    /// 5. hash not found: return Ok(false) (no matching employee)
    /// 6. return the result of authentication / validation attempt
    ///
    pub fn authenticate(
        &mut self,
        employee_handler: &mut EmployeeHandler,
        employee_id: i32,
        password: &str,
    ) -> Result<bool, ApplicationError> {
        if self.current_attempts >= self.max_attempts {
            println!("Maximum attempts reached, exiting program.");
            process::exit(1);
        }

        // increment attempts
        self.current_attempts += 1;

        // calls dbmanager get_emp_hash fn
        match employee_handler.get_employee_hash(employee_id)? {
            Some(stored_hash) => {
                Ok(argon2::verify_encoded(&stored_hash, password.as_bytes()).unwrap_or(false))
            }
            None => Ok(false),
        }
    }
    /// function used to hash user input password strings
    ///
    /// takes a user input string password, and processes it
    /// using argon2 hash_encoded. This generates a salt, and
    /// hash uses default argon2 config.
    ///
    ///# Arguments
    ///
    ///* 'password' - reference to user input password string
    ///
    ///# Returns
    ///
    ///* 'Result<String>' - return hashed string on success
    ///* 'Result<argon2::Error' - returns error on failure
    ///
    pub fn hash_password(password: &str) -> Result<String, ApplicationError> {
        let config = Config::default();
        argon2::hash_encoded(
            password.as_bytes(),
            &Authenticator::generate_salt(),
            &config,
        )
        .map_err(|e| ApplicationError::PasswordHashError(e.to_string()))
    }

    /// function to generate a salt for password hashing
    ///
    /// generates an array of 16 random 8-bit integers,
    /// (16 random integers with a value between 0-255)
    /// Used in the password hashing process by argon2
    ///
    ///# Returns
    ///
    ///* 'array of integers, [u8; 16]' - array of 16 random 8-bit integers
    ///
    ///
    fn generate_salt() -> [u8; 16] {
        rand::thread_rng().gen::<[u8; 16]>()
    }
}
/// function to manage the login process
///
/// loops 0 - max_attepmts times, accepting user input.
/// upon valid auth credentials provided, returns true.
/// else max_attempts reached return false.
///
///
///# Arguments
///
///* 'database' - ref to boxed implementation of DatabaseManager
///
///# Returns
///
///* 'Ok(true)' - when operation is successful.
///* 'Ok(false)' - when operation fails.
///* 'Err(OperationError)' - would likely return a OperationError::DatabaseError
///
///# Errors
///
/// This function returns the error : DatabaseError::NotFoundError if
/// the provided client_id does not match an existing client.
/// Could also return one of the other db errors as defined in database.rs
///
pub fn login_handler(database: &mut dyn DatabaseManager) -> Result<bool, ApplicationError> {
    let mut employee_handler = EmployeeHandler::new(database.clone_box())?;
    let mut authenticator = Authenticator::new();

    // iterates until max success or max_attempts reached
    // for any value in the range 0 - max_attempts
    // for _ in 0..authenticator.max_attempts {
    loop {
        println!("\nPlease enter your Employee ID number: ");
        let employee_id = get_integer_input()?; // to prop error if needed
        println!("\nPlease enter your Employee password: ");
        let password = get_string_input()?; // to prop error if needed

        match authenticator.authenticate(&mut employee_handler, employee_id, &password)? {
            true => {
                println!("\nEmployee successfully authenticated.");
                return Ok(true);
            }
            false => {
                println!(
                    "\nAuthentication attempt failed. You have used {} of {} attempts. \
                    Please try again.",
                    authenticator.current_attempts, authenticator.max_attempts
                );

                // upon reaching max attempts, returns false / ends program
                if authenticator.current_attempts >= authenticator.max_attempts {
                    println!("\nYou have reached the maximum allowed login attempts. Goodbye.");
                    return Ok(false);
                }
            }
        }
    }
}
