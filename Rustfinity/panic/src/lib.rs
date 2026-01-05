pub fn get_database_url() -> String {
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => panic!("DATABASE_URL environment variable is not set.")

    };

    if !db_url.starts_with("postgresql://") {
        panic!("DATABASE_URL must start with 'postgresql://'");
    }

    db_url    // Your code here...
}

/// Example usage
pub fn main() {
    std::env::set_var("DATABASE_URL", "postgresql://localhost");

    let db_url = get_database_url();
    println!("Database URL: {}", db_url);

    std::env::remove_var("DATABASE_URL"); // Missing variable scenario
    get_database_url();

    std::env::set_var("DATABASE_URL", "mysql://localhost"); // Invalid prefix scenario
    get_database_url();
}
