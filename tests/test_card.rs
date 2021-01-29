//! Tests for specific card implementations.


#[cfg(test)]
pub mod test_card {
    
    use dominion::game::Game;
    use dominion::cards::dominion::*;

    #[test]
    pub fn test_gardens() {
        let mut game = Game::default();
        let callbacks = dominion::sample_client::callbacks();
        
        let (p1v, others) = game.players.split_at_mut(1);
        let player1 = p1v.get_mut(0).unwrap();

        game.supply.insert(Box::new(Gardens), 200);
    
        let _ = player1.gain_to_hand(Box::new(Gardens), &mut game.supply, &mut game.trash, others, &callbacks);
        
        //println!("{}", player1.hand[5].victory_points(player1));  
        assert_eq!(player1.victory_points(), 4);
    }
}
