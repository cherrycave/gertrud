use reqwest::RequestBuilder;

use crate::ClientApi;

pub mod server_details;

trait AuthenticateClientRequest {
    fn authenticate_client_request(self, client_api: &ClientApi) -> Self;
}

impl AuthenticateClientRequest for RequestBuilder {
    fn authenticate_client_request(self, client_api: &ClientApi) -> Self {
        let token = client_api.client_api_key.to_string();
        self.bearer_auth(token)
    }
}
