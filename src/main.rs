use axum::{
    extract::Json,
    routing::post,
    Router,
};
use ollama_rs::{Ollama, ChatMessage, ChatMessageRequest};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

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
    // 创建 Ollama 实例
    let ollama = Ollama::default();

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
    Json(payload): Json<ChatRequest>,
    ollama: axum::extract::State<Ollama>,
) -> Json<ChatResponse> {
    let model = payload.model;
    let prompt = payload.prompt;

    // 构建聊天消息请求
    let request = ChatMessageRequest::new(
        model,
        vec![ChatMessage::user(prompt)],
    );

    // 调用 ollama 的 chat 功能
    let res = ollama.send_chat_messages(request).await;

    // 处理响应
    match res {
        Ok(chat_res) => Json(ChatResponse {
            response: chat_res.response,
        }),
        Err(_) => Json(ChatResponse {
            response: "Error occurred while processing the chat.".to_string(),
        }),
    }
}
