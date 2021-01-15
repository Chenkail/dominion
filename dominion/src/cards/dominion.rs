//! Cards from the original Dominion set (2nd edition)

use super::prelude::*;

card!(Artisan);
impl Card for Artisan {
    name!("Artisan");
    cost!(6);
    types!(vec![ActionCard]);
    fn action_effects(&self, player: &mut Player, supply: &mut HashMap<Box<dyn Card>, u8>, _: &mut Vec<Player>) {
        
    }
}

placeholder_card!(Bandit, "Bandit", 5);

placeholder_card!(Bureaucrat, "Bureaucrat", 4);

placeholder_card!(Cellar, "Cellar", 2);

placeholder_card!(Chapel, "Chapel", 2);

placeholder_card!(CouncilRoom, "Council Room", 5);

placeholder_card!(Festival, "Festival", 5);

placeholder_card!(Gardens, "Gardens", 4);

placeholder_card!(Harbinger, "Harbinger", 3);

placeholder_card!(Laboratory, "Laboratory", 5);

placeholder_card!(Library, "Library", 5);

basic_action!(Market, "Market", 5, 1, 1, 1, 1);

placeholder_card!(Merchant, "Merchant", 3);

placeholder_card!(Militia, "Militia", 4);

placeholder_card!(Mine, "Mine", 5);

placeholder_card!(Moat, "Moat", 2);

placeholder_card!(Moneylender, "Moneylender", 4);

placeholder_card!(Poacher, "Poacher", 4);

placeholder_card!(Remodel, "Remodel", 4);

placeholder_card!(Sentry, "Sentry", 5);

placeholder_card!(Smithy, "Smithy", 4);

placeholder_card!(ThroneRoom, "Throne Room", 4);

placeholder_card!(Vassal, "Vassal", 3);

placeholder_card!(Village, "Village", 3);

placeholder_card!(Witch, "Witch", 5);

placeholder_card!(Workshop, "Workshop", 3);
