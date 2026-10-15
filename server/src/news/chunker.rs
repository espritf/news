/// Splits `text` into word-bounded chunks no longer than `max_chars`, so each chunk stays
/// within the embedding model's context window.
pub fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let extra = if current.is_empty() { 0 } else { 1 };
        if !current.is_empty() && current.len() + extra + word.len() > max_chars {
            chunks.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}
