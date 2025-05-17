#![allow(unused_imports)] // 作用于整个文件 消除没有使用的导入警告
#![allow(dead_code)] // 消除 未使用的类型/函数/枚举变体 警告
#![allow(unused_variables)] // 消除 未使用的变量/参数

use axum::body::Body;
use axum::body::{Bytes, HttpBody};
use axum::routing::post;
use axum::{
    BoxError, Json, RequestExt, Router, async_trait,
    extract::FromRequest,
    http::Request,
    response::{IntoResponse, Response},
};
use chrono::Local;
use serde::Deserialize;
use std::fmt::Write;

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u8,
    address: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for User
where
    B: HttpBody + Send + 'static,
    B::Data: Into<Bytes> + Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = (Response); // 返回一个错误的响应信息

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        todo!()
    }
}

fn main() {
    println!("Starting server on port 8080");
}
