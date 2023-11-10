use tiktoken_rs::p50k_base;

pub fn get_text_token_count(text: &str) -> usize {
    let bpe = p50k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(text);
    tokens.len()
}
