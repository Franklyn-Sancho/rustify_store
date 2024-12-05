use tokio_postgres::Client;
use std::sync::Arc;

pub struct AppState {
    pub db_client: Arc<Client>,
}

impl AppState {
    // Função para inicializar o estado com o cliente do banco de dados
    pub fn new(client: Client) -> Self {
        AppState {
            db_client: Arc::new(client),
        }
    }
}
