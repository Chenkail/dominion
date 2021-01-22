#[cfg(test)]
mod test_player {
    use dominion::game::player::*;
    //use dominion::game::*;

    #[test]
    fn test_player_init() {
        let player = Player::default();

        //we check to see if everything is initalized correctly
        assert!(player.hand.len() == 5);
        assert!(player.deck.len() == 5);
        assert!(player.resources.actions == 0);
        //player.print_deck(); --> should be shuffled correctly
    }

    #[test]
    fn test_player_draw() {
        let mut player = Player::default();
        //draw 5, make sure everything checks out
        player.draw_cards(5);
        assert!(player.hand.len() == 10 && player.deck.is_empty());
        
        //test unreasonable draw
        player.draw_cards(5);
        assert!(player.hand.len() == 10 && player.deck.is_empty());

        //make a new player
        player = Player::default();
        player.draw_cards(14);
        assert!(player.hand.len() == 10 && player.deck.is_empty());
    }

}