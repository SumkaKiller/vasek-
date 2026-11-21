use std::io::{self, Write};
mod llm;
mod conservation;
use crate::conservation::Message;

pub const apikey: &str = include_str!("api_key.txt").trim_ascii();


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    //pub const apikey: &str = include_str!("api_key.txt").trim_ascii();
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

        let raw_text = llm::llm_req(&client, &history).await?;

        println!("колонка ответила: {}", raw_text);

        history.push(Message {
            role: "assistant".to_string(),
            content: raw_text,
        });
    }
    Ok(())
}
