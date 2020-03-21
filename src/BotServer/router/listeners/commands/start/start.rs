use crate::consts::consts::*;
use crate::bot_req::tg_req::*;
use serde_json::{Result, Value};
use crate::consts::classes::*;

pub async fn start_command(res:TgJsonData<'_>){
    match res.getParams() {
        Some(vec) => {
            println!("Параметры пригли {}", vec.len());
            start_with_parameters(vec, &res).await;
        },
        None => {
            println!("Параметры ne пригли");
            start_without_parameters(res.chatId).await;
        }
    }
}

async fn start_with_parameters(params:std::vec::Vec<&'_ str>, res:&TgJsonData<'_>){
    let attr:Attribute = TgJsonData::newCmdAttr(params.get(0).unwrap()).unwrap();
    let send_str:String = format!(
        "https://api.telegram.org/{}/sendMessage?chat_id={}&text=You sent option {} and attr {}",
        TOKEN, 
        res.chatId,
        attr.option,
        attr.val
    );
    println!("Query {}", send_str);
    https_req(send_str).await;
}

 async fn start_without_parameters(chatId:i64){
    let send_str:String = format!(
        "https://api.telegram.org/{}/sendMessage?chat_id={}&text=start",
        TOKEN, 
        chatId
    );
    println!("egr {}", send_str);
    https_req(send_str).await;
    
}
