use crate::openai::completion;
use async_openai::Client as OpenAIClient;

pub fn evaluator<'a>(openai_client: &'a OpenAIClient) -> Box<dyn Fn(&str) + 'a> {
    Box::new(|line: &str| {
        println!("Prompt: {}", line);
        let completion_result = completion(openai_client, line);
        async {
            match completion_result.await {
                Ok(response) => {
                    println!("Response: {}", response.choices[0].text);
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        };
    })
}
