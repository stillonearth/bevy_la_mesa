# bevy_la_mesa

[![Crates.io](https://img.shields.io/crates/v/bevy_la_mesa.svg)](https://crates.io/crates/bevy_la_mesa)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/bevyengine/bevy#license)
[![Crates.io](https://img.shields.io/crates/d/bevy_la_mesa.svg)](https://crates.io/crates/bevy_la_mesa)

Plugin for building card-based games on bevy. Handles deck, cards, hand and table.

## Events

|   Event             | Descritpion                           |
| ------------------- | ------------------------------------- |
| `RenderDeck`        | render deck on table                  |
| `DeckRendered`      | when deck is rendered                 |
| `CardHover`         | mouse card hover on                   |
| `CardOut`           | mouse card pointer off                |
| `CardPress`         | press card                            |
| `DeckShuffle`       | shuffle deck of cards                 |
| `DrawHand`          | draw cards from deck to hard          |
| `PlaceCardOnTable`  | place card from hand or deck on table |
| `AlignCardsInHand`  | align cards in hands spacially        |
| `DiscardCardToDeck` | place card from back to deck          |
