use crate::full_text_searcher::lemmatization::Lemmatization;

pub struct LinguisticProcessor {
    lemmatization: Lemmatization
}

impl LinguisticProcessor {
    pub fn new() -> LinguisticProcessor {
        LinguisticProcessor {
            lemmatization: Lemmatization::new()
        }
    }

    pub fn lemmatize(&self, tokens: Vec<String>) -> Vec<String> {
        tokens
            .iter()
            .map(|token| self.lemmatization.get_original_word(token))
            .collect()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_lemmatize() {
        todo!()
    }
}