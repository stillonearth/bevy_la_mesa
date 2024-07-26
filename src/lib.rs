pub mod events;
pub mod utils;

use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_tweening::TweeningPlugin;
use events::*;
use std::{fmt::Debug, marker::PhantomData};

pub trait CardMetadata {
    type Output;

    fn filename(&self) -> String;
}

#[derive(Component)]
pub struct Card<CardType> {
    pub pickable: bool,
    pub transform: Option<Transform>,
    pub data: CardType,
}

#[derive(Component, PartialEq, PartialOrd)]
pub struct Chip<T> {
    pub data: T,
    // todo this should be moved to metadata
    pub turn_activation_1: usize,
    pub turn_activation_2: usize,
}

#[derive(Component)]
pub struct DeckArea {
    pub marker: usize,
}

#[derive(Component)]
pub struct Deck {
    pub marker: usize,
}

#[derive(Component, Default)]
pub struct PlayArea {
    pub marker: usize,
    pub player: usize,
}

#[derive(Component, Default, Debug, PartialEq)]
pub struct ChipArea {
    pub marker: usize,
    pub player: usize,
}

#[derive(Component)]
pub struct HandArea {
    pub player: usize,
}

#[derive(Component)]
pub struct Hand {
    pub player: usize,
}

#[derive(Component)]
pub struct CardOnTable {
    pub marker: usize,
    pub player: usize,
}

#[derive(Default, Resource)]
pub struct LaMesaPluginSettings<T: Send + Clone + Sync + Debug + CardMetadata + 'static> {
    pub num_players: usize,
    pub hand_size: usize,
    pub back_card_path: String,
    pub deck: Vec<T>,
}

#[derive(Default)]
pub struct LaMesaPlugin<
    T: Send + Clone + Sync + Debug + CardMetadata + 'static,
    P: Send + Clone + Sync + Debug + PartialEq + 'static,
>(pub PhantomData<(T, P)>);

impl<
        T: Send + Clone + Sync + Debug + CardMetadata + 'static,
        P: Send + Clone + Sync + Debug + PartialEq + 'static,
    > Plugin for LaMesaPlugin<T, P>
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, handle_render_deck::<T>)
            .add_systems(
                Update,
                (
                    handle_card_hover::<T>,
                    handle_card_out::<T>,
                    handle_deck_shuffle::<T>,
                    handle_draw_hand::<T>,
                    handle_place_card_on_table::<T>,
                    handle_place_card_off_table::<T>,
                    handle_render_deck::<T>,
                    handle_align_cards_in_hand::<T>,
                    handle_align_chips_on_table::<P>,
                ),
            )
            .add_plugins((DefaultPickingPlugins, TweeningPlugin))
            .add_event::<CardHover>()
            .add_event::<CardOut>()
            .add_event::<CardPress>()
            .add_event::<DeckShuffle>()
            .add_event::<DrawHand>()
            .add_event::<RenderDeck>()
            .add_event::<PlaceCardOnTable>()
            .add_event::<AlignCardsInHand>()
            .add_event::<PlaceCardOffTable>()
            .add_event::<AlignChipsOnTable<P>>();
    }
}

pub const DECK_WIDTH: f32 = 5.0 * 2.6;
