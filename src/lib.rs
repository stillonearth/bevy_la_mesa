pub mod events;

use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;
use events::*;
use std::{fmt::Debug, marker::PhantomData};

pub trait CardMetadata {
    type Output;

    fn front_image_filename(&self) -> String;
    fn back_image_filename(&self) -> String;
}

#[derive(Component)]
pub struct Card<CardType> {
    pub pickable: bool,
    pub transform: Option<Transform>,
    pub data: CardType,
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
pub struct LaMesaPluginSettings {
    pub num_players: usize,
}

#[derive(Default)]
pub struct LaMesaPlugin<T: Send + Clone + Sync + Debug + CardMetadata + 'static>(
    pub PhantomData<T>,
);

impl<T: Send + Clone + Sync + Debug + CardMetadata + 'static> Plugin for LaMesaPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, handle_render_deck::<T>)
            .add_systems(
                Update,
                (
                    handle_align_cards_in_hand::<T>,
                    handle_card_hover::<T>,
                    handle_card_out::<T>,
                    handle_deck_shuffle::<T>,
                    handle_discard_card_to_deck::<T>,
                    handle_draw_to_hand::<T>,
                    handle_draw_to_table::<T>,
                    handle_place_card_on_table::<T>,
                    handle_render_deck::<T>,
                )
                    .chain(),
            )
            .add_plugins(TweeningPlugin)
            .add_message::<AlignCardsInHand>()
            .add_message::<CardHover>()
            .add_message::<CardOut>()
            .add_message::<CardPress>()
            .add_message::<DeckRendered>()
            .add_message::<DeckShuffle>()
            .add_message::<DiscardCardToDeck>()
            .add_message::<DrawToHand>()
            .add_message::<DrawToTable>()
            .add_message::<PlaceCardOnTable>()
            .add_message::<RenderDeck<T>>();
    }
}

pub const DECK_WIDTH: f32 = 5.0 * 2.6;
