use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error};
use uuid::Uuid;

/// Represents a product entity with its attributes.
#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,                    // Unique identifier for the product (UUID format).
    pub name: String,                // Name of the product.
    pub description: Option<String>, // Optional description of the product, can be null.
    pub price: Decimal,              // Price of the product, using Decimal for precision.
    pub stock: i32,                  // Quantity of the product available in stock.
}

impl Product {
    /// Creates a new product in the database and returns the created product.
    pub async fn create_product(
        client: &Client,
        name: &str,
        description: Option<&str>,
        price: Decimal,
        stock: i32,
    ) -> Result<Product, Error> {
        // Generate a new UUID for the product.
        let id = Uuid::new_v4();

        // Execute the SQL insert query, returning the newly created product attributes.
        let row = client
            .query_one(
                "INSERT INTO products (id, name, description, price, stock) 
                 VALUES ($1, $2, $3, $4, $5) 
                 RETURNING id, name, description, price, stock",
                &[&id, &name, &description, &price, &stock], // Pass Decimal directly.
            )
            .await?;

        // Construct and return the Product struct from the database response.
        Ok(Product {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            price: row.get(3),
            stock: row.get(4),
        })
    }

    /// Retrieves a product from the database by its ID.
    pub async fn get_product(client: &Client, product_id: Uuid) -> Result<Option<Product>, Error> {
        // Query the database to fetch the product with the given ID.
        let rows = client
            .query(
                "SELECT id, name, description, price, stock FROM products WHERE id = $1",
                &[&product_id],
            )
            .await?;

        // If a matching row is found, construct and return the Product struct.
        if let Some(row) = rows.get(0) {
            Ok(Some(Product {
                id: row.get(0),
                name: row.get(1),
                description: row.get(2),
                price: row.get(3),
                stock: row.get(4),
            }))
        } else {
            // Return None if no product matches the given ID.
            Ok(None)
        }
    }

    /// Deletes a product from the database by its ID.
    pub async fn delete_product(client: &Client, product_id: Uuid) -> Result<bool, Error> {
        // Execute the SQL delete query for the specified product ID.
        let result = client
            .execute("DELETE FROM products WHERE id = $1", &[&product_id])
            .await?;

        // Return true if at least one row was affected (product deleted), otherwise false.
        Ok(result > 0)
    }
}

