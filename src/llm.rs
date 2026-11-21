use serde_json::{json, Value};
use crate::apikey;
use crate::conservation::Conversation;



pub async fn  llm_req(client: &reqwest::Client) -> Result<String, Box<dyn std::error::Error>> {

    //let client = reqwest::Client::new();
    let res: Value = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", apikey))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": "deepseek/deepseek-v4-flash",
            "messages": his,
            "max_tokens": 1000,
            "reasoning": {
                "enabled": false
            }
        }))
        .send()
        .await?
        .json()
        .await?;

    let raw_text: &str = res["choices"][0]["message"]["content"].as_str().ok_or_else(||std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid responsr"))?;

    Ok(raw_text.to_string())
}
