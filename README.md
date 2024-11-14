# bevy_la_mesa

Plugin for building card-based games on bevy. Made during bevy jam #5

![Screenshot 2024-08-06 125359](https://github.com/user-attachments/assets/2ce34ebe-262c-43b6-85d8-4c81568ac199)

## Versions

| bevy_la_mesa | bevy |
| ------------ | -----|
| 0.07         | 0.14 |

## Plugin Settings 

```rust
// Setup card types and chip types

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ChipType;

#[derive(Default, Clone, Debug)]
pub struct Kard {
    pub card_type: CardType,
    pub filename: String,
}

impl CardMetadata for Kard {
    type Output = Kard;

    fn filename(&self) -> String {
        self.filename.clone()
    }
}

// Setup Plugin
app.add_plugins((LaMesaPlugin::<Kard, ChipType>::default(), ObjPlugin))
    .insert_resource(LaMesaPluginSettings {
        num_players: 2,
        hand_size: 5,
        back_card_path: "tarjetas/back.png".to_string(),
    });
```

## Events

|   Event           | Descritpion                           |
| ----------------- | ------------------------------------- |
| RenderDeck        | render deck on table                  |
| CardHover         | mouse card hover on                   |
| CardOut           | mouse card pointer off                |
| CardPress         | press card                            |
| DeckShuffle       | shuffle deck of cards                 |
| DrawHand          | draw cards from deck to hard          |
| PlaceCardOnTable  | place card from hand or deck on table |
| AlignCardsInHand  | align cards in hands spacially        |
| PlaceCardOffTable | remove card from table to deck        |
| AlignChipsOnTable | align chips on table                  |
