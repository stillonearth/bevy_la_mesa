use bevy::prelude::*;
use bevy_defer::*;
use bevy_tweening::{lens::*, *};

use rand::prelude::*;
use std::fmt::Debug;
use std::time::Duration;

use crate::{
    Card, CardMetadata, CardOnTable, Deck, DeckArea, Hand, HandArea, PlayArea, DECK_WIDTH,
};

// Events

#[derive(Message)]
pub struct RenderDeck<T: Send + Clone + Sync + Debug + CardMetadata + 'static> {
    pub deck_entity: Entity,
    pub deck: Vec<T>,
}

#[derive(Message)]
pub struct DeckRendered {}

#[derive(Message)]
pub struct DeckShuffle {
    pub deck_entity: Entity,
    pub duration: u64,
}

#[derive(Message)]
pub struct AlignCardsInHand {
    pub player: usize,
}

#[derive(Message)]
pub struct PlaceCardOnTable {
    pub card_entity: Entity,
    pub marker: usize,
    pub player: usize,
}

#[derive(Message)]
pub struct DiscardCardToDeck {
    pub card_entity: Entity,
    pub deck_entity: Entity,
}

#[derive(Message)]
pub struct DrawToHand {
    pub deck_entity: Entity,
    pub num_cards: usize,
    pub player: usize,
}

#[derive(Message)]
pub struct DrawToTable {
    pub deck_entity: Entity,
    pub play_area_markers: Vec<usize>,
    pub player: usize,
    pub duration: u64,
}

#[derive(Message)]
pub struct CardHover {
    pub entity: Entity,
}

impl From<Pointer<Over>> for CardHover {
    fn from(event: Pointer<Over>) -> Self {
        CardHover {
            entity: event.entity,
        }
    }
}

#[derive(Message)]
pub struct CardOut {
    pub entity: Entity,
}

impl From<Pointer<Out>> for CardOut {
    fn from(event: Pointer<Out>) -> Self {
        CardOut {
            entity: event.entity,
        }
    }
}

#[derive(Message)]
pub struct CardPress {
    pub entity: Entity,
}

impl From<Pointer<Press>> for CardPress {
    fn from(event: Pointer<Press>) -> Self {
        CardPress {
            entity: event.entity,
        }
    }
}

// Event Handlers
pub fn handle_card_hover<T>(
    mut commands: Commands,
    mut hover: MessageReader<CardHover>,
    mut cards_in_hand: Query<(Entity, &mut Card<T>, &Hand, &mut Transform)>,
) where
    T: Send + Sync + Debug + 'static,
{
    hover.read().for_each(|hover| {
        if let Ok((_, card, hand, _transform)) = cards_in_hand.get_mut(hover.entity) {
            if card.pickable && card.transform.is_some() {
                let start_translation = card.transform.unwrap().translation;
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(100),
                    TransformPositionLens {
                        start: start_translation,
                        end: start_translation
                            + match hand.player {
                                1 => Vec3::new(0., 0.7 / 3.0, 0.7 / 3.0),
                                _ => Vec3::new(0., 0.7 / 3.0, 0.0),
                            },
                    },
                );

                commands.entity(hover.entity).insert(TweenAnim::new(tween));
            }
        }
    });
}

pub fn handle_card_out<T>(
    mut commands: Commands,
    mut out: MessageReader<CardOut>,
    mut query: Query<(Entity, &Card<T>, &Hand, &mut Transform)>,
) where
    T: Send + Sync + Debug + 'static,
{
    out.read().for_each(|hover| {
        if let Ok((_, card, _, transform)) = query.get_mut(hover.entity) {
            if card.pickable && card.transform.is_some() {
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(100),
                    TransformPositionLens {
                        start: transform.translation,
                        end: card.transform.unwrap().translation,
                    },
                );

                commands.entity(hover.entity).insert(TweenAnim::new(tween));
            }
        }
    });
}

pub fn handle_deck_shuffle<T>(
    mut commands: Commands,
    mut shuffle: MessageReader<DeckShuffle>,
    query_cards: Query<(Entity, &Card<T>, &mut Transform, &Deck)>,
    query_deck: Query<(Entity, &Transform, &DeckArea), Without<Deck>>,
) where
    T: Send + Clone + Sync + Debug + 'static,
{
    shuffle.read().for_each(|shuffle| {
        let shuffle_deck = query_deck.get(shuffle.deck_entity).unwrap().2;

        // list all cards whose parent is deck
        let cards: Vec<(Entity, &Card<T>, &Transform)> = query_cards
            .iter()
            .filter(|(_, _, _, deck)| deck.marker == shuffle_deck.marker)
            .map(|(entity, card, transform, _)| (entity, card, transform))
            .collect();

        // shuffle the cards
        let mut rng = rand::thread_rng();
        let mut shuffled = cards.clone();
        shuffled.shuffle(&mut rng);

        // once cards shuffled reorder them with animation
        let duration = shuffle.duration;
        let random_offset_right = Vec3::new(0.0, 0.0, -2.6);
        let random_offset_left = Vec3::new(0.0, 0.0, 2.6);

        // find deck with deck number
        let mut deck_translation = query_deck
            .iter()
            .find(|(_, _, deck)| deck.marker == shuffle_deck.marker)
            .unwrap()
            .1
            .translation;
        deck_translation.y = 0.0;

        for (i, (entity, _, transform)) in shuffled.iter().enumerate() {
            // choose random 3 to the left or 3 to the right
            let random_offset = if i % 2 == 0 {
                random_offset_right
            } else {
                random_offset_left
            };

            let initial_translation = transform.translation;
            let new_offset = Vec3::new(deck_translation.x, i as f32 * 0.01, deck_translation.z);

            let idle_tween = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration * i as u64),
                TransformPositionLens {
                    start: initial_translation,
                    end: initial_translation,
                },
            );

            let tween1 = Tween::new(
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
                    end: deck_translation + random_offset + Vec3::new(0.0, i as f32 * 0.01, 0.0),
                },
            );

            let tween3 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: deck_translation + random_offset + Vec3::new(0.0, i as f32 * 0.01, 0.0),
                    end: new_offset,
                },
            );

            let seq = idle_tween.then(tween1).then(tween2).then(tween3);

            commands.entity(*entity).insert(TweenAnim::new(seq));
        }
    });
}

pub fn handle_place_card_on_table<T>(
    mut commands: Commands,
    mut place_card_on_table: MessageReader<PlaceCardOnTable>,
    mut set: ParamSet<(
        Query<(Entity, &mut Transform, &PlayArea)>,
        Query<(Entity, &Card<T>, &mut Transform)>,
    )>,
) where
    T: Send + Clone + Sync + Debug + 'static,
{
    for event in place_card_on_table.read() {
        let binding = set.p0();
        let play_area_transform = binding
            .iter()
            .find(|(_, _, play_area)| {
                play_area.marker == event.marker && play_area.player == event.player
            })
            .map(|(_, transform, _)| transform);

        if play_area_transform.is_none() {
            continue;
        }
        let play_area_transform = play_area_transform.unwrap();
        let play_area_translation = play_area_transform.translation;
        let play_area_rotation = play_area_transform.rotation;

        let binding = set.p1();
        let card_transform = binding
            .get(event.card_entity)
            .map(|(_, _, transform)| transform)
            .unwrap();
        let card_translation = card_transform.translation;
        let card_rotation = card_transform.rotation;

        let duration = 75;

        let tween0 = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(duration),
            TransformRotationLens {
                start: card_rotation,
                end: play_area_rotation,
            },
        );

        let tween1 = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(duration),
            TransformPositionLens {
                start: card_translation,
                end: play_area_translation,
            },
        );

        let seq = tween0.then(tween1);

        commands
            .entity(event.card_entity)
            .remove::<Hand>()
            .insert(CardOnTable {
                marker: event.marker,
                player: event.player,
            })
            .insert(TweenAnim::new(seq));
    }
}

pub fn handle_discard_card_to_deck<T>(
    mut commands: Commands,
    mut place_card_off_table: MessageReader<DiscardCardToDeck>,
    mut set: ParamSet<(
        Query<(Entity, &mut Transform, &Card<T>)>,
        Query<(Entity, &mut Transform, &DeckArea)>,
        Query<(Entity, &mut Transform, &Deck)>,
    )>,
) where
    T: Send + Clone + Sync + Debug + 'static,
{
    let duration = 150;
    for event in place_card_off_table.read() {
        let binding = set.p0();

        if binding.get(event.card_entity).is_err() {
            continue;
        }

        let card_transform = binding
            .get(event.card_entity)
            .map(|(_, transform, _)| transform)
            .unwrap();

        let card_translation = card_transform.translation;
        let card_rotation = card_transform.rotation;

        // get highest card on deck
        let binding = set.p1();
        let discard_deck_marker = binding.get(event.deck_entity).unwrap().2.marker;

        let deck_transform = binding
            .iter()
            .filter(|(_, _, deck)| deck.marker == discard_deck_marker)
            .max_by_key(|(_, transform, _)| (transform.translation.y * 100.0) as usize)
            .unwrap()
            .1;
        let deck_translation = deck_transform.translation;
        let deck_rotation = deck_transform.rotation;

        let binding = set.p2();
        let number_cards_on_deck = binding
            .iter()
            .filter(|(_, _, deck)| deck.marker == discard_deck_marker)
            .count();

        let final_translation =
            deck_translation + Vec3::new(0.0, number_cards_on_deck as f32 * 0.01, 0.0);

        let tween0 = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(duration),
            TransformRotationLens {
                start: card_rotation,
                end: deck_rotation * Quat::from_rotation_x(std::f32::consts::PI),
            },
        );

        let tween1 = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_millis(duration),
            TransformPositionLens {
                start: card_translation,
                end: final_translation,
            },
        );

        let seq = tween0.then(tween1);

        commands
            .entity(event.card_entity)
            .remove::<Hand>()
            .remove::<CardOnTable>()
            .insert(Deck {
                marker: discard_deck_marker,
            })
            .insert(TweenAnim::new(seq));
    }
}

pub fn handle_draw_to_table<T>(
    mut commands: Commands,
    mut er_draw_hand: MessageReader<DrawToTable>,
    q_play_area_area: Query<(Entity, &mut Transform, &PlayArea)>,
    q_cards: Query<(Entity, &Card<T>, &mut Transform, &Deck), Without<PlayArea>>,
    q_decks: Query<(Entity, &DeckArea)>,
) where
    T: Send + Clone + Sync + Debug + CardMetadata + 'static,
{
    let offset = Vec3::new(0.0, -0.0, 2.6);

    er_draw_hand.read().for_each(|draw| {
        let offset = offset
            * match draw.player {
                1 => 1.0,
                _ => -1.0,
            };

        let result = q_decks.get(draw.deck_entity);
        if result.is_err() {
            return;
        }

        let draw_deck = result.unwrap().1;

        let cards: Vec<(Entity, &Card<T>, &Transform)> = q_cards
            .iter()
            .filter(|(_, _, _, deck)| deck.marker == draw_deck.marker)
            .map(|(entity, card, transform, _)| (entity, card, transform))
            .collect();

        // sort cards by z-position
        let mut sorted = cards.clone();
        sorted.sort_by(|a, b| b.2.translation.y.partial_cmp(&a.2.translation.y).unwrap());

        // draw the first `num_cards` cards
        for (i, (entity, card, transform)) in sorted
            .iter_mut()
            .take(draw.play_area_markers.len())
            .enumerate()
        {
            let initial_translation = transform.translation;
            let initial_rotation = transform.rotation;
            let new_offset = Vec3::new(0.0, i as f32 * 0.01, 0.0);
            let play_area_marker = draw.play_area_markers[i];

            // find deck by deck_marker
            let play_area_transform = q_play_area_area
                .iter()
                .find(|(_, _, deck)| deck.marker == play_area_marker && deck.player == draw.player)
                .unwrap()
                .1;
            let play_area_translation = play_area_transform.translation;
            let play_area_rotation = play_area_transform.rotation;

            let idle_tween = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis((draw.duration * 4) * (i) as u64),
                TransformPositionLens {
                    start: initial_translation,
                    end: initial_translation,
                },
            );

            let slide = initial_translation + offset;
            let tween1 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(draw.duration),
                TransformPositionLens {
                    start: initial_translation,
                    end: slide,
                },
            );

            let mut slide_flat = slide;
            slide_flat.y = 0.0;

            let tween2 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(draw.duration),
                TransformPositionLens {
                    start: slide,
                    end: slide_flat + new_offset,
                },
            );

            let tween3 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(
                    (draw.duration * 4) * (draw.play_area_markers.len() - i) as u64,
                ),
                TransformPositionLens {
                    start: slide_flat + new_offset,
                    end: slide_flat + new_offset,
                },
            );

            // rotate angle depends on who player is
            let end_rotation = match draw.player {
                1 => Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                _ => {
                    Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
                        * Quat::from_rotation_y(std::f32::consts::PI)
                }
            };

            let tween4 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(draw.duration),
                TransformRotationLens {
                    start: initial_rotation,
                    end: end_rotation,
                },
            );

            let tween5 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(draw.duration),
                TransformPositionLens {
                    start: slide_flat + new_offset,
                    end: slide_flat + new_offset,
                },
            );

            let tween6 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(draw.duration),
                TransformRotationLens {
                    start: end_rotation,
                    end: play_area_rotation,
                },
            );

            let tween7 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(draw.duration),
                TransformPositionLens {
                    start: slide_flat + new_offset,
                    end: play_area_translation,
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
                transform: Some(Transform::from_translation(play_area_translation)),
                data: card.data.clone(),
            };

            commands
                .entity(*entity)
                .insert(TweenAnim::new(seq))
                .insert(CardOnTable {
                    marker: play_area_marker,
                    player: draw.player,
                })
                .remove::<Deck>()
                .insert(card);
        }
    });
}

pub fn handle_draw_to_hand<T>(
    mut commands: Commands,
    mut er_draw_hand: MessageReader<DrawToHand>,
    mut set: ParamSet<(
        Query<(Entity, &mut Transform, &HandArea)>,
        Query<(Entity, &mut Transform, &DeckArea)>,
        Query<(Entity, &Card<T>, &mut Transform, &Deck)>,
    )>,
    cards_in_hand: Query<&Hand>,
) where
    T: Send + Clone + Sync + Debug + CardMetadata + 'static,
{
    let duration = 75;
    let offset = Vec3::new(0.0, -0.0, 2.6);

    er_draw_hand.read().for_each(|draw| {
        let offset = offset
            * match draw.player {
                1 => 1.0,
                _ => -1.0,
            };

        // find global position of hand with player number
        let binding = set.p0();
        let q = binding
            .iter()
            .find(|(_, _, hand)| hand.player == draw.player)
            .map(|(_, transform, _)| transform);

        if q.is_none() {
            return;
        }

        let hand_transform = q.unwrap();
        let hand_translation = hand_transform.translation;
        let hand_rotation = hand_transform.rotation;

        // find position of deck
        let binding = set.p1();

        let deck = binding.get(draw.deck_entity);
        if deck.is_err() {
            return;
        }

        let deck = deck.unwrap();
        let hand_deck_marker = deck.2.marker;
        // find deck by deck_marker
        let deck_transform = binding
            .iter()
            .find(|(_, _, deck)| deck.marker == hand_deck_marker)
            .unwrap()
            .1;
        let deck_translation = deck_transform.translation;
        // deck_translation.z = 0.0;
        let _deck_rotation = deck_transform.rotation;
        let hand_deck_offset = deck_translation - hand_translation;

        // list all cards whose parent is deck
        let binding = set.p2();
        let cards: Vec<(Entity, &Card<T>, &Transform)> = binding
            .iter()
            .filter(|(_, _, _, deck)| deck.marker == hand_deck_marker)
            .map(|(entity, card, transform, _)| (entity, card, transform))
            .collect();

        // sort cards by z-position
        let mut sorted = cards.clone();
        sorted.sort_by(|a, b| b.2.translation.y.partial_cmp(&a.2.translation.y).unwrap());

        // number cards in hand
        let cards_in_hand = cards_in_hand
            .iter()
            .filter(|hand| hand.player == draw.player)
            .count();
        let cards_to_draw = draw.num_cards - cards_in_hand;
        // draw the first `num_cards` cards
        for (i, (entity, card, transform)) in sorted.iter_mut().take(cards_to_draw).enumerate() {
            let initial_translation = transform.translation;

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
            let tween1 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: initial_translation,
                    end: slide,
                },
            );

            let mut slide_flat = slide;
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
                Duration::from_millis((duration * 4) * (draw.num_cards - i) as u64),
                TransformPositionLens {
                    start: slide_flat + new_offset,
                    end: slide_flat + new_offset,
                },
            );

            // rotate angle depends on who player is
            let end_rotation = match draw.player {
                1 => Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
                _ => {
                    Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
                        * Quat::from_rotation_y(std::f32::consts::PI)
                }
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

            let end_transform = hand_translation
                + Vec3::new(
                    (cards_in_hand + i) as f32 * 2.6 - DECK_WIDTH / 2.0,
                    0.0,
                    0.0,
                );
            let tween7 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: slide_flat + new_offset - hand_deck_offset,
                    end: end_transform,
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
                transform: Some(Transform::from_translation(end_transform)),
                data: card.data.clone(),
            };

            commands
                .entity(*entity)
                .insert(TweenAnim::new(seq))
                .remove::<Deck>()
                // .insert(PickableBundle::default())
                .insert(card);

            let pause = Duration::from_millis((duration * 4) * (i) as u64)
                + Duration::from_millis(duration)
                + Duration::from_millis(duration)
                + Duration::from_millis((duration * 4) * (draw.num_cards - i) as u64)
                + Duration::from_millis(duration)
                + Duration::from_millis(duration)
                + Duration::from_millis(duration)
                + Duration::from_millis(duration);
            let pause = pause.as_secs_f32();

            let card_entity = entity.clone();
            let player = draw.player;
            commands.spawn_task(move || async move {
                AsyncWorld.sleep(pause).await;
                AsyncWorld.entity(card_entity).insert(Hand { player })?;

                Ok(())
            });
        }
    });
}

pub fn preload_card_images() {}

pub fn handle_render_deck<T>(
    mut commands: Commands,
    deck: Query<(&Transform, &DeckArea)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut er_render_deck: MessageReader<RenderDeck<T>>,
    mut ew_deck_rendered: MessageWriter<DeckRendered>,
) where
    T: Send + Clone + Sync + Debug + CardMetadata + 'static,
{
    for render in er_render_deck.read() {
        // load deck
        let card_deck = render.deck.clone();
        let (deck_transform, deck_area) = deck.get(render.deck_entity).unwrap();
        let deck_translation = deck_transform.translation;
        let deck_rotation = deck_transform.rotation;

        for (i, card) in card_deck.iter().enumerate() {
            let back_texture = asset_server.load(card.clone().back_image_filename());
            let back_material = materials.add(StandardMaterial {
                base_color_texture: Some(back_texture.clone()),
                ..Default::default()
            });

            let face_texture = asset_server.load(card.clone().front_image_filename());
            let face_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
                base_color_texture: Some(face_texture.clone()),
                ..Default::default()
            });

            let transform = Transform::from_translation(
                deck_translation + Vec3::new(0.0, 0.01 * (i as f32), 0.0),
            )
            .with_rotation(
                deck_rotation
                    * Quat::from_rotation_x(std::f32::consts::PI)
                    * Quat::from_rotation_y(std::f32::consts::PI),
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
                    Deck {
                        marker: deck_area.marker,
                    },
                    Pickable::default(),
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
                    transform,
                ))
                .observe(on_card_over)
                .observe(on_card_out)
                .observe(on_card_click)
                .with_children(|parent| {
                    // face
                    parent.spawn((
                        Mesh3d(
                            meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
                        ),
                        MeshMaterial3d(face_material),
                    ));

                    // back
                    parent.spawn((
                        Mesh3d(
                            meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
                        ),
                        MeshMaterial3d(back_material),
                        Transform::IDENTITY
                            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                    ));
                });
        }

        ew_deck_rendered.write(DeckRendered {});
    }
}

fn on_card_click(click: On<Pointer<Click>>, mut ew_card: MessageWriter<CardPress>) {
    ew_card.write(CardPress {
        entity: click.event().entity,
    });
}

fn on_card_over(click: On<Pointer<Over>>, mut ew_card: MessageWriter<CardHover>) {
    ew_card.write(CardHover {
        entity: click.event().entity,
    });
}

fn on_card_out(click: On<Pointer<Out>>, mut ew_card: MessageWriter<CardOut>) {
    ew_card.write(CardOut {
        entity: click.event().entity,
    });
}

pub fn handle_align_cards_in_hand<T>(
    mut commands: Commands,
    mut cards_in_hand: Query<(Entity, &mut Card<T>, &Hand, &mut Transform)>,
    mut er_align_cards_in_hand: MessageReader<AlignCardsInHand>,
) where
    T: Send + Clone + Sync + Debug + CardMetadata + 'static,
{
    for event in er_align_cards_in_hand.read() {
        let mut cards = cards_in_hand
            .iter_mut()
            .filter(|(_, _, hand, _)| hand.player == event.player)
            .collect::<Vec<_>>();
        cards.sort_by(|a, b| a.3.translation.x.partial_cmp(&b.3.translation.x).unwrap());

        // animate x position change
        for (i, (entity, card, _, transform)) in cards.iter_mut().enumerate() {
            let original_translation = transform.translation;
            let mut new_translation = original_translation;
            new_translation.x = i as f32 * 2.6 - DECK_WIDTH / 2.0;

            let tween = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(75),
                TransformPositionLens {
                    start: original_translation,
                    end: new_translation,
                },
            );

            card.transform = Some(Transform::from_translation(new_translation));

            commands.entity(*entity).insert(TweenAnim::new(tween));
        }
    }
}
