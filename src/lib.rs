use std::collections::HashMap;

use reqwest::Url;
use serde::Serialize;

pub mod body;
pub mod objects;
pub struct Spotify {
    pub authorization: String,
}

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Spotify {
    pub fn new(authorization: &str) -> Self {
        Spotify {
            authorization: authorization.to_string(),
        }
    }

    fn request<T>(
        &self,
        method: Method,
        url: String,
        query: Option<HashMap<&str, String>>,
        body: Option<&T>,
    ) -> Option<(String, bool)>
    where
        T: Serialize + ?Sized,
    {
        let url = {
            if let Some(query) = query {
                Url::parse_with_params(
                    format!("https://api.spotify.com/v1/{}", url).as_str(),
                    query,
                )
                .unwrap()
                .to_string()
            } else {
                format!("https://api.spotify.com/v1/{}", url)
            }
        };

        let client = reqwest::blocking::Client::new();

        match method {
            Method::GET => {
                let response = client
                    .get(url)
                    .header("Accept", "application/json")
                    .header("Content-Type", "application/json")
                    .bearer_auth(self.authorization.as_str())
                    .send()
                    .unwrap();
                if response.status().is_success() {
                    Some((response.text().unwrap(), true))
                } else {
                    Some((response.text().unwrap(), false))
                }
            }
            Method::POST => {
                client
                    .post(url)
                    .bearer_auth(self.authorization.as_str())
                    .json(body.unwrap())
                    .send()
                    .unwrap();
                None
            }
            Method::PUT => {
                client
                    .put(url)
                    .bearer_auth(self.authorization.as_str())
                    .json(body.unwrap())
                    .send()
                    .unwrap();
                None
            }
            Method::DELETE => {
                client
                    .put(url)
                    .bearer_auth(self.authorization.as_str())
                    .send()
                    .unwrap();
                None
            }
        }
    }
}
