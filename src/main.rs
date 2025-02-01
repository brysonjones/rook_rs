use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Black,
    Rook
}

#[derive(Debug)]
pub struct Card {
    pub color: Color,
    pub value: u8,
}

impl Card {
    pub fn new(color: Color, value: u8) -> Self {
        Self { color, value }
    }

    pub fn get_value(&self) -> u8 {
        match self.color {
            Color::Rook => 20,
            _ => self.value,
        }
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn new_rook() -> Self {
        Self { color: Color::Rook, value: 20 }
    }
}

fn create_test_cards() -> Vec<Card> {
    vec![
        Card::new(Color::Red, 10),
        Card::new(Color::Yellow, 5),
        Card::new(Color::Green, 1),
        Card::new(Color::Black, 14),
        Card::new_rook(),
    ]
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(57);
        
        // Add number cards for each color
        for color in [Color::Red, Color::Yellow, Color::Green, Color::Black] {
            for value in 1..=14 {
                cards.push(Card::new(color, value));
            }
        }
        
        // Add the Rook card
        cards.push(Card::new(Color::Rook, 0));
        
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    pub fn deal_hand(&mut self, num_cards: usize) -> Option<Vec<Card>> {
        if self.remaining() < num_cards {
            return None;
        }

        let mut hand = Vec::with_capacity(num_cards);
        for _ in 0..num_cards {
            hand.push(self.draw().unwrap());
        }
        Some(hand)
    }
}

fn main() {
    let cards = create_test_cards();
    for card in &cards {
        println!("Card value: {}", card.get_value());
        println!("Card color: {:?}", card.get_color());
        println!("Full Card: {:?}", card);
    }
    println!("First card: {:?}", cards[0]); // Try adding this line
}
