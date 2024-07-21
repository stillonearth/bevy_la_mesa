use bevy::prelude::*;
use bevy_mod_picking::{events::*, prelude::*};
use bevy_tweening::{lens::*, *};

use crate::{Card, Deck};

use rand::prelude::*;
use std::time::Duration;

// Events
#[derive(Event)]
pub struct CardHover {
    pub entity: Entity,
}

#[derive(Event)]
pub struct DeckShuffle {
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

// Event Handlers
pub fn handle_card_hover<T>(
    mut commands: Commands,
    mut hover: EventReader<CardHover>,
    mut query: Query<(Entity, &Card<T>, &mut Transform)>,
) where
    T: Send + Sync + 'static,
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
    T: Send + Sync + 'static,
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
    mut query_cards: Query<(Entity, &Parent, &Card<T>, &mut Transform)>,
    mut query_deck: Query<(Entity, &Deck)>,
) where
    T: Send + Sync + 'static,
{
    shuffle.read().for_each(|shuffle| {
        // list all cards whose parent is deck
        let cards: Vec<(Entity, &Card<T>, &Transform)> = query_cards
            .iter()
            .filter(|(_, parent, _, _)| parent.index() == shuffle.entity.index())
            .map(|(entity, _, card, transform)| (entity, card, transform))
            .collect();

        // shuffle the cards
        let mut rng = rand::thread_rng();
        let mut shuffled = cards.clone();
        shuffled.shuffle(&mut rng);

        // once cards shuffled reorder them with animation

        for (i, (entity, _, transform)) in shuffled.iter().enumerate() {
            let random_offset = Vec3::new(
                rng.gen_range(-2.5..2.5),
                rng.gen_range(-2.5..2.5),
                rng.gen_range(-2.5..2.5),
            );
            let initial_translation = transform.translation.clone();

            let tween1 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: initial_translation,
                    end: initial_translation + random_offset,
                },
            );

            let tween2 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: initial_translation + random_offset,
                    end: initial_translation + random_offset + Vec3::new(0.0, 0.0, i as f32 * 0.01),
                },
            );

            let tween3 = Tween::new(
                EaseFunction::QuadraticIn,
                Duration::from_millis(300),
                TransformPositionLens {
                    start: initial_translation
                        + random_offset
                        + Vec3::new(0.0, 0.0, i as f32 * 0.01),
                    end: Vec3::new(0.0, 0.0, i as f32 * 0.01),
                },
            );

            let seq = tween1.then(tween2).then(tween3);

            commands.entity(*entity).insert(Animator::new(seq));
        }
    });

    // out.read().for_each(|hover| {
    //     if let Ok((_, card, transform)) = query.get_mut(hover.entity) {
    //         if card.pickable && card.transform.is_some() {
    //             let tween = Tween::new(
    //                 EaseFunction::QuadraticIn,
    //                 Duration::from_millis(300),
    //                 TransformPositionLens {
    //                     start: transform.translation.clone(),
    //                     end: card.transform.unwrap().translation.clone(),
    //                 },
    //             );

    //             commands.entity(hover.entity).insert(Animator::new(tween));
    //         }
    //     }
    // });
}
