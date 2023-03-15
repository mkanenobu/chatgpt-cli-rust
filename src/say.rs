use tokio::process::{Child, Command};

#[allow(dead_code)]
pub async fn say(text: &str) -> Child {
    Command::new("sh")
        .arg("-c")
        .arg(format!("say {}", text))
        .spawn()
        .expect("Failed to execute `say` command")
}
