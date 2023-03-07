use crate::full_text_searcher::stop_words::StopWords;

#[derive(Debug)]
pub struct Tokenizer {
    tokens: Vec<String>,
    stop_words: StopWords
}

impl Tokenizer {
    pub fn new(text: &str) -> Tokenizer {
        Tokenizer {
            tokens: text
                .split_whitespace()
                .map(|word| word.to_lowercase())
                .map(|word| word.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '?', '!', '*'][..], ""))
                .collect(),
            stop_words: StopWords::new()
        }
    }

    pub fn tokenize(&mut self) -> Vec<String> {
        self.tokens
            .drain_filter(|word| !self.stop_words.is_stop_word(word))
            .collect()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_tokenize() {
        todo!()
    }
}