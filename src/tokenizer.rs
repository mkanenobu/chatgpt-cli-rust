use tiktoken_rs::get_bpe_from_model;
use anyhow::Result;

pub fn get_text_token_count(model: &str, text: &str) -> Result<usize> {
    let encoder = get_bpe_from_model(model)?;
    let tokens = encoder.encode_with_special_tokens(text);
    Ok(tokens.len())
}
