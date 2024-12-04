use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio_postgres::Client;
use uuid::Uuid;

/// Represents an item in an order.
#[derive(Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: Decimal,
}

impl OrderItem {
    /// Checks if the requested quantity of a product is available in stock.
    /// Returns true if there is enough stock, otherwise false.
    pub async fn check_stock(
        client: &Client,
        product_id: Uuid,
        requested_quantity: i32,
    ) -> Result<bool, Box<dyn Error>> {
        // Query to retrieve the stock for the given product.
        let row = client
            .query_one("SELECT stock FROM products WHERE id = $1", &[&product_id])
            .await?;

        let stock: i32 = row.get(0);

        // Returns true if the stock is greater than or equal to the requested quantity.
        Ok(stock >= requested_quantity)
    }

    /// Creates an order item by first checking stock availability.
    /// If stock is sufficient, it inserts the order item and updates the product stock.
    pub async fn create_order_item(
        client: &Client,
        order_id: Uuid,
        product_id: Uuid,
        quantity: i32,
        price: Decimal,
    ) -> Result<OrderItem, Box<dyn Error>> {
        // Check if the product is in stock.
        let is_in_stock = Self::check_stock(client, product_id, quantity).await?;

        // Return an error if stock is insufficient.
        if !is_in_stock {
            return Err("Insufficient stock for the product".into());
        }

        // If stock is available, create the order item.
        let id = Uuid::new_v4();

        // Insert the order item into the database and retrieve the created item.
        let row = client
            .query_one(
                "INSERT INTO order_items (id, order_id, product_id, quantity, price) 
                 VALUES ($1, $2, $3, $4, $5) 
                 RETURNING id, order_id, product_id, quantity, price",
                &[&id, &order_id, &product_id, &quantity, &price],
            )
            .await?;

        // Decrement the product stock by the quantity ordered.
        client
            .execute(
                "UPDATE products SET stock = stock - $1 WHERE id = $2",
                &[&quantity, &product_id],
            )
            .await?;

        // Return the newly created OrderItem.
        Ok(OrderItem {
            id: row.get(0),
            order_id: row.get(1),
            product_id: row.get(2),
            quantity: row.get(3),
            price: row.get(4),
        })
    }

    /// Retrieves all items associated with a specific order.
    pub async fn get_order_items(
        client: &Client,
        order_id: Uuid,
    ) -> Result<Vec<OrderItem>, Box<dyn Error>> {
        // Query to retrieve all order items for the given order.
        let rows = client
            .query(
                "SELECT id, order_id, product_id, quantity, price 
                 FROM order_items WHERE order_id = $1",
                &[&order_id],
            )
            .await?;

        // Map the result rows into a vector of OrderItem objects.
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

    /// Deletes an order item by its ID.
    /// Returns true if the item was successfully deleted, false otherwise.
    pub async fn delete_order_item(
        client: &Client,
        item_id: Uuid,
    ) -> Result<bool, Box<dyn Error>> {
        // Execute the deletion query.
        let result = client
            .execute("DELETE FROM order_items WHERE id = $1", &[&item_id])
            .await?;

        // Return true if any rows were deleted, otherwise false.
        Ok(result > 0)
    }
}
