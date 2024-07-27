// main.rs
//
// Created by Edward Johnson 07/11/24
// SNHU - CS499 - Final Project
//
// https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html
// https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments
//

//! # SNHU Financial - Client Management System - Crate
//!
//! This is the primary / main module for SNHU Investment Firm's
//! client management system
//!
//! This application uses a MySQL remote database implementation, and
//! provides separate modules to meet the various needs of the firm.
//!
//! ## Dependencies
//!
//! * auth.rs - Contains authentication and cryptography related functions
//!     to the application. Including hashing of passwords, and authentication
//!     checks. Uses the [Argon2](https://docs.rs/argon2/latest/argon2/) crate.
//!     
//! * data_structs.rs - Contains data structures used to facilitate
//!     local operations within the application.
//!
//! * database.rs - Contains the MySQL database connection & implementation.
//!     Also provide the DatabaseManager trait / interface, to allow a variety
//!     of query operations to be performed on the database.
//!     Uses the [MySQL](https://docs.rs/mysql/latest/mysql/) crate.
//!
//! * errors.rs - Contains the various custom error definitions that are needed
//!     to handle the various results from operations within the application.
//!
//! * firm_models.rs - Contains the application's core structures, the Employee,
//!     and Client structs. Also includes the implemented functions that
//!     define their behaviors within the system.
//!
//! * menu.rs - Contains menu related application logic. Directs the flow of the
//!     information through application depending on user input.
//!
//! * operation_handlers.rs - Manages and controls the flow of data between the
//!     database and local data structures.
//!     Includes:
//!     - ClientHandler struct, with implemented function for managing Client
//!         related operations.
//!     - EmployeeHandler struct, with implemented function for managing Employee
//!         related operations.
//!     - Transaction struct, with implemented function for ensuring consistency
//!         of operations between local and remote data.
//!
//! * util.rs - Utility functions used for gathering, validating, and sanitizing
//!     user input.
//!
//!
//!

extern crate argon2;
extern crate config;
extern crate mysql;
extern crate rand;
extern crate regex_syntax;
extern crate thiserror;

mod auth;
mod data_structs;
mod database;
mod errors;
mod firm_models;
mod menu;
mod operation_handlers;
mod util;

use database::DatabaseManager;
use firm_models::Employee;

use crate::auth::login_handler;
use crate::database::MySqlDatabase;
use crate::errors::{ApplicationError, DatabaseError};
use crate::menu::Menu;

/// This is the main function
///
/// This function initializes the primary dependency for the application.
/// it calls the initial employee set up function, it proceeds to call the
/// login function, and if successful, then starts the main menu looping for
/// the application. Otherwise, the application closes.
///     Passes/injects dependency throughout the application with mutable db.
///
///# Returns
///
///* 'Result<()>' - On success, Ok() and result
///* 'Result<ApplicationError>' - on failure, returns ApplicationError
///
///# Errors
/// returns an ApplicationError when db initialization fails,
/// when login attempts fail, or if an error occurs
/// during main menu looping
///
fn main() -> Result<(), ApplicationError> {
    //  : type annotation for mutable db.
    //  Box containing trait object implementation of DatabaseManager
    //  assigned to a box containing new MySqlDatabase instance
    let mut db: Box<dyn DatabaseManager> = Box::new(MySqlDatabase::new()?);

    // call initial database seed method.
    // only generates initial employees when db empty
    initial_employee_setup(&mut *db)?;

    // if login_handler returns true
    if login_handler(&mut *db)? {
        // begin program's main menu looping
        let mut menu = Menu::new(db)?;
        menu.run()?;
    } else {
        println!("Login process failed. Goodbye.")
    }
    Ok(())
}
/// This function provides initial Employee seed to remote database
///
/// function will only execute database additions if it detects
/// that the database table for employees is empty.
/// If empty, any name + password combinations in the employees vector
/// will be used to seed the database.
/// If not empty, the function immediately returns with Ok result.
///
///# Arguments
///
///* 'database: &mut dyn DatabaseManager' - mut ref to object implementing DbManager.
///         form of dependency management / injection
///
///# Returns
///
///* 'Result<()>' - On success, Ok() and result
///* 'Result<ApplicationError>' - on failure, returns ApplicationError
///
///# Errors
///* 'DatabaseError::QueryError' - when duplicate employee found
///* 'ApplicationError::DatabaseError' - when separate database error occurs
///
fn initial_employee_setup(database: &mut dyn DatabaseManager) -> Result<(), ApplicationError> {
    // check if employee_id 1 exists, if Ok and Some, db not empty, return result Ok(())
    if let Ok(Some(_)) = database.get_employee_hash(1) {
        println!("Database was previously seeded! Use an existing account.");
        return Ok(());
    }
    // employees added to this vector will be added to the database if it is currently empty.
    // in the format shown. was used to insert initial test data,
    let employees = vec![
    //("name1", "password1"),
    //("name2", "password2"),
    ];
    // initiates a database transaction
    database.begin_transaction()?;

    // hondles the addition / db modification with closure
    let result: Result<(), ApplicationError> = (|| {
        for (e_name, e_password) in employees {
            // iterates through array,each name/pass pair
            // maps the name/pass to implemented Employee struct
            let employee = Employee::new(0, e_name, e_password)?;
            // attempts to add new Employee to db, matches result to one of the 3 outcomes
            match database.new_employee(&employee) {
                Ok(_) => println!("Added the employee: {} to database.", e_name),
                Err(DatabaseError::QueryError(e)) if e.contains("duplicate") => {
                    println!("That employee already exists: {}", e_name);
                    continue;
                }
                Err(e) => return Err(ApplicationError::DatabaseError(e)),
            }
        }
        Ok(()) // return result Ok
    })();

    if result.is_err() {
        // when any step of transaction generates an error, rollsback changes
        database.rollback_transaction()?;
    } else {
        // otherwise commit the changes when done
        database.commit_transaction()?;
    }
    result
}
