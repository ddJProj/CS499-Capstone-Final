// operation_handlers.rs
//
// Created by Edward Johnson 07/11/24
// SNHU - CS499 - Final Project
//

//! This module provides object handler definitions for the two primary
//! structures in the application, Employee, and Client.
//! This includes providing dependencies and consistent operations
//! throughout the program.

// imports the Box struct from the standard library boxed module
use std::boxed::Box;
// imports the HashMap struct from the standard library collections module
use std::collections::HashMap;

// imports all public items from the data_structs module
use crate::data_structs::*;
// imports all public items from the database module
use crate::database::*;
// imports all public items from the firm_models module
use crate::firm_models::*;

// imports all public items from the errors module
use crate::errors::ApplicationError;

//
// ********************************************
// operation_handlers.rs module definitions begin here:
// ********************************************
//
//
//
/// employee handler represented here.
///
/// Manages / handles / delegates all employee related operations
///
///# Fields
///
///* `stored_hashes: HashMap<i32, String>` - hashmap of stored employee id / password hash
///         pairings. Used to avoid need to keep querying the database during authentication.
///         retrieves the hash for an employee then uses it to validate the auth process
///
///* `database: Box<dyn DatabaseManager>` - box containing DatabaseManager implementation of db
///
pub struct EmployeeHandler {
    // Store employee hashes for ref when authorizing login
    stored_hashes: HashMap<i32, String>,
    // database connection to perform relevant employee operations
    database: Box<dyn DatabaseManager>,
}

impl EmployeeHandler {
    /// constructor function for the EmployeeHandler
    ///
    /// # Arguments
    ///
    /// * `database: Box<dyn DatabaseManager>` - The database manager implemented database
    ///
    ///# Returns
    ///
    ///* 'Result<Option<String>, ApplicationError>' -
    ///     on success:
    ///         Ok(()) - the instance of EmployeeHandler
    ///     on fail:
    ///         ApplicationError - the relevant Application error
    ///         
    // we will use a somewhat "lazy" approach to caching employee hashes.
    // load them as needed, and then store them locally
    pub fn new(database: Box<dyn DatabaseManager>) -> Result<Self, ApplicationError> {
        let stored_hashes = HashMap::new();
        Ok(Self {
            stored_hashes,
            database,
        })
    }

    /// retrieves a specific employee hash
    ///
    /// retrieval function for a specific employee hash, used in
    /// the authentication process
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable ref to instance of employehandler
    /// * `employee_id: i32` - employee id corresponding to the hash to retrieve
    ///
    ///
    ///# Returns
    ///
    ///* 'Result<Option<String>, ApplicationError>' -
    ///     on success:
    ///         Ok(()) - ok status,, and the requested hash
    ///     on fail:
    ///         ApplicationError - the relevant Application error
    ///         
    // we will use a somewhat "lazy" approach to caching employee hashes.
    // load them as needed, and then store them locally
    pub fn get_employee_hash(
        &mut self,
        employee_id: i32,
    ) -> Result<Option<String>, ApplicationError> {
        if let Some(hash) = self.stored_hashes.get(&employee_id) {
            return Ok(Some(hash.clone()));
        }
        match self.database.get_employee_hash(employee_id)? {
            Some(hash) => {
                self.stored_hashes.insert(employee_id, hash.clone()); // store the hash in hashmap
                Ok(Some(hash))
            }
            None => Ok(None),
        }
    }

    /// Add a new employee object to storage
    ///
    /// adds a new employee object to both the local storage
    /// structures, and the remote database
    ///
    /// # Arguments
    ///
    /// * `&mut self` - mutable reference to the employeehandler instance
    /// * `employee: &Employee` - the employee object to add to storage
    ///
    ///
    ///# Returns
    ///
    ///* 'Result<(), ApplicationError> ' -
    ///     on success:
    ///         Ok(()) - ok status, the employee operation was successful
    ///     on fail:
    ///         ApplicationError - the relevant Application error
    ///         
    pub fn add_new_employee(&mut self, employee: &Employee) -> Result<(), ApplicationError> {
        let transaction = Transaction::new(&mut self.database)?;
        transaction.db.new_employee(employee)?;
        self.stored_hashes.insert(
            employee.get_employee_id(),
            employee.get_employee_hash().to_string(),
        );
        transaction.commit()?;
        Ok(()) // ok status returned on success
    }

    ///function to modify the details of an employee
    ///
    ///updates/modifies an existing employee object in both
    ///local, and remote storage structures
    ///
    ///# Arguments
    ///
    /// * `&mut self` -mutable reference to self(EmployeeManager instance)
    /// * `employee: &Employee` -Reference to a specific Employee object

    ///# Returns
    ///
    ///* 'Result<(), ApplicationError> ' -
    ///     on success:
    ///         Ok(()) - ok status, the employee operation was successful
    ///     on fail:
    ///         ApplicationError - the relevant Application error
    ///         
    pub fn modify_employee(&mut self, employee: &Employee) -> Result<(), ApplicationError> {
        let transaction = Transaction::new(&mut self.database)?;
        transaction.db.update_employee(employee)?;
        self.stored_hashes.insert(
            employee.get_employee_id(),
            employee.get_employee_hash().to_string(),
        );
        transaction.commit()?;
        Ok(()) // ok status returned on success
               //
    }

    ///function used to remove/erase an employee from storage
    ///
    ///removes/erases an employee object from both local, and remote storage
    ///structures.
    ///
    ///# Arguments
    ///
    /// * `&mut self` -mutable reference to self(EmployeeManager instance)
    /// * `employee_id: i32` - employee_id that corresponds to the matching employee
    ///
    ///# Returns
    ///
    ///     on success:
    ///         Ok(()) - ok status, the employee operation was successful
    ///     on fail:
    ///         ApplicationError - the relevant Application error
    ///
    pub fn delete_employee(&mut self, employee_id: i32) -> Result<(), ApplicationError> {
        let transaction = Transaction::new(&mut self.database)?;
        transaction.db.remove_employee(employee_id)?;
        self.stored_hashes.remove(&employee_id);
        transaction.commit()?;
        Ok(()) // ok status returned on success
    }
}

/// Client handler represented here.
///
/// Manages / handles / delegates all client related operations
///
///# Fields
///
///* `local_avl_tree: AVLTree<Client>` - The primary local data storage object, an AVL tree of  Clients
///* `database: Box<dyn DatabaseManager>` - box containing DatabaseManager implementation of db
///* `employee_client_pairs: HashMap<i32, Vec<i32>>` - hashmap containing employe id keys /
///         a value vector of the clients they have assigned to them
///
pub struct ClientHandler {
    /// the local avltree built from clients in database
    local_avl_tree: AVLTree<Client>,

    /// local hashmap for O(1) employee / client pairings
    /// uses int <asn_employee_id> key, value is vector of int <client_id>s
    employee_client_pairs: HashMap<i32, Vec<i32>>,

    /// smart pointer to databaseManager
    database: Box<dyn DatabaseManager>,
}

/// https://doc.rust-lang.org/book/ch03-04-comments.html
/// https://doc.rust-lang.org/rust-by-example/meta/doc.html

impl ClientHandler {
    /// constructor for newclienthManager instance
    ///
    ///This function creates an instance of the ClientManager, retrieves all clients from the
    ///database, updates the local avl tree structure, as well as the hashmap of pairings
    ///containing employee keys and client value vectors
    ///
    /// # Arguments
    ///
    /// * `database: Box<dyn DatabaseManager>` - mutable reference to MySql database instance
    ///
    ///# Returns
    ///
    ///* 'Result<Self, ApplicationError>' -
    ///     on success:
    ///         Ok(()) - ok status and new ClientManager Instance
    ///     on fail:
    ///         ApplicationError - If an error9 occurs due to a failure at any point of the
    ///             initialization of data structures, data operations, data retrieval, or
    ///             transactions
    ///
    pub fn new(database: Box<dyn DatabaseManager>) -> Result<Self, ApplicationError> {
        let clients = database.get_clients().map_err(ApplicationError::from)?; // clients is the vector containing clients, or err
        let mut local_avltree = AVLTree::new();
        let mut employee_client_pairs = HashMap::new();
        for client in clients {
            employee_client_pairs
                .entry(client.get_asn_employee())
                .or_insert_with(Vec::new)
                .push(client.get_client_id());
            local_avltree.insert(client)?; // call insert method on each client
        }

        Ok(Self {
            local_avl_tree: local_avltree,
            database,
            employee_client_pairs,
        })
    }

    ///single client retrieval method
    ///
    ///retrieves a single client instance by the provided
    ///client_id value, if the client exists in the database/ structurs
    ///
    ///# Arguments
    ///
    /// * `&self` - reference to self (ClientManager instance)
    /// * `id: i32` - The target client id by which to locate a Client
    ///
    ///# Returns
    ///
    ///* 'Result<&Client, ApplicationError> ' -
    ///     on success:
    ///         Ok(()) - Ok status, and the matching Client object for provided ID
    ///     on fail:
    ///         ApplicationError - the relevant Application error such as NoMatchFound
    ///    
    pub fn get_client(&self, id: i32) -> Result<&Client, ApplicationError> {
        self.local_avl_tree.find(id)
    }

    /// client list by employee pair retrieval method
    ///
    /// Using a provided employee id, retrieves the client list
    /// for a specific employee.
    ///
    ///# Arguments
    ///
    /// * `&self` - reference to self (ClientManager instance)
    /// * `employee_id: i32` - the employee_id we are targetting
    ///
    ///# Returns
    ///
    ///* 'Option<&Vec<i32>>' -
    ///     on success:
    ///         Ok(()) - Ok status, and the vector containing all clients that are assigned
    ///                 to a particular employee
    ///     on fail:
    ///         ApplicationError - the relevant Application error
    ///    
    pub fn get_clients_for_employee(&self, employee_id: i32) -> Option<&Vec<i32>> {
        self.employee_client_pairs.get(&employee_id)
    }

    /// Updating an existing client in the database, and in local storage.
    ///
    /// uses the transaction system to update both the local and remote data sources
    /// for a specific Client instance
    ///
    ///# Arguments
    ///
    /// * `&mut self` - mutable reference to self(ClientMAnager instance)
    /// * `client: &Client` - Reference to a specific Client object
    ///
    ///# Returns
    ///
    ///* 'Result<(), ApplicationError> ' -
    ///     on success:
    ///         Ok(()) -
    ///     on fail:
    ///         ApplicationError - the relevant Application error
    ///    
    pub fn update_client(&mut self, client: &Client) -> Result<(), ApplicationError> {
        let transaction = Transaction::new(&mut self.database)?;
        transaction
            .db
            .update_client(client)
            .map_err(ApplicationError::from)?;
        self.local_avl_tree.remove(client.get_client_id())?;
        self.local_avl_tree.insert(client.clone())?;
        transaction.commit()?;

        Ok(()) // return okay
    }
    ///add new client object to data storage
    ///
    ///adds a new client object instance to both the remote database, and the
    ///local data structures.
    ///
    ///# Arguments
    ///
    /// * `&mut self` - mutable reference to self(ClientMAnager instance)
    /// * `client: &Client` - Reference to a specific Client object
    ///
    ///# Returns
    ///
    ///* 'Result<(), ApplicationError> ' -
    ///     on success:
    ///         Ok(()) -
    ///     on fail:
    ///         ApplicationError - the relevant Application error
    ///
    pub fn new_client(&mut self, client: &Client) -> Result<(), ApplicationError> {
        let transaction = Transaction::new(&mut self.database)?;
        transaction.db.new_client(client)?;
        self.local_avl_tree.insert(client.clone())?;
        transaction.commit()?;

        Ok(())
    }
    ///removes a client object from data storage
    ///
    ///removel a client object instance from both the remote database, and the
    ///local data structures.
    ///
    ///
    ///# Arguments
    ///
    /// * `&mut self` - mutable reference to self(ClientMAnager instance)
    /// * `client: &Client` - Reference to a specific Client object
    ///
    ///# Returns
    ///
    ///* 'Result<(), ApplicationError> ' -
    ///     on success:
    ///         Ok(()) -
    ///     on fail:
    ///         ApplicationError - the relevant Application error
    ///
    pub fn remove_client(&mut self, client: &Client) -> Result<(), ApplicationError> {
        let transaction = Transaction::new(&mut self.database)?;
        transaction.db.remove_client(client)?;
        self.local_avl_tree.remove(client.get_client_id())?;

        transaction.commit()?;
        Ok(())
    }
}

/* idea for this transaction system
// video : "This is why dependency injection is useful"
// https://www.youtube.com/watch?v=od3kAD4V9a4
//
// https://dev.mysql.com/doc/refman/8.4/en/commit.html
// https://www.mysqltutorial.org/mysql-stored-procedure/mysql-transactions/
// https://www.freecodecamp.org/news/how-to-use-mysql-transactions/
*/

/// Represents the transaction struct program data operations
///
/// The Transaction struct is created to help manage consistency of operations
/// and data between the database and local data strucures / storage.
///
///
///# Fields
///
///* `db: &'a mut Box<dyn DatabaseManager>` - mutable ref to boxed DatabaseManager
///     database implementation.
///
///* `completed: bool` - holds the status of the transaction
///
///
pub struct Transaction<'a> {
    db: &'a mut Box<dyn DatabaseManager>,
    completed: bool,
}

impl<'a> Transaction<'a> {
    /// constructor function for Transaction
    ///
    /// creates the new transaction for maintaining data changes
    /// and consistency between  local / remote sources
    ///
    ///# Arguments
    ///
    /// * `db: &'a mut Box<dyn DatabaseManager>` - mutable reference to boxed
    ///         DatabaseManager implemented database object
    ///
    ///# Returns
    ///
    ///* 'Result<Self, ApplicationError>' -
    ///     on success:
    ///         Ok(()) - ok status + new transaction
    ///     on fail:
    ///         ApplicationError - returned upon failure to generate new transaction
    ///    
    pub fn new(db: &'a mut Box<dyn DatabaseManager>) -> Result<Self, ApplicationError> {
        db.begin_transaction()?;
        Ok(Transaction {
            db,
            completed: false,
        })
    }

    ///function to commit a transaction instance
    ///
    ///finalizes a transaction and allows the changes to be committed
    ///
    ///# Arguments
    ///
    /// * `&mut self` - mutable reference to the transaction
    ///
    ///# Returns
    ///
    ///* 'Result<(), ApplicationError>' -
    ///     on success:
    ///         Ok(()) - the transaction successfully initiated
    ///     on fail:
    ///         ApplicationError - returned upon failure to commit the transactio
    ///                 and/or changes to the split data sources
    ///
    // Note: do not use reference to mutable here
    pub fn commit(mut self) -> Result<(), ApplicationError> {
        self.db.commit_transaction()?;
        self.completed = true;
        Ok(())
    }
}
impl<'a> Drop for Transaction<'a> {
    /// Drop the transaction if changes arent committed
    ///
    /// This function gets called  if a  particular transaction
    /// somehow goes out of scope. If the transaction did not commit,
    /// this functino will roll it back
    ///
    ///# Arguments
    ///
    /// * `&mut self` - mutable reference to the transaction
    ///
    ///# Returns
    ///
    ///
    fn drop(&mut self) {
        if !self.completed {
            let _ = self.db.rollback_transaction();
        }
    }
}
