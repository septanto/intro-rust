#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let deck = Deck { cards: vec![]};
    println!("Here's your deck: {:?}", deck);
}
