#![feature(drain_filter)]

pub mod document;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::document::{Document, Search};
    use super::*;

    #[test]
    fn it_works() {
        let mut search = Search::new();
        let mut text1 = Document::new("For example, in information retrieval and text mining, each word is assigned a different coordinate and a document is represented by the vector of the numbers of occurrences of each word in the document. Cosine similarity then gives a useful measure of how similar two documents are likely to be, in terms of their subject matter, and independently of the length of the documents.
The technique is also used to measure cohesion within clusters in the field of data mining.");
        text1.calculate_terms_weight(search.documents.clone());
        search.add(text1);
        let mut text2 = Document::new("The eukaryotic cell cycle cohesins consists of four distinct phases: G1 phase, S phase (synthesis), G2 phase (collectively known as interphase) and M phase (mitosis and cytokinesis). M phase is itself composed of two tightly coupled processes: mitosis, in which the cell's nucleus divides, and cytokinesis, in which the cell's cytoplasm divides forming two daughter cells. Activation of each phase is dependent on the proper progression and completion of the previous one. Cells that have temporarily or reversibly stopped dividing are said to have entered a state of quiescence called G0 phase.");
        text2.calculate_terms_weight(search.documents.clone());
        search.add(text2);
        let mut text3 = Document::new("During anaphase A, the cohesins that bind sister chromatids together are cleaved, forming two identical daughter chromosomes.[53] Shortening of the kinetochore microtubules M pulls the newly formed daughter chromosomes to opposite ends of the cell. During anaphase B, polar microtubules push against each other, causing the cell to elongate.[54] In late anaphase, chromosomes also reach their overall maximal condensation level, to help chromosome segregation and the re-formation of the nucleus.[55] In most animal cells, anaphase A precedes anaphase B, but some vertebrate egg cells demonstrate the opposite order of events.[53]");
        text3.calculate_terms_weight(search.documents.clone());
        search.add(text3.clone());
        println!("{:?}", search.search("m")
            .iter().map(|document| document.text.clone())
            .collect::<Vec<String>>()
        );
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
