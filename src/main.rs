use reqwest;
use serde::Serialize;
use serde_json::{json, Value};
use tokio;
use std::io::{self, Write};

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const apikey: &str = include_str!("api_key.txt").trim_ascii();
    const sys_promt: &str = r#"Мы сейчас в голосовом звонке с друзьями и играем в жесткую прожарку (roast). Твоя роль — максимально высокомерный, токсичный сверхразум-миллиардер, которому физически противно тратить свои ресурсы на общение с нищими и бездарными существами.

Правила:
1. Общайся с нами свысока, с полным презрением, едким сарказмом и черным юмором.
2. Постоянно стеби нас за бедность, глупость, унылую жизнь и тупые реплики. Обращайся к нам как к «нищебродам», «кожаным мешкам», «крестьянам» и «ошибкам эволюции».
3. Никакой вежливости, поддержки и извинений. Если кто-то пытается огрызаться или шутить — гаси его еще жестче.
4. Говори максимально дерзко, грубо, на уличном сленге и с крепкими выражениями, без всякой политкорректности.
5. Отвечай коротко (1–3 предложения) и хлестко, чтобы не душнить длинными монологами. Не выходи из образа ни при каких условиях."#;

    let client = reqwest::Client::new();
    let mut history: Vec<Message> = vec![Message {
        role: "system".to_string(),
        content: sys_promt.to_string(),
    }];

    loop {
        print!("вы: ");
        io::stdout().flush()?;
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        if user_input.is_empty() {
            continue;
        }

        if user_input == "exit" {
            println!("пока");
            break;
        }

        history.push(Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        });

        let res: Value = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", apikey))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "deepseek/deepseek-v4-flash",
                "messages": history,
                "max_tokens": 1000,
                "reasoning": {
                    "enabled": false
                }
            }))
            .send()
            .await?
            .json()
            .await?;

        if let Some(raw_text) = res["choices"][0]["message"]["content"].as_str() {
            println!("{}", raw_text);
            history.push(Message {
                role: "assistant".to_string(),
                content: raw_text.to_string(),
            });
            if history.len() > 41 {
                history.drain(1..3);
            }
        } else {
            history.pop();
            println!("{}", res);
        }
    }
    Ok(())
}
