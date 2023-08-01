use super::super::super::super::super::base::http::HttpClient;
use hex;
use hmac::{Hmac, Mac};
use itertools::Itertools;
use log::{error};
use reqwest::header::HeaderMap;
use reqwest::{Method, Response, StatusCode};
use serde_json::value::Value;
use sha2::Sha256;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct BinanceHttpClient {
    http_client: HttpClient,
    base_url: String,
    api_key: String,
    api_secret: String,
}

#[allow(dead_code)]
impl BinanceHttpClient {
    pub fn new(base_url: &str, api_key: &str, api_secret: &str) -> Self {
        let http_client = HttpClient::new();
        Self {
            http_client: http_client,
            base_url: String::from(base_url),
            api_key: String::from(api_key),
            api_secret: String::from(api_secret),
        }
    }

    async fn package_request(
        &self,
        method: &Method,
        url: &str,
        need_sign: bool,
        params: &mut HashMap<String, Value>,
    ) -> Option<Response> {
        // env_logger::init();
        let mut uri = String::from(url);
        let mut data_json = String::new();

        if method == Method::GET || method == Method::DELETE {
            if !params.is_empty() {
                let mut strl: Vec<String> = Vec::new();
                for key in params.keys().sorted() {
                    let value = params.get(key).unwrap();
                    if value.is_string() {
                        strl.push(format!("{}={}", key, value.as_str().unwrap()));
                    }else {
                        strl.push(format!("{}={}", key, value));
                    }
                }
                for i in 0..strl.len() {
                    if i == 0 {
                        data_json.push_str(&strl[i]);
                    } else {
                        data_json.push('&');
                        data_json.push_str(&strl[i]);
                    }
                }
                uri = format!("{}?{}", &uri, &data_json);
            }
        } else {
            if !params.is_empty() {
                match serde_json::to_string(&params) {
                    Ok(result) => data_json = result,
                    Err(e) => error!("error on parase params: {}", e),
                }
            }
        }

        let mut headers = HeaderMap::new();
        if need_sign {
            let str_to_sign = format!("{}", &data_json);
            // println!("{str_to_sign}");

            let mut hmac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes()).unwrap();
            hmac.update(str_to_sign.as_bytes());
            let sign_bytes = hmac.finalize().into_bytes();
            let sign = hex::encode(sign_bytes);

            uri = format!("{}&signature={}", &uri, &sign);

            headers.insert("X-MBX-APIKEY", self.api_key.parse().unwrap());
            headers.insert("Content-Type", "application/json".parse().unwrap());
        }
        headers.insert("User-Agent", "nautilus_alarm".parse().unwrap());
        let url = format!("{}{}", self.base_url, uri);
        // println!("{},{},{:?},{}", &method.as_str(), url, headers, data_json);
        return self
            .http_client
            .send_request(&method.as_str(), &url, headers, &data_json)
            .await;
    }

    pub async fn send(
        &self,
        method: Method,
        url: &str,
        need_sign: bool,
        params: &mut HashMap<String, Value>,
    ) -> Option<String> {
        // let data: HashMap<String, Value> = HashMap::new();
        if let Some(response) = self.package_request(&method, url, need_sign, params).await {
            if response.status() == StatusCode::OK {
                match response.text().await {
                    Ok(response_data) => {
                        return Some(response_data);
                    },
                    Err(e) => {
                        error!("error on parse response: {:?}", e);
                        return None;
                    }
                }
            } else {
                panic!(
                    "code status error: {}-{}",
                    response.status(),
                    response.text().await.unwrap()
                );
            }
        } else {
            panic!(
                "none response: {},{},{},{:?}",
                &method.as_str(),
                url,
                need_sign,
                params
            );
        }
    }

    // todo
    pub fn check_response_data(&self, data_s: Option<String>) -> Option<String> {
        match data_s {
            Some(data) => {
                if !data.is_empty() {
                    if data.contains("code") {
                        panic!("code: {}", data);
                    }else {
                        return Some(data);
                    }
                } else {
                    panic!("response is empty");
                }
            }
            None => {
                panic!("handle response failed")
            }
        }
    }
}
