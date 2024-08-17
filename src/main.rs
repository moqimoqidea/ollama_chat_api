use axum::{
    extract::State,
    response::Json,
    routing::post,
    Router,
};
use ollama_rs::{
    Ollama,
    generation::chat::{ChatMessage, ChatMessageResponse},
    generation::chat::request::ChatMessageRequest,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct ChatRequest {
    model: String,
    prompt: String,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
}

#[tokio::main]
async fn main() {
    // 使用 Arc<Mutex<Ollama>> 来共享 Ollama 实例，并保证线程安全
    let ollama = Arc::new(Mutex::new(Ollama::default()));

    // 构建路由
    let app = Router::new()
        .route("/chat", post(chat_handler))
        .with_state(ollama);

    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 处理 POST 请求的 handler
async fn chat_handler(
    State(ollama): State<Arc<Mutex<Ollama>>>,
    Json(payload): Json<ChatRequest>,
) -> Json<ChatResponse> {
    let model = payload.model;
    let prompt = payload.prompt;

    // 使用 ChatMessageRequest 来构建聊天消息请求
    let request = ChatMessageRequest::new(
        model,
        vec![ChatMessage::user(prompt)],
    );

    // 调用 ollama 的 chat 功能
    let ollama = ollama.lock().await;
    let res: Result<ChatMessageResponse, _> = ollama.send_chat_messages(request).await;

    // 处理响应
    match res {
        Ok(chat_res) => {
            // 提取 message 中的内容
            let response_text = chat_res.message.map(|msg| msg.content).unwrap_or_else(|| "No response".to_string());
            Json(ChatResponse {
                response: response_text,
            })
        },
        Err(_) => Json(ChatResponse {
            response: "Error occurred while processing the chat.".to_string(),
        }),
    }
}
