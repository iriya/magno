use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct TranslateRequest {
    q: String,
    target: String,
    #[serde(rename = "format")]
    format_type: String,
}

#[derive(Deserialize)]
struct TranslateResponse {
    data: TranslateData,
}

#[derive(Deserialize)]
struct TranslateData {
    translations: Vec<Translation>,
}

#[derive(Deserialize)]
struct Translation {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

/// 调用 Google Cloud Translation API (Basic v2)
/// api_key: 你的谷歌云 API Key
/// text: 需要翻译的文本
/// target_lang: 目标语言（例如中文简写 "zh-CN" 或繁体 "zh-TW"）
pub async fn google_translate(text: &str, target_lang: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = std::env::var("GOOGLE_TRANSLATE_API_KEY")
            .map_err(|_| "未找到 GOOGLE_TRANSLATE_API_KEY 环境变量，请检查配置")?;

    let url = format!(
        "https://translation.googleapis.com/language/translate/v2?key={}",
        api_key
    );

    let client = reqwest::Client::new();
    let body = TranslateRequest {
        q: text.to_string(),
        target: target_lang.to_string(),
        format_type: "text".to_string(),
    };

    let res = client.post(&url)
        .json(&body)
        .send()
        .await?
        .json::<TranslateResponse>()
        .await?;

    if let Some(translation) = res.data.translations.into_iter().next() {
        Ok(translation.translated_text)
    } else {
        Err("未能获取到翻译结果".into())
    }
}