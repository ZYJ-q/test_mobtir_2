use std::collections::VecDeque;
use std::{collections::HashMap, fs, time::Duration};

use chrono::{DateTime, NaiveDateTime, Utc};
use log::{debug, info, warn};
use serde_json::{Map, Value};
// use tokio::{sync::broadcast::{self, Receiver}};
use test_alarm::adapters::binance::futures::http::actions::BinanceFuturesApi;
use test_alarm::base::ssh::SshClient;
use test_alarm::base::wxbot::WxbotHttpClient;
use test_alarm::actors::*;
// use test_alarm::models::http_data::*;

#[warn(unused_mut, unused_variables, dead_code)]
async fn real_time(
    binance: &Vec<Value>,
    // binance_futures_api: BinanceFuturesApi,
    symbols: &Vec<Value>,
    mut ssh_api: SshClient,
    wx_robot: WxbotHttpClient,
    ori_fund: f64,
) {
    //rece: &mut Receiver<&str>){
    info!("get ready for real time loop");
    let mut running = false;

    // 每个品种的上一个trade_id
    let mut last_trade_ids: HashMap<String, u64> = HashMap::new();
    for symbol_v in symbols {
        let symbol = String::from(symbol_v.as_str().unwrap());
        let symbol = format!("{}", symbol);
        last_trade_ids.insert(symbol, 0);
    }

    // 交易历史
    

    // let mut total_trade: VecDeque<Value> = VecDeque::new();

    // 净值数据
    
    

    info!("begin real time loop");
    // 监控循环
    loop {
        info!("again");
        // json对象
        let mut response: Map<String, Value> = Map::new();
        let mut json_data: Map<String, Value> = Map::new();
        let mut map: Map<String, Value> = Map::new();
        map.insert(String::from("productId"), Value::from("TRADER_001"));
        let now = Utc::now();
        let date = format!("{}", now.format("%Y/%m/%d %H:%M:%S"));
        

        

        
        // 监控服务器状态
        info!("server process");
        // let mut server_status: VecDeque<Value> = VecDeque::new();


        // print!("running的值是否被改变{}", running);

        // 时间
        // map.insert(String::from("time"), Value::from(date));

    // // 账户总余额
    // if let Some(date) = binance_futures_api.total_account(None).await{
    //     let v: Value = serde_json::from_str(&date).unwrap();
    //     // println!("账户总余额{:?}",v);

    // }


    for f_config in binance {
        let mut trade_histories: VecDeque<Value> = VecDeque::new();
        
        let binance_config = f_config.as_object().unwrap();
        let binance_futures_api=BinanceFuturesApi::new(
            binance_config
                .get("base_url")
                .unwrap()
                .as_str()
                .unwrap(),
            binance_config
                .get("api_key")
                .unwrap()
                .as_str()
                .unwrap(),
            binance_config
                .get("secret_key")
                .unwrap()
                .as_str()
                .unwrap(),
        );
        let name = binance_config.get("name").unwrap().as_str().unwrap();
        for symbol_v in symbols {
                
            let symbol = symbol_v.as_str().unwrap();
            let symbol = format!("{}", symbol);
            if let Some(data) = binance_futures_api.trade_hiostory(&symbol).await {
                let v: Value = serde_json::from_str(&data).unwrap();
                println!("历史数据{:?}, 名字{}", v, name);

                match v.as_array() {
                    Some(value) => {
                        if value.len() == 0 {
                            continue;
                        } else {

                            for a in 0..value.len() {

                                let mut trade_object: Map<String, Value> = Map::new();
                                trade_object.insert(String::from("tra_symbol"), Value::from(value[a].as_object().unwrap().get("symbol").unwrap().as_str().unwrap()));
                                trade_object.insert(
                                    String::from("th_id"),
                                    Value::from(
                                        value[a]
                                            .as_object()
                                            .unwrap()
                                            .get("id")
                                            .unwrap()
                                            .as_u64()
                                            .unwrap(),
                                    ),
                                );
                                // trade_object
                                //     .insert(String::from("tra_id"), Value::from(1));
                                trade_object.insert(
                                    String::from("side"),
                                    Value::from(
                                        value[a]
                                            .as_object()
                                            .unwrap()
                                            .get("side")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("price"),
                                    Value::from(
                                        value[a]
                                            .as_object()
                                            .unwrap()
                                            .get("price")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("qty"),
                                    Value::from(
                                        value[a]
                                            .as_object()
                                            .unwrap()
                                            .get("qty")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("realized_pnl"),
                                    Value::from(
                                        value[a]
                                            .as_object()
                                            .unwrap()
                                            .get("realizedPnl")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("quote_qty"),
                                    Value::from(
                                        value[a]
                                            .as_object()
                                            .unwrap()
                                            .get("quoteQty")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("position_side"),
                                    Value::from(
                                        value[a]
                                            .as_object()
                                            .unwrap()
                                            .get("positionSide")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("tra_commision"),
                                    Value::from(
                                        value[a]
                                            .as_object()
                                            .unwrap()
                                            .get("commission")
                                            .unwrap()
                                            .as_str()
                                            .unwrap(),
                                    ),
                                );
                                trade_object.insert(
                                    String::from("tra_order_id"),
                                    Value::from(
                                        value[a]
                                          .as_object()
                                          .unwrap()
                                          .get("orderId")
                                          .unwrap()
                                          .as_u64()
                                          .unwrap(),
                                    ),
                                );
                                let millis = value[a]
                                    .as_object()
                                    .unwrap()
                                    .get("time")
                                    .unwrap()
                                    .as_i64()
                                    .unwrap();
                                let datetime: DateTime<Utc> = DateTime::from_utc(
                                    NaiveDateTime::from_timestamp_millis(millis).unwrap(),
                                    Utc,
                                );
                                // info!("datetime: {}", datetime);
                                let time = format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));

                                trade_object.insert(String::from("tra_time"), Value::from(time.clone()));

                                // let year_time = format!("{}", datetime.format("%Y-%m-%d"));
                                // match value[i].as_object().unwrap().get("buyer") {
                                //     Some(buyer) => {
                                //         trade_object.insert(
                                //             String::from("is_buyer"),
                                //             Value::Bool(buyer.as_bool().unwrap()),
                                //         );
                                //     }
                                //     None => {
                                //         trade_object.insert(String::from("is_buyer"), Value::Null);
                                //     }
                                // }
                                // println!("时间:{:?}, year_time:{:?}", &time, &year_time);
                                match value[a].as_object().unwrap().get("maker") {
                                    Some(maker) => {
                                        trade_object.insert(
                                            String::from("is_maker"),
                                            Value::Bool(maker.as_bool().unwrap()),
                                        );
                                    }
                                    None => {
                                        trade_object.insert(String::from("is_maker"), Value::Null);
                                    }
                                }
                                trade_histories.push_back(Value::from(trade_object));

                                
                                
                                if trade_histories.len() > 1000 {
                                    trade_histories.pop_front();
                                }
                            }
                            

                            
                        }

                    }
                    None => {
                        continue;
                    }
                }
            }
        }

        let res = trade_mapper::TradeMapper::insert_trade(Vec::from(trade_histories.clone()), name);
        println!("插入历史交易数据是否成功{},账户名{:?}", res, name);

         
    }

        

        // 成交历史(更新所有)
        info!("trade history");

    

            

        
        



        

        // 输出日志
        // debug!("writing {}", json_file);


        

        // let net_worth_res = trade_mapper::NetWorkMapper::insert_net_worth(Vec::from(net_worth_histories.clone()));
        // print!("输出的净值数据信息{}", net_worth_res);

        // 等待下次执行
        info!("waiting for next real time task...({})", 1000 * 10);
        tokio::time::delay_for(Duration::from_millis(1000 * 10)).await;
    }
}

#[warn(unused_mut, unused_variables)]
#[tokio::main]
async fn main() {
    // 日志
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();

    init();

    // 测试用api
    // let api_key="JwYo1CffkOLqmv2sC3Qhe2Qu5GgzbeLVw2BxWB5HgK6tnmc8yGfkzLuDImBgDkXm";
    // let api_secret="7FtQARZqM2PDgIZ5plr3nwEVYBXXbvmSuvmpf6Viz9e7Cq2B87grRTG3VZQiEC5C";

    // 连接数据库
    // let config_db: Value =
    //     serde_json::from_str(&fs::read_to_string("./configs/database.json").unwrap()).unwrap();

    // 读取配置
    let config: Value = serde_json::from_str(
        &fs::read_to_string("./configs/total.json").expect("Unable to read file"),
    )
    .expect("Unable to parse");

    // 任务间通信信道
    // let (send, mut rece) = broadcast::channel(32);

    // 创建任务
    let real_time_handle = tokio::spawn(async move {
        // let mut futures_config: Map<String, Value> = Map::new();
        
        // let mut servers_config = Map::new();
        let binance_config = config.get("Binance").unwrap();
        // let binance_future_config = binance_config.get("futures").unwrap();
        let binance_future_config = binance_config.get("futures").unwrap().as_array().unwrap();
        let server_config = config.get("Server").unwrap();
        let symbols = config.get("Symbols").unwrap().as_array().unwrap();
        let key = config.get("Alarm").unwrap().get("webhook").unwrap().as_str().unwrap();
        // info!("获取key");
        let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
        wxbot.push_str(key);
        info!("wxbot  {}", wxbot);
        let wx_robot = WxbotHttpClient::new(&wxbot);
        info!("preparing...");

        // for s_config in server_config{
        //     let obj = s_config.as_object().unwrap(); 
        //     let host = obj.get("host").unwrap().as_str().unwrap();
        //     let port = obj.get("port").unwrap().as_str().unwrap();
        //     let username = obj.get("username").unwrap().as_str().unwrap();
        //     let password = obj.get("password").unwrap().as_str().unwrap();
        //     let root_path = obj.get("root_path").unwrap().as_str().unwrap();
        //     let root_name = obj.get("root_name").unwrap().as_str().unwrap();
        //     servers_config.insert(String::from("host"), Value::from(host));
        //     servers_config.insert(String::from("port"), Value::from(port));
        //     servers_config.insert(String::from("username"), Value::from(username));
        //     servers_config.insert(String::from("password"), Value::from(password));
        //     servers_config.insert(String::from("root_path"), Value::from(root_path));
        //     servers_config.insert(String::from("root_name"), Value::from(root_name));
        // }
        
        
        
        let ssh_api = SshClient::new(
            server_config.get("host").unwrap().as_str().unwrap(),
            server_config.get("port").unwrap().as_str().unwrap(),
            server_config.get("username").unwrap().as_str().unwrap(),
            server_config.get("password").unwrap().as_str().unwrap(),
            server_config.get("root_path").unwrap().as_str().unwrap(),
            server_config.get("root_name").unwrap().as_str().unwrap(),
        );
        

        
        // for f_config in binance_future_config{
        //     let obj = f_config.as_object().unwrap(); 
        //     let base_url = obj.get("base_url").unwrap().as_str().unwrap();
        //     let api_key = obj.get("api_key").unwrap().as_str().unwrap();
        //     let secret_key = obj.get("secret_key").unwrap().as_str().unwrap();
        //     futures_config.insert(String::from("base_url"), Value::from(base_url));
        //     futures_config.insert(String::from("api_key"), Value::from(api_key));
        //     futures_config.insert(String::from("secret_key"), Value::from(secret_key));
        // }

        info!("created ssh client");
        // let binance_futures_api=BinanceFuturesApi::new(
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("base_url")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("api_key")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("secret_key")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        // );

        
        info!("created http client");
        real_time(binance_future_config, symbols, ssh_api, wx_robot, 500.0).await;
    });

    // 开始任务
    info!("alarm begin(binance account)");
    real_time_handle.await.unwrap();
    info!("alarm done");
}
