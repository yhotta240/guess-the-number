use rand::Rng;
use rand::thread_rng;
use std::sync::Arc;
use tokio::sync::Mutex;

/// ゲームの状態を保持する構造体
#[derive(Clone)]
pub struct GameState {
    pub secret_number: u32,
    pub attempts: u32,
}

impl GameState {
    /// 新しいゲーム状態を作成
    pub fn new() -> Self {
        Self {
            secret_number: thread_rng().gen_range(1..=100),
            attempts: 0,
        }
    }

    /// ゲームをリセットして新しい秘密の数字を生成
    pub fn reset(&mut self) {
        self.secret_number = thread_rng().gen_range(1..=100);
        self.attempts = 0;
    }
}

/// 共有状態の型エイリアス
pub type SharedState = Arc<Mutex<GameState>>;
