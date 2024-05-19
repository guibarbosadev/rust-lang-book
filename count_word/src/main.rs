use std::collections::HashMap;

fn main() {
    let text = String::from("Um ninho de mafagafos tinha sete mafagafinhos. Quem desmafagar o ninho de mafagafos bom desmafagafador será.");
    let mut words = HashMap::new();

    for word in text.to_lowercase().split_whitespace() {
        let count = words.entry(String::from(word)).or_insert(0);
        *count += 1;
    }

    println!("{:?}", words);
}
