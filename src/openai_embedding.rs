use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct OpenAIEmbeddingRequest {
    pub input: String,
    pub model: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenAIEmbeddingResponse {
    pub model: String,
    pub data: Vec<Data>,
    pub usage: Tokens,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub embedding: Vec<f32>,
}

#[derive(Deserialize, Debug)]
pub struct Tokens {
    pub prompt_tokens: i32,
    pub total_tokens: i32,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub async fn get_embedding(
    uri: &str,
    prompt: &str,
    model: &str,
    token: &str,
) -> Result<OpenAIEmbeddingResponse> {
    let openai_embed_req = OpenAIEmbeddingRequest {
        input: prompt.to_string(),
        model: model.to_string(),
    };

    let mut headers = HeaderMap::new();
    let header_authz = format!("Bearer {}", token.to_string());
    headers.insert(AUTHORIZATION, header_authz.parse().unwrap());
    headers.insert(ACCEPT, "application/json".to_string().parse().unwrap());
    headers.insert(
        CONTENT_TYPE,
        "application/json".to_string().parse().unwrap(),
    );

    let client = reqwest::Client::new();
    let resp = client
        .post(uri)
        .json(&openai_embed_req)
        .headers(headers)
        .send()
        .await?;

    let response = resp.json::<OpenAIEmbeddingResponse>().await?;
    Ok(response)
}
