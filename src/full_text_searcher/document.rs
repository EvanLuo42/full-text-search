use crate::full_text_searcher::linguistic_processor::LinguisticProcessor;
use crate::full_text_searcher::term_weight::TermWeight;
use crate::full_text_searcher::tokenizer::Tokenizer;

#[derive(Clone, Debug)]
pub struct Document {
    pub text: String,
    pub(crate) terms: Vec<String>,
    pub terms_weight: Vec<(String, f32)>
}

impl Document {
    pub fn new(text: &str) -> Document {
        Document {
            text: text.to_string(),
            terms: Vec::new(),
            terms_weight: Vec::new()
        }
    }

    pub fn calculate_terms_weight(&mut self, documents: Vec<Document>) {
        let mut tokenizer = Tokenizer::new(self.text.as_str());
        let processor = LinguisticProcessor::new();
        let tokens = tokenizer.tokenize();
        let terms = processor.lemmatize(tokens);

        self.terms = terms.clone();
        self.terms_weight = TermWeight::new().calculate_weight(terms, documents)
    }
}