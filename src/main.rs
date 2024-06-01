use axum::Router;
use rs_aksem::{app_state, handler};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 初始化 Axum 全局状态
    let app_state = Arc::new(app_state::new(()));
    // 实例化路由
    let routes = handler::routes(app_state);
    // 实例化 App
    let app = Router::new().merge(routes);

    // Tcp 监听器
    // TODO: addr 应该通过配置或全局静态配置获取
    let tcp_listener = TcpListener::bind("0.0.0.0:8888").await?;

    // 启动服务
    // FIXME: 修复错误
    axum::serve(tcp_listener, app).await?;

    Ok(())
}
