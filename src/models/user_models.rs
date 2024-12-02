use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use tokio_postgres::{types::ToSql, Client, Error};
use uuid::Uuid;

/// Struct that represents a user in the system.
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,        // Unique identifier for the user (UUID format).
    pub name: String,    // Name of the user.
    pub email: String,   // Email address of the user.
    pub password: String, // Hashed password of the user.
}

impl User {
    /// Checks if a given email already exists in the database.
    pub async fn email_exists(client: &Client, email: &str) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM users WHERE email = $1";
        let rows = client.query(query, &[&email]).await?;

        if let Some(row) = rows.get(0) {
            let count: i64 = row.get(0);
            Ok(count > 0)
        } else {
            Ok(false)
        }
    }

    /// Creates a new user and stores it in the database.
    pub async fn create_user(
        client: &Client,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<User, Error> {
        // Hash the user's password.
        let hashed_password = hash(password, DEFAULT_COST).unwrap();
        // Generate a new UUID for the user.
        let id = Uuid::new_v4();

        // Execute the insert query.
        client
            .execute(
                "INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)",
                &[&id, &name, &email, &hashed_password],
            )
            .await?;

        // Return the created user as a struct.
        Ok(User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            password: hashed_password,
        })
    }

    /// Retrieves a user from the database by their ID.
    pub async fn get_user(client: &Client, user_id: Uuid) -> Result<Option<User>, Error> {
        // Query the database for the user with the given ID.
        let rows = client
            .query(
                "SELECT id, name, email, password FROM users WHERE id = $1",
                &[&user_id],
            )
            .await?;

        if let Some(row) = rows.get(0) {
            // Construct and return the user if found.
            Ok(Some(User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                password: row.get(3),
            }))
        } else {
            Ok(None)
        }
    }

    /// Authenticates a user by verifying their email and password.
    pub async fn authenticate_user(
        client: &Client,
        email: &str,
        password: &str,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        // Query the database for the user by email.
        let rows = client
            .query(
                "SELECT id, name, email, password FROM users WHERE email = $1",
                &[&email],
            )
            .await?;

        if let Some(row) = rows.get(0) {
            // Construct the user struct from the query result.
            let user = User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
                password: row.get(3),
            };

            // Verify the provided password against the stored hash.
            if verify(password, &user.password)? {
                return Ok(Some(user));
            }
        }

        Ok(None)
    }

    /// Deletes a user from the database by their ID.
    pub async fn delete_user(client: &Client, user_id: Uuid) -> Result<bool, Error> {
        // Execute the delete query for the given user ID.
        let result = client
            .execute("DELETE FROM users WHERE id = $1", &[&user_id])
            .await?;

        // Return true if at least one row was affected, otherwise false.
        Ok(result > 0)
    }
}
