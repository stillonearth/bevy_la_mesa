use bevy::prelude::*;
use bevy_mod_picking::{events::*, prelude::*};
use bevy_tweening::{lens::*, *};

use rand::prelude::*;
use std::fmt::Debug;
use std::time::Duration;

use crate::{Card, Deck, DeckArea, HandArea, DECK_WIDTH};

// Events
#[derive(Event)]
pub struct CardHover {
    pub entity: Entity,
}

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

// Event Handlers
pub fn handle_card_hover<T>(
    mut commands: Commands,
    mut hover: EventReader<CardHover>,
    mut query: Query<(Entity, &Card<T>, &mut Transform)>,
) where
    T: Send + Sync + Debug + 'static,
{
    hover.read().for_each(|hover| {
        if let Ok((_, card, transform)) = query.get_mut(hover.entity) {
            if card.pickable && card.transform.is_some() {
                let tween = Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_millis(300),
                    TransformPositionLens {
                        start: transform.translation.clone(),
                        end: card.transform.unwrap().translation.clone() + Vec3::new(0., 0.0, 0.5),
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
                        start: transform.translation.clone(),
                        end: card.transform.unwrap().translation.clone(),
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
    T: Send + Sync + Debug + 'static,
{
    shuffle.read().for_each(|shuffle| {
        println!("shuffle deck");

        // list all cards whose parent is deck
        let cards: Vec<(Entity, &Card<T>, &Transform)> = query_cards
            .iter()
            .map(|(entity, card, transform, _)| (entity, card, transform))
            .collect();

        // shuffle the cards
        let mut rng = rand::thread_rng();
        let mut shuffled = cards.clone();
        shuffled.shuffle(&mut rng);

        println!("shuffled cards: {:?}", shuffled.len());

        // once cards shuffled reorder them with animation
        let duration = 75;
        let random_offset_right = Vec3::new(3.0, -0.0, 0.0);
        let random_offset_left = Vec3::new(-3.0, -0.0, 0.0);

        for (i, (entity, _, transform)) in shuffled.iter().enumerate() {
            // choose random 3 to the left or 3 to the right
            let random_offset = if i % 2 == 0 {
                random_offset_right
            } else {
                random_offset_left
            };

            let initial_translation = transform.translation.clone();
            let new_offset = Vec3::new(0.0, i as f32 * 0.01, 0.0);

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
                    end: random_offset + Vec3::new(0.0, i as f32 * 0.01, 0.0),
                },
            );

            let tween3 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: initial_translation + random_offset + new_offset,
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
    T: Send + Sync + Debug + 'static,
{
    shuffle.read().for_each(|shuffle| {
        // find global position of hand with player number
        let binding = set.p0();
        let hand_transform = binding
            .iter()
            .find(|(_, _, hand)| hand.player == shuffle.player)
            .map(|(_, transform, _)| transform)
            .unwrap();
        let hand_translation = hand_transform.clone().translation;
        let hand_rotation = hand_transform.clone().rotation;

        // find position of deck
        let binding = set.p1();
        let deck_transform = binding
            .iter()
            .find(|(_, _, _deck)| true)
            .map(|(_, transform, _)| transform)
            .unwrap();
        let deck_translation = deck_transform.clone().translation;
        let _deck_rotation = deck_transform.clone().rotation;

        // list all cards whose parent is deck
        let binding = set.p2();
        let cards: Vec<(Entity, &Card<T>, &Transform)> = binding
            .iter()
            .map(|(entity, card, transform, _)| (entity, card, transform))
            .collect();

        // sort cards by z-position
        let mut sorted = cards.clone();
        sorted.sort_by(|a, b| a.2.translation.z.partial_cmp(&b.2.translation.z).unwrap());

        let duration = 75;
        let offset = Vec3::new(-3.0, -0.0, 0.0);

        let hand_deck_offset = deck_translation - hand_translation;
        // rotate hand deck offset by 90 degrees
        // let hand_deck_offset =
        //     Vec3::new(hand_deck_offset.x, -hand_deck_offset.z, hand_deck_offset.y);

        // draw the first `num_cards` cards
        for (i, (entity, _card, transform)) in sorted.iter().take(shuffle.num_cards).enumerate() {
            let initial_translation = transform.translation.clone();
            let initial_rotation = transform.rotation.clone();
            let new_offset = Vec3::new(0.0, i as f32 * 0.01, 0.0);

            let idle_tween = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis((duration * 4) * (i) as u64),
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
                    end: initial_translation + offset,
                },
            );

            let tween2 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: initial_translation + offset,
                    end: offset + new_offset,
                },
            );

            let tween3 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis((duration * 4) * (shuffle.num_cards - i) as u64),
                TransformPositionLens {
                    start: offset + new_offset,
                    end: offset + new_offset,
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
                    start: initial_rotation.clone(),
                    end: end_rotation,
                },
            );

            let tween5 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(duration),
                TransformPositionLens {
                    start: offset + new_offset,
                    end: offset + new_offset - hand_deck_offset,
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
                    start: offset + new_offset - hand_deck_offset,
                    end: -hand_deck_offset + Vec3::new(i as f32 * 2.6 - DECK_WIDTH / 2.0, 0.0, 0.0),
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

            commands
                .entity(*entity)
                .insert(Animator::new(seq))
                .insert(HandArea {
                    player: shuffle.player,
                })
                .remove::<DeckArea>();
        }
    });
}

// pub fn handle_move_card_to_hand(
//     mut commands: Commands,
//     mut move_card: EventReader<MoveCardToHand>,
//     mut query: Query<(Entity, &Card<()>, &Transform)>,
// ) {
//     move_card.read().for_each(|move_card| {
//         if let Ok((entity, mut card, transform)) = query.get(move_card.card_entity) {

//             // let mut new_transform = transform.clone();
//             // new_transform.translation = Vec3::new(0.0, 0.0, 0.0);
//             // new_transform.rotation = Quat::IDENTITY;

//             // commands.entity(entity).insert(new_transform);
//             // commands.entity(entity).remove::<Parent>();
//         }
//     });
// }
