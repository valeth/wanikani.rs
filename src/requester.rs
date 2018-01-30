use reqwest::{Client as ReqwestClient, Url};
use reqwest::header::{Authorization, Bearer};
use reqwest::header::Headers;
use serde::de::DeserializeOwned;
use serde_json;
use ::Error;
use ::BASE_URL;

pub trait WaniKaniRequester {
    fn api_request<T>(&self, api_key: &String, resource: String) -> Result<T, Error>
        where T: DeserializeOwned;
}

impl WaniKaniRequester for ReqwestClient {
    fn api_request<T>(&self, api_key: &String, resource: String) -> Result<T, Error>
        where T: DeserializeOwned
    {
        let uri = Url::parse(&format!("{}/{}", BASE_URL, resource)).unwrap();
        let res = self.get(uri).headers(build_headers(api_key.to_owned())).send().unwrap();
        serde_json::from_reader(res).map_err(Error::from)
    }
}

fn build_headers(api_key: String) -> Headers {
    let mut headers = Headers::new();
    headers.set(Authorization(Bearer { token: api_key }));
    return headers;
}
