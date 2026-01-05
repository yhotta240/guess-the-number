use axum::{
    Json,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::cmp::Ordering;

use crate::game::SharedState;
use crate::models::{ErrorResponse, InputData, ResponseData};

/// エラーレスポンス
pub struct ApiError(String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: self.0 }),
        )
            .into_response()
    }
}

/// 推測値の範囲を検証
fn validate_guess(guess: u32) -> Result<(), String> {
    if guess < 1 || guess > 100 {
        return Err("予想は1から100の数字でなければなりません".to_string());
    }
    Ok(())
}

/// 数当てゲームのエンドポイントハンドラー
pub async fn guess_number(
    State(state): State<SharedState>,
    payload: Result<Json<InputData>, JsonRejection>,
) -> Result<Json<ResponseData>, ApiError> {
    // JSON解析エラーをキャッチ
    let Json(input) = payload.map_err(|e| {
        tracing::warn!("無効なリクエスト形式: {:?}", e);
        ApiError("無効なリクエスト形式です．数値を入力してください".to_string())
    })?;

    let mut game = state.lock().await;

    // リセット要求があればゲームをリセット
    if input.reset.unwrap_or(false) {
        game.reset();
        tracing::info!("ゲームがリセットされました");
        tracing::info!("シークレッナンバー: {}", game.secret_number);
        return Ok(Json(ResponseData::reset()));
    }

    // 推測値の検証
    validate_guess(input.guess).map_err(|e| {
        tracing::warn!("無効な推測値: {}", input.guess);
        ApiError(e)
    })?;

    let guess = input.guess;
    game.attempts += 1;

    tracing::info!("試行 {}: 予想 = {}", game.attempts, guess);

    let response = match guess.cmp(&game.secret_number) {
        Ordering::Less => {
            tracing::debug!("小さすぎ");
            ResponseData::too_low(game.attempts)
        }
        Ordering::Greater => {
            tracing::debug!("大きすぎ");
            ResponseData::too_high(game.attempts)
        }
        Ordering::Equal => {
            tracing::info!("正解！ 試行回数: {}", game.attempts);
            ResponseData::correct(game.attempts)
        }
    };

    Ok(Json(response))
}
