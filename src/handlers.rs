use axum::{extract::State, Json};
use std::cmp::Ordering;

use crate::game::SharedState;
use crate::models::{InputData, ResponseData};

/// 数当てゲームのエンドポイントハンドラー
pub async fn guess_number(
    State(state): State<SharedState>,
    Json(input): Json<InputData>,
) -> Json<ResponseData> {
    let mut game = state.lock().await;

    // リセット要求があればゲームをリセット
    if input.reset.unwrap_or(false) {
        game.reset();
        println!("ゲームがリセットされました");
        println!("シークレッナンバー: {}", game.secret_number);
        return Json(ResponseData::reset());
    }

    let guess = input.guess;
    game.attempts += 1;

    println!("試行 {}: 予想 = {}", game.attempts, guess);

    let response = match guess.cmp(&game.secret_number) {
        Ordering::Less => {
            println!("小さすぎ！");
            ResponseData::too_low(game.attempts)
        }
        Ordering::Greater => {
            println!("大きすぎ！");
            ResponseData::too_high(game.attempts)
        }
        Ordering::Equal => {
            println!("正解！ 試行回数: {}", game.attempts);
            ResponseData::correct(game.attempts)
        }
    };

    Json(response)
}
