use axum::Router;
use rs_aksem::{app_state, handler};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 创建数据库连接池
    // TODO: database_url 应该通过配置或全局静态配置获取
    let pool = db::must_connect_pool("mysql://root:mypass@127.0.0.1:3306/test").await;

    // 初始化 Axum 全局状态
    let app_state = Arc::new(app_state::new(pool));
    // 实例化路由
    let routes = handler::routes(app_state);
    // 实例化 App
    let app = Router::new().merge(routes);

    // Tcp 监听器
    // TODO: addr 应该通过配置或全局静态配置获取
    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    let tcp_listener = TcpListener::bind(addr).await?;
    // TODO: 应该使用日志库打印
    println!("Listen on http://{}", addr.to_string());

    // 启动服务
    // FIXME: 修复错误
    axum::serve(tcp_listener, app).await?;

    Ok(())
}
