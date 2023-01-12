use crate::openai::completion;
use async_openai::Client as OpenAIClient;
use spinoff::{Color, Spinner, Spinners};

pub async fn evaluator<'a>(openai_client: &'a OpenAIClient, line: &str) {
    let spinner = Spinner::new(Spinners::Dots, "Waiting for response...", Color::White);
    let completion_result = completion(openai_client, line).await;
    spinner.stop();

    match completion_result {
        Ok(response) => {
            println!("{}", response.choices[0].text);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    };
}
