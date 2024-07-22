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
        app.add_systems(Startup, (render_deck::<T>, render_hands_area::<T>))
            .add_systems(
                Update,
                (
                    handle_card_hover::<T>,
                    handle_card_out::<T>,
                    handle_deck_shuffle::<T>,
                    handle_draw_hand::<T>,
                    handle_card_press::<T>,
                ),
            )
            .add_plugins((DefaultPickingPlugins, TweeningPlugin))
            .add_event::<CardHover>()
            .add_event::<CardOut>()
            .add_event::<CardPress>()
            .add_event::<DeckShuffle>()
            .add_event::<DrawHand>()
            .add_event::<MoveCardToHand>();
    }
}

pub const DECK_WIDTH: f32 = 5.0 * 2.6;

pub fn render_hands_area<T>(mut commands: Commands)
where
    T: Send + Clone + Sync + Debug + 'static,
{
    commands
        .spawn((
            Name::new("HandArea"),
            TransformBundle {
                local: Transform::from_translation(Vec3::new(0.0, 0.0, 3.7))
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 4.0)),
                ..default()
            },
            HandArea { player: 1 },
        ))
        .with_children(|_parent| {
            // parent.spawn()
        });
}

pub fn render_deck<T>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    plugin_settings: Res<LaMesaPluginSettings<T>>,
) where
    T: Send + Clone + Sync + Debug + CardMetadata + 'static,
{
    // load deck
    let card_deck = plugin_settings.deck.clone();
    let deck_transform = Transform::from_translation(Vec3::new(0.0, -0.7, 4.0))
        .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0));

    commands.spawn((
        TransformBundle {
            local: deck_transform,
            ..default()
        },
        Name::new("Deck"),
        DeckArea,
        InheritedVisibility::default(),
    ));

    for (i, card) in card_deck.iter().enumerate() {
        let face_texture = asset_server.load(card.clone().filename());
        let face_material = materials.add(StandardMaterial {
            base_color_texture: Some(face_texture.clone()),
            ..Default::default()
        });

        let face_texture = asset_server.load(plugin_settings.back_card_path.clone());
        let back_material = materials.add(StandardMaterial {
            base_color_texture: Some(face_texture.clone()),
            ..Default::default()
        });

        let transform = Transform::from_translation(Vec3::new(0.0, 0.01 * (i as f32), 0.0))
            .with_rotation(
                Quat::from_rotation_x(std::f32::consts::PI / 2.0)
                    * Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
                    * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)
                    * Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
            );

        // Draw Deck
        commands
            .spawn((
                Name::new("Card"),
                Card {
                    pickable: false,
                    transform: None,
                    data: card.clone(),
                },
                Deck,
                PbrBundle {
                    mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
                    transform,
                    ..default()
                },
                PickableBundle::default(),
                On::<Pointer<Over>>::send_event::<CardHover>(),
                On::<Pointer<Down>>::send_event::<CardPress>(),
                On::<Pointer<Out>>::send_event::<CardOut>(),
            ))
            .with_children(|parent| {
                // face
                parent.spawn((
                    PbrBundle {
                        mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
                        material: face_material,
                        ..default()
                    },
                    PickableBundle {
                        pickable: Pickable {
                            is_hoverable: false,
                            ..default()
                        },
                        ..default()
                    },
                ));
                // back
                parent.spawn((
                    PbrBundle {
                        mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
                        material: back_material,
                        transform: Transform::IDENTITY
                            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                        ..default()
                    },
                    PickableBundle {
                        pickable: Pickable {
                            is_hoverable: false,
                            ..default()
                        },
                        ..default()
                    },
                ));
            });
    }
}
