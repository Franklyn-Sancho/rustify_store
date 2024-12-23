use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio_postgres::Client;
use uuid::Uuid;

/// Represents a payment made for an order.
#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub payment_method: String,
    pub status: String,
}

impl Payment {
    /// Creates a new payment for a given order.
    /// The payment is inserted into the database with a default 'pending' status.
    pub async fn create_payment(
        client: &Client,
        order_id: Uuid,
        payment_method: &str,
    ) -> Result<Payment, Box<dyn Error>> {
        // Generate a new UUID for the payment.
        let id = Uuid::new_v4();

        // The default status will be 'pending'.
        let status = "pending";

        // Insert the payment into the database.
        let row = client
            .query_one(
                "INSERT INTO payments (id, order_id, payment_method, status) 
                 VALUES ($1, $2, $3, $4) 
                 RETURNING id, order_id, payment_method, status",
                &[&id, &order_id, &payment_method, &status],
            )
            .await?;

        // Return the created payment.
        Ok(Payment {
            id: row.get(0),
            order_id: row.get(1),
            payment_method: row.get(2),
            status: row.get(3),
        })
    }

    pub async fn verify_payment_owner(
        client: &Client,
        payment_id: &Uuid,
        user_id: Uuid,
    ) -> Result<bool, tokio_postgres::Error> {
        let query = "
            SELECT COUNT(*)
            FROM payments
            INNER JOIN orders ON payments.order_id = orders.id
            WHERE payments.id = $1 AND orders.user_id = $2
        ";
    
        let row = client.query_one(query, &[payment_id, &user_id]).await?;
        Ok(row.get::<_, i64>(0) > 0)
    }
    

    pub async fn update_payment_status(
        client: &Client,
        payment_id: Uuid,
    ) -> Result<bool, Box<dyn Error>> {
        // Updates the payment status to "paid" or "completed"
        let result = client
            .execute(
                "UPDATE payments SET status = 'paid' WHERE id = $1",
                &[&payment_id],
            )
            .await?;

        // Returns true if the status was updated
        Ok(result > 0)
    }

    pub async fn update_payment_method(
        client: &Client,
        payment_id: Uuid,
        new_payment_method: &str,
    ) -> Result<bool, Box<dyn Error>> {
        // Updates the payment method in the database.
        let result = client
            .execute(
                "UPDATE payments SET payment_method = $1 WHERE id = $2",
                &[&new_payment_method, &payment_id],
            )
            .await?;

        // Returns true if the method was updated
        Ok(result > 0)
    }

    /// Retrieves a payment associated with a given order.
    pub async fn get_payment(
        client: &Client,
        order_id: Uuid,
    ) -> Result<Option<Payment>, Box<dyn Error>> {
        // Query the database to retrieve the payment for the given order ID.
        let rows = client
            .query(
                "SELECT id, order_id, payment_method, status 
                 FROM payments WHERE order_id = $1",
                &[&order_id],
            )
            .await?;

        // If a matching row is found, construct and return the Payment struct.
        if let Some(row) = rows.get(0) {
            Ok(Some(Payment {
                id: row.get(0),
                order_id: row.get(1),
                payment_method: row.get(2),
                status: row.get(3),
            }))
        } else {
            // Return None if no payment matches the given order ID.
            Ok(None)
        }
    }
}
