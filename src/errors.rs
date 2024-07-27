// errors.rs
//
// Created by Edward Johnson 07/11/24
// SNHU - CS499 - Final Project
//

//! This module defines manual error handling for the cases that needed
//! more in depth error handling process established.

// imports the standard library fmt module
use std::fmt;
// imports Error items from thiserror module
use std::env::VarError;
use std::num::ParseIntError;
use thiserror::Error;

//
// ********************************************
// errors.rs module definitions begin here:
// ********************************************
//

/// represent various custom errors that can occur
///
/// This enum contains definition of various error types
/// that can occur during the operation of this application.
///
///# Variants
///
///* `PasswordHashError` - occurs from failure to hash
///* `PassValidationError` - occurs from failure to validate pass
///* `AuthenticationError` - occurs from failing to authenticate
///* `IoError` - when i/o related errors occur
///* `ConfigError` - when a configuration error occurs
///* `DatabaseError` - represents various database errors
///* `NoMatchError` - when a match cannot be found
///* `InputError` - occurs from input specific errors
#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Hashing of password failed: {0}")]
    PasswordHashError(String), // auth errors
    #[error("Error validating password: {0}")]
    PassValidationError(String),
    #[error("Authentication error occurred: {0}")]
    AuthenticationError(String),
    #[error("Io error occurred: {0}")]
    IoError(#[from] std::io::Error),
    #[error("A Configuration error occurred: {0}")]
    ConfigError(String),
    #[error("Database Error: {0}")]
    DatabaseError(DatabaseError), // db errors
    #[error("Error, no match was found: {0}")]
    NoMatchError(String),
    #[error("Input error occurred: {0}")]
    InputError(String),
    #[error("Error occured with an Env variable: {0}")]
    EnvirnVarError(String),
    #[error("Error occurredb when parsing integer: {0}")]
    ParseIntError(String),
}

/// represent database specific error types
///
/// DatabaseError enum contains the various database
/// specific errors that can occur during system operations.
///
///# Variants
///
///* `ConnectionError` - failure to establish database connection
///* `QueryError` - occurs from a query failure
///* `TransactionError` - transaction specific failure error
///* `NotFoundError` - when a value cannot be located in database
#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    TransactionError(String),
    NotFoundError(String),
}

impl fmt::Display for DatabaseError {
    /// Formatting for DatabaseError display
    ///
    /// This function provides the readable error messages for
    /// the various DatabaseErrors that can occur
    ///
    ///# Arguments
    ///
    ///* '&self' - reference to DatabaseError
    ///* 'f: &mut fmt::Formatter' - mutable ref to formatter
    ///
    ///# Returns
    ///
    ///* 'fmt::Result' - the Result / formatted message
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::ConnectionError(message) => {
                write!(f, "A Connection related error occurred: {}", message)
            }
            DatabaseError::QueryError(message) => {
                write!(f, "A Query related error occurred: {}", message)
            }
            DatabaseError::TransactionError(message) => {
                write!(f, "A Transaction related error occurred: {}", message)
            }
            DatabaseError::NotFoundError(message) => {
                write!(f, "A Not found error occurred: {}", message)
            }
        }
    }
}

impl std::error::Error for DatabaseError {}

impl From<DatabaseError> for ApplicationError {
    /// Converts DatabaseErrors into ApplicationErrors
    ///
    /// Implementation provides simple method to convert DatabaseErrors
    /// into ApplicationErrors
    ///
    ///# Arguments
    ///
    ///* 'err: DatabaseError' - The error of type DatabaseError to convert
    ///
    ///# Returns
    ///
    ///* 'ApplicationError' - Resulting ApplicationError
    ///
    ///
    fn from(err: DatabaseError) -> Self {
        ApplicationError::DatabaseError(err)
    }
}

impl From<config::ConfigError> for ApplicationError {
    /// Converts ConfigError into ApplicationErrors
    ///
    /// Provides simple method for converting config::ConfigError
    /// into ApplicationErrors
    ///
    ///# Arguments
    ///
    ///* 'err: config::ConfigError' - Error being converted
    ///
    ///# Returns
    ///
    ///* 'ApplicationError' - The result / ApplicationError
    fn from(err: config::ConfigError) -> Self {
        ApplicationError::ConfigError(err.to_string())
    }
}

impl From<mysql::Error> for ApplicationError {
    /// Converts mysql::Error into ApplicationErrors
    ///
    /// provides a simple method for converting mysql::Errors
    /// into ApplicationErrors
    ///
    ///# Arguments
    ///
    ///* 'err: mysql::Error' -
    ///
    ///# Returns
    ///
    ///* 'ApplicationError' - result as ApplicationError
    fn from(err: mysql::Error) -> Self {
        ApplicationError::DatabaseError(DatabaseError::ConnectionError(err.to_string()))
    }
}

impl From<VarError> for ApplicationError {
    /// Converts VarError::Error into ApplicationErrors
    ///
    /// provides a simple method for converting VarError::Errors
    /// into ApplicationErrors
    ///
    ///# Arguments
    ///
    ///* 'err: VarError::Error' -
    ///
    ///# Returns
    ///
    ///* 'ApplicationError' - result as ApplicationError

    fn from(err: VarError) -> Self {
        ApplicationError::EnvirnVarError(err.to_string())
    }
}

impl From<ParseIntError> for ApplicationError {
    /// Converts ParseIntError::Error into ApplicationErrors
    ///
    /// provides a simple method for converting ParseIntError::Errors
    /// into ApplicationErrors
    ///
    ///# Arguments
    ///
    ///* 'err: ParseIntError::Error' -
    ///
    ///# Returns
    ///
    ///* 'ApplicationError' - result as ApplicationError

    fn from(err: ParseIntError) -> Self {
        ApplicationError::ParseIntError(err.to_string())
    }
}
