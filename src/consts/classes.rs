use serde_json::{Value};
use std::error::Error;
use core::result::Result;
use super::consts::*;
pub struct TgJsonData<'a>  {
   pub chatId:i64,
   pub msgCmd:&'a str,
   pub isCommand:bool
}
pub struct Attribute<'a>{
    pub option:&'a str,
    pub val:&'a str
}
/*
ОБРАБОТАТЬ ОШИБКИ, ЕСЛИ ДАННЫЕ ДЖОЙСОНЕ НЕ ВЕРНЫЕ!!!
*/
impl TgJsonData<'_>{
    pub fn new (json:&Value) -> Result<self::TgJsonData<'_>, Box<dyn Error>>{

        let mut data = TgJsonData {
            isCommand:false,
            chatId : json["message"]["chat"]["id"].as_i64().unwrap(),
           // msgType : json["message"]["entities"][0]["type"].as_str().unwrap(),
            msgCmd : json["message"]["text"].as_str().unwrap(),
        };

        if json["message"]["entities"][0]["type"].as_str().unwrap() == "bot_command" {
            data.isCommand = true;
        }
        //println!("CЗапрос {} {} {}", data.isCommand, data.chatId, data.msgCmd);
        Ok(data)
    }

    pub fn getCmd(&self) -> Result<&'_ str, Box <dyn Error>>{
        let msg:&'_ str = &self.msgCmd;
        let cmd:&'_ str = match msg.find(" "){
            Some(num) => {&msg[1..num]},
            None => {&msg[1..]}
        };
        if CMD_LIST().contains(&cmd) && self.isCommand{
            Ok(cmd)
        }
        else{
                //ОБРАБОТАТЬ ОШИБКИ
                Ok(cmd)
        }
    }

    pub fn getParams(&self) -> Option<std::vec::Vec<&'_ str>>{
        let msg:&'_ str = &self.msgCmd.trim();
        let params:&'_ str;
        if msg.find(" ").is_none() || !&self.isCommand {
           return None;
        }
        else {

            params = &msg[msg.find(" ").unwrap()+1..];
        }
        let mut from:usize = 0;
        let mut strs: std::vec::Vec<&'_ str> = params.rsplit(' ').collect();
        println!("params {}",strs.len());
        /*for (i,ch) in params.chars().enumerate(){
            if ch == ' ' {
                strs.push(&params[from..i]);
                from = i+1;
            }
            else if params.len() == i+1{
                strs.push(&params[from..]);
            }
        };*/
        //ОБРАБОТАТЬ ОШИБКИ
        Some(strs)
    }

    pub fn newCmdAttr<'b>(parametr:&'b str)->Option<Attribute>{
        let attrs: std::vec::Vec<&'b str> = parametr.rsplit('=').collect();
        if attrs.len()<2 {
            return None;
        }
        Some (
            Attribute{
                option:attrs.get(0).unwrap(),
                val:attrs.get(1).unwrap()
            }
        )
    }


}
