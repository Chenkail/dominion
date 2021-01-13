//! Macros for defining cards

/// Create a placeholder for a card - SHOULD NOT BE USED FOR ACTUAL CARDS
#[macro_export]
macro_rules! card {
    ($struct_name:ident, $name:expr, $cost:expr) => {
        pub struct $struct_name;
        impl Card for $struct_name {
            fn cost(&self) -> i32 {
                return $cost as i32;
            }
            
            fn name(&self) -> &str {
                return $name;
            }

            fn types(&self) -> Vec<&str> {
                return Vec::new();
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
        fn types(&self) -> Vec<&str> { $types }
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
    ($struct:ident, $name:expr, $cost:expr, $cards:expr, $actions:expr, $buys:expr, $coins:expr) => {
        pub struct $struct;
        impl Card for $struct {
            name!($name);
            cost!($cost);
            types!(vec!["Action"]);
            basic_action_effects!($cards, $actions, $buys, $coins);
        }
    };
}

#[macro_export]
macro_rules! basic_treasure {
    ($struct:ident, $name:expr, $cost:expr, $value:expr) => {
        pub struct $struct;
        impl Card for $struct {
            name!($name);
            cost!($cost);
            types!(vec!["Treasure"]);
            treasure_value!($value);
        }
    };
}

#[macro_export]
macro_rules! basic_victory {
    ($struct:ident, $name:expr, $cost:expr, $points:expr) => {
        pub struct $struct;
        impl Card for $struct {
            name!($name);
            cost!($cost);
            types!(vec!["Victory"]);
            victory_points!($points);
        }
    };
}

#[macro_export]
macro_rules! basic_curse {
    ($struct:ident, $name:expr, $cost:expr, $points:expr) => {
        pub struct $struct;
        impl Card for $struct {
            name!($name);
            cost!($cost);
            types!(vec!["Curse"]);
            curse_points!($points);
        }
    };
}
