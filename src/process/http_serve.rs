use anyhow::Result;
use axum::{routing::get, Router};
use std::{net::SocketAddr, path::Path};
use tracing::info;

pub async fn process_http_serve(path: &Path, port: u16) -> Result<()> {
    //绑定一个路由
    let router = Router::new().route("/", get(index_handler));

    //生成listener
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Serving {:?} on port {}", path, addr);

    //绑定路由和监听器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    //执行到这里就会启动服务， 等待http请求进来
    //使用rest client 测试，index路径： http://localhost:8080/
    axum::serve(listener, router).await?;

    Ok(())
}

async fn index_handler() -> &'static str {
    "Hello World"
}
