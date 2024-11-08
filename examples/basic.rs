use bevy::input::common_conditions::input_toggle_active;
use bevy::{color::palettes::basic::*, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_la_mesa::events::{DeckShuffle, DrawHand, RenderDeck};
use bevy_la_mesa::{CardMetadata, DeckArea, LaMesaPlugin, LaMesaPluginSettings};

// // Main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LaMesaPlugin::<PokerCard, Chip>::default())
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (button_system, start_game))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
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
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        IsDefaultUiCamera,
    ));

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

#[derive(Default, Clone, Debug)]
pub struct PokerCard {
    pub value: u8,
    pub suit: String,
    pub filename: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Chip {}

#[allow(clippy::vec_init_then_push)]
pub fn load_poker_deck() -> Vec<PokerCard> {
    let mut deck: Vec<PokerCard> = vec![];

    // Clubs
    deck.push(PokerCard {
        value: 1,
        suit: "Clubs".to_string(),
        filename: "card-clubs-1.png".to_string(),
    });
    deck.push(PokerCard {
        value: 2,
        suit: "Clubs".to_string(),
        filename: "card-clubs-2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Clubs".to_string(),
        filename: "card-clubs-3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Clubs".to_string(),
        filename: "card-clubs-4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Clubs".to_string(),
        filename: "card-clubs-5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Clubs".to_string(),
        filename: "card-clubs-6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Clubs".to_string(),
        filename: "card-clubs-7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Clubs".to_string(),
        filename: "card-clubs-8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Clubs".to_string(),
        filename: "card-clubs-9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Clubs".to_string(),
        filename: "card-clubs-10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Clubs".to_string(),
        filename: "card-clubs-11.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Clubs".to_string(),
        filename: "card-clubs-12.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Clubs".to_string(),
        filename: "card-clubs-13.png".to_string(),
    });
    // Diamonds
    deck.push(PokerCard {
        value: 1,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-1.png".to_string(),
    });
    deck.push(PokerCard {
        value: 2,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-11.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-12.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Diamonds".to_string(),
        filename: "card-diamonds-13.png".to_string(),
    });
    // Hearts
    deck.push(PokerCard {
        value: 1,
        suit: "Hearts".to_string(),
        filename: "card-hearts-1.png".to_string(),
    });
    deck.push(PokerCard {
        value: 2,
        suit: "Hearts".to_string(),
        filename: "card-hearts-2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Hearts".to_string(),
        filename: "card-hearts-3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Hearts".to_string(),
        filename: "card-hearts-4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Hearts".to_string(),
        filename: "card-hearts-5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Hearts".to_string(),
        filename: "card-hearts-6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Hearts".to_string(),
        filename: "card-hearts-7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Hearts".to_string(),
        filename: "card-hearts-8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Hearts".to_string(),
        filename: "card-hearts-9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Hearts".to_string(),
        filename: "card-hearts-10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Hearts".to_string(),
        filename: "card-hearts-11.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Hearts".to_string(),
        filename: "card-hearts-12.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Hearts".to_string(),
        filename: "card-hearts-13.png".to_string(),
    });
    // Spades
    deck.push(PokerCard {
        value: 1,
        suit: "Spades".to_string(),
        filename: "card-spades-1.png".to_string(),
    });
    deck.push(PokerCard {
        value: 2,
        suit: "Spades".to_string(),
        filename: "card-spades-2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Spades".to_string(),
        filename: "card-spades-3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Spades".to_string(),
        filename: "card-spades-4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Spades".to_string(),
        filename: "card-spades-5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Spades".to_string(),
        filename: "card-spades-6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Spades".to_string(),
        filename: "card-spades-7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Spades".to_string(),
        filename: "card-spades-8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Spades".to_string(),
        filename: "card-spades-9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Spades".to_string(),
        filename: "card-spades-10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Spades".to_string(),
        filename: "card-spades-11.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Spades".to_string(),
        filename: "card-spades-12.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Spades".to_string(),
        filename: "card-spades-13.png".to_string(),
    });

    deck
}

impl CardMetadata for PokerCard {
    type Output = PokerCard;

    fn filename(&self) -> String {
        self.filename.clone()
    }
}
