use std::collections::HashMap;
use rand::prelude::*;

const ORDER: usize = 2;


fn generate_markov_chain( text: &str, order: usize ) -> HashMap<String, Vec<String>> {
    let mut markov_chain = HashMap::new();
    let words = text.split_whitespace().collect::<Vec<&str>>();
    for i in 0..words.len() - order {
        let key = words[i..i + order].join(" ");
        let value = words[i + order].to_string();
        markov_chain.entry(key).or_insert(Vec::new()).push(value);
    }
    markov_chain
}

fn generate_quote( markov_chain: HashMap<String, Vec<String>>, _order: usize, length: usize ) -> String {
    let mut rng = thread_rng();
    let mut quote = String::new();
    let mut key = markov_chain.keys().choose(&mut rng).unwrap().to_string();
    quote.push_str(&key);
    quote.push(' ');
    for _ in 0..length {
        let values = markov_chain.get(&key).unwrap();
        let value = values.choose(&mut rng).unwrap();
        quote.push_str(value);
        quote.push(' ');
        let words = key.split_whitespace().collect::<Vec<&str>>();
        key = words[1..].join(" ");
        key.push(' ');
        key.push_str(value);
    }
    quote
}

fn main() {
    let text = "I am the very model of a modern major general";
    let markov_chain = generate_markov_chain(text, ORDER);
    let quote = generate_quote(markov_chain, ORDER, 30);
    println!("{}", quote);
}