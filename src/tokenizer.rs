use tiktoken_rs::get_bpe_from_model;

pub fn get_text_token_count(model: &str, text: &str) -> usize {
    let encoder = get_bpe_from_model(model).unwrap();
    let tokens = encoder.encode_with_special_tokens(text);
    tokens.len()
}
