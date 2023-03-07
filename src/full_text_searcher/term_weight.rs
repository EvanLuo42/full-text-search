use crate::full_text_searcher::document::Document;

pub struct TermWeight;

impl TermWeight {
    pub fn new() -> TermWeight {
        TermWeight {}
    }

    pub fn calculate_weight(&self, terms: Vec<String>, documents: Vec<Document>) -> Vec<(String, f32)> {
        let mut weights = Vec::new();
        let mut terms_no_duplicates: Vec<String> = terms.clone();
        terms_no_duplicates.dedup();
        terms_no_duplicates
            .iter()
            .for_each(|term| {
                let tf = terms.iter().filter(|&t| t == term).count() as f32;
                let df = documents
                    .iter()
                    .filter(|document| document.terms.contains(term))
                    .count() as u32;
                let n = documents.len() as u32;
                if df == 0 {
                    weights.push((term.clone(), tf));
                    return;
                }
                weights.push((term.clone(), tf * ((n / df) as f32).log10()));
            });
        weights
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_calculate_weight() {
        todo!()
    }
}