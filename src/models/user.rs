use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error};
use uuid::Uuid;

/// Struct that represents a user in the system.
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,         
    pub name: String,    
    pub email: String,    
    pub password: String, 
}

impl User {
    /// Creates a new user in the database.
    ///
    /// This method receives the user data (name, email, and password), hashes the password
    /// and inserts the information into the database. It returns the created user or an error in case of failure.
    pub async fn create_user(
        client: &Client,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<User, Error> {
        // Hashes the user's password
        let hashed_password = hash(password, DEFAULT_COST).unwrap();

        // Generates a unique UUID for the new user
        let id = Uuid::new_v4();

        // Inserts the new user into the database
        client
            .execute(
                "INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)",
                &[&id, &name, &email, &hashed_password],
            )
            .await?;

        // Returns the created user with the provided data
        Ok(User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            password: hashed_password,
        })
    }

    /// Retrieves a user by ID.
    ///
    /// This method queries the database to find the user with the provided ID.
    /// It returns the found user or `None` if the user is not found.
    pub async fn get_user(client: &Client, user_id: Uuid) -> Result<Option<User>, Error> {
        // Queries the database to get the user with the provided ID
        let rows = client
            .query(
                "SELECT id, name, email, password FROM users WHERE id = $1",
                &[&user_id], // Passing &user_id directly
            )
            .await?;

        // Checks if the user was found
        if let Some(row) = rows.get(0) {
            Ok(Some(User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                password: row.get(3),
            }))
        } else {
            Ok(None) // Returns None if the user was not found
        }
    }
}
