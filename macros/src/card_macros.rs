//! Macros for defining cards

/// Declares a card struct
///
/// Format:
/// ```
/// # use dominion::cards::prelude::*;
/// card!(Market);
/// ```
#[macro_export]
macro_rules! card {
    ($struct_name:ident) => {
        #[derive(Clone, Serialize, Deserialize)]
        pub struct $struct_name;
    };
}

/// Creates a placeholder Card
///
/// Format: `placeholder_card!(StructName, "Card Name", cost);`
///
/// For example:
/// ```
/// # use dominion::cards::prelude::*;
/// placeholder_card!(UnimplementedCard, "Unimplemented Card", 0);
/// ```
#[macro_export]
macro_rules! placeholder_card {
    ($struct_name:ident, $name:expr, $cost:expr) => {
        /// PLACEHOLDER CARD
        card!($struct_name);
        placeholder_effects!($struct_name, $name, $cost);
    };
}

/// Creates a placeholder for a card's effects
///
/// Format: `placeholder_effects!(StructName, "Card Name", cost);`
///
/// For example:
/// ```
/// # use dominion::cards::prelude::*;
/// card!(UnimplementedCard);
/// placeholder_effects!(UnimplementedCard, "Unimplemented Card", 0);
/// ```
#[macro_export]
macro_rules! placeholder_effects {
    ($struct_name:ident, $name:expr, $cost:expr) => {
        #[typetag::serde]
        /// PLACEHOLDER - NO EFFECTS IMPLEMENTED
        impl Card for $struct_name {
            fn name(&self) -> &str { $name }
            fn types(&self) -> Vec<CardType> { Vec::new() }
            fn coin_cost(&self) -> usize { $cost }
            fn description(&self) -> &str { "PLACEHOLDER CARD" }
        }
    };
}

/// Sets the card's name to be displayed
///
/// For example:
/// ```
/// # use dominion::cards::prelude::*;
/// card!(MiningVillage);
/// #[typetag::serde]
/// impl Card for MiningVillage {
///     name!("Mining Village");
///     cost!(4);
///     types!(vec![Action]);
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! name {
    ($name:expr) => {
        fn name(&self) -> &str { $name }
    };
}

/// Set the card's cost
///
/// Format: `cost!(coins, debt, potions);`
/// (If ``potions`` is present, ``debt`` is required, otherwise both are optional)
///
/// For example:
/// ```
/// # use dominion::cards::prelude::*;
/// // http://wiki.dominionstrategy.com/index.php/Golem
/// card!(Golem);
/// #[typetag::serde]
/// impl Card for Golem {
///     name!("Golem");
///     cost!(4, 0, 1);
///     types!(vec![Action]);
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! cost {
    ($coins:expr) => {
        fn coin_cost(&self) -> usize { $coins }
    };
    ($coins:expr, $debt:expr) => {
        fn coin_cost(&self) -> usize { $coins }
        fn debt_cost(&self) -> usize { $debt }
    };
    ($coins:expr, $debt:expr, $potions:expr) => {
        fn coin_cost(&self) -> usize { $coins }
        fn debt_cost(&self) -> usize { $debt }
        fn potion_cost(&self) -> usize { $potions }
    };
}

/// Sets a card's types
///
/// For example:
/// ```
/// # use dominion::cards::prelude::*;
/// #
/// # card!(MyCard);
/// # #[typetag::serde]
/// impl Card for MyCard {
/// # name!("My Card");
/// # cost!(0);
///     // ...
///     types!(vec![Action, Victory]);
/// }
/// ```
#[macro_export]
macro_rules! types {
    ($types:expr) => {
        fn types(&self) -> Vec<CardType> { $types }
    };
}

/// Sets a treasure card's coin value to some fixed amount
///
/// For example, Gold could be declared as follows:
/// ```
/// use dominion::cards::prelude::*;
///
/// card!(Gold);
/// #[typetag::serde]
/// impl Card for Gold {
///     name!("Gold");
///     cost!(6);
///     types!(vec![Treasure]);
///     treasure_value!(3);
/// }
/// ```
#[macro_export]
macro_rules! treasure_value {
    ($value:expr) => {
        fn treasure_value(&self, _: &Player) -> usize { $value }
    };
}

/// Sets a victory/curse card's point value to some fixed amount
///
/// For example, Province could be declared as follows:
/// ```
/// use dominion::cards::prelude::*;
///
/// card!(Province);
/// #[typetag::serde]
/// impl Card for Province {
/// # name!("Province");
/// # cost!(8);
/// # types!(vec![Victory]);
///     // ...
///     victory_points!(6);
/// }
/// ```
/// and the basic Curse card would be:
/// ```
/// use dominion::cards::prelude::*;
///
/// card!(BasicCurse);
/// #[typetag::serde]
/// impl Card for BasicCurse {
/// # name!("Curse");
/// # cost!(0);
/// # types!(vec![Curse]);
///     // ...
///     victory_points!(-1);
/// }
/// ```
#[macro_export]
macro_rules! victory_points {
    ($points:expr) => {
        fn victory_points(&self, _: &Player) -> isize { $points }
    };
}

/// Effects for an action with no effects other than drawing cards
/// and/or adding actions/buys/coins for the turn
///
/// Format: `basic_on_play_effects!(cards, actions, buys, coins);`
///
/// Example:
/// ```
/// use dominion::cards::prelude::*;
///
/// card!(Market);
/// #[typetag::serde]
/// impl Card for Market {
///     name!("Market");
///     cost!(5);
///     types!(vec![Action]);
///     basic_on_play_effects!(1, 1, 1, 1);
/// }
/// ```
#[macro_export]
macro_rules! basic_on_play_effects {
    ($cards:expr, $actions:expr, $buys:expr, $coins:expr) => {
        fn effects_on_play(&self, player: &mut Player, _: &mut Supply, _: &mut CardDeck, _: &mut PlayerSlice, _: &Callbacks) {
            player.draw_cards($cards);
            player.add_actions($actions);
            player.add_buys($buys);
            player.add_coins($coins);
        }
    };
}

/// Implementation in one line for an action which has no effects other than
/// drawing cards and/or adding actions/buys/coins for the turn
///
/// Format: `basic_action!(StructName, "Card Name", cost, cards, actions, buys, coins);`
///
/// For example, Market could be declared as follows:
/// ```
/// # use dominion::cards::prelude::*;
/// basic_action!(Market, "Market", 5, 1, 1, 1, 1);
/// ```
#[macro_export]
macro_rules! basic_action {
    ($struct_name:ident, $name:expr, $cost:expr, $cards:expr, $actions:expr, $buys:expr, $coins:expr) => {
        card!($struct_name);
        #[typetag::serde]
        impl Card for $struct_name {
            name!($name);
            cost!($cost);
            types!(vec![Action]);
            basic_on_play_effects!($cards, $actions, $buys, $coins);
        }
    };
}

/// Implementation in one line for a treasure card with a fixed coin value
///
/// Format: `basic_treasure!(StructName, "Card Name", cost, value);`
///
/// For example, Silver could be declared as follows:
/// ```
/// # use dominion::cards::prelude::*;
/// basic_treasure!(Silver, "Silver", 3, 2);
/// ```
#[macro_export]
macro_rules! basic_treasure {
    ($struct_name:ident, $name:expr, $cost:expr, $value:expr) => {
        card!($struct_name);
        #[typetag::serde]
        impl Card for $struct_name {
            name!($name);
            cost!($cost);
            types!(vec![Treasure]);
            treasure_value!($value);
        }
    };
}

/// Implementation in one line for a victory card with a fixed points value
///
/// Format: `basic_victory!(StructName, "Card Name", cost, points);`
///
/// For example, Province could be declared as follows:
/// ```
/// # use dominion::cards::prelude::*;
/// basic_victory!(Province, "Province", 8, 6);
/// ```
#[macro_export]
macro_rules! basic_victory {
    ($struct_name:ident, $name:expr, $cost:expr, $points:expr) => {
        card!($struct_name);
        #[typetag::serde]
        impl Card for $struct_name {
            name!($name);
            cost!($cost);
            types!(vec![Victory]);
            victory_points!($points);
        }
    };
}

/// Implementation in one line for a curse card with a fixed points value
///
/// Note that points should (presumably) be negative
///
/// Format: `basic_curse!(StructName, card_name, cost, points);`
///
/// For example, the basic Curse card could be declared as follows (the struct
/// name is BasicCurse because the name Curse is already used for the card type):
/// ```
/// # use dominion::cards::prelude::*;
/// basic_curse!(BasicCurse, "Curse", 0, -1);
/// ```
#[macro_export]
macro_rules! basic_curse {
    ($struct_name:ident, $name:expr, $cost:expr, $points:expr) => {
        card!($struct_name);
        #[typetag::serde]
        impl Card for $struct_name {
            name!($name);
            cost!($cost);
            types!(vec![Curse]);
            victory_points!($points);
        }
    };
}
