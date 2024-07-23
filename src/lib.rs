pub mod events;
pub mod utils;

use bevy::prelude::*;
use bevy_mod_picking::{
    events::*, picking_core::Pickable, prelude::On, DefaultPickingPlugins, PickableBundle,
};
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

#[derive(Component)]
pub struct DeckArea;

#[derive(Component)]
pub struct Deck;

#[derive(Component)]
pub struct HandArea {
    pub player: usize,
}

#[derive(Component)]
pub struct Hand {
    pub player: usize,
}

#[derive(Default, Resource)]
pub struct LaMesaPluginSettings<T: Send + Clone + Sync + Debug + CardMetadata + 'static> {
    pub num_players: usize,
    pub back_card_path: String,
    pub deck: Vec<T>,
}

#[derive(Default)]
pub struct LaMesaPlugin<T: Send + Clone + Sync + Debug + CardMetadata + 'static>(
    pub PhantomData<T>,
);

impl<T: Send + Clone + Sync + Debug + CardMetadata + 'static> Plugin for LaMesaPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (handle_render_deck::<T>, render_hands_area::<T>))
            .add_systems(
                Update,
                (
                    handle_card_hover::<T>,
                    handle_card_out::<T>,
                    handle_deck_shuffle::<T>,
                    handle_draw_hand::<T>,
                    handle_card_press::<T>,
                    handle_render_deck::<T>,
                ),
            )
            .add_plugins((DefaultPickingPlugins, TweeningPlugin))
            .add_event::<CardHover>()
            .add_event::<CardOut>()
            .add_event::<CardPress>()
            .add_event::<DeckShuffle>()
            .add_event::<DrawHand>()
            .add_event::<MoveCardToHand>()
            .add_event::<RenderDeck>();
    }
}

pub const DECK_WIDTH: f32 = 5.0 * 2.6;

pub fn render_hands_area<T>(mut commands: Commands)
where
    T: Send + Clone + Sync + Debug + 'static,
{
    commands.spawn((
        Name::new("HandArea"),
        TransformBundle {
            local: Transform::from_translation(Vec3::new(0.0, 0.0, 3.7))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 4.0)),
            ..default()
        },
        HandArea { player: 1 },
    ));
}
