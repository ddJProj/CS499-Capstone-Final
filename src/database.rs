// database.rs
//
// Created by Edward Johnson 07/11/24
// SNHU - CS499 - Final Project
//

//! This module handle direct database communications. This is
//! in the form of using queries to update / add / remove data
//! from the database, and to perform retrievals of stored data
//! for local operations.

// Examples used:
// https://crates.io/crates/config
// https://api.rocket.rs/v0.4/rocket/config/struct.ConfigBuilder
// https://www.reddit.com/r/rust/comments/1akmv4j/whats_the_best_practice_for_configuration/
// imports items needed for defining external files for configuration/secrets
use config::{Config, File};
// imports the Path struct from the standard library path module
use std::borrow::Cow;
use std::env;
use std::path::{Path, PathBuf};

// imports the Queryable trait from the mysql crate, prelude module
use mysql::prelude::*;
// imports all public items from the mysql crate
use mysql::{params, OptsBuilder, Pool, SslOpts};

// imports all public items from the firm_models module
use crate::firm_models::*;
// imports necessary errors from errors module
use crate::errors::{ApplicationError, DatabaseError};
//
// ********************************************
// database.rs module definitions begin here:
// ********************************************
//

/// Represents the MySQL database connection
///
///encapsulates the mySQL database connection pool, and
///related configuration / connection details that are needed
///to communicate with the remote database.
///
///# Fields
///
///* `pool: Pool` - A connection pool used to manage db connections
///
#[derive(Clone, Debug)]
pub struct MySqlDatabase {
    pool: Pool,
    // populate with db related
}

impl MySqlDatabase {
    /// Creates a new instance of the MySqlDatabase
    /// This is the constructor function to initialize a new
    /// instance of the database struct. It pulls connection
    /// secrets from a config.toml file, configures the connection
    /// and establishes the new connection pool.
    ///
    ///# Returns
    ///
    ///* 'Result<Self, ApplicationError>' - The result of new connection, either:
    ///     'Ok(MySqlDatabase)' - The new, successfully established db connection
    ///     'ApplicationError' - An error referencing the cause of the failure
    ///
    pub fn new() -> Result<Self, ApplicationError> {
        let (user, pw, host, port, db_name, cert_path) = if let Ok(env) = env::var("ENVIRONMENT") {
            if env == "remote" {
                // use for remote hoshing on oceanDigital
                (
                    // TODO: add more detailed handling such as with the following closure:
                    // .map_err(|e| ApplicationError::VarNAMEHERE(format!("MSGCONTENT: {}", e)))?,
                    env::var("DB_USERNAME")?,
                    env::var("DB_PASSWORD")?,
                    env::var("DB_HOST")?,
                    env::var("DB_PORT")?.parse::<u16>()?,
                    env::var("DB_NAME")?, // added for remote hosting
                    env::var("DB_CA_CERT")?,
                    //
                )
            } else {
                // local execution / developing
                let path_config = Path::new("config.toml");
                let config = Config::builder()
                    .add_source(File::from(path_config).required(true))
                    .build()?;

                (
                    // TODO: add more detailed handling such as with the following closure:
                    // .map_err(|e| ApplicationError::VarNAMEHERE(format!("MSGCONTENT: {}", e)))?,
                    config.get_string("database.username")?,
                    config.get_string("database.password")?,
                    config.get_string("database.host")?,
                    config.get_int("database.port")? as u16,
                    config.get_string("database.name")?,
                    config.get_string("database.ca_cert")?,
                )
            }
        } else {
            // local execution / developing
            let path_config = Path::new("config.toml");
            let config = Config::builder()
                .add_source(File::from(path_config).required(true))
                .build()?;

            (
                // TODO: add more detailed handling such as with the following closure:
                // .map_err(|e| ApplicationError::VarNAMEHERE(format!("MSGCONTENT: {}", e)))?,
                config.get_string("database.username")?,
                config.get_string("database.password")?,
                config.get_string("database.host")?,
                config.get_int("database.port")? as u16,
                config.get_string("database.name")?,
                config.get_string("database.ca_cert")?,
            )
        };

        let cert = PathBuf::from(cert_path); // https://doc.rust-lang.org/std/path/struct.PathBuf.html
                                             // https://docs.rs/mysql/latest/mysql/struct.SslOpts.html#
        let ssl_opts = SslOpts::default()
            // https://blog.logrocket.com/using-cow-rust-efficient-memory-utilization/
            .with_root_cert_path(Some(Cow::Owned(cert))) // https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html
            .with_danger_skip_domain_validation(false);

        // https://stackoverflow.com/questions/77823790/how-to-format-string-passed-to-poolnew-of-mysql-crate-in-rust
        let opts = OptsBuilder::new()
            .user(Some(user))
            .pass(Some(pw))
            .ip_or_hostname(Some(host))
            .tcp_port(port as u16)
            .db_name(Some(db_name))
            .ssl_opts(Some(ssl_opts));

        let pool = Pool::new(opts).map_err(|e| ApplicationError::ConfigError(e.to_string()))?;

        let _conn = pool
            .get_conn()
            .map_err(|e| ApplicationError::ConfigError(e.to_string()))?;

        Ok(MySqlDatabase { pool })
    }
}
/// definition for the interface that is used to manage
/// database based operations in the application
///
/// this trait contains the methods needed to manage all
/// of the operations that involve connecting to the remote
/// database, querying data, both as retrieval, and to
/// update or add values. the implementation will handle specifics.
///
pub trait DatabaseManager {
    /// init fn to create box clone of DatabaseManager implementation.
    fn clone_box(&self) -> Box<dyn DatabaseManager>;
    /// init fn to initiate a new transaction
    fn begin_transaction(&mut self) -> Result<(), DatabaseError>;
    /// init fn to commit the changes / current transaction
    fn commit_transaction(&mut self) -> Result<(), DatabaseError>;
    /// init fn to back data from this transaction
    fn rollback_transaction(&mut self) -> Result<(), DatabaseError>;
    /// init fn to get all clients from the database.
    fn get_clients(&self) -> Result<Vec<Client>, DatabaseError>;
    /// init fn to save new client to database.
    fn new_client(&mut self, client: &Client) -> Result<(), DatabaseError>;
    /// init fn to update client instance in database
    fn update_client(&mut self, client: &Client) -> Result<(), DatabaseError>;
    /// init fn to remove client instance from database
    fn remove_client(&mut self, client: &Client) -> Result<(), DatabaseError>;
    /// init fn to retrieve employee pass_hash from database
    fn get_employee_hash(&mut self, employee_id: i32) -> Result<Option<String>, DatabaseError>;
    /// init fn to save employee instance in database
    fn new_employee(&mut self, employee: &Employee) -> Result<(), DatabaseError>;
    /// init fn to update employee instance in database
    fn update_employee(&mut self, employee: &Employee) -> Result<(), DatabaseError>;
    /// init fn to remove employee instance from database
    fn remove_employee(&mut self, employee_id: i32) -> Result<(), DatabaseError>;
}

impl Clone for Box<dyn DatabaseManager> {
    /// remove employee instance from database
    ///
    /// # Arguments
    ///
    /// * `employee_id` - employee_id for this instance
    ///
    ///# Returns
    ///
    ///* 'Self' - Self, the cloned, boxed DatabaseManager
    ///
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl DatabaseManager for MySqlDatabase {
    /// Creates a new boxed clone of the implemented DatabaseManager
    ///
    /// # Arguments
    ///
    /// * `&self` - reference to self, the implementation of DatabaseManager
    ///
    ///# Returns
    ///
    ///* 'Box<dyn DatabaseManager>' - a new/cloned, boxed DatabaseManager
    ///
    fn clone_box(&self) -> Box<dyn DatabaseManager> {
        Box::new(self.clone())
    }

    /// initiates / begins new database transaction
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable reference to MySql database instance
    ///
    ///# Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///     on success:
    ///         Ok(()) - the transaction successfully initiated
    ///     on fail:
    ///         DatabaseError::ConnectionError - if db connection fails to init
    ///         DatabaseError::TransactionError - if the transaction query fails to init
    ///
    ///# Errors
    ///
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::TransactionError' - failure to execute transaction related query on database
    ///
    fn begin_transaction(&mut self) -> Result<(), DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        conn.query_drop("START TRANSACTION")
            .map_err(|e| DatabaseError::TransactionError(e.to_string()))
    }
    /// attempts to commit the transaction
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable reference to MySql database instance
    ///
    /// # Returns
    ///
    /// 'clients' vector of `Client` objects on success
    ///
    ///# Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///     on success:
    ///     on fail:
    ///         ConnectionError with failure to get pool/establish connection
    ///         TransactionError on failure to commit changes for the transaction
    ///
    ///# Errors
    ///
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::TransactionError' - failure to execute transaction related query on database
    ///
    fn commit_transaction(&mut self) -> Result<(), DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        conn.query_drop("COMMIT")
            .map_err(|e| DatabaseError::TransactionError(e.to_string()))
    }
    /// attempts to rollback the transaction
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable reference to MySql database instance
    ///
    ///# Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///     on success:
    ///         Ok(()) when the transaction successfully is rolled back
    ///     on fail:
    ///         ConnectionError when the database connection cannot be established
    ///         TransactionError on failure to commit changes for the transaction
    ///# Errors
    ///
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::TransactionError' - failure to execute transaction related query on database
    ///
    fn rollback_transaction(&mut self) -> Result<(), DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        conn.query_drop("ROLLBACK")
            .map_err(|e| DatabaseError::TransactionError(e.to_string()))
    }
    /// attempt to get all clients from the database.
    ///
    /// # Arguments
    ///
    /// * `&self` - reference to (self) MySql database instance
    ///
    /// # Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///* 'clients' vector of `Client` objects on success
    ///     on success:
    ///         Ok(Vec<Client>)- vector of all client objects & Ok status update
    ///     on fail:
    ///         ConnectionError when the database connection cannot be established
    ///         QueryError on failure to successfully process this select query
    ///
    ///# Errors
    ///
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::QueryError' - failure to execute query on the database
    ///
    fn get_clients(&self) -> Result<Vec<Client>, DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        let clients = conn
            .query_map(
                "SELECT client_id, client_name, client_service, assigned_employee FROM clients",
                |(client_id, client_name, client_service, assigned_employee)| {
                    Client::new(client_id, client_name, client_service, assigned_employee)
                },
            )
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        Ok(clients)
    }
    /// attempt to save new client to database.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable reference to MySql database instance
    /// * `client` - Client object instance to save
    ///
    ///# Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///     on success:
    ///         Ok(()) status update when client successfully inserted
    ///     on fail:
    ///         ConnectionError when the database connection cannot be established
    ///         QueryError on failure to successfully process this insert into query for clients table
    ///
    ///# Errors
    ///
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::QueryError' - failure to execute query on the database
    ///

    /// return `DatabaseError` if client fails to save
    // implement inserting new client row into database
    fn new_client(&mut self, client: &Client) -> Result<(), DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        conn.exec_drop(
            "INSERT INTO clients (client_name, client_service, assigned_employee) VALUES (?, ?, ?)",
            (
                client.get_client_name(),
                client.get_client_service(),
                client.get_asn_employee(),
            ),
        )
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
    }
    /// attempt to update client instance in database
    ///
    // TODO: client or client_id?
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable reference to MySql database instance
    /// * `client` - the updated Client instance
    ///
    ///# Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///     on success:
    ///         Ok(()) status update when client successfully updated
    ///     on fail:
    ///         ConnectionError when the database connection cannot be established
    ///         QueryError on failure to successfully process this update query for clients table
    ///
    /// # Errors
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::QueryError' - failure to execute query on the database
    ///
    /// return `DatabaseError` if client update fails
    // implement updating a client row (service / employee partner) in db
    fn update_client(&mut self, client: &Client) -> Result<(), DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        conn.exec_drop(
            "UPDATE clients SET client_name = ?, client_service = ?, assigned_employee = ? WHERE client_id = ?",
            (client.get_client_name(), client.get_client_service(), client.get_asn_employee(), client.get_client_id()),
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))
    }
    /// attempt to remove client instance from database
    ///
    // TODO: client or client_id?
    ///
    /// # Arguments
    ///
    /// * `client` - The `Client` instance to remove
    ///
    ///# Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///     on success:
    ///         Ok(()) status update when client successfully removed
    ///     on fail:
    ///         ConnectionError when the database connection cannot be established
    ///         QueryError on failure to successfully process this delete from query for clients table
    ///
    /// # Errors
    ///
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::QueryError' - failure to execute query on the database
    ///
    /// return `DatabaseError` if client removal fails
    // implement function to remove a client row from the db
    fn remove_client(&mut self, client: &Client) -> Result<(), DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        conn.exec_drop(
            "DELETE FROM clients WHERE client_id = ?",
            (client.get_client_id(),),
        )
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
    }
    /// attempt to retrieve employee pass_hash from database
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable reference to MySql database instance
    /// * `employee_id` - employee_id for this instance
    ///
    ///# Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///     on success:
    ///         Ok(()) status update when client successfully inserted
    ///     on fail:
    ///         ConnectionError when the database connection cannot be established
    ///         QueryError on failure to successfully process this insert into query for clients table
    ///
    ///  `Option<String>` with pass_hash on success
    ///
    /// # Errors
    ///
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::QueryError' - failure to execute query on the database
    ///
    /// return `DatabaseError` if failure to access database / could not find.
    // query database for the employee with ID
    // return their password hash if found, or None if not found
    fn get_employee_hash(&mut self, employee_id: i32) -> Result<Option<String>, DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        let result: Option<String> = conn
            .exec_first(
                "SELECT hashed_password FROM employees WHERE employee_id = :id",
                params! {
                "id" => employee_id
                },
            )
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        Ok(result)
    }
    /// attempt to save employee instance in database
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable reference to MySql database instance
    /// * `employee` - Employee instance to add to database
    ///
    ///# Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///     on success:
    ///         Ok(()) status update when employee successfully inserted
    ///     on fail:
    ///         ConnectionError when the database connection cannot be established
    ///         QueryError on failure to successfully process this insert into query for employee table
    ///
    /// # Errors
    ///
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::QueryError' - failure to execute query on the database
    ///
    /// return `DatabaseError` if employee fails to save
    fn new_employee(&mut self, employee: &Employee) -> Result<(), DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        conn.exec_drop(
            "INSERT INTO employees (employee_name, hashed_password) VALUES (:name, :password)",
            params! {
            "name" => employee.get_employee_name(),
            "password" => employee.get_employee_hash(),
            },
        )
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
    }
    /// attempt to update employee instance in database
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable reference to MySql database instance
    /// * `employee` - Employee instance to update in db
    ///
    ///# Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///     on success:
    ///         Ok(()) status update when employee successfully updated
    ///     on fail:
    ///         ConnectionError when the database connection cannot be established
    ///         QueryError on failure to successfully process this update query for employee table
    ///
    /// # Errors
    ///
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::QueryError' - failure to execute query on the database
    ///
    /// return `DatabaseError` if employee fails to update in db
    fn update_employee(&mut self, employee: &Employee) -> Result<(), DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        conn.exec_drop(
            "UPDATE employees SET employee_name = :name, hashed_password = :password WHERE employee_id = :id",
           params! {
            "name" => employee.get_employee_name(),
            "password" => employee.get_employee_hash(),
            "id" => employee.get_employee_id(),
            },
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))
    }
    /// attempt to remove employee instance from database
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable reference to MySql database instance
    /// * `employee_id` - employee_id for this instance
    ///
    ///# Returns
    ///
    ///* 'Result<(), DatabaseError> ' -
    ///     on success:
    ///         Ok(()) status update when employee successfully removed
    ///     on fail:
    ///         ConnectionError when the database connection cannot be established
    ///         QueryError on failure to successfully process this delete from query for employee table
    ///
    /// # Errors
    ///
    ///* 'DatabaseError::ConnectionError' - failure to establish connection to the database
    ///* 'DatabaseError::QueryError' - failure to execute query on the database
    ///
    fn remove_employee(&mut self, employee_id: i32) -> Result<(), DatabaseError> {
        let mut conn = self
            .pool
            .get_conn()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;
        conn.exec_drop(
            "DELETE FROM employees WHERE employee_id = :id",
            params! {
            "id" => employee_id,
            },
        )
        .map_err(|e| DatabaseError::QueryError(e.to_string()))
    }
}
