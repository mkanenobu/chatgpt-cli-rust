use tiktoken_rs::get_bpe_from_model;

pub fn get_text_token_count(model: &str, text: &str) -> usize {
    let mut m = model;
    if m.starts_with("gpt-4o") {
        // if model is gpt-4o then use gpt-4, tiktoken-rs does not support gpt-4o
        m = "gpt-4"
    }
    let encoder = get_bpe_from_model(m).unwrap();
    let tokens = encoder.encode_with_special_tokens(text);
    tokens.len()
}
