use bevy::prelude::*;
use bevy_mod_picking::{events::*, prelude::*};
use bevy_tweening::{lens::*, *};

use rand::prelude::*;
use std::fmt::Debug;
use std::time::Duration;

use crate::{Card, CardMetadata, Deck, DeckArea, Hand, HandArea, LaMesaPluginSettings, DECK_WIDTH};

// Events

#[derive(Event)]
pub struct RenderDeck;

#[derive(Event)]
pub struct DeckShuffle {
    pub deck_entity: Entity,
}

#[derive(Event)]
pub struct MoveCardToHand {
    pub card_entity: Entity,
    pub player: usize,
}

#[derive(Event)]
pub struct DrawHand {
    pub deck_entity: Entity,
    pub num_cards: usize,
    pub player: usize,
}

#[derive(Event)]
pub struct CardHover {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Over>>> for CardHover {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        CardHover {
            entity: event.target,
        }
    }
}

#[derive(Event)]
pub struct CardOut {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Out>>> for CardOut {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        CardOut {
            entity: event.target,
        }
    }
}

#[derive(Event)]
pub struct CardPress {
    pub card_entity: Entity,
}

impl From<ListenerInput<Pointer<Down>>> for CardPress {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        CardPress {
            card_entity: event.target,
        }
    }
}

// Event Handlers
pub fn handle_card_hover<T>(
    mut commands: Commands,
    mut hover: EventReader<CardHover>,
    mut query: Query<(Entity, &mut Card<T>, &mut Transform)>,
) where
    T: Send + Sync + Debug + 'static,
{
    hover.read().for_each(|hover| {
        if let Ok((_, mut card, transform)) = query.get_mut(hover.entity) {
            if card.pickable && card.transform.is_some() {
                card.transform = Some(transform.clone());
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(300),
                    TransformPositionLens {
                        start: transform.translation,
                        end: card.transform.unwrap().translation + Vec3::new(0., 0.7, 0.7),
                    },
                );

                commands.entity(hover.entity).insert(Animator::new(tween));
            }
        }
    });
}

pub fn handle_card_out<T>(
    mut commands: Commands,
    mut out: EventReader<CardOut>,
    mut query: Query<(Entity, &Card<T>, &mut Transform)>,
) where
    T: Send + Sync + Debug + 'static,
{
    out.read().for_each(|hover| {
        if let Ok((_, card, transform)) = query.get_mut(hover.entity) {
            if card.pickable && card.transform.is_some() {
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(300),
                    TransformPositionLens {
                        start: transform.translation,
                        end: card.transform.unwrap().translation,
                    },
                );

                commands.entity(hover.entity).insert(Animator::new(tween));
            }
        }
    });
}

pub fn handle_deck_shuffle<T>(
    mut commands: Commands,
    mut shuffle: EventReader<DeckShuffle>,
    query_cards: Query<(Entity, &Card<T>, &mut Transform, &Deck)>,
) where
    T: Send + Clone + Sync + Debug + 'static,
{
    shuffle.read().for_each(|_shuffle| {
        // list all cards whose parent is deck
        let cards: Vec<(Entity, &Card<T>, &Transform)> = query_cards
            .iter()
            .map(|(entity, card, transform, _)| (entity, card, transform))
            .collect();

        // shuffle the cards
        let mut rng = rand::thread_rng();
        let mut shuffled = cards.clone();
        shuffled.shuffle(&mut rng);

        // once cards shuffled reorder them with animation
        let duration = 75;
        let random_offset_right = Vec3::new(2.6, -0.0, 0.0);
        let random_offset_left = Vec3::new(-2.6, -0.0, 0.0);

        for (i, (entity, _, transform)) in shuffled.iter().enumerate() {
            // choose random 3 to the left or 3 to the right
            let random_offset = if i % 2 == 0 {
                random_offset_right
            } else {
                random_offset_left
            };

            let initial_translation = transform.translation;
            let new_offset = Vec3::new(
                initial_translation.x,
                i as f32 * 0.01,
                initial_translation.z,
            );

            let mut initial_translation_no_y = initial_translation.clone();
            initial_translation_no_y.y = 0.0;

            let idle_tween = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration * i as u64),
                TransformPositionLens {
                    start: initial_translation,
                    end: initial_translation,
                },
            );

            let tween1: Tween<Transform> = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: initial_translation,
                    end: initial_translation + random_offset,
                },
            );

            let tween2 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: initial_translation + random_offset,
                    end: initial_translation_no_y
                        + random_offset
                        + Vec3::new(0.0, i as f32 * 0.01, 0.0),
                },
            );

            let tween3 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: initial_translation_no_y
                        + random_offset
                        + Vec3::new(0.0, i as f32 * 0.01, 0.0),
                    end: new_offset,
                },
            );

            let seq = idle_tween.then(tween1).then(tween2).then(tween3);

            commands.entity(*entity).insert(Animator::new(seq));
        }
    });
}

pub fn handle_draw_hand<T>(
    mut commands: Commands,
    mut shuffle: EventReader<DrawHand>,
    mut set: ParamSet<(
        Query<(Entity, &mut Transform, &HandArea)>,
        Query<(Entity, &mut Transform, &DeckArea)>,
        Query<(Entity, &Card<T>, &mut Transform, &Deck)>,
    )>,
) where
    T: Send + Clone + Sync + Debug + 'static,
{
    shuffle.read().for_each(|shuffle| {
        // find global position of hand with player number
        let binding = set.p0();
        let hand_transform = binding
            .iter()
            .find(|(_, _, hand)| hand.player == shuffle.player)
            .map(|(_, transform, _)| transform)
            .unwrap();
        let hand_translation = hand_transform.translation;
        let hand_rotation = hand_transform.rotation;

        // find position of deck
        let binding = set.p1();
        let deck_transform = binding
            .iter()
            .find(|(_, _, _deck)| true)
            .map(|(_, transform, _)| transform)
            .unwrap();
        let deck_translation = deck_transform.translation;
        // deck_translation.z = 0.0;
        let _deck_rotation = deck_transform.rotation;

        // list all cards whose parent is deck
        let binding = set.p2();
        let cards: Vec<(Entity, &Card<T>, &Transform)> = binding
            .iter()
            .map(|(entity, card, transform, _)| (entity, card, transform))
            .collect();

        // sort cards by z-position
        let mut sorted = cards.clone();
        sorted.sort_by(|a, b| a.2.translation.y.partial_cmp(&b.2.translation.y).unwrap());

        let duration = 75;
        let offset = Vec3::new(-4.0, -0.0, 0.0);

        let hand_deck_offset = deck_translation - hand_translation;

        // draw the first `num_cards` cards
        for (i, (entity, card, transform)) in sorted.iter_mut().take(shuffle.num_cards).enumerate()
        {
            let initial_translation = transform.translation;

            println!("initial_translation: {:?}", initial_translation);
            let initial_rotation = transform.rotation;
            let new_offset = Vec3::new(0.0, i as f32 * 0.01, 0.0);

            let idle_tween = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis((duration * 4) * (i) as u64),
                TransformPositionLens {
                    start: initial_translation,
                    end: initial_translation,
                },
            );

            let slide = initial_translation + offset;
            let tween1: Tween<Transform> = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: initial_translation,
                    end: slide,
                },
            );

            let mut slide_flat = slide.clone();
            slide_flat.y = 0.0;

            let tween2 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: slide,
                    end: slide_flat + new_offset,
                },
            );

            let tween3 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis((duration * 4) * (shuffle.num_cards - i) as u64),
                TransformPositionLens {
                    start: slide_flat + new_offset,
                    end: slide_flat + new_offset,
                },
            );

            // rotate angle depends on who player is
            let end_rotation = match shuffle.player {
                1 => Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                _ => Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
            };

            let tween4 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformRotationLens {
                    start: initial_rotation,
                    end: end_rotation,
                },
            );

            let tween5 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: slide_flat + new_offset,
                    end: slide_flat + new_offset - hand_deck_offset,
                },
            );

            let tween6 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformRotationLens {
                    start: end_rotation,
                    end: hand_rotation,
                },
            );

            let tween7 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: slide_flat + new_offset - hand_deck_offset,
                    end: hand_translation + Vec3::new(i as f32 * 2.6 - DECK_WIDTH / 2.0, 0.0, 0.0),
                },
            );

            let seq = idle_tween
                .then(tween1)
                .then(tween2)
                .then(tween3)
                .then(tween4)
                .then(tween5)
                .then(tween6)
                .then(tween7);

            let card = Card::<T> {
                pickable: true,
                transform: Some(Transform::from_translation(
                    -hand_deck_offset + Vec3::new(i as f32 * 2.6 - DECK_WIDTH / 2.0, 0.0, 0.0),
                )),
                data: card.data.clone(),
            };

            commands
                .entity(*entity)
                .insert(Animator::new(seq))
                .insert(Hand {
                    player: shuffle.player,
                })
                .remove::<Deck>()
                .insert(PickableBundle::default())
                .insert(card);
        }
    });
}

pub fn handle_card_press<T>(
    mut card_press: EventReader<CardPress>,
    _query_cards: Query<(Entity, &Card<T>, &mut Transform, &Deck)>,
) where
    T: Send + Clone + Sync + Debug + 'static,
{
    for event in card_press.read() {
        println!("Card Pressed: {:?}", event.card_entity);
    }
}

pub fn handle_render_deck<T>(
    mut commands: Commands,
    deck: Query<(&Transform, &DeckArea)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    plugin_settings: Res<LaMesaPluginSettings<T>>,
    mut er_render_deck: EventReader<RenderDeck>,
) where
    T: Send + Clone + Sync + Debug + CardMetadata + 'static,
{
    for _render in er_render_deck.read() {
        // load deck
        let card_deck = plugin_settings.deck.clone();
        let deck_translation = deck.iter().next().unwrap().0.clone().translation;

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

            let transform = Transform::from_translation(
                deck_translation + Vec3::new(0.0, 0.01 * (i as f32), 0.0),
            )
            .with_rotation(
                Quat::from_rotation_x(std::f32::consts::PI)
                    * Quat::from_rotation_y(std::f32::consts::PI),
            );
            // .with_rotation(
            //     Quat::from_rotation_x(std::f32::consts::PI / 2.0)
            //         * Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
            //         * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)
            //         * Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
            // );

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
                            mesh: meshes
                                .add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
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
                            mesh: meshes
                                .add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
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
}
