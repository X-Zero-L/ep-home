use std::collections::HashMap;
use reqwest::header::HeaderMap;
use serde_json::value::Value;

async fn get(url: String) -> Result<HashMap<String, Value>, reqwest::Error>{
    Ok(reqwest::get(url).await?.json::<HashMap<String, Value>>().await?)
}

async fn post(url: String, body: HashMap<String, Value>) -> Result<HashMap<String, Value>, reqwest::Error>{
    // post 请求要创建client
    let client = reqwest::Client::new();

    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // 发起post请求并返回
    Ok(client.post(url).headers(headers).json(&body).send().await?.json::<HashMap<String, Value>>().await?)
}

pub async fn get_usage(start: String, end: Option<String>) -> Result<HashMap<String, Value>, ()>{
    //let url = format!("https://api.nyanners.moe/openai/v2/usage?start_date={}&end_date={}", start, end);
    let mut url = format!("https://api.nyanners.moe/openai/v2/usage?start_date={}", start);
    if end.is_some() {
        let end = end.unwrap();
        url = format!("{}&end_date={}", url, end);
    }
    let resp = get(url).await;
    if resp.is_err() {
        return Err(());
    }
    let resp = resp.unwrap();
    if resp.contains_key("error") {
        return Err(());
    }
    Ok(resp)
    /*
    // 解析为String
    let mut resp_str = String::new();
    for (key, value) in resp.unwrap() {
        resp_str.push_str(&format!("{}: {},\n", key, value));
    }
    // 结尾去掉多余的逗号
    resp_str.pop();
    resp_str.pop();
    resp_str
    */
}