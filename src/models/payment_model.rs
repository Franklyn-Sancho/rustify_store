use tokio_postgres::Client;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub payment_method: String,
    pub status: String,
    pub created_at: String,
}

impl Payment {
    /// Create a new payment
    pub async fn create_payment(
        client: &Client,
        order_id: Uuid,
        payment_method: String,
        status: String,
    ) -> Result<Payment, Box<dyn Error>> {
        let id = Uuid::new_v4();

        let row = client
            .query_one(
                "INSERT INTO payments (id, order_id, payment_method, status, created_at) 
                 VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP) 
                 RETURNING id, order_id, payment_method, status, created_at",
                &[&id, &order_id, &payment_method, &status],
            )
            .await?;

        Ok(Payment {
            id: row.get(0),
            order_id: row.get(1),
            payment_method: row.get(2),
            status: row.get(3),
            created_at: row.get(4),
        })
    }

    /// Retrieve a payment by order_id
    pub async fn get_payment(
        client: &Client,
        order_id: Uuid,
    ) -> Result<Option<Payment>, Box<dyn Error>> {
        let rows = client
            .query(
                "SELECT id, order_id, payment_method, status, created_at 
                 FROM payments WHERE order_id = $1",
                &[&order_id],
            )
            .await?;

        if let Some(row) = rows.get(0) {
            Ok(Some(Payment {
                id: row.get(0),
                order_id: row.get(1),
                payment_method: row.get(2),
                status: row.get(3),
                created_at: row.get(4),
            }))
        } else {
            Ok(None)
        }
    }

    /// Update payment status
    pub async fn update_payment_status(
        client: &Client,
        payment_id: Uuid,
        status: String,
    ) -> Result<bool, Box<dyn Error>> {
        let result = client
            .execute(
                "UPDATE payments SET status = $1 WHERE id = $2",
                &[&status, &payment_id],
            )
            .await?;

        Ok(result > 0)
    }
}
