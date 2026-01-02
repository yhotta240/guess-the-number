use serde::{Deserialize, Serialize};

/// クライアントからの入力データ
#[derive(Deserialize)]
pub struct InputData {
    pub guess: u32,
    pub reset: Option<bool>, // リセットフラグ（オプション）
}

/// サーバーからのレスポンスデータ
#[derive(Serialize)]
pub struct ResponseData {
    pub status: String,  // "correct", "too_low", "too_high", "reset"
    pub message: String, // メッセージ
    pub attempts: u32,   // 試行回数
}

impl ResponseData {
    /// リセット時のレスポンスを生成
    pub fn reset() -> Self {
        Self {
            status: "reset".to_string(),
            message: "新しいゲームを開始しました！1から100までの数字を予想してください。"
                .to_string(),
            attempts: 0,
        }
    }

    /// 小さすぎる場合のレスポンスを生成
    pub fn too_low(attempts: u32) -> Self {
        Self {
            status: "too_low".to_string(),
            message: "小さすぎ！".to_string(),
            attempts,
        }
    }

    /// 大きすぎる場合のレスポンスを生成
    pub fn too_high(attempts: u32) -> Self {
        Self {
            status: "too_high".to_string(),
            message: "大きすぎ！".to_string(),
            attempts,
        }
    }

    /// 正解時のレスポンスを生成
    pub fn correct(attempts: u32) -> Self {
        Self {
            status: "correct".to_string(),
            message: format!("正解！やったね！試行回数は {} 回です。", attempts),
            attempts,
        }
    }
}
