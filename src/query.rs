// use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

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

pub async fn query() -> Result<(), reqwest::Error> {
    let prompt = "Build a lesson plan on the third conditional";

    // let api_key = std::env::var("OPEN_AI").unwrap();

    // let client = Client::new();
    // let response_body: Post = client
    //     .post("https://api.openai.com/v1/completions")
    //     .bearer_auth(api_key)
    //     .json(
    //         &serde_json::json!({"model": "text-davinci-003", "prompt": prompt, "max_tokens": 100}),
    //     )
    //     .send()
    //     .await?
    //     .json()
    //     .await?;

    let response_body = Post {
        id: "cmpl-3OGvkhP19sWhlYZOT5UtAC7ajvDzy".to_owned(),
        object: "text_completion".to_owned(),
        created: 1626913964,
        model: "davinci:2020-05-03".to_owned(),
        choices: vec![Choice {
            text: "\n\nObjective \n\nAt the end of this lesson plan, the students should understand the concept and proper use of the third conditional.\n\nEstimated duration\n\n45 minutes\n\nMaterials\n\nHandouts of example sentences, worksheet with fill-in-the-blank exercises \n\nProcedures \n\n1. Introduction (5 minutes)\n\na. Ask the students to name situations where they think the past might be discussed. \n\nb."
                .to_owned(),
            index: 0,
            logprobs: Value::Null,
            finish_reason: "length".to_owned(),
        }],
    };
    println!("{:#?}", response_body);
    println!("");
    println!("Prompt: {}", prompt);
    println!("{}", response_body.choices[0].text);

    Ok(())
}
