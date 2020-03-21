use hyper::{Client, Request, Uri};
use hyper::client::HttpConnector;
use hyper_proxy::{Proxy, ProxyConnector, Intercept};
use typed_headers::Credentials;
use std::error::Error;
use futures::{future,executor};
use serde_json::{Result, Value};
use futures::{TryFutureExt, TryStreamExt};
pub async fn https_req(url:String) -> core::result::Result<std::string::String, Box<dyn Error>>{
    
    println!("Fut_https");
    let proxy = {
        let proxy_uri = "http://189.90.245.230:8080".parse::<Uri>().unwrap();
        let mut proxy = Proxy::new(Intercept::All, proxy_uri);
        proxy.set_authorization(Credentials::basic("John Doe", "Agent1234").unwrap());
        let connector = HttpConnector::new();
        let proxy_connector = ProxyConnector::from_proxy(connector, proxy)
        .expect("Connecting to proxy server is failed");
        proxy_connector
    };
//проверить на ошибку
    let uri: Uri = "http://my-remote-website.com".parse().unwrap();
    let mut req = Request::get(uri.clone()).body(hyper::Body::empty())
    .expect("Connect failed");
    if let Some(headers) = proxy.http_headers(&uri) {
        req.headers_mut().extend(headers.clone().into_iter());
    }
    let client = Client::builder().build(proxy);
    
    let fut_http = client.request(req)
        .map_err(|e|{panic!("HTTP response failed {:?}",e)})
        .and_then(|res| res.into_body().map_ok(|x|x.to_vec()).try_concat())
        .map_ok(move |body| ::std::str::from_utf8(&body).unwrap().to_string());
    let fut_https = client.get(url.parse().unwrap())
        .map_err(|e|{panic!("HTTPS response failed {:?}",e)})
        .and_then(|res| res.into_body().map_ok(|x|x.to_vec()).try_concat())
        .map_ok(move |body| ::std::str::from_utf8(&body)
        .expect("Converting body to str failed")
        .to_string()).await.unwrap();
    Ok(fut_https)
}
pub async fn https_req_json(url:String)-> core::result::Result<Value, Box<dyn Error>> {
    let mut res:std::string::String = https_req(url).await
    .expect("Https req failed");
    let json: Value = serde_json::from_str(res.as_mut_str())
    .expect("Json formating is failed");
    Ok(json)
}