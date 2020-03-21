#[allow(dead_code)]
use core::result::Result;
use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use futures::TryStreamExt as _;
use std::error::Error;
use super::router::router;
use crate::consts::consts::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
   let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(router)) }
   });



    let addr:SocketAddr = SocketAddr::new(
        IpAddr::V4(
            Ipv4Addr::new(
                SERVER_IP[0], 
                SERVER_IP[1], 
                SERVER_IP[2], 
                SERVER_IP[3]
            )
        ), 
        SERVER_PORT
    );
    let server = Server::bind(&addr).serve(make_svc);

    println!("Сервер запустился http://{}", addr);
    server.await;
    Ok(())
}

