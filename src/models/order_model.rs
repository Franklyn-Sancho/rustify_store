use std::error::Error;

use serde::{Deserialize, Serialize};
use tokio_postgres::Client;
use uuid::Uuid;

/// Represents an order in the system.
#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: String,
}

impl Order {
    /// Creates a new order for a given user with a default 'pending' status.
    /// The order is inserted into the database and the order details are returned.
    pub async fn create_order(client: &Client, user_id: Uuid) -> Result<Order, Box<dyn Error>> {
        // Generate a new UUID for the order.
        let id = Uuid::new_v4();

        // Default status is set to 'pending'.
        let status = "pending";

        // Insert the new order into the database.
        let row = match client
            .query_one(
                "INSERT INTO orders (id, user_id, status) 
                 VALUES ($1, $2, $3) 
                 RETURNING id, user_id, status",
                &[&id, &user_id, &status],
            )
            .await
        {
            Ok(row) => row,
            Err(err) => {
                // Log error to help identify issues with the query.
                eprintln!("Error inserting order into database: {:?}", err);
                return Err(Box::new(err));
            }
        };

        // Return the created order.
        Ok(Order {
            id: row.get(0),
            user_id: row.get(1),
            status: row.get(2), // Status returned from the database
        })
    }

    pub async fn verify_order_owner(
        client: &Client,
        order_id: Uuid,
        user_id: Uuid,
    ) -> Result<bool, tokio_postgres::Error> {
        let query = "SELECT COUNT(*) FROM orders WHERE id = $1 AND user_id = $2";
        let row = client.query_one(query, &[&order_id, &user_id]).await?;
        Ok(row.get::<_, i64>(0) > 0)
    }

    /// Retrieves an order from the database by its ID.
    pub async fn get_order(
        client: &Client,
        order_id: Uuid,
    ) -> Result<Option<Order>, Box<dyn Error>> {
        // Query the database to retrieve the order by its ID.
        let rows = client
            .query(
                "SELECT id, user_id, status FROM orders WHERE id = $1",
                &[&order_id],
            )
            .await?;

        // If a matching row is found, construct and return the Order struct.
        if let Some(row) = rows.get(0) {
            Ok(Some(Order {
                id: row.get(0),
                user_id: row.get(1),
                status: row.get(2),
            }))
        } else {
            // Return None if no order matches the given ID.
            Ok(None)
        }
    }

    /// Deletes an order from the database by its ID.
    /// Returns true if the order was successfully deleted, otherwise false.
    pub async fn delete_order(client: &Client, order_id: Uuid) -> Result<bool, Box<dyn Error>> {
        // Execute the SQL delete query for the specified order ID.
        let result = client
            .execute("DELETE FROM orders WHERE id = $1", &[&order_id])
            .await?;

        // Return true if the order was deleted (i.e., if at least one row was affected).
        Ok(result > 0)
    }
}
