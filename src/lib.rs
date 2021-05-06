use std::collections::HashMap;

use reqwest::Url;
use serde::Serialize;

mod objects;

pub struct Spotify {
    authorization: String,
}

enum Method {
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
        url: &str,
        query: HashMap<&str, &str>,
        body: Option<&T>,
    ) -> Option<String>
    where
        T: Serialize + ?Sized,
    {
        let url = Url::parse_with_params(
            format!("https://api.spotify.com/v1/{}", url).as_str(),
            query,
        )
        .unwrap()
        .to_string();

        let client = reqwest::blocking::Client::new();

        match method {
            Method::GET => {
                let response = client
                    .get(url)
                    .bearer_auth(self.authorization.as_str())
                    .send();
                let text = response.unwrap().text().unwrap();

                Some(text)
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
            Method::PUT => None,
            Method::DELETE => None,
        }
    }
}
