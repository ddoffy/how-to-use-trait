use std::fmt::{self, Display, Formatter};

// Base error type
trait Error: Display {
    fn code(&self) -> u32;
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// Specific error types
struct DatabaseError {
    code: u32,
    message: String,
    source: Option<Box<dyn Error>>,
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "DatabaseError {}: {}", self.code, self.message)
    }
}

impl Error for DatabaseError {
    fn code(&self) -> u32 {
        self.code
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|s| s.as_ref())
    }
}

impl DatabaseError {
    fn new(code: u32, message: String, source: Option<Box<dyn Error>>) -> Self {
        DatabaseError {
            code,
            message,
            source,
        }
    }
}

struct ValidationError {
    code: u32,
    message: String,
    source: Option<Box<dyn Error>>,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ValidationError {}: {}", self.code, self.message)
    }
}

impl Error for ValidationError {
    fn code(&self) -> u32 {
        400 // Bad request code
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|s| s.as_ref())
    }
}

// Service that uses these error types
struct UserService;

impl UserService {
    fn create_user(&self, username: &str, email: &str) -> Result<(), Box<dyn Error>> {
        if username.is_empty() {
            return Err(Box::new(ValidationError {
                code: 1001,
                message: "Username cannot be empty".to_string(),
                source: None,
            }));
        }

        if email.is_empty() {
            return Err(Box::new(ValidationError {
                code: 1002,
                message: "Email cannot be empty".to_string(),
                source: None,
            }));
        }

        // Simulate a database error
        Err(Box::new(DatabaseError {
            code: 2001,
            message: "Database connection failed".to_string(),
            source: None,
        }))
    }

    fn save_database(&self, username: &str, email: &str) -> Result<(), Box<dyn Error>> {
        // Simulate a database error
        if username == "admin" {
            return Err(Box::new(DatabaseError::new(
                2002,
                "Admin user cannot be created".to_string(),
                None,
            )));
        }

        // Simulate a validation error
        if email.is_empty() {
            return Err(Box::new(ValidationError {
                code: 1002,
                message: "Email cannot be empty".to_string(),
                source: None,
            }));
        }

        // Simulate a successful save
        Ok(())
    }
}

fn handle_error(err: &dyn Error) -> (u32, String) {
    let status_code = match err.code() {
        1001 | 1002 => 400, // Bad request
        2001 | 2002 => 500, // Internal server error
        _ => 500,
    };

    let message = err.to_string();
    let mut current_err = err.source();
    let mut cause_chain = Vec::new();

    // Build the cause chain
    while let Some(cause) = current_err {
        cause_chain.push(cause.to_string());
        current_err = cause.source();
    }

    // Format the full error response
    let full_message = if cause_chain.is_empty() {
        message
    } else {
        format!(
            "{}\nCaused by: {}",
            message,
            cause_chain.join("\nCaused by: ")
        )
    };

    (status_code, full_message)
}

pub fn demonstrate_error_handling_architecture() {
    let user_service = UserService;

    // Attempt to create a user
    match user_service.create_user("admin", "") {
        Ok(_) => println!("Successfully created user"),
        Err(err) => {
            let (status_code, message) = handle_error(err.as_ref());
            println!("Status code: {}, Message: {}", status_code, message);
        }
    }

    // Attempt to save the database
    match user_service.save_database("admin", "") {
        Ok(_) => println!("Successfully saved database"),
        Err(err) => {
            let (status_code, message) = handle_error(err.as_ref());
            println!("Status code: {}, Message: {}", status_code, message);
        }
    }

    // Attempt to create another user
    match user_service.create_user("johndoe", "") {
        Ok(_) => println!("Successfully created user"),
        Err(err) => {
            let (status_code, message) = handle_error(err.as_ref());
            println!("Status code: {}, Message: {}", status_code, message);
        }
    }

    // Attempt to save the database
    match user_service.save_database("johndoe", "jdoe@example.com") {
        Ok(_) => println!("Successfully saved database"),
        Err(err) => {
            let (status_code, message) = handle_error(err.as_ref());
            println!("Status code: {}, Message: {}", status_code, message);
        }
    }
}
