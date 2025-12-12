/*
You can trigger an unrecoverable error using the panic! macro, for example: panic!("This is an unrecoverable error!").

In this challenge, you will write a function that retrieves the value of an environment variable named DATABASE_URL if the environment variable is not set, you will exit the program using panic!, it makes sense to use panic! instead of Result<T, E> because the program will not be able to run database queries if this value isn't available.

Just to make it more interesting, there is another requirement, your function must validate that the value starts with postgresql://, this is not a full proof validation for real life applications, but we'll keep it simple for this challenge.
*/

pub fn get_database_url() -> String {
    let url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable is not set.");
    if url.starts_with("postgresql://") {
        return url;
    }
    panic!("DATABASE_URL must start with 'postgresql://'")
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
