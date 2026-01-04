/* サーバーに推測を送信する関数 */
async function sendGuess(params) {
  const response = await fetch("/api/guess", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(params),
  });
  return response;
}

let bestScore = localStorage.getItem("bestScore") ? parseInt(localStorage.getItem("bestScore")) : null;

// 最高記録を表示
if (bestScore) {
  document.getElementById("bestScore").textContent = bestScore;
}

const guessInput = document.getElementById("guessInput");
const submitBtn = document.getElementById("submitGuess");
const feedbackDiv = document.getElementById("feedback");
const attemptsSpan = document.getElementById("attempts");
const resetBtn = document.getElementById("resetBtn");
const resetScoreBtn = document.getElementById("resetScoreBtn");

// 送信ボタンのクリックイベント
submitBtn.addEventListener("click", handleGuess);

// Enterキーでの送信
guessInput.addEventListener("keypress", (e) => {
  if (e.key === "Enter") {
    handleGuess();
  }
});

// リセットボタンのクリックイベント
resetBtn.addEventListener("click", resetGame);

// ベストスコアリセットボタンのクリックイベント
resetScoreBtn.addEventListener("click", resetBestScore);

async function handleGuess() {
  const guess = parseInt(guessInput.value);

  if (!guess || guess < 1 || guess > 100) {
    showFeedback("error", "1から100までの数字を入力してください");
    return;
  }

  // ボタンをローディング状態に
  submitBtn.classList.add("loading");
  submitBtn.disabled = true;

  try {
    const response = await sendGuess({ guess });

    // エラーレスポンスをチェック
    if (!response.ok) {
      const errorData = await response.json();
      showFeedback("error", errorData.error || "サーバーエラーが発生しました");
      return;
    }

    const data = await response.json();

    // 試行回数を更新
    attemptsSpan.textContent = data.attempts;

    if (data.status === "correct") {
      handleCorrectGuess(data);
    } else if (data.status === "too_low") {
      showFeedback("too-low", data.message);
    } else if (data.status === "too_high") {
      showFeedback("too-high", data.message);
    } else {
      showFeedback("info", data.message);
    }
  } catch (error) {
    showFeedback("error", "サーバとの通信に失敗しました");
    console.error("エラー:", error);
  } finally {
    submitBtn.classList.remove("loading");
    submitBtn.disabled = false;
    guessInput.value = "";
    guessInput.focus();
  }
}

function handleCorrectGuess(response) {
  // 最高記録を更新
  if (!bestScore || response.attempts < bestScore) {
    bestScore = response.attempts;
    localStorage.setItem("bestScore", bestScore);
    document.getElementById("bestScore").textContent = bestScore;
  }

  showFeedback("correct", response.message);
  attemptsSpan.textContent = response.attempts;

  // リセットボタンを表示
  resetBtn.style.display = "block";
  submitBtn.disabled = true;
  guessInput.disabled = true;
}

/* フィードバック表示関数 */
function showFeedback(type, message) {
  feedbackDiv.className = `feedback-section ${type}`;
  feedbackDiv.innerHTML = `<p class="feedback-text">${message}</p>`;
}

/* ゲームリセット関数 */
async function resetGame() {
  try {
    // サーバーにリセット要求を送信
    const response = await sendGuess({ guess: 0, reset: true });

    // UI をリセット
    attemptsSpan.textContent = "0";
    guessInput.disabled = false;
    guessInput.value = "";
    submitBtn.disabled = false;
    resetBtn.style.display = "none";

    showFeedback("info", response.message);
    guessInput.focus();
  } catch (error) {
    console.error("リセット失敗:", error);
    showFeedback("error", "リセットに失敗しました");
  }
}

/* ベストスコアリセット関数 */
function resetBestScore() {
  if (confirm("ベストスコアをリセットしますか？")) {
    localStorage.removeItem("bestScore");
    bestScore = null;
    document.getElementById("bestScore").textContent = "-";
    console.log("ベストスコアをリセットしました");
  }
}
