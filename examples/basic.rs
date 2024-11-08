use bevy::{color::palettes::basic::*, prelude::*};
use bevy_la_mesa::events::{DeckShuffle, DrawHand, RenderDeck};
use bevy_la_mesa::utils::{load_poker_deck, Chip, PokerCard};
use bevy_la_mesa::{DeckArea, LaMesaPlugin, LaMesaPluginSettings};

// // Main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaMesaPlugin::<PokerCard, Chip>::default())
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (button_system, start_game))
        .insert_resource(LaMesaPluginSettings {
            num_players: 1,
            hand_size: 7,
            back_card_path: "background.png".into(),
        })
        .insert_resource(GameState {
            game_started: false,
        })
        .run();
}

#[derive(Resource)]
struct GameState {
    game_started: bool,
}

/// set up lights and scene
fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
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

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: materials.add(Color::BLACK),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        DeckArea { marker: 1 },
        Name::new("Deck 1 -- Play Cards"),
    ));
}

fn start_game(
    mut game_state: ResMut<GameState>,
    mut ew_render_deck: EventWriter<RenderDeck<PokerCard>>,
) {
    if game_state.game_started {
        return;
    }

    ew_render_deck.send(RenderDeck::<PokerCard> {
        marker: 1,
        deck: load_poker_deck(),
    });

    game_state.game_started = true;
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct ButtonShuffleDeck;

#[derive(Component)]
pub struct ButtonDrawHand;

pub fn button_system(
    mut set: ParamSet<(
        Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
                &ButtonShuffleDeck,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
                &ButtonDrawHand,
            ),
            (Changed<Interaction>, With<Button>),
        >,
    )>,
    decks: Query<(Entity, &DeckArea)>,
    mut text_query: Query<&mut Text>,
    mut ew_shuffle: EventWriter<DeckShuffle>,
    mut ew_draw: EventWriter<DrawHand>,
) {
    if decks.iter().count() == 0 {
        return;
    }

    // let deck_entity = decks.iter().next().unwrap().0;

    for (interaction, mut color, mut border_color, children, _) in &mut set.p0().iter_mut() {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                ew_shuffle.send(DeckShuffle { deck_marker: 1 });
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }

    for (interaction, mut color, mut border_color, children, _) in &mut set.p1().iter_mut() {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                ew_draw.send(DrawHand {
                    deck_marker: 1,
                    num_cards: 5,
                    player: 1,
                });
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn setup_ui(mut commands: Commands, _sasset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(65.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("UI"),
        ))
        .with_children(|parent| {
            // Shuffle
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(350.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonShuffleDeck,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "shuffle deck",
                        TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });

            // Draw hands
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(350.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonDrawHand,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "draw hand",
                        TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}
