use axum::{
    extract::State,
    response::Json,
    routing::post,
    Router,
};
use futures::StreamExt;
use ollama_rs::{
    Ollama,
    generation::chat::{ChatMessage, ChatMessageResponseStream},
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
    stream: Option<bool>, // 新增 stream 参数，默认为 None
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
    let stream = payload.stream.unwrap_or(true); // 默认为 true

    // 使用 ChatMessageRequest 来构建聊天消息请求
    let request = ChatMessageRequest::new(
        model,
        vec![ChatMessage::user(prompt)],
    );

    let ollama = ollama.lock().await;

    if stream {
        // 流式输出
        let mut stream_res: ChatMessageResponseStream = ollama
            .send_chat_messages_stream(request)
            .await
            .unwrap();

        let mut full_response = String::new();
        while let Some(Ok(partial_res)) = stream_res.next().await {
            if let Some(assistant_message) = partial_res.message {
                full_response.push_str(&assistant_message.content);
            }
        }

        Json(ChatResponse {
            response: full_response,
        })
    } else {
        // 非流式输出
        let res = ollama.send_chat_messages(request).await;

        match res {
            Ok(chat_res) => {
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
}
