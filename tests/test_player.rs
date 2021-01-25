//! Tests for player-related methods and structs

#[cfg(test)]
mod test_player {
    use dominion::game::{gamedata::*, player::*};
    use dominion::cards::dominion::*;

    #[test]
    fn test_player_init() {
        let player = Player::new_with_default_deck(0);

        //we check to see if everything is initalized correctly
        assert!(player.hand.len() == 5);
        assert!(player.deck.len() == 5);
        assert!(player.resources.actions == 0);
        //player.print_deck(); --> should be shuffled correctly
    }

    #[test]
    fn test_player_draw() {
        let mut player = Player::new_with_default_deck(0);
        //draw 5, make sure everything checks out
        player.draw_cards(5);
        assert!(player.hand.len() == 10 && player.deck.is_empty());
        
        //test unreasonable draw
        player.draw_cards(5);
        assert!(player.hand.len() == 10 && player.deck.is_empty());

        
        player = Player::new_with_default_deck(0);
        player.draw_cards(14);
        assert!(player.hand.len() == 10 && player.deck.is_empty());
    }

    #[test]
    fn test_player_discard() {
        let mut player = Player::new_with_default_deck(0);
        let first_vec = vec![0,2,4];
        player.discard_given_indexes(first_vec);
        assert!(player.hand.len() == 2 && player.discard.len() == 3);

        let second_vec = vec![0];
        player.discard_given_indexes(second_vec);
        assert!(player.hand.len() == 1 && player.discard.len() == 4);

        let third_vec = vec![0];
        player.discard_given_indexes(third_vec);
        assert!(player.hand.is_empty() && player.discard.len() == 5);

        let fourth_vec = vec![0];
        player.discard_given_indexes(fourth_vec);
        assert!(player.hand.is_empty() && player.discard.len() == 5);
        
    }

    #[test]
    fn test_player_trash() {
        let mut player = Player::new_with_default_deck(0);
        let mut game = Game::default();
        player.trash_given_indexes(vec![0,1,2,3], &mut game.trash);
        assert!(player.hand.len() == 1 && game.trash.len() == 4 && player.discard.is_empty());

        player.trash_given_indexes(vec![0], &mut game.trash);        
        assert!(player.hand.is_empty() && game.trash.len() == 5 && player.discard.is_empty());

        player.trash_given_indexes(vec![0], &mut game.trash);   
        assert!(player.hand.is_empty() && game.trash.len() == 5 && player.discard.is_empty());
        
    }

    #[test]
    fn test_player_play_action() {
        let mut game = Game::default();
        //let a: [Player] = game.players[1..];
        

        //this doesn't work. i don't know how to pass the playerslice into the method here
        //i also don't know why we're passing in a playerslice isntead of just the entire game but
        //game.players[0].gain_to_hand(Box::new(Market), &mut game.supply, &mut game.players[1..3]);
        
    }

    #[test]
    fn test_player_buy() {

    }

    #[test]
    fn test_player_gain() {

    }

}
