use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{u32};

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

fn cos_similarity(vec_1: Vec<(String, f32)>, vec_2: Vec<(String, f32)>) -> f32 {
    let mut new_vec_2 = Vec::new();
    for (key, _value) in vec_1.iter() {
        let mut counter = 0;
        for (key2, value2) in vec_2.iter() {
            if key == key2 {
                counter += 1;
                new_vec_2.push((key2, *value2));
            }
        }
        if counter == 0 {
            new_vec_2.push((key, 0.0));
        }
    }
    let n = vec_1.len();
    let mut ab_dot_product = 0.0;
    let mut a_square_sum = 0.0;
    let mut b_square_sum = 0.0;
    for i in 0..n {
        ab_dot_product += (new_vec_2[i].1 * vec_1[i].1) as f32;
        a_square_sum += vec_1[i].1.powf(2.0) as f32;
        b_square_sum += new_vec_2[i].1.powf(2.0) as f32;
    }
    if ab_dot_product / (a_square_sum.sqrt() * b_square_sum.sqrt()) == f32::NAN {
        return 0.0;
    }
    ab_dot_product / (a_square_sum.sqrt() * b_square_sum.sqrt())
}

#[derive(Clone, Debug)]
pub struct Document {
    pub text: String,
    terms: Vec<String>,
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

pub struct Lemmatization {
    pairs: HashMap<String, String>
}

impl Lemmatization {
    pub fn new() -> Lemmatization {
        let path = Path::new("./lemmatization-en.txt");
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why.to_string()),
            Ok(file) => file,
        };
        let mut pairs = HashMap::new();
        for line in BufReader::new(file).lines() {
            let line = line.expect("Could not parse line");
            let mut words = line.split_whitespace();
            let word = words.next().unwrap();
            let lemma = words.next().unwrap();
            pairs.insert(lemma.to_string(), word.to_string());
        }
        Lemmatization { pairs }
    }

    pub fn get_original_word(&self, lemma: &str) -> String {
        self.pairs
            .get(lemma)
            .unwrap_or(&lemma.to_string())
            .to_string()
    }
}

#[derive(Debug)]
pub struct Tokenizer {
    pub tokens: Vec<String>,
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

#[derive(Debug)]
struct StopWords {
    words: Vec<String>
}

impl StopWords {
    fn new() -> StopWords {
        let path = Path::new("./stop_words.txt");
        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why.to_string()),
            Ok(file) => file,
        };
        StopWords {
            words: BufReader::new(file)
                .lines()
                .map(|line| line.expect("Could not parse line"))
                .collect()
        }
    }
    fn is_stop_word(&self, word: &str) -> bool {
        self.words.contains(&word.to_string())
    }
}