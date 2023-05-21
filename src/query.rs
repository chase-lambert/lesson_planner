use reqwest::Client;
use serde::Deserialize;
// use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Post {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: i64,
    pub logprobs: ::serde_json::Value,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
}

// #[allow(dead_code)]
pub async fn run_query(prompt: &str) -> Result<Post, reqwest::Error> {
    println!("Your prompt: {prompt}");
    let api_key = std::env::var("OPENAI_API_KEY").expect("No variable found");
    let client = Client::new();
    let params = &serde_json::json!({"model": "text-ada-001", "prompt": prompt, "max_tokens": 100});
    let response_body: Post = client
        .post("https://api.openai.com/v1/completions")
        .bearer_auth(api_key)
        .json(params)
        .send()
        .await?
        .json()
        .await?;

    // let response_body = Post {
    //     id: "cmpl-3OGvkhP19sWhlYZOT5UtAC7ajvDzy".to_owned(),
    //     object: "text_completion".to_owned(),
    //     created: 1626913964,
    //     model: "davinci:2020-05-03".to_owned(),
    //     choices: vec![Choice {
    //         text: "... I had purple hair. It was a phase,\"I don't miss it because purpl
    // e has a lot of pigment. I mean it doesn't wash out. So again it is a throw away.\"\n\n2.
    // It motivated her to make better"
    //             .to_owned(),
    //         index: 0,
    //         logprobs: Value::Null,
    //         finish_reason: "length".to_owned(),
    //     }],
    // };
    // println!("{:#?}", response_body);
    // println!("");
    // println!("Once upon a time {}", response_body.choices[0].text);

    Ok(response_body)
}
