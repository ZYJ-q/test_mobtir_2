use std::collections::VecDeque;
use std::{collections::HashMap, fs, time::Duration};

use chrono::{DateTime, NaiveDateTime, Utc, Local};
use log::{debug, info, warn, error};
use serde_json::{Map, Value};
// use tokio::{sync::broadcast::{self, Receiver}};
use insert_trader_binance::adapters::binance::futures::http::actions::BinanceFuturesApi;
use insert_trader_binance::adapters::binance::papi::http::actions::BinancePapiApi;
use insert_trader_binance::base::ssh::SshClient;
use insert_trader_binance::base::wxbot::WxbotHttpClient;
use insert_trader_binance::actors::*;
// use test_alarm::models::http_data::*;

#[warn(unused_mut, unused_variables, dead_code)]
async fn real_time(
    symbols: &Vec<Value>,
    mut ssh_api: SshClient,
    wx_robot: WxbotHttpClient,
    ori_fund: f64,
) {
    //rece: &mut Receiver<&str>){
    info!("get ready for real time loop");
    let mut running = false;
    let mut end = 1;
    let mut time_id = 1;

    // 每个品种的上一个trade_id
    // let mut last_trade_ids: HashMap<String, u64> = HashMap::new();
    let mut symbols_histories: VecDeque<Value> = VecDeque::new();
    // for symbol_v in symbols {
    //     let symbol = String::from(symbol_v.as_str().unwrap());
    //     let symbol = format!("{}", symbol);
    //     last_trade_ids.insert(symbol, 0);
    // }

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

        println!("长度{}", symbols.len());
        
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


    let binance = trade_mapper::TradeMapper::get_positions().unwrap();

    let binance_futures_api=BinanceFuturesApi::new(
        "https://fapi.binance.com",
        &binance[0].api_key,
        &binance[0].secret_key,
    );

    if let Some(data) = binance_futures_api.get_symbols(None).await {
        let v: Value = serde_json::from_str(&data).unwrap();
        let symbols = v.as_object().unwrap().get("symbols").unwrap().as_array().unwrap();

        for symbol in symbols{
            let mut symbol_object: Map<String, Value> = Map::new();
            let sym = symbol.as_object().unwrap().get("symbol").unwrap().as_str().unwrap();
            symbol_object.insert(String::from("symbol"), Value::from(sym));
            symbols_histories.push_back(Value::from(symbol_object));

        }
    }


    println!("symbol长度{}", symbols_histories.len());






    

     
    
    for f_config in binance {
        let tra_name = &f_config.name;
        let mut trade_histories: VecDeque<Value> = VecDeque::new();
         
        if &f_config.tra_venue == "Binance" && &f_config.r#type == "Futures" {
            
            let binance_futures_api=BinanceFuturesApi::new(
                "https://fapi.binance.com",
                &f_config.api_key,
                &f_config.secret_key,
            );
            let name = f_config.tra_id;
            for symbol_v in symbols {
                let symbol = symbol_v.as_str().unwrap();
                let symbol = format!("{}", symbol);
                println!("symbol{}", symbol);
                info!("waiting for next real time task...({}) {}", 3000 * 10, name);
                tokio::time::delay_for(Duration::from_millis(300 * 10)).await;
                if let Some(data) = binance_futures_api.trade_hiostory(&symbol, &end, &time_id).await {
                    let v: Value = serde_json::from_str(&data).unwrap();
                    // println!("历史数据{:?}, 名字{}", v, name);
    
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

                                    trade_object.insert(
                                        String::from("name"), 
                                        Value::from(name)
                                    );


                                    trade_object.insert(
                                        String::from("type"),
                                        Value::from("Futures"),
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
                                        .as_u64()
                                        .unwrap();
                                    // let datetime: DateTime<Utc> = DateTime::from_utc(
                                    //     NaiveDateTime::from_timestamp_millis(millis).unwrap(),
                                    //     Utc,
                                    // );
                                    // // info!("datetime: {}", datetime);
                                    // let time = format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));
    
                                    // println!("时间{}", millis);
    
                                    trade_object.insert(String::from("tra_time"), Value::from(millis));
    
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
                } else {
                    error!("Can't get bian_futures {} traders.", name);
                    continue;
                }
                
            }
    
        }



        if &f_config.tra_venue == "Binance" && &f_config.r#type == "Papi" {
            let binance_papi_api=BinancePapiApi::new(
                "https://papi.binance.com",
                &f_config.api_key,
                &f_config.secret_key,
            );
            let name = f_config.tra_id;
            for symbol_v in symbols {
                let symbol = symbol_v.as_str().unwrap();
                let symbol = format!("{}", symbol);
                info!("waiting for next real time task...({}), {}", 3000 * 10, name);
                tokio::time::delay_for(Duration::from_millis(300 * 10)).await;
                if let Some(data) = binance_papi_api.trade_hiostory(&symbol, &end, &time_id).await {
                    let v: Value = serde_json::from_str(&data).unwrap();
                    // println!("历史数据{:?}, 名字{}", v, name);
    
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

                                    trade_object.insert(
                                        String::from("name"), 
                                        Value::from(name)
                                    );


                                    trade_object.insert(
                                        String::from("type"),
                                        Value::from("Papi_um"),
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
                                        .as_u64()
                                        .unwrap();
                                    // let datetime: DateTime<Utc> = DateTime::from_utc(
                                    //     NaiveDateTime::from_timestamp_millis(millis).unwrap(),
                                    //     Utc,
                                    // );
                                    // // info!("datetime: {}", datetime);
                                    // let time = format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"));
    
                                    // println!("时间{}", millis);
    
                                    trade_object.insert(String::from("tra_time"), Value::from(millis));
    
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
                } else {
                    error!("Can't get bian_papi {} traders.", name);
                    continue;
                }
                
            }
    
        }

        let res = trade_mapper::TradeMapper::insert_trade(Vec::from(trade_histories.clone()));
        println!("插入历史交易数据是否成功{}", res);
        

        
       

         
    }

   
        

    println!("end{} time_id{}", end, time_id);



    


    let times = Local::now().format("%Y-%m-%d %H:00:00").to_string();
    let date_time: NaiveDateTime = NaiveDateTime::parse_from_str(&times, "%Y-%m-%d %H:%M:%S").unwrap();
    let time = date_time.timestamp_millis();
        println!("time{}", time);
        let last_time = time - 1000*60*60*24 * end;
        println!("last_time{}", last_time);
        if time_id == 24 {
            time_id = 1;
            if end != 0 {
                end -= 1
            } else {
                end = 0
            }
        } else {
            if last_time < time {
                time_id += 1
            } else if last_time == time  {
                time_id = time_id
            } else {
                time_id -= 1
            }
        } 


        // let time_min = Local::now().timestamp_millis();
        // let last_time_min = time_min - 1000*60*60*24 * minut_end;
        // if time_minut_id == 1440 {
        //     time_minut_id = 1;
        //     if minut_end != 0 {
        //         minut_end -= 1
        //     } else {
        //         minut_end = 0
        //     }
        // } else {
        //     if last_time_min < time_min {
        //         time_minut_id += 1
        //     } else if last_time_min == time_min  {
        //         time_minut_id = time_minut_id
        //     } else {
        //         time_minut_id -= 1
        //     }
        // }

    

            

        
        



        

        // 输出日志
        // debug!("writing {}", json_file);


        

        // let net_worth_res = trade_mapper::NetWorkMapper::insert_net_worth(Vec::from(net_worth_histories.clone()));
        // print!("输出的净值数据信息{}", net_worth_res);

        // 等待下次执行
        info!("waiting for next real time task...({})", 3000 * 10);
        tokio::time::delay_for(Duration::from_millis(3000 * 10)).await;
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

let symb: Value = serde_json::from_str(
    &fs::read_to_string("./configs/symbols.json").expect("Unable to read file"),
)
.expect("Unable to parse");

    // 任务间通信信道
    // let (send, mut rece) = broadcast::channel(32);

    // 创建任务
    let real_time_handle = tokio::spawn(async move {
        // let mut futures_config: Map<String, Value> = Map::new();
        
        // let mut servers_config = Map::new();
        let binance_config = config.get("Binance").unwrap();
        let bybit_config = config.get("ByBit").unwrap();

        // let symbols = symb.get("symbols").unwrap().as_array().unwrap();
        // println!("symbols{:?}", symbols);
        
        // let binance_future_config = binance_config.get("futures").unwrap();
        let binance_future_config = binance_config.get("futures").unwrap().as_array().unwrap();
        let bybit_futures_config = bybit_config.get("futures").unwrap().as_array().unwrap();
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
        real_time(symbols, ssh_api, wx_robot, 500.0).await;
    });

    // 开始任务
    info!("alarm begin(binance account)");
    real_time_handle.await.unwrap();
    info!("alarm done");
}
