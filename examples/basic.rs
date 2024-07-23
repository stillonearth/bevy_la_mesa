// use bevy::{color::palettes::basic::*, prelude::*};
// use bevy_la_mesa::events::{DeckShuffle, DrawHand};
// use bevy_la_mesa::utils::{load_poker_deck, PokerCard};
// use bevy_la_mesa::{DeckArea, LaMesaPlugin, LaMesaPluginSettings};

// // // Main
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(LaMesaPlugin::<PokerCard>::default())
//         .add_systems(Startup, (setup, setup_ui))
//         .add_systems(Update, (button_system,))
//         .insert_resource(LaMesaPluginSettings::<PokerCard> {
//             num_players: 1,
//             deck: load_poker_deck(),
//         })
//         .run();
// }

// /// set up lights and scene
// fn setup(mut commands: Commands) {
//     // light
//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     });

//     // camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
// }

// const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
// const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
// const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// #[derive(Component)]
// pub struct ButtonShuffleDeck;

// #[derive(Component)]
// pub struct ButtonDrawHand;

// pub fn button_system(
//     mut set: ParamSet<(
//         Query<
//             (
//                 &Interaction,
//                 &mut BackgroundColor,
//                 &mut BorderColor,
//                 &Children,
//                 &ButtonShuffleDeck,
//             ),
//             (Changed<Interaction>, With<Button>),
//         >,
//         Query<
//             (
//                 &Interaction,
//                 &mut BackgroundColor,
//                 &mut BorderColor,
//                 &Children,
//                 &ButtonDrawHand,
//             ),
//             (Changed<Interaction>, With<Button>),
//         >,
//     )>,
//     decks: Query<(Entity, &DeckArea)>,
//     mut text_query: Query<&mut Text>,
//     mut ew_shuffle: EventWriter<DeckShuffle>,
//     mut ew_draw: EventWriter<DrawHand>,
// ) {
//     if decks.iter().count() == 0 {
//         return;
//     }

//     let deck_entity = decks.iter().next().unwrap().0;

//     for (interaction, mut color, mut border_color, children, _) in &mut set.p0().iter_mut() {
//         let mut _text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Pressed => {
//                 // text.sections[0].value = "Press".to_string();
//                 *color = PRESSED_BUTTON.into();
//                 border_color.0 = RED.into();

//                 ew_shuffle.send(DeckShuffle { deck_entity });
//             }
//             Interaction::Hovered => {
//                 // text.sections[0].value = "Hover".to_string();
//                 *color = HOVERED_BUTTON.into();
//                 border_color.0 = Color::WHITE;
//             }
//             Interaction::None => {
//                 // text.sections[0].value = "Button".to_string();
//                 *color = NORMAL_BUTTON.into();
//                 border_color.0 = Color::BLACK;
//             }
//         }
//     }

//     for (interaction, mut color, mut border_color, children, _) in &mut set.p1().iter_mut() {
//         let mut _text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Pressed => {
//                 // text.sections[0].value = "Press".to_string();
//                 *color = PRESSED_BUTTON.into();
//                 border_color.0 = RED.into();

//                 ew_draw.send(DrawHand {
//                     deck_entity,
//                     num_cards: 5,
//                     player: 1,
//                 });
//             }
//             Interaction::Hovered => {
//                 // text.sections[0].value = "Hover".to_string();
//                 *color = HOVERED_BUTTON.into();
//                 border_color.0 = Color::WHITE;
//             }
//             Interaction::None => {
//                 // text.sections[0].value = "Button".to_string();
//                 *color = NORMAL_BUTTON.into();
//                 border_color.0 = Color::BLACK;
//             }
//         }
//     }
// }

// pub fn setup_ui(mut commands: Commands, _sasset_server: Res<AssetServer>) {
//     commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     width: Val::Percent(100.0),
//                     height: Val::Px(65.0),
//                     align_items: AlignItems::Start,
//                     justify_content: JustifyContent::Center,
//                     ..default()
//                 },
//                 ..default()
//             },
//             Name::new("UI"),
//         ))
//         .with_children(|parent| {
//             // Shuffle
//             parent
//                 .spawn((
//                     ButtonBundle {
//                         style: Style {
//                             width: Val::Px(350.0),
//                             height: Val::Px(65.0),
//                             border: UiRect::all(Val::Px(5.0)),
//                             // horizontally center child text
//                             justify_content: JustifyContent::Center,
//                             // vertically center child text
//                             align_items: AlignItems::Center,
//                             ..default()
//                         },
//                         border_color: BorderColor(Color::BLACK),
//                         border_radius: BorderRadius::MAX,
//                         background_color: NORMAL_BUTTON.into(),
//                         ..default()
//                     },
//                     ButtonShuffleDeck,
//                 ))
//                 .with_children(|parent| {
//                     parent.spawn(TextBundle::from_section(
//                         "barajar cartas",
//                         TextStyle {
//                             // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                             font_size: 40.0,
//                             color: Color::srgb(0.9, 0.9, 0.9),
//                             ..default()
//                         },
//                     ));
//                 });

//             // Draw hands
//             parent
//                 .spawn((
//                     ButtonBundle {
//                         style: Style {
//                             width: Val::Px(350.0),
//                             height: Val::Px(65.0),
//                             border: UiRect::all(Val::Px(5.0)),
//                             justify_content: JustifyContent::Center,
//                             align_items: AlignItems::Center,
//                             ..default()
//                         },
//                         border_color: BorderColor(Color::BLACK),
//                         border_radius: BorderRadius::MAX,
//                         background_color: NORMAL_BUTTON.into(),
//                         ..default()
//                     },
//                     ButtonDrawHand,
//                 ))
//                 .with_children(|parent| {
//                     parent.spawn(TextBundle::from_section(
//                         "dibujar la mano",
//                         TextStyle {
//                             // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                             font_size: 40.0,
//                             color: Color::srgb(0.9, 0.9, 0.9),
//                             ..default()
//                         },
//                     ));
//                 });
//         });
// }
