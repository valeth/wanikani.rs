use reqwest::{Client as ReqwestClient, Url};
use reqwest::header::{Authorization, Bearer, Headers};
use serde::de::DeserializeOwned;
use serde_json;
use ::{Result, Error, BASE_URL};
use ::filters::Filter;

pub trait WaniKaniRequester {
    fn api_request<T, F>(&self, api_key: &String, resource: String, filter: F) -> Result<T>
        where T: DeserializeOwned,
              F: Filter;
}

impl WaniKaniRequester for ReqwestClient {
    fn api_request<T, F>(&self, api_key: &String, resource: String, filter: F) -> Result<T>
        where T: DeserializeOwned,
              F: Filter
    {
        let uri = Url::parse(&format!("{}/{}", BASE_URL, resource))?;
        let res = self
            .get(uri)
            .headers(build_headers(api_key.to_owned()))
            .query(&filter)
            .send()?;

        serde_json::from_reader(res).map_err(Error::from)
    }
}

fn build_headers(api_key: String) -> Headers {
    let mut headers = Headers::new();
    headers.set(Authorization(Bearer { token: api_key }));
    return headers;
}
