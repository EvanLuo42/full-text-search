use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

#[cfg(test)]
mod test {
    #[test]
    fn test_get_original_word() {
        todo!()
    }
}
