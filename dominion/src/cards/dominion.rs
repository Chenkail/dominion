//! Cards from the original Dominion set (2nd edition)

use super::prelude::*;

card!(Artisan, "Artisan", 6);

card!(Bandit, "Bandit", 5);

card!(Bureaucrat, "Bureaucrat", 4);

card!(Cellar, "Cellar", 2);

card!(Chapel, "Chapel", 2);

card!(CouncilRoom, "Council Room", 5);

card!(Festival, "Festival", 5);
action!(Festival, 0, 2, 1, 2);

card!(Gardens, "Gardens", 4);

card!(Harbinger, "Harbinger", 3);

card!(Laboratory, "Laboratory", 5);
action!(Laboratory, 2, 1, 0, 0);

card!(Library, "Library", 5);

card!(Market, "Market", 5);
action!(Market, 1, 1, 1, 1);

card!(Merchant, "Merchant", 3);

card!(Militia, "Militia", 4);

card!(Mine, "Mine", 5);

card!(Moat, "Moat", 2);

card!(Moneylender, "Moneylender", 4);

card!(Poacher, "Poacher", 4);

card!(Remodel, "Remodel", 4);

card!(Sentry, "Sentry", 5);

card!(Smithy, "Smithy", 4);
impl Action for Smithy {
    fn effects(&self, player: &mut Player, _: &mut Game) {
        player.draw_cards(3);
    }
}

card!(ThroneRoom, "Throne Room", 4);

card!(Vassal, "Vassal", 3);

card!(Village, "Village", 3);
impl Action for Village {
    fn effects(&self, player: &mut Player, _: &mut Game) {
        player.draw_cards(1);
        player.add_actions(2);
    }
}

card!(Witch, "Witch", 5);

card!(Workshop, "Workshop", 3);
