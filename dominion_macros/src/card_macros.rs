//! Macros for defining cards

/// Declare card struct and derive Clone
#[macro_export]
macro_rules! card {
    ($struct_name:ident) => {
        #[derive(Clone)]
        pub struct $struct_name;
    };
}

/// Create a placeholder for a card - SHOULD NOT BE USED FOR ACTUAL CARDS
#[macro_export]
macro_rules! placeholder_card {
    ($struct_name:ident, $name:expr, $cost:expr) => {
        card!($struct_name);
        impl Card for $struct_name {
            fn cost(&self) -> i32 {
                $cost as i32
            }
            
            fn name(&self) -> &str {
                $name
            }

            fn types(&self) -> Vec<CardType> {
                Vec::new()
            }
        }
    };
}

#[macro_export]
macro_rules! name {
    ($name:expr) => {
        fn name(&self) -> &str { $name }
    };
}

#[macro_export]
macro_rules! cost {
    ($cost:expr) => {
        fn cost(&self) -> i32 { $cost }
    };
}

#[macro_export]
macro_rules! types {
    ($types:expr) => {
        fn types(&self) -> Vec<CardType> { $types }
    };
}

#[macro_export]
macro_rules! treasure_value {
    ($value:expr) => {
        fn treasure_value(&self, _: &Player) -> i32 { $value }
    };
}

#[macro_export]
macro_rules! victory_points {
    ($points:expr) => {
        fn victory_points(&self, _: &Player) -> i32 { $points }
    };
}

#[macro_export]
macro_rules! curse_points {
    ($points:expr) => {
        fn curse_points(&self, _: &Player) -> i32 { $points }
    };
}

#[macro_export]
macro_rules! basic_action_effects {
    ($cards:expr, $actions:expr, $buys:expr, $coins:expr) => {
        fn action_effects(&self, player: &mut Player, _: &mut Game) {
            player.draw_cards($cards);
            player.add_actions($actions);
            player.add_buys($buys);
            player.add_coins($coins);
        }
    };
}

#[macro_export]
macro_rules! basic_action {
    ($struct_name:ident, $name:expr, $cost:expr, $cards:expr, $actions:expr, $buys:expr, $coins:expr) => {
        card!($struct_name);
        impl Card for $struct_name {
            name!($name);
            cost!($cost);
            types!(vec![ActionCard]);
            basic_action_effects!($cards, $actions, $buys, $coins);
        }
    };
}

#[macro_export]
macro_rules! basic_treasure {
    ($struct_name:ident, $name:expr, $cost:expr, $value:expr) => {
        card!($struct_name);
        impl Card for $struct_name {
            name!($name);
            cost!($cost);
            types!(vec![TreasureCard]);
            treasure_value!($value);
        }
    };
}

#[macro_export]
macro_rules! basic_victory {
    ($struct_name:ident, $name:expr, $cost:expr, $points:expr) => {
        card!($struct_name);
        impl Card for $struct_name {
            name!($name);
            cost!($cost);
            types!(vec![VictoryCard]);
            victory_points!($points);
        }
    };
}

#[macro_export]
macro_rules! basic_curse {
    ($struct_name:ident, $name:expr, $cost:expr, $points:expr) => {
        card!($struct_name);
        impl Card for $struct_name {
            name!($name);
            cost!($cost);
            types!(vec![CurseCard]);
            curse_points!($points);
        }
    };
}
