use crate::config::config_init;
use crate::model::db_conn_init;
use crate::router::start_route;

pub mod middleware;
pub mod router;
pub mod handler;
pub mod service;
pub mod model;
pub mod config;
pub mod utils;
pub mod entities;

pub async fn run() {

    //初始化
    config_init().await;

    //链接数据库
    db_conn_init().await;

    //开启路由
    start_route().await;
}