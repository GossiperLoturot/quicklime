use crate::*;

pub async fn request_llm(
    cache: &mut lru::LruCache<app::Query, String>,
    input: &str,
    mode: usize,
    config: &app::Config,
) -> anyhow::Result<String> {
    let query: app::Query = (input.into(), mode);
    if let Some(output) = cache.get(&query) {
        log::info!("[cache hit] text: {}, mode: {}", input, mode);
        return Ok(output.into());
    }
    log::info!("[cache miss] text: {}, mode: {}", input, mode);

    let response = match config.llm {
        app::LLM_CHATGPT => request_llm_chatgpt(input, mode, config).await?,
        app::LLM_GROK => request_llm_grok(input, mode, config).await?,
        _ => unreachable!(),
    };

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "failed to request: {:?}",
            response.text().await
        ));
    }

    let data = response.json::<serde_json::Value>().await?;
    let extractor = jsonpath_rust::JsonPath::try_from("$.choices[*].message.content")?;
    let output: String = extractor
        .find(&data)
        .as_array()
        .and_then(|arr| arr.iter().flat_map(|item| item.as_str()).next())
        .unwrap_or("")
        .into();

    cache.put(query, output.clone());

    Ok(output)
}

pub async fn request_llm_chatgpt(
    input: &str,
    mode: usize,
    config: &app::Config,
) -> anyhow::Result<tauri_plugin_http::reqwest::Response> {
    let prompt = match mode {
        app::MODE_TRANSLATION => serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "system",
                    "content": format!("You are a professional translation engine. Please translate the text into {} without explanation.", config.language)
                },
                {
                    "role": "assistant",
                    "content": "Yes, I understand. Please give me the sentence. I reply only the translated sentence, otherwise reply empty string."
                },
                {
                    "role": "user",
                    "content": input
                }
            ]
        }),
        app::MODE_POLISHING => serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a professional polishing engine. Please polish the text."
                },
                {
                    "role": "assistant",
                    "content": "Yes, I understand. Please give me the sentence. I reply only the polished sentence, otherwise reply empty string."
                },
                {
                    "role": "user",
                    "content": input
                }
            ]
        }),
        app::MODE_COMPLETION => serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a professional completion engine. Please complete the text."
                },
                {
                    "role": "assistant",
                    "content": "Yes, I understand. Please give me the sentence."
                },
                {
                    "role": "user",
                    "content": input
                }
            ]
        }),
        _ => unreachable!(),
    };
    let client = tauri_plugin_http::reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&config.token)
        .header("Content-Type", "application/json")
        .json(&prompt)
        .send()
        .await?;
    Ok(response)
}

async fn request_llm_grok(
    input: &str,
    mode: usize,
    config: &app::Config,
) -> anyhow::Result<tauri_plugin_http::reqwest::Response> {
    let prompt = match mode {
        app::MODE_TRANSLATION => serde_json::json!({
            "model": "grok-2-latest",
            "messages": [
                {
                    "role": "system",
                    "content": format!("You are a professional translation engine. Please translate the text into {} without explanation.", config.language)
                },
                {
                    "role": "assistant",
                    "content": "Yes, I understand. Please give me the sentence."
                },
                {
                    "role": "user",
                    "content": input
                }
            ]
        }),
        app::MODE_POLISHING => serde_json::json!({
            "model": "grok-2-latest",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a professional polishing engine. Please polish the text."
                },
                {
                    "role": "assistant",
                    "content": "Yes, I understand. Please give me the sentence."
                },
                {
                    "role": "user",
                    "content": input
                }
            ]
        }),
        app::MODE_COMPLETION => serde_json::json!({
            "model": "grok-2-latest",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a professional completion engine. Please complete the text."
                },
                {
                    "role": "assistant",
                    "content": "Yes, I understand. Please give me the sentence."
                },
                {
                    "role": "user",
                    "content": input
                }
            ]
        }),
        _ => unreachable!(),
    };
    let client = tauri_plugin_http::reqwest::Client::new();
    let response = client
        .post("https://api.x.ai/v1/chat/completions")
        .bearer_auth(&config.token)
        .header("Content-Type", "application/json")
        .json(&prompt)
        .send()
        .await?;
    Ok(response)
}
