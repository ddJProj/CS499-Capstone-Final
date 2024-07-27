// firm_models.rs
//
// Created by Edward Johnson 07/11/24
// SNHU - CS499 - Final Project
//

//! Defines the core objects used within the application.
//! These are the Employee struct, and the Client struct,
//! as well as the values and implemented functions
//! required for various data operations.

// imports public Authenticator items from the auth module
use crate::auth::*;
// imports all public items from the errors module
use crate::errors::ApplicationError;

//
// ********************************************
// firm_models.rs module definitions begin here:
// ********************************************
//

/// represents an employee within the system
///
/// contains all required information for a firm employee
/// within the system: id, name, and hashed password
///
///# Fields
///
///* `employee_id` - i32 integr value, unique employee identifier
///* `employee_name` - string, name of the employee
///* `hashed_password` - string, hashed password that was input
// declare and define employee struct
#[derive(Clone, Debug, PartialEq)]
pub struct Employee {
    employee_id: i32, // integer
    employee_name: String,
    hashed_password: String,
}

impl Employee {
    /// Creates a new instance of the Employee struct
    ///
    /// initializes new instance of Employee struct.  containing
    /// values required of an Employee object
    ///
    ///# Arguments
    ///
    ///* 'employee_id' - i32 integer value of an employee id
    ///* 'name' - reference to employee name string
    ///* 'password' - reference to string data "password"
    ///
    ///# Returns
    ///
    ///* 'Self' - returns static Employee object
    ///
    ///
    pub fn new(employee_id: i32, name: &str, password: &str) -> Result<Self, ApplicationError> {
        let hashed_password = Authenticator::hash_password(password)?;

        Ok(Employee {
            employee_id,
            employee_name: name.to_string(),
            hashed_password,
        })
    }

    // accessor method to return employee id value
    ///
    /// returns i32 integer value for the employee_id
    ///
    ///# Arguments
    ///
    ///* '&self' - reference to self
    ///
    ///# Returns
    ///
    ///* 'i32' - the 32-bit integer value of self.employee_id
    ///
    pub fn get_employee_id(&self) -> i32 {
        self.employee_id
    }

    /// accessor method to return ref to employee name
    ///
    /// returns reference to the string data stored by the
    /// String employee_id
    ///
    ///# Arguments
    ///
    ///* '&self' - a reference to self
    ///
    ///# Returns
    ///
    ///* '&str' - a reference to string data (self.name)
    ///
    pub fn get_employee_name(&self) -> &str {
        &self.employee_name
    }
    /// accessor method to return ref to employee pass hash
    ///
    /// returns reference to the string data stored by the
    /// String hashed_password
    ///
    ///# Arguments
    ///
    ///* '&self' - a reference to self
    ///
    ///# Returns
    ///
    ///* '&str' - a reference to string data (self.hashed_password)
    ///
    pub fn get_employee_hash(&self) -> &str {
        &self.hashed_password
    }
}

// trait to allow access of id/key from AVL tree
pub trait Identification {
    fn get_key(&self) -> i32;
}

impl Identification for Client {
    /// get / accessor method to retrieve a key value
    ///
    /// key value for the AVL_tree. Uses unique 32-bit integer
    /// for each new client instance. used to sort the AVL tree
    /// key value for tree, is the client_id value of a Client
    ///
    ///
    ///# Arguments
    ///
    ///* '&self' - a reference to self
    ///
    ///# Returns
    ///
    ///* 'i32' - 32-bit integer value, the "key"/ client_id number
    ///
    fn get_key(&self) -> i32 {
        self.get_client_id()
    }
}

/// represents a Client within the application
///
/// Client structure encapsulates the data that the application
/// requires from each client in order to perform operations for
/// that client
///
///# Fields
///
///* `client_id` - i32 unique client_id integer value
///* `client_name` - owned String, this Client's name
///* `client_service` - i32 client_service integer value
///* `asn_employee_id` - i32 asn_employee_id integer value
///
/// # Examples
///
/// * 'Abraham James' - sample names taken from: https://homepage.net/name_generator/
///
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Client {
    client_id: i32, // integer
    client_name: String,
    client_service: i32,  // integer
    asn_employee_id: i32, // integer, id of the employee assigned to this client
}

// implement our client structure
impl Client {
    // public function to instantiate the client struct
    // similar to constructor in other OOP langs
    pub fn new(
        client_id: i32,
        client_name: String,
        client_service: i32,
        asn_employee_id: i32,
    ) -> Self {
        Client {
            client_id,
            client_name,
            client_service,
            asn_employee_id,
        }
    }
    /// set / mutator function for a client service
    ///
    /// used to set local client_service value
    ///
    ///# Arguments
    ///
    ///* '&self' - a reference to self
    ///
    ///# Returns
    ///
    ///* 'i32' - a 32-bit integer value (self.client_service)
    ///
    pub fn change_client_service(&mut self, service: i32) {
        self.client_service = service;
    }
    /// get / accessor method for Client.client_service
    ///
    /// returns the integer value stored in the
    /// 32-bit integer, self.client_service
    ///
    ///
    ///# Arguments
    ///
    ///* '&self' - a reference to self
    ///
    ///# Returns
    ///
    ///* 'i32' - a 32-bit integer value (self.client_service)
    ///
    pub fn get_client_service(&self) -> i32 {
        self.client_service
    }

    /// get / accessor method for Client.get_client_id
    ///
    /// returns the integer value stored in the
    /// 32-bit integer, self.get_client_id
    ///
    ///
    ///# Arguments
    ///
    ///* '&self' - a reference to self
    ///
    ///# Returns
    ///
    ///* 'i32' - a 32-bit integer value (self.get_client_id)
    ///
    pub fn get_client_id(&self) -> i32 {
        self.client_id
    }

    // get method like accessor in other OOP langs
    // returns i32 integer
    ///
    ///
    ///
    /// returns reference to the string data stored by
    /// owned String hashed_password
    ///
    ///# Arguments
    ///
    ///* '&self' - a reference to self
    ///
    ///# Returns
    ///
    ///* '&str' - a reference to string data (self.hashed_password)
    ///
    pub fn get_client_name(&self) -> &str {
        &self.client_name
    }

    /// get / accessor method for Client.asn_employee_id
    ///
    /// returns the integer value stored in the
    /// 32-bit integer, self.asn_employee_id
    ///
    ///# Arguments
    ///
    ///* '&self' - a reference to self
    ///
    ///# Returns
    ///
    ///* 'i32' - a 32-bit integer value (self.asn_employee_id)
    ///
    pub fn get_asn_employee(&self) -> i32 {
        self.asn_employee_id
    }

    /// mutator / set method for client_id
    ///
    /// sets local id value for a client to value provided from db.
    /// ids are auto generated by db on creation of instance & addition to the database.
    ///
    ///# Arguments
    ///
    ///* '&mut self' - a mutable reference to self
    ///* 'id' - a 32-bit integer value used to set self.client_id
    ///
    pub fn set_client_id(&mut self, id: i32) {
        self.client_id = id;
    }
}
