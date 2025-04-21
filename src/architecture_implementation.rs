// Core domain trait
trait User {
    fn id(&self) -> u32;
    fn username(&self) -> String;
}

trait AuthProvider {
    type Error;
    type Token;

    fn authenticate(&self, username: &str, password: &str) -> Result<Self::Token, Self::Error>;
    fn validate_token(&self, token: &Self::Token) -> Result<u32, Self::Error>;
}

trait UserRepository {
    type Error;

    fn find_by_id(&self, id: u32) -> Result<Box<dyn User>, Self::Error>;
    fn find_by_username(&self, username: &str) -> Result<Box<dyn User>, Self::Error>;
    fn save(&self, user: &dyn User) -> Result<(), Self::Error>;
}

// Implementation for specific technologies
struct PostgresUserRepository {
    connection_string: String,
}

impl UserRepository for PostgresUserRepository {
    type Error = String;

    fn find_by_id(&self, id: u32) -> Result<Box<dyn User>, Self::Error> {
        // Simulate a database lookup
        println!("Finding user by ID {} in PostgreSQL", id);
        Ok(Box::new(SimpleUser {
            id,
            username: format!("user{}", id),
        }))
    }

    fn find_by_username(&self, username: &str) -> Result<Box<dyn User>, Self::Error> {
        // Simulate a database lookup
        println!("Finding user by username {} in PostgreSQL", username);
        Ok(Box::new(SimpleUser {
            id: 1,
            username: username.to_string(),
        }))
    }

    fn save(&self, user: &dyn User) -> Result<(), Self::Error> {
        // Simulate saving to the database
        println!("Saving user {} to PostgreSQL", user.username());
        Ok(())
    }
}

struct SimpleUser {
    id: u32,
    username: String,
}

impl User for SimpleUser {
    fn id(&self) -> u32 {
        self.id
    }

    fn username(&self) -> String {
        self.username.clone()
    }
}

// Auth Service that depends on traits, not concrete implementations
struct AuthService<A, B>
where
    A: AuthProvider,
    B: UserRepository,
{
    auth_provider: A,
    user_repository: B,
}

impl<A, B> AuthService<A, B>
where
    A: AuthProvider,
    B: UserRepository,
{
    fn new(auth_provider: A, user_repository: B) -> Self {
        AuthService {
            auth_provider,
            user_repository,
        }
    }

    fn login(&self, username: &str, password: &str) -> Result<u32, A::Error> {
        let token = self.auth_provider.authenticate(username, password)?;
        let user_id = self.auth_provider.validate_token(&token)?;
        Ok(user_id)
    }

    fn get_user(&self, id: u32) -> Result<Box<dyn User>, B::Error> {
        self.user_repository.find_by_id(id)
    }
}

impl AuthProvider for PostgresUserRepository {
    type Error = String;
    type Token = String;

    fn authenticate(&self, username: &str, password: &str) -> Result<Self::Token, Self::Error> {
        // Simulate authentication
        println!("Authenticating user {} in PostgreSQL", username);
        if username == "admin" && password == "password" {
            Ok("token123".to_string())
        } else {
            Err("Invalid credentials".to_string())
        }
    }

    fn validate_token(&self, token: &Self::Token) -> Result<u32, Self::Error> {
        // Simulate token validation
        println!("Validating token {} in PostgreSQL", token);
        if token == "token123" {
            Ok(1)
        } else {
            Err("Invalid token".to_string())
        }
    }
}

pub fn demonstrate_architecture_implementation() {
    // Example usage
    let postgres_repo = PostgresUserRepository {
        connection_string: "postgres://user:password@localhost/db".to_string(),
    };

    let postgres_auth = PostgresUserRepository {
        connection_string: "postgres://user:password@localhost/db".to_string(),
    };

    let auth_service = AuthService::new(postgres_auth, postgres_repo);

    match auth_service.login("username", "password") {
        Ok(user_id) => println!("Logged in user ID: {}", user_id),
        Err(e) => println!("Login failed: {}", e),
    }

    match auth_service.get_user(1) {
        Ok(user) => println!("Found user: {} with ID {}", user.username(), user.id()),
        Err(e) => println!("Error finding user: {}", e),
    }
}
