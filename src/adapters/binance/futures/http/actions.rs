use chrono::{Utc, DateTime, NaiveDateTime, Local};
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use super::client::BinanceHttpClient;

pub struct BinanceFuturesApi {
    client: BinanceHttpClient,
}

impl BinanceFuturesApi {
    pub fn new(base_url: &str, api_key: &str, api_secret: &str) -> Self {
        let client = BinanceHttpClient::new(base_url, api_key, api_secret);
        Self { client: client }
    }

    pub async fn account(&self, recv_window: Option<u8>) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        match recv_window {
            Some(recvwindow) => {
                params.insert(
                    String::from("recvWindow"),
                    recvwindow.to_string().parse().unwrap(),
                );
            }
            None => {}
        }
        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/account", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                println!("账户信息{:?}", data);
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    pub async fn total_account(&self, recv_window: Option<u8>) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        match recv_window {
            Some(recvwindow) => {
                params.insert(
                    String::from("recvWindow"),
                    recvwindow.to_string().parse().unwrap(),
                );
            }
            None => {}
        }
        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/balance", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                println!("账户总余额{:?}", data);
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    pub async fn position_risk(&self, symbol: Option<&str>) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        match symbol {
            Some(symbol_s) => {
                params.insert(
                    String::from("symbol"),
                    String::from(symbol_s).parse().unwrap(),
                );
            }
            None => {}
        }
        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/positionRisk", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    pub async fn trade_hiostory(&self, symbol: &str, end:&i64, time_id:&i64) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(String::from("symbol"), Value::from(symbol));

        



        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));
        let time = Local::now().timestamp_millis();
        let last_time = time - 1000*60*60*24 * end;

        let mut end_times = 0;
        
        if time_id == &1 {
            end_times = time - 1000*60*60*24 * end;
        } else {
            end_times = last_time + 1000*60*60 * (time_id -1)
        }

        
        let start_datetime: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp_millis(end_times).unwrap(), Utc,);
        let end_datetime: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp_millis(&end_times + 1000*60*60).unwrap(), Utc,);
         
        let end_time= format!("{} ", end_datetime.format("%Y-%m-%d %H:%M:%S"));
        let start_time = format!("{} ", start_datetime.format("%Y-%m-%d %H:%M:%S"));
        println!("-------------end------------{:?}", end_time);
        println!("------------start----------{}", start_time);

        params.insert(String::from("endTime"), Value::from(&end_times + 1000*60*60));
        if end != &0 {
            if time_id == &1 {
                params.insert(String::from("startTime"), Value::from(time - 1000*60*60*24 * end));
            } else {
                params.insert(String::from("startTime"), Value::from(last_time + 1000*60*60 * (time_id -1)));
            }
        }

        let response = self
            .client
            .send(Method::GET, "/fapi/v1/userTrades", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    pub async fn position(&self, symbol: Option<&str>) -> Option<String> {
        let mut params: HashMap<String, Value> = HashMap::new();
        match symbol {
            Some(s) => {
                params.insert(String::from("symbol"), Value::from(s));

            }
            None => {}
        }

        let now_time = Utc::now().timestamp_millis();
        params.insert(String::from("timestamp"), Value::from(now_time));

        let response = self
            .client
            .send(Method::GET, "/fapi/v2/positionRisk", true, &mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }
}
