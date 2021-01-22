#[cfg(test)]
mod test_player {
    use dominion::game::player::*;
    use dominion::game::*;

    #[test]
    fn test_player_init() {
        let player = Player::default();

        //we check to see if everything is initalized correctly
        assert!(player.hand.len() == 5);
    }
}
