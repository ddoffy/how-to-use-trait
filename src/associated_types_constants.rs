trait Database {
    type Connection;
    type Error;
    type QueryResult;

    const MAX_CONNECTIONS: usize = 100;
    const DEFAULT_TIMEOUT: u32 = 30;

    fn connect(&self, connection_string: &str) -> Result<Self::Connection, Self::Error>;
    fn excute_query(
        &self,
        conn: &Self::Connection,
        query: &str,
    ) -> Result<Self::QueryResult, Self::Error>;
    fn close_connection(&self, conn: &mut Self::Connection) -> Result<(), Self::Error>;
}

struct PostgresDatabase;

impl Database for PostgresDatabase {
    type Connection = PgConnection;
    type Error = String;
    type QueryResult = Vec<Row<u32>>;

    fn connect(&self, connection_string: &str) -> Result<Self::Connection, Self::Error> {
        println!(
            "Connecting to PostgreSQL with connection string: {}",
            connection_string
        );
        Ok(PgConnection {})
    }

    fn excute_query(
        &self,
        conn: &Self::Connection,
        query: &str,
    ) -> Result<Self::QueryResult, Self::Error> {
        println!("Executing query on {}: {}", conn, query);
        // Simulate query execution
        Ok(vec![
            Row { data: 1 },
            Row { data: 2 },
            Row { data: 3 },
            Row { data: 4 },
            Row { data: 5 },
            Row { data: 6 },
        ])
    }

    fn close_connection(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        println!("Closing connection: {}", conn);
        *conn = PgConnection {}; // Reset connection
        Ok(())
    }
}

struct PgConnection {}

impl std::fmt::Display for PgConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PgConnection")
    }
}

#[derive(Debug)]
struct Row<T> {
    pub data: T,
}

// Usage
pub fn demonstrate_database() {
    println!("Demonstrating database abstraction with associated types and constants");
    println!("Max connections: {}", PostgresDatabase::MAX_CONNECTIONS);
    println!(
        "Default timeout: {} seconds",
        PostgresDatabase::DEFAULT_TIMEOUT
    );

    let db = PostgresDatabase {};
    let connection_string = "host=localhost;user=postgres;password=secret;dbname=test";
    let conn = db
        .connect(connection_string)
        .expect("Failed to connect to database");
    let query = "SELECT * FROM users";

    let result = db
        .excute_query(&conn, query)
        .expect("Failed to execute query");

    println!("Query result: {:?}", result);
}
