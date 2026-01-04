mod game;
mod handlers;
mod models;

use axum::{Router, routing::post};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing_subscriber;

use game::GameState;
use handlers::guess_number;

#[tokio::main]
async fn main() {
    // ロギングの初期化
    tracing_subscriber::fmt::init();

    // ゲーム状態の初期化
    let game_state = Arc::new(Mutex::new(GameState::new()));

    let serve_dir = ServeDir::new("public");

    let app = Router::new()
        // 数当てゲームのエンドポイントを設定
        .route("/api/guess", post(guess_number))
        // 静的ファイルの提供
        .nest_service("/", serve_dir)
        // 状態を共有
        .with_state(game_state.clone());

    // すべてのネットワークインターフェースでバインド
    let listener = match TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            tracing::error!("ポート 3000 へのバインドに失敗しました: {}", e);
            std::process::exit(1);
        }
    };

    println!("========================================");
    println!("🎮 数当てゲームサーバーを起動しました");
    println!("========================================");
    println!("ローカル: http://localhost:3000");
    println!("ローカル: http://127.0.0.1:3000");

    // ローカルIPアドレスを取得して表示
    if let Ok(hostname) = hostname::get() {
        if let Some(hostname_str) = hostname.to_str() {
            println!("ホスト名: http://{}:3000", hostname_str);
        }
    }

    // ネットワークインターフェースのIPアドレスを表示
    match local_ip_address::local_ip() {
        Ok(ip) => {
            println!("LAN内のデバイスから: http://{}:3000", ip);
        }
        Err(_) => {
            println!("LAN内のIPアドレスの取得に失敗しました");
        }
    }

    println!("========================================");
    println!(
        "シークレッナンバー: {}",
        game_state.lock().await.secret_number
    );
    println!("========================================");

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("サーバーエラー: {}", e);
        std::process::exit(1);
    }
}
