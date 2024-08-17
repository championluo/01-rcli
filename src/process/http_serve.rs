use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

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
    let state = HttpServeState { path: path.clone() };

    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();

    //绑定一个路由
    let router = Router::new()
        .nest_service("/tower", dir_service)
        .route("/*path", get(file_handler))
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
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    //注意这里的Path要使用 axum::extract::Path
    //通过axum::extract, 可以传入任意类型的变量
    Path(path): Path<String>,
) -> (StatusCode, String) {
    //通过将启动时候的参数dir和path拼接起来，得到具体的文件路径
    let p = std::path::Path::new(&state.path).join(path);
    info!("Read file {:?}", p);

    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    } else {
        //read_to_string 这里使用了 read_to_string, 如果文件是个二进制文件则会读取失败
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            // path: PathBuf::from("src/process"),
            path: PathBuf::from("."),
        });
        //这个方法会合并 路径， 并读取合并后路径指定的文件
        let (status, content) =
        // file_handler(State(state),Path("http_serve.rs".to_string())).await;
        file_handler(State(state),Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
