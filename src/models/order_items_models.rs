use tokio_postgres::Client;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: f64,
}

impl OrderItem {
    /// Create a new order item
    pub async fn create_order_item(
        client: &Client,
        order_id: Uuid,
        product_id: Uuid,
        quantity: i32,
        price: f64,
    ) -> Result<OrderItem, Box<dyn Error>> {
        let id = Uuid::new_v4();

        let row = client
            .query_one(
                "INSERT INTO order_items (id, order_id, product_id, quantity, price) 
                 VALUES ($1, $2, $3, $4, $5) 
                 RETURNING id, order_id, product_id, quantity, price",
                &[&id, &order_id, &product_id, &quantity, &price],
            )
            .await?;

        Ok(OrderItem {
            id: row.get(0),
            order_id: row.get(1),
            product_id: row.get(2),
            quantity: row.get(3),
            price: row.get(4),
        })
    }

    /// Retrieve all items for a specific order
    pub async fn get_order_items(
        client: &Client,
        order_id: Uuid,
    ) -> Result<Vec<OrderItem>, Box<dyn Error>> {
        let rows = client
            .query(
                "SELECT id, order_id, product_id, quantity, price 
                 FROM order_items WHERE order_id = $1",
                &[&order_id],
            )
            .await?;

        Ok(rows
            .iter()
            .map(|row| OrderItem {
                id: row.get(0),
                order_id: row.get(1),
                product_id: row.get(2),
                quantity: row.get(3),
                price: row.get(4),
            })
            .collect())
    }

    /// Delete an order item by ID
    pub async fn delete_order_item(
        client: &Client,
        item_id: Uuid,
    ) -> Result<bool, Box<dyn Error>> {
        let result = client
            .execute("DELETE FROM order_items WHERE id = $1", &[&item_id])
            .await?;

        Ok(result > 0)
    }
}
