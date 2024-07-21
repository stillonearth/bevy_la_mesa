pub mod events;
pub mod ui;
pub mod utils;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{
    debug::DebugPickingMode, events::*, picking_core::Pickable, prelude::On, DefaultPickingPlugins,
    PickableBundle,
};
use bevy_tweening::TweeningPlugin;
use events::*;
use ui::{button_system, setup_ui};
use utils::PokerCard;

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

#[derive(Resource)]
pub struct LaMesaPluginSettings {
    pub num_players: usize,
}

// Main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (render_deck, render_hands, setup, setup_ui))
        .add_systems(
            Update,
            (
                handle_card_hover::<PokerCard>,
                handle_card_out::<PokerCard>,
                handle_deck_shuffle::<PokerCard>,
                handle_draw_hand::<PokerCard>,
                button_system,
            ),
        )
        .add_plugins((
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            DefaultPickingPlugins,
            TweeningPlugin,
        ))
        .insert_resource(DebugPickingMode::Normal)
        .insert_resource(LaMesaPluginSettings { num_players: 1 })
        .add_event::<CardHover>()
        .add_event::<CardOut>()
        .add_event::<DeckShuffle>()
        .add_event::<DrawHand>()
        .add_event::<MoveCardToHand>()
        .run();
}

/// set up lights and scene
fn setup(mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub const DECK_WIDTH: f32 = 5.0 * 2.6;

pub fn render_hands(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    plugin_settings: Res<LaMesaPluginSettings>,
) {
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

pub fn render_deck(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // load deck
    let poker_deck = utils::load_poker_deck();
    let deck_transform =
        Transform::IDENTITY.with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0));

    commands.spawn((
        TransformBundle {
            local: deck_transform,
            ..default()
        },
        Name::new("Deck"),
        DeckArea,
        InheritedVisibility::default(),
    ));

    for (i, card) in poker_deck.iter().enumerate() {
        let face_texture = asset_server.load(card.clone().filename);
        let face_material = materials.add(StandardMaterial {
            base_color_texture: Some(face_texture.clone()),
            ..Default::default()
        });

        let face_texture = asset_server.load("card-back1.png");
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
