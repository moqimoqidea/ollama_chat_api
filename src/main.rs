use anyhow::Error;
use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    response::Json,
    routing::post,
};
use futures::Stream;
use futures::StreamExt;
use ollama_rs::{
    generation::chat::request::ChatMessageRequest,
    generation::chat::{ChatMessage, ChatMessageResponseStream},
    Ollama,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Deserialize)]
struct ChatRequest {
    model: String,
    prompt: String,
    stream: Option<bool>, // 默认为 true
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
}

#[tokio::main]
async fn main() {
    // 使用 Arc<Mutex<Ollama>> 来共享 Ollama 实例，并保证线程安全
    let ollama = Arc::new(Mutex::new(
        Ollama::new("http://localhost".to_string(), 11434)));

    // 构建路由
    let app = axum::Router::new()
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
) -> Result<Json<ChatResponse>, Sse<impl Stream<Item=Result<Event, Error>>>> {
    let model = payload.model;
    let prompt = payload.prompt;
    let stream = payload.stream.unwrap_or(true); // 默认为 true

    // 使用 ChatMessageRequest 来构建聊天消息请求
    let request = ChatMessageRequest::new(
        model,
        vec![ChatMessage::user(prompt)],
    );

    if stream {
        // 创建异步通道
        let (tx, rx) = mpsc::channel(100);

        // 克隆 ollama 以便在线程中使用
        let ollama = ollama.clone();

        // 启动异步任务来处理流式响应
        tokio::spawn(async move {
            let ollama = ollama.lock().await;

            let mut stream_res: ChatMessageResponseStream = match ollama
                .send_chat_messages_stream(request)
                .await
            {
                Ok(stream) => stream,
                Err(_) => {
                    let _ = tx.send(Err(anyhow::anyhow!("Failed to start chat stream"))).await;
                    return;
                }
            };

            // 处理流式响应
            while let Some(Ok(partial_res)) = stream_res.next().await {
                if let Some(assistant_message) = partial_res.message {
                    // 将响应内容包装在 JSON 中
                    let json_data = json!({ "content": assistant_message.content }).to_string();
                    let event = Event::default().data(json_data);
                    if tx.send(Ok(event)).await.is_err() {
                        // 如果客户端断开连接，停止发送
                        break;
                    }
                }
            }
        });

        // 返回流式响应
        Err(Sse::new(ReceiverStream::new(rx)).keep_alive(KeepAlive::default()))
    } else {
        // 非流式输出，返回标准 JSON 响应
        let ollama = ollama.lock().await;

        let res = ollama.send_chat_messages(request).await;

        // 生成完整的响应内容
        let response_text = match res {
            Ok(chat_res) => chat_res.message.map(|msg| msg.content).unwrap_or_else(|| "No response".to_string()),
            Err(_) => "Error occurred while processing the chat.".to_string(),
        };

        // 返回 JSON 格式的响应
        Ok(Json(ChatResponse { response: response_text }))
    }
}
