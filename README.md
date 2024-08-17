# Ollama Chat API

This project is a Rust-based web service that uses `axum` and `ollama-rs` to provide a RESTful API interface for interacting with Ollama's chat models. The API supports both streaming output and standard JSON responses.

## Prerequisites

Before running this service, make sure to complete the following steps:

1. **Install and Start the Ollama Service**: Ensure that the Ollama service is up and running at `http://localhost:11434`.
2. **Pull the Model**: After starting the Ollama service, pull the `qwen2:0.5b` model (or another model of your choice). Use the following command:

```bash
ollama pull qwen2:0.5b
```

## Starting the Service

Once the prerequisites are met, you can start the service using `cargo`:

```bash
cargo run
```

The service will start and listen on `http://127.0.0.1:3000`.

## API Usage

### 1. Standard JSON Response

To receive a one-time JSON response, make a POST request to the API with the `stream` parameter set to `false`.

#### Example Request:

```bash
curl --location 'http://127.0.0.1:3000/chat' \
--header 'Content-Type: application/json' \
--data '{
    "model": "qwen2:0.5b",
    "prompt": "Why is the sky blue?",
    "stream": false
}'
```

#### Example Response:

```json
{
  "response": "The reason why the sky appears to be blue is due to the Rayleigh scattering effect. This phenomenon occurs when light passing through a cloud or fog causes small, spherical molecules in the cloud to scatter light more than they would under normal conditions of air scattering. The resulting change in light intensity can cause the clouds around us to appear to be blue.\n\nThis process is commonly used for scientific research and it also helps to reduce noise pollution by dispersing microwaves emitted from electronic devices like televisions, radios, and computers. The sky appears blue because this phenomenon occurs when sunlight passes through the cloud's molecules and causes them to scatter light more than normal, causing a blue light spectrum.\n\nThe sky seems blue to us due to Rayleigh scattering effect which is also called Rayleigh scattering or Rayleigh Doppler Effect. It can be seen as clouds scatter blue light more heavily than normal while red light gets scattered less much. This means that blue light appears to come from behind the clouds, giving it a cloud-like appearance.\n\nThe reason why this phenomenon occurs in the sky is due to the way light interacts with air molecules and how these molecules reflect light in different ways depending on their properties. The sun's rays are reflected by the water droplets and other small particles inside of our atmosphere which helps to give us the blue color we see."
}
```

### 2. Streaming Output (SSE)

To receive a streaming response via Server-Sent Events (SSE), make a POST request to the API with the `stream` parameter set to `true` or omit the parameter.

#### Example Request:

```bash
curl --location 'http://127.0.0.1:3000/chat' \
--header 'Content-Type: application/json' \
--data '{
    "model": "qwen2:0.5b",
    "prompt": "Why is the sky blue?",
    "stream": true
}'
```

#### Response Format:

Each part of the response will be sent as `text/event-stream`, containing a JSON object like this:

```text
data:{"content":"The"}

data:{"content":" sky"}

data:{"content":" is"}

data:{"content":" blue"}

data:{"content":" because"}

data:{"content":" it"}

data:{"content":" reflects"}

data:{"content":" sunlight"}

.............................

data:{"content":" This"}

data:{"content":" effect"}

data:{"content":" of"}

data:{"content":" natural"}

data:{"content":" phenomena"}

data:{"content":" plays"}

data:{"content":" an"}

data:{"content":" important"}

data:{"content":" role"}

data:{"content":" too"}

data:{"content":" in"}

data:{"content":" maintaining"}

data:{"content":" our"}

data:{"content":" view"}

data:{"content":"able"}

data:{"content":" skies"}

data:{"content":" as"}

data:{"content":" we"}

data:{"content":" age"}

data:{"content":" and"}

data:{"content":" the"}

data:{"content":" world"}

data:{"content":" around"}

data:{"content":" us"}

data:{"content":" does"}

data:{"content":" not"}

data:{"content":" always"}

data:{"content":" seem"}

data:{"content":" as"}

data:{"content":" blue"}

data:{"content":" as"}

data:{"content":" it"}

data:{"content":" should"}

data:{"content":" be"}

data:{"content":"."}

data:{"content":""}
```

The client will receive these events incrementally until the generation is complete.

## Project Structure

- `main.rs`: Main service logic, defining the `/chat` route and its handler.
- `Cargo.toml`: Project dependency management, defining the dependencies and their versions.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests to improve this project. If you have any questions or suggestions, don't hesitate to reach out!
