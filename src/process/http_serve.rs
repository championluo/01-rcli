use anyhow::Result;
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tracing::info;

//定义一个结构体， 用于保存http服务的配置
// 这里的path是PathBuf， 所以可以接收任何类型的文件路径， 包括目录
// 这里的path的所有权会在函数结束后释放， 所以这里可以直接使用PathBuf， 而不用clone
#[allow(dead_code)]
#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    //生成listener
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    //打印日志，这句移到前面，path的所有权后面会释放掉，为了少一次clone，所以这里放到前面
    info!("Serving {:?} on port {}", path, addr);

    //先创建一个state
    let state = HttpServeState { path };

    //绑定一个路由
    let router = Router::new()
        //这里加上*path， 表示匹配任意路径， 然后在路由中处理
        .route("/*path", get(index_handler))
        //Arc的作用就是讲外部变量带入到异步线程中，会创建一个内存区域，所有传入这个state的异步线程的线程都可以访问到这个内存区域
        //直到state的引用计数归0，内存区域才会释放
        .with_state(Arc::new(state));

    //绑定路由和监听器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    //执行到这里就会启动服务， 等待http请求进来
    //使用rest client 测试，index路径： http://localhost:8080/
    axum::serve(listener, router).await?;

    Ok(())
}

// State(state): State<Arc<HttpServeState>>
// 这里直接写成这样称为 pattern match，可以直接match类型
// 注意这里的返回值要改成 string, 因为入参 PathBuf 比起 path 的区别就是 PathBuf 里面相当于一个 string
async fn index_handler(
    State(state): State<Arc<HttpServeState>>,
    //注意这里的Path要使用 axum::extract::Path
    Path(path): Path<String>,
) -> String {
    //通过format！返回个String， 重启服务后，可以看到异步线程能够获取到 state 变量，并打印出来
    format!("{:?}, {:?}", state, path)
}
