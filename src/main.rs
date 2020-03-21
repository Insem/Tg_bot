use hyper::{Client, Request, Uri};
use hyper::client::HttpConnector;
use futures::{TryFutureExt, TryStreamExt};
use hyper_proxy::{Proxy, ProxyConnector, Intercept};
use typed_headers::Credentials;
use std::error::Error;
use bot_req::tg_req::https_req_json;
use DataBase::server::DB;
use futures::executor::block_on;
use BotServer::server::*;
pub(crate) mod consts;
pub(crate) mod bot_req;
mod DataBase;
mod BotServer;
#[tokio::main]
 async fn main() {
    /*let resp = https_req_json("https://api.telegram.org/bot1063777732:AAEnAMLJZmiOg-q7D5OW8AA1DpTFzQTX4rw/getMe").await
    .expect("Bot req failed");
    println!("Response {:?}", resp);*/


    //let db = DB::start();
    //db.insert();
  start_server().await;


}