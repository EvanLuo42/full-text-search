use crate::full_text_searcher::cos_similarity;
use crate::full_text_searcher::document::Document;

#[derive(Debug)]
pub struct Search {
    pub(crate) documents: Vec<Document>
}

impl Search {
    pub fn new() -> Search {
        Search {
            documents: Vec::new()
        }
    }

    pub fn add(&mut self, document: Document) {
        self.documents.push(document);
    }

    pub fn search(&mut self, query: &str) -> Vec<Document> {
        let mut result_documents = Vec::new();
        let mut query_document = Document::new(query);
        query_document.calculate_terms_weight(self.documents.clone());
        self.add(query_document.clone());
        self.documents.remove(self.documents.len() - 1);
        self.documents
            .iter()
            .for_each(|document| {
                result_documents.push(document.clone())
            });
        result_documents.sort_by(|a, b| {
            let a_similarity = cos_similarity(a.terms_weight.clone(), query_document.terms_weight.clone());
            let b_similarity = cos_similarity(b.terms_weight.clone(), query_document.terms_weight.clone());
            b_similarity.total_cmp(&a_similarity).reverse()
        });
        result_documents
    }
}