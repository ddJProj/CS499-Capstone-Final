// menu.rs
//
// Created by Edward Johnson 07/11/24
// SNHU - CS499 - Final Project
//

//! This module implements the menu related interface for
//! managing clients and their service choices. Handles input operations

use std::result::Result;

// imports all public items from the database module
use crate::database::*;
// imports all public items from the util module
use crate::util::get_integer_input;
// imports all public items from the operation_handlers module
use crate::operation_handlers::*;
// imports all public items from the errors module
use crate::errors::ApplicationError;
//
// ********************************************
// menu.rs module definitions begin here:
// ********************************************
//

/// Represents the menu in the system
///
/// Contains dependencies for performing system operations
/// related to clients and employees. This is where the core
/// interactions between user interface inputs and data handlers
/// take place in the system.
///
///# Fields
///
///* `client_handler` - Dependency manages client specific operations
///* `employee_handler` - Dependency manages employee specific operations
///
#[allow(dead_code)] // since employee_handler is not actively used
pub struct Menu {
    client_handler: ClientHandler,
    employee_handler: EmployeeHandler,
}

impl Menu {
    /// Creates a new instance of the menu struct
    ///
    /// implements new instance of menu struct, containing
    /// dependencies needed for performing application operations,
    /// client_handler, and employee_handler.
    ///
    ///# Arguments
    ///
    ///* 'database' - boxed trait obj that implements DatabaseManager
    ///
    ///# Returns
    ///
    ///* 'Result<Ok(Self)>' - On success, returns Ok(Self / Menu implementation)
    ///* 'Result<DatabaseError>' - on failure, returns Err(DatabaseError)
    ///
    ///# Errors
    ///
    /// Error occurs if either dependencies fail to initialize,
    /// client_handler, or employee_handler
    ///
    pub fn new(database: Box<dyn DatabaseManager>) -> Result<Self, ApplicationError> {
        let client_handler = ClientHandler::new(database.clone_box())?;
        let employee_handler = EmployeeHandler::new(database)?;
        Ok(Self {
            client_handler,
            employee_handler,
        })
    }
    /// Executes looping for the main Menu system
    ///
    /// This method implements the primary menu looping operation,
    /// Provide user with possible actions, an directs the flow of
    /// data within the application. This function in run for the
    /// full lifecycle of the application.  Actions include:
    /// - Printing a client list
    /// - Changing client service choices
    /// - Exiting the application
    ///
    ///# Arguments
    ///
    ///* '&mut self' - a mutable reference to the menu struct
    ///
    ///# Returns
    ///
    ///* 'Result<Ok()>' - On success, returns Ok()
    ///* 'Result<ApplicationError>' - on failure, returns Err(ApplicationError)
    ///
    pub fn run(&mut self) -> Result<(), ApplicationError> {
        loop {
            self.display_menu();
            let menu_choice = get_integer_input()?;

            match MainMenuChoice::convert_i32(menu_choice) {
                Some(MainMenuChoice::PrintClientList) => {
                    if let Err(e) = self.display_clients_handler() {
                        println!("\nError displaying clients: {}", e);
                    }
                }
                Some(MainMenuChoice::ChangeServiceChoice) => {
                    if let Err(e) = self.change_service_handler() {
                        println!("\nError changing service: {}", e);
                    }
                }
                Some(MainMenuChoice::ChangeClientEmployeePair) => {
                    if let Err(e) = self.change_client_employee_pair() {
                        println!("\nError changing client pairing: {}", e);
                    }
                }
                Some(MainMenuChoice::ExitProgram) => {
                    println!("\nGoodbye.");
                    break;
                }
                _ => {
                    println!("\nSelect a valid menu option.");
                }
            }
        }
        Ok(())
    }
    /// Manages operations related to changing customer service choices
    ///
    /// Handles user input related to selecting individual clients by
    /// their id values. If a match is found, then calls the change
    /// service function Returns the result of the changes, or returned
    /// Error. For no match found, returns NoMatchError
    ///
    ///# Arguments
    ///
    ///* '&mut self' - Reference to mutable self
    ///
    ///# Returns
    ///
    ///* 'Ok(())' - when operation is successful.
    ///* 'Err(DatabaseError)' - when the client is not found.
    ///
    ///# Errors
    ///
    /// This function returns the error : DatabaseError::NotFoundError if
    /// the provided client_id does not match an existing client.
    ///
    fn customer_choice_handler(&mut self) -> Result<(), ApplicationError> {
        let client_id = get_integer_input()?;
        match self.client_handler.get_client(client_id) {
            Ok(client) => {
                let new_service = self.select_valid_service()?;
                if new_service != ClientServiceChoice::ReturnMenu {
                    let mut updated_client = client.clone();
                    updated_client.change_client_service(new_service as i32);
                    self.client_handler.update_client(&updated_client)?;
                }
                Ok(()) // ok result when client found
            }
            Err(e) => Err(e), // simplified to handle error props from tree
        }
    }
    /// change customer service choice manager
    ///
    /// outputs a confirmation message notifying the user of their
    /// menu selection option
    ///
    ///# Arguments
    ///
    ///* '&mut self' - Reference to mutable self
    ///
    ///# Returns
    ///
    ///* 'Ok(())' - when operation is successful.
    ///* 'Err(DatabaseError)' - when the client is not found.
    ///
    ///# Errors
    ///
    /// This function returns the error : DatabaseError::NotFoundError if
    /// the provided client_id does not match an existing client.
    ///
    fn change_service_handler(&mut self) -> Result<(), ApplicationError> {
        println!("\nYou chose option: Change Client Service Choice");
        println!("Please enter the client ID of the client you would like to modify.");
        self.customer_choice_handler()?;
        Ok(())
    }
    /// display clients manager function
    ///
    /// simply outputs a status message notifying the user of their selection
    /// before calling the display clients function
    ///
    ///# Arguments
    ///
    ///* '&mut self' - Reference to mutable self
    ///    
    ///
    ///# Returns
    ///
    ///* 'Ok(())' - when operation is successful.
    ///* 'Err(ApplicationError)' - on failure (input error/ application error)
    ///
    fn display_clients_handler(&mut self) -> Result<(), ApplicationError> {
        println!("\nYou chose option: Print a Client List");
        println!("Provide the employee ID whose client list you would like to return.");
        self.display_clients()?;
        Ok(())
    }

    /// change display choice for changing employee client pairing
    ///
    /// outputs a confirmation message notifying the user of their
    /// menu selection option, also outputs the initial request for
    /// a client's client id value.
    ///
    ///# Arguments
    ///
    ///* '&mut self' - Reference to mutable self
    ///
    ///# Returns
    ///
    ///* 'Result<Self, ApplicationError>' -
    ///     on success:
    ///         Ok(())
    ///     on fail:
    ///         ApplicationError - If an error occurs due to a failure at any point of the
    ///             call chain, it will be returned here

    ///# Errors
    ///
    /// This function returns the error : DatabaseError::NotFoundError if
    /// the provided client_id does not match an existing client.
    ///
    fn change_client_employee_pair(&mut self) -> Result<(), ApplicationError> {
        println!("\nYou chose option: Change Client / Employee pairing");
        println!(
            "Please enter the client ID of the client whose pairing you would like to change."
        );
        println!("You may also enter 0 to return to the previous menu.\n");
        self.client_pairing_handler()
    }

    /// Manages operations related to changing customer employee pairings
    ///
    /// Handles user input related to selecting individual clients by
    /// their id values. If a match is found, then calls the change
    /// employee pairing function. Returns Ok and result of the changes, or returned
    /// Error. For no match found, returns NoMatchError
    ///
    ///# Arguments
    ///
    ///* '&mut self' - Reference to mutable self
    ///
    ///# Returns
    ///
    ///* 'Result<(), ApplicationError>' -
    ///     on success:
    ///         Ok(()) - Return Ok and the result for any non-failure operation outcomes
    ///     on fail:
    ///         ApplicationError - If an
    ///             initialization of data structures, data operations, data retrieval, or
    ///             transactions
    ///
    ///# Behaviors
    ///
    ///* 1. Prompts user to provide a client_id integer
    ///* 2. attempts to locate an existing client object using ID value
    ///* 3. if located, prompts to provide the employee_id value for new pairing
    ///* 4. if valid employee match found, updates the client's employee pairing
    ///* 5. updates the user with output related to their provided values
    ///
    fn client_pairing_handler(&mut self) -> Result<(), ApplicationError> {
        let client_id = get_integer_input()?;

        let client = match self.client_handler.get_client(client_id) {
            Ok(client_match) => client_match.clone(), // clone here, to not upset borrow checker
            Err(e) => {
                println!("An error occurred while locating the client: {}", e);
                return Ok(());
            }
        };

        let new_employee_id = self.get_new_pair_employee_id()?;

        if new_employee_id != 0 {
            let mut updated_client = client;
            updated_client.change_client_employee_pair(new_employee_id);

            match self.client_handler.update_client(&updated_client) {
                Ok(_) => {
                    println!(
                        "Client: {} is now paired with Employee: {}",
                        client_id, new_employee_id
                    );
                }
                Err(e) => {
                    println!("An error occurred while updating the client:{}", e);
                }
            }
        }
        Ok(())
    }

    /// new employee for client pairing selection function
    ///
    /// handle gathering of new employee_id for client-employee pairing changes
    /// checks that the employee is valid, and allows user to return to the
    /// previous menu without committing changes if desired.
    ///
    ///# Arguments
    ///
    ///* '&mut self' - mutable reference to self
    ///
    ///# Returns
    ///
    ///* 'Result<i32 integer, ApplicationError>' -
    ///     on success:
    ///         Ok(0) - ok status and return to previous menu selected
    ///         Ok(target_employee_id) - ok status and the valid, located employee_id
    ///     on fail:
    ///         ApplicationError - If an error occurs due to a failure at any point of these
    ///         operations, return the ApplicationError. This is only when errors occur, as
    ///         function will continue asking for valid input if invalid input provided.
    ///
    fn get_new_pair_employee_id(&mut self) -> Result<i32, ApplicationError> {
        loop {
            println!(
                "\nPlease enter the ID of the employee you would like to pair with the client. Enter 0 to return to previous menu."
            );
            let target_employee_id = get_integer_input()?;
            if target_employee_id == 0 {
                return Ok(0); // return to previous menu selected
            }

            match self
                .employee_handler
                .is_valid_employee_id(target_employee_id)
            {
                Ok(true) => return Ok(target_employee_id),
                Ok(false) => println!("Invalid ID provided. Please enter a valid employee ID, or 0 to return to previous menu."),
                Err(e) => {
                    println!("An error occurred while fetching employee ID: {}", e);
                    return Err(e);
                }
            }
        }
    }

    //
    //
    //
    //-------------------------------------------------------
    //-------------------------------------------------------
    //
    //

    /// console output function to generate clients list
    ///
    /// outputs a list of clients to console using the
    /// provided employe id number
    ///
    ///# Arguments
    ///
    ///* '&mut self' - Reference to mutable self
    ///# Returns
    ///
    ///* 'Ok(())' - when operation is successful.
    ///* 'Err(ApplicationError)' - on failure (input error/ application error)
    ///
    ///# Behavior
    ///
    /// 1. calls the integer input method to get an employee id number
    /// 2. if the employee id has a match, and there are clients assigned
    /// 3. prints header for clients list
    /// 4. iterates through the vector of clients associated with employee id
    /// 5. outputs each client's details to the console
    /// 6. else no clients found, outputs error, none found message
    ///
    fn display_clients(&mut self) -> Result<(), ApplicationError> {
        let employee_id = get_integer_input()?;
        match self.client_handler.get_clients_for_employee(employee_id) {
            Some(client_ids) => {
                println!("\nClients for Employee ID: {}", employee_id);
                println!("ID# | Client's Name | Service Selected (1 = Brokerage, 2 = Retirement)");
                println!("¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯");

                for &client_id in client_ids {
                    match self.client_handler.get_client(client_id) {
                        Ok(client) => {
                            println!(
                                "{}.  | {}   selected option {}",
                                client.get_client_id(),
                                client.get_client_name(),
                                client.get_client_service()
                            );
                        }
                        Err(_e) => {
                            // handles error prop from avltree returns
                            println!("\nWarning: Client with ID {} not found", client_id);
                        }
                    }
                }
            }
            None => {
                println!("\nNo clients found for Employee ID: {}", employee_id);
            }
        }
        Ok(())
    }

    /// main menu console display / output function
    ///
    /// outputs the list of possible menu choices to the console
    ///
    ///# Arguments
    ///
    ///* '&self' - Reference to self
    ///
    fn display_menu(&self) {
        println!("\nWhat would you like to do?");
        println!("DISPLAY the client list (enter 1)");
        println!("CHANGE a client's choice (enter 2)");
        println!("CHANGE a client's employee partner (enter 3)");
        println!("Exit the program.. (enter 4)");
        println!("\nPlease provide a selection matching a valid menu option. ");
    }

    /// client service selection sub-menu function
    ///
    /// sub-menu to handle outputting possible sub-menu selections, as well as
    /// gathering and processing input for sub-menu actions related to selecting
    /// a new service choice for a client
    ///
    ///# Arguments
    ///
    ///* '&self' - Reference to self
    ///
    ///# Returns
    ///
    ///* 'ClientServiceChoice' - the selected ClientServiceChoice enum variant
    ///
    fn select_valid_service(&self) -> Result<ClientServiceChoice, ApplicationError> {
        loop {
            println!("\nClient Service Sub-menu - Options include:");
            println!("0: Return to previous menu");
            println!("1: Brokerage");
            println!("2: Retirement");
            println!(
                "\nPlease enter the client's new service choice, or 0 to return to previous menu."
            );
            let service_choice = get_integer_input()?;

            if let Some(service) = ClientServiceChoice::convert_i32(service_choice) {
                return Ok(service);
            } else {
                println!("Valid options are: 0, 1, or 2. Please enter a valid selection.");
            }
        }
    }
}

/// The constant / enum values for handling menu options
///
/// Enum containing definition of constant values for the
/// various main menu options. Used to map user inputs
/// to specific menu actions.
///
///# Variants
///
///* `DefaultMenuValue` - default menu choice (-1)
///* `PrintClientList` - option to print client list (1)
///* `ChangeServiceChoice` - option to change client service (2)
///* `ExitProgram` - option for exit program (3)
///
#[derive(Clone, Debug, PartialEq)]
pub enum MainMenuChoice {
    DefaultMenuValue = -1,
    PrintClientList = 1,
    ChangeServiceChoice = 2,
    ChangeClientEmployeePair = 3,
    ExitProgram = 4,
}

impl MainMenuChoice {
    /// Converts user input value to enum variant.
    ///
    /// Takes user input and then attempts to match it to one of the defined
    /// enum variant/constant for MainMenuChoice.
    ///
    ///# Arguments
    ///
    ///* 'value' - i32 integer user input value for menu choice
    ///
    ///# Returns
    ///
    ///* 'Some(MainMenuChoice)' - for matching explicitly defined enum variant
    ///* 'None' - returns None when no match to a defined enum variant
    ///
    pub fn convert_i32(value: i32) -> Option<MainMenuChoice> {
        match value {
            -1 => Some(MainMenuChoice::DefaultMenuValue),
            1 => Some(MainMenuChoice::PrintClientList),
            2 => Some(MainMenuChoice::ChangeServiceChoice),
            3 => Some(MainMenuChoice::ChangeClientEmployeePair),
            4 => Some(MainMenuChoice::ExitProgram),
            _ => None,
        }
    }
}
/// The constant / enum values for handling menu options
///
/// Enum containing definition of constant values for the
/// various client service options. Used to map user inputs
/// to specific menu actions.
///
///# Variants
///
///* `ReturnMenu` - return to prev menu option (0)
///* `Brokerage` - option for select service, brokerage (1)
///* `Retirement` - option for select service, retirement (2)
///
#[derive(Clone, Debug, PartialEq)]
pub enum ClientServiceChoice {
    ReturnMenu,
    Brokerage,
    Retirement,
}

impl ClientServiceChoice {
    /// Converts user input value to enum variant.
    ///
    /// Takes user input and then attempts to match it to one of the defined
    /// enum variant/constant for ClientServiceChoice.
    ///
    ///
    ///# Arguments
    ///
    ///* 'value' - user input i32 integer value for service choice
    ///
    ///# Returns
    ///
    ///* 'Some(ClientServiceChoice)' - for matching explicitly defined enum variant
    ///* 'None' - returns None when no match to a defined enum variant
    ///
    fn convert_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::ReturnMenu),
            1 => Some(Self::Brokerage),
            2 => Some(Self::Retirement),
            _ => None,
        }
    }
}
