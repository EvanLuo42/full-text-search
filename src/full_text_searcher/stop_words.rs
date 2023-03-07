use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct StopWords {
    words: Vec<String>
}

impl StopWords {
    pub(crate) fn new() -> StopWords {
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
    pub(crate) fn is_stop_word(&self, word: &str) -> bool {
        self.words.contains(&word.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::full_text_searcher::stop_words::StopWords;

    #[test]
    fn test_is_stop_word() {
        let stop_words = StopWords::new();
        assert!(stop_words.is_stop_word("the"));
        assert!(!stop_words.is_stop_word("centralize"));
    }
}