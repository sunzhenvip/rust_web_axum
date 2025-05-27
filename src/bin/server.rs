#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

use std::net::SocketAddr;
use axum_weibo::helloworld::greeter_server::{Greeter, GreeterServer};
use axum_weibo::helloworld::{HelloReply, HelloRequest};
use tonic::{Code, Request, Response, Status};
use tonic::transport::Server;

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self,request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("欢迎使用1server");
        let rsp = HelloReply{
            message: "hello world".to_string(),
        };
        // 错误返回示例
        // Err(Status::new(Code::InvalidArgument,"参数错误"))
        Ok(Response::new(rsp))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "127.0.0.1:50001".parse::<SocketAddr>().unwrap();
    // let addr = "[::1]:50001]".parse::<SocketAddr>().unwrap();
    let addr = "[::1]:50051".parse::<SocketAddr>().unwrap();
    // 如果你的操作系统允许 dual stack（双栈），那么 [::]:50001 会同时监听：
    // 所有 IPv6 地址（包括 [::1]）
    // 所有 IPv4 地址（如 127.0.0.1, 0.0.0.0
    // let addr = "[::]:50001".parse::<SocketAddr>().unwrap();
    // let addr = "127.0.0.1:50001".parse::<SocketAddr>().unwrap();

    let g  = MyGreeter::default();
    // let g = GreeterServer::new(greeter);

    Server::builder()
        .add_service(GreeterServer::new(g))
        .serve(addr)
        .await?;

    Ok(())
}
