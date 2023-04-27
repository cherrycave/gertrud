use reqwest::RequestBuilder;

use crate::ClientApi;

trait AuthenticateApplicationRequest {
    fn authenticate_application_request(self, client_api: &ClientApi) -> Self;
}

impl AuthenticateApplicationRequest for RequestBuilder {
    fn authenticate_application_request(self, client_api: &ClientApi) -> Self {
        self.bearer_auth(client_api.client_api_key.to_string())
    }
}
