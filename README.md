<p align="center">
    <h1 align="center" >🦀 rs-openai</h1>
    <p align="center">
        A Rust crate for easy serving of OpenAI's GPT-3 API, with rate limiting and token use tracking out of the box.
    </p>
</p>

## Features
- [x] Rust crate for API access
- [x] Base API server
- [x] Rate Limiting based off of user ID
- [ ] Per user token tracking

## Usage
Create a `.env` at the root of this project and fill out your API Key

```dotenv
GPT_KEY=...
```

Then run `cargo run` in this directory to start the server on port `:8000`.

## With Docker
```bash
$ docker build . -t rs-openai:latest
$ docker run -p 8000:8000 rs-openai:latest
```