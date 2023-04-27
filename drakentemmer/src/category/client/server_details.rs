use async_trait::async_trait;

use crate::ClientApi;

use self::response::ServerDetailsResponse;

use super::AuthenticateClientRequest;

mod response;

#[async_trait]
pub trait ServerDetails {
    async fn get_server_details(&self, identifier: &str) -> eyre::Result<ServerDetailsResponse>;
}

#[async_trait]
impl ServerDetails for ClientApi {
    async fn get_server_details(&self, identifier: &str) -> eyre::Result<ServerDetailsResponse> {
        let _permit = self.drakentemmer.semaphore.acquire().await;

        let response: ServerDetailsResponse = self
            .drakentemmer
            .client
            .get(format!(
                "{}/api/client/servers/{identifier}",
                self.drakentemmer.base_url
            ))
            .authenticate_client_request(self)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}
