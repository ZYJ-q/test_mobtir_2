pub struct TradeMapper;
pub struct PositionMapper;

pub struct NetWorkMapper;
// use super::http_data::TradeRe;
use crate::actors::database::get_connect;
// use log::info;
use mysql::*;
use mysql::prelude::*;
use serde_json::Value;
// use super::db_data::Trade;


impl TradeMapper {
  // 插入数据
  pub fn insert_trade(trades:Vec<Value>, name: &str) -> bool {
    // 连接数据库
    let mut conn = get_connect();
    // let query_id = conn.exec_first(, params)
    let mut value = "";

    


    

    if name == "Angus" {
      value =r"INSERT IGNORE INTO trade_histories_3 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
      VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
    } else if name == "trader02" {
      value = r"INSERT IGNORE INTO trade_histories_4 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
      VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
    } else if name == "trader04" {
      value = r"INSERT IGNORE INTO trade_histories_5 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
      VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
    } else if name == "xh01_feng4_virtual" {
      value = r"INSERT IGNORE INTO trade_histories_7 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
      VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
    } else if name == "xh02_b20230524_virtual" {
      value = r"INSERT IGNORE INTO trade_histories_8 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
      VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
    } else if name == "xh03_feng3_virtual" {
      value = r"INSERT IGNORE INTO trade_histories_9 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
      VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
    } else if name == "xh04_20230524_virtual" {
      value = r"INSERT IGNORE INTO trade_histories_10 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
      VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
    } else {
      value = r"INSERT IGNORE INTO trate_histories_6 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
      VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)"; 
    }



    let flag = conn.exec_batch(
      value,
      trades.iter().map(|p| params! {
        "th_id" => &p["th_id"],
        "tra_symbol" => &p["tra_symbol"],
        "tra_order_id" => &p["tra_order_id"],
        // "tra_id" => &p["tra_id"],
        "tra_commision" => &p["tra_commision"],
        "tra_time" => &p["tra_time"].as_str(),
        "is_maker" => &p["is_maker"].to_string(),
        "position_side" => &p["position_side"],
        "price" => &p["price"],
        "qty" => &p["qty"],
        "quote_qty" => &p["quote_qty"],
        "realized_pnl" => &p["realized_pnl"],
        "side" => &p["side"],
      })
    );

  // let um1 = conn.query_map(
  // "select * from trate_histories",
  // |(th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)| {
  //     Trade{th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side}
  // }
  // ).unwrap();

  // println!("查询到的数据{:?}", um1);

    match flag {
      Ok(_c) => {
        println!("insert success!");
        return true;
      },
      Err(e) => {
        eprintln!("error:{}", e);
        return false;
      }
    }
  }


  // pub fn update_trade() -> bool {
  //   let mut conn = get_connect();
  //   let mut value = "";


    

  //   if name == "Angus" {
  //     value ="UPDATE `trade_histories_3` set `tra_time` = TRIM(BOTH '"' FROM `tra_time`)";
  //   } else if name == "trader02" {
  //     value = r"INSERT IGNORE INTO trade_histories_4 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
  //     VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
  //   } else if name == "trader04" {
  //     value = r"INSERT IGNORE INTO trade_histories_5 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
  //     VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
  //   } else if name == "xh01_feng4_virtual" {
  //     value = r"INSERT IGNORE INTO trade_histories_7 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
  //     VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
  //   } else if name == "xh02_b20230524_virtual" {
  //     value = r"INSERT IGNORE INTO trade_histories_8 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
  //     VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
  //   } else if name == "xh03_feng3_virtual" {
  //     value = r"INSERT IGNORE INTO trade_histories_9 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
  //     VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
  //   } else if name == "xh04_20230524_virtual" {
  //     value = r"INSERT IGNORE INTO trade_histories_10 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
  //     VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)";
  //   } else {
  //     value = r"INSERT IGNORE INTO trate_histories_6 (th_id, tra_symbol, tra_order_id, tra_commision, tra_time, is_maker, position_side, price, qty, quote_qty, realized_pnl, side)
  //     VALUES (:th_id, :tra_symbol, :tra_order_id, :tra_commision, :tra_time, :is_maker, :position_side, :price, :qty, :quote_qty, :realized_pnl, :side)"; 
  //   }

  //   let res = conn.exec_drop(stmt, params)
  // }



}




