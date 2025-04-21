// A set of focused traits that can be composed together to create a more complex behavior.
trait Authenticator {
    fn authenticate(&self, credentials: Credentials) -> bool;
}

trait Authorizer {
    fn authorize(&self, user_id: u32, resource: &str, action: &str) -> bool;
}

trait Logger {
    fn log(&self, level: LogLevel, message: &str);
}

struct Credentials {
    username: String,
    password: String,
}

enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

// Oauth2 authenticator struct
struct Oauth2Authenticator {
    client_id: String,
    client_secret: String,
}

impl Authenticator for Oauth2Authenticator {
    fn authenticate(&self, credentials: Credentials) -> bool {
        // Perform OAuth2 authentication
        println!(
            "Authenticating with OAuth2 for user: {}",
            credentials.username
        );
        true
    }
}

impl Authorizer for Oauth2Authenticator {
    fn authorize(&self, user_id: u32, resource: &str, action: &str) -> bool {
        // Perform OAuth2 authorization
        println!(
            "Authorizing user {} for action {} on resource {}",
            user_id, action, resource
        );
        true
    }
}

struct RoleBasedAuthorizer {
    // Would contain role definitions and permissions
}

impl Authorizer for RoleBasedAuthorizer {
    fn authorize(&self, user_id: u32, resource: &str, action: &str) -> bool {
        // Perform role-based authorization
        println!(
            "Authorizing user {} for action {} on resource {}",
            user_id, action, resource
        );
        true
    }
}

struct FileLogger {
    file_path: String,
}

impl Logger for FileLogger {
    fn log(&self, level: LogLevel, message: &str) {
        // Log to a file
        println!("Logging to file {}: {:?}", self.file_path, message);
    }
}

// A composite security system that uses multiple traits
struct SecuritySystem<A, Z, L> {
    authenticator: A,
    authorizer: Z,
    logger: L,
}

impl<A: Authenticator, Z: Authorizer, L: Logger> SecuritySystem<A, Z, L> {
    fn new(authenticator: A, authorizer: Z, logger: L) -> Self {
        SecuritySystem {
            authenticator,
            authorizer,
            logger,
        }
    }

    fn check_access(
        &self,
        credentials: Credentials,
        user_id: u32,
        resource: &str,
        action: &str,
    ) -> bool {
        if self.authenticator.authenticate(credentials) {
            if self.authorizer.authorize(user_id, resource, action) {
                self.logger.log(LogLevel::Info, "Access granted");
                return true;
            } else {
                self.logger.log(LogLevel::Warning, "Access denied");
            }
        } else {
            self.logger.log(LogLevel::Error, "Authentication failed");
        }
        false
    }
}

pub fn demonstrate_composition_and_extensions() {
    let oauth2_authenticator = Oauth2Authenticator {
        client_id: "client_id".to_string(),
        client_secret: "[your client secret key]".to_string(),
    };

    let role_based_authorizer = RoleBasedAuthorizer {};

    let file_logger = FileLogger {
        file_path: "log.txt".to_string(),
    };

    let security_system =
        SecuritySystem::new(oauth2_authenticator, role_based_authorizer, file_logger);

    let credentials = Credentials {
        username: "user".to_string(),
        password: "password".to_string(),
    };

    security_system.check_access(credentials, 1, "resource", "action");
}
