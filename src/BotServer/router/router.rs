use core::result::Result;
use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use futures::TryStreamExt as TSE;
use std::error::Error;
use crate::consts::functs::*;
use crate::consts::consts::*;
use crate::consts::classes::*;
use super::listeners as listens;
use crate::bot_req::tg_req::https_req;

pub async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
           let mut res_str = body_to_string(req.into_body()).await.unwrap();
            let tgJson:serde_json::Value = serde_json::from_str(res_str.as_mut_str())
                .expect("Json formating is failed");
            let tgRes: TgJsonData = TgJsonData::new(&tgJson).expect("Json formating is failed");;
            //Рутуем команды из телеграмма
            let cmd = tgRes.getCmd().unwrap();
            if tgRes.isCommand{
                
                match cmd{
                    "start" => {
                        
                        println!("Запрос старт прошел");
                        listens::commands::start::start::start_command(tgRes).await;
                        
                        //println!("Res url {}", res_str.as_mut_str());
                    }
                    _ =>{
                        println!("Command not fined");
                        //обработать
                        
                    }
                }
                   
            }
            else{
                println!("It's not command");
                println!("Res url {}", res_str.as_mut_str());
               
            }

        },
        TSE => {
            println!("Nope");
            *response.status_mut() = StatusCode::NOT_FOUND;
           
        },
    };

    Ok(response)

}