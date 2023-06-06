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
    let subject = "English";
    let grade_level = "10th Grade";
    let prompt = format!(
        r#"""
As a {subject} teacher, I need a comprehensive lesson plan on {prompt} for my {grade_level} students. The lesson plan should cover the following aspects:

1. Objective: Engage students in understanding the key concepts, skills, or themes related to {prompt}.
2. Warm-up Activity: Begin the lesson with an engaging activity or discussion to pique students' interest and activate prior knowledge.
3. Introduction: Provide background information or context on {prompt} to set the foundation for the lesson.
4. Content Exploration: Explore the main content or skills related to {prompt} through interactive teaching methods such as presentations, demonstrations, or multimedia resources.
5. Hands-on Practice: Engage students in hands-on activities, exercises, or experiments that reinforce their understanding of {prompt}.
6. Application and Extension: Provide opportunities for students to apply what they have learned to real-life situations or engage in higher-order thinking tasks to extend their understanding.
7. Assessment: Include formative or summative assessment strategies to gauge students' learning and provide feedback.
8. Differentiation: Consider strategies to differentiate instruction based on students' varying abilities, interests, or learning styles.
9. Resources and Materials: Provide a list of additional resources, references, or materials that can further support students' learning on {prompt}.
10. Reflection: Encourage students to reflect on their learning and make connections to their own lives or the world around them.

Please generate a detailed lesson plan with step-by-step instructions, engaging activities, and suitable assessment methods for {grade_level} students. The lesson plan should be aligned with the {subject} curriculum standards and incorporate interactive and student-centered teaching strategies. Please make sure the response is Markdown formatted just like this prompt is.

Thank you!
"""#
    );

    let api_key = std::env::var("OPENAI_API_KEY").expect("No variable found");
    let client = Client::new();
    let model = "text-davinci-003";
    // let model = "gpt-4";
    let params = &serde_json::json!({"model": model, "prompt": prompt, "max_tokens": 1000});
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

    println!("{:#?}", response_body);
    Ok(response_body)
}
