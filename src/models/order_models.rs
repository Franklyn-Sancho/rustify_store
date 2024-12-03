use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: String,
}

impl Order {
    pub async fn create_order(
        client: &Client,
        user_id: Uuid,
        status: String,
    ) -> Result<Order, Error> {
        // Generate a new UUID for the product.
        let id = Uuid::new_v4();

        // Execute the SQL insert query, returning the newly created product attributes.
        let row = client
            .query_one(
                "INSERT INTO order (id, user_id, status) 
                 VALUES ($1, $2, $3) 
                 RETURNING id, user_id, status",
                &[&id, &user_id, &status], // Pass Decimal directly.
            )
            .await?;

        // Construct and return the Product struct from the database response.
        Ok(Order {
            id: row.get(0),
            user_id: row.get(1),
            status: row.get(2),
        })
    }

    /// Retrieves a product from the database by its ID.
    pub async fn get_order(client: &Client, order_id: Uuid) -> Result<Option<Order>, Error> {
        // Query the database to fetch the product with the given ID.
        let rows = client
            .query(
                "SELECT id, user_id, status FROM order WHERE id = $1",
                &[&order_id],
            )
            .await?;

        // If a matching row is found, construct and return the Product struct.
        if let Some(row) = rows.get(0) {
            Ok(Some(Order {
                id: row.get(0),
                user_id: row.get(1),
                status: row.get(2),
            }))
        } else {
            // Return None if no product matches the given ID.
            Ok(None)
        }
    }

    /// Deletes a product from the database by its ID.
    pub async fn delete_order(client: &Client, order_id: Uuid) -> Result<bool, Error> {
        // Execute the SQL delete query for the specified product ID.
        let result = client
            .execute("DELETE FROM order WHERE id = $1", &[&order_id])
            .await?;

        // Return true if at least one row was affected (product deleted), otherwise false.
        Ok(result > 0)
    }
}
