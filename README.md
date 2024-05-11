# Side project generator

![feris logo](./src/public/assets/img/logo.png)


LLM powered side projects generator.

*Assistant speaks in French.*

## Requirements

- [Ollama](https://ollama.com/) < 1.35 (there is an issue with the response format of 1.35+ version)
- [LLM model of your choice (default: llama3)](https://ollama.com/library)

## Quickstart

First we need to define environment variables values :
- OLLAMA_BASE_URL : Your Ollama API endpoint URL (default : http://localhost:11434)
- LLM_MODEL : the model of your choice (default : llama3)

Then, run the application with :

```sh
cargo run
```

The application should run at http://localhost:8080.

## Demo

https://youtu.be/BMTvc3OF9Pk
