pub mod document;
pub mod search;
pub mod term_weight;
pub mod linguistic_processor;
pub mod lemmatization;
pub mod tokenizer;
pub mod stop_words;

fn cos_similarity(vec_1: Vec<(String, f32)>, vec_2: Vec<(String, f32)>) -> f32 {
    let new_vec_2 = align_vec_dimension(&vec_1, &vec_2);
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

fn align_vec_dimension(vec_1: &Vec<(String, f32)>, vec_2: &Vec<(String, f32)>) -> Vec<(String, f32)> {
    let mut new_vec_2 = Vec::new();
    for (key, _value) in vec_1.iter() {
        let mut counter = 0;
        for (key2, value2) in vec_2.iter() {
            if key == key2 {
                counter += 1;
                new_vec_2.push((key2.clone(), *value2));
            }
        }
        if counter == 0 {
            new_vec_2.push((key.clone(), 0.0));
        }
    }
    new_vec_2
}