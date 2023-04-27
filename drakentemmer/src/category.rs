use crate::Drakentemmer;
pub mod application;
pub mod client;

pub trait GetClientApi {
    fn client(self, client_api_key: &str) -> ClientApi;
}

pub trait GetApplicationApi {
    fn application(self, application_api_key: &str) -> ApplicationApi;
}

pub struct ClientApi {
    drakentemmer: Drakentemmer,
    client_api_key: String,
}

#[allow(dead_code)]
pub struct ApplicationApi {
    drakentemmer: Drakentemmer,
    application_api_key: String,
}

impl GetClientApi for Drakentemmer {
    fn client(self, client_api_key: &str) -> ClientApi {
        ClientApi {
            drakentemmer: self,
            client_api_key: client_api_key.to_string(),
        }
    }
}

impl GetApplicationApi for Drakentemmer {
    fn application(self, application_api_key: &str) -> ApplicationApi {
        ApplicationApi {
            drakentemmer: self,
            application_api_key: application_api_key.to_string(),
        }
    }
}
