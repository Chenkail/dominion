//! Defines the Card trait
//!
//! See the trait for usage and examples

use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};

use itertools::Itertools;
use dyn_clonable::*;

use crate::game::{Game, Player};
use crate::game::callbacks::Callbacks;
use crate::types::*;

/// The Card trait that each card should implement
///
/// Several macros exist for quickly defining simple cards or effects; see
/// the [`dominion_macros`] crate documentation for those.
/// If this trait is manually implemented, `#[typetag::serde]`
/// must be placed before the line containing `impl Card for`.
///
/// The most basic implementation of a new card with custom behavior looks
/// like the following:
/// ```
/// use dominion::cards::prelude::*;
///
/// card!(MyCard);
/// #[typetag::serde]
/// impl Card for MyCard {
///     name!("My Card");
///     cost!(5);
///     types!(vec![Action]);
///
///     // Add your custom behavior here
/// }
/// ```
///
/// If you are making a crate with custom cards and would like to document the
/// cards, you have to declare the card struct by hand, e.g.:
/// ```
/// use dominion::cards::prelude::*;
///
/// /// This is my custom card!
/// #[derive(Clone, Serialize, Deserialize)]
/// pub struct MyCard;
///
/// #[typetag::serde]
/// impl Card for MyCard {
///     name!("My Card");
///     cost!(5);
///     types!(vec![Action]);
///
///     // Add your custom behavior here
/// }
/// ```
///
/// All other methods are optional, though for certain card types it doesn't
/// make sense not to have custom implementations for some of these methods.
/// For example, all action cards should generally implement `effects_on_play()`,
/// and all victory cards should implement `victory_points()`. Note that every
/// method from print_types() onward is automatically implemented and should
/// probably not be overridden.
///
/// [`dominion_macros`]: ../../dominion_macros/index.html

#[clonable]
#[allow(unused_variables)]
#[typetag::serde(tag = "card")]
pub trait Card: Clone {
    /// The name on the card (e.g. "Throne Room")
    fn name(&self) -> &str;
    /// The card's [types](CardType)
    fn types(&self) -> Vec<CardType>;
    /// The card text (this will often be blank, as is the case with all the
    /// basic treasures/victory cards)
    fn description(&self) -> &str { "" }

    /// How much the card costs to buy, in coins
    fn coin_cost(&self) -> usize;
    /// Debt in the card cost
    fn debt_cost(&self) -> usize { 0 }
    /// Potions needed to buy the card
    fn potion_cost(&self) -> usize { 0 }

    /// The number of coins the card is worth (if it is a treasure card)
    fn treasure_value(&self, player: &Player) -> usize { 0 }
    /// The number of potions the card is worth (if it is a potion treasure card)
    fn potion_value(&self, player: &Player) -> usize { 0 }

    /// The number of points the card is worth (if it is a victory/curse card)
    fn victory_points(&self, player: &Player) -> isize { 0 }

    // Setup
    /// Heirloom to add at start of game
    fn heirloom(&self) -> Option<Box<dyn Card>> { None }

    // Effects and triggers
    /// The card's effects when played from hand
    fn effects_on_play(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {}
    /// The card's effects when used as a reaction
    fn effects_on_react(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {}
    /// Effects to trigger when this card is bought (but not gained through some other means)
    fn effects_on_buy(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {}
    /// Effects to trigger when this card is gained
    fn effects_on_gain(&self, game: &mut Game, player_index: usize, callbacks: &Callbacks) {}

    /// The trigger for a reaction card
    fn reaction_trigger(&self) -> Option<ReactionTrigger> { None }

    // Type check methods - these should generally not be overridden
    /// Print out all types a card has, separated by commas
    fn print_types(&self) { println!("{}", self.types().iter().format(", ")) }
    /// Check if this card is an Action
    fn is_action(&self) -> bool { self.types().contains(&CardType::Action) }
    /// Check if this card is an Attack
    fn is_attack(&self) -> bool { self.types().contains(&CardType::Attack) }
    /// Check if this card is a Reaction
    fn is_reaction(&self) -> bool { self.types().contains(&CardType::Reaction) }
    /// Check if this card is a Treasure
    fn is_treasure(&self) -> bool { self.types().contains(&CardType::Treasure) }
    /// Check if this card is a Victory card
    fn is_victory(&self) -> bool { self.types().contains(&CardType::Victory) }
    /// Check if this card is a Curse card
    fn is_curse(&self) -> bool { self.types().contains(&CardType::Curse) }
}

impl fmt::Display for dyn Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Debug for dyn Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Hash for dyn Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().to_lowercase().hash(state);
    }
}

impl PartialEq for dyn Card {
    fn eq(&self, other: &Self) -> bool {
        self.name().to_lowercase().eq(&other.name().to_lowercase())
    }
}

impl Eq for dyn Card {}

impl Ord for dyn Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name().to_lowercase().cmp(&other.name().to_lowercase())
    }
}

impl PartialOrd for dyn Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
