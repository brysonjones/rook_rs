use rook_rs;

#[test]
fn test_new_deck_size() {
    let deck = rook_rs::Deck::new();
    assert_eq!(deck.remaining(), 57);
}

#[test]
fn test_draw_card() {
    let mut deck = rook_rs::Deck::new();
    let card = deck.draw();
    assert!(card.is_some());
    assert_eq!(deck.remaining(), 56);
}

#[test]
fn test_deal_hand() {
    let mut deck = rook_rs::Deck::new();
    let hand = deck.deal_hand(5).unwrap();
    assert_eq!(hand.len(), 5);
    assert_eq!(deck.remaining(), 52);
}

#[test]
fn test_deal_too_many_cards() {
    let mut deck = rook_rs::Deck::new();
    let result = deck.deal_hand(58);  // More than deck size
    assert!(result.is_none());
    assert_eq!(deck.remaining(), 57);  // Deck should be unchanged
}
