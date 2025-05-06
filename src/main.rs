use bevy::{prelude::*, utils::HashMap};
use itertools::Itertools;
use rand::prelude::*;

fn face_value_map() -> HashMap<String, u8> {
    let faces = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];
    let values = 2..=14;

    faces
        .into_iter()
        .zip(values)
        .map(|(face, value)| (face.to_string(), value))
        .collect()
}

fn display_card(card: &Card) -> String {
    let suit = card.0.clone();
    let card = card.1.as_str();
    match card {
        "J" => format!("Jack of {}", suit).to_string(),
        "Q" => format!("Queen of {}", suit).to_string(),
        "K" => format!("King of {}", suit).to_string(),
        "A" => format!("Ace of {}", suit).to_string(),
        card => format!("{} of {}", card, suit).to_string(),
    }
}

fn value_face_reverse_map() -> HashMap<u8, String> {
    let faces = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];
    let values = 2..=14;

    faces
        .into_iter()
        .zip(values)
        .map(|(face, value)| (value, face.to_string()))
        .collect()
}

#[derive(Debug, Clone)]
pub struct Card(String, String, u8);

fn face_cards() -> impl Iterator<Item = Card> {
    let face_value = face_value_map();
    vec![
        vec!["Clubs", "Diamonds", "Hearts", "Spades"],
        vec![
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
        ],
    ]
    .into_iter()
    .map(|x| {
        x.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    })
    .multi_cartesian_product()
    .map(move |pair| {
        // pattern match Vec<String> of length 2
        let [suit, value]: [String; 2] = pair.try_into().expect("Expected a pair");
        Card(
            suit,
            value.clone(),
            face_value.get(&value).unwrap().to_owned().clone(),
        )
    })
}

#[derive(Component, Debug, Clone)]
struct Player {
    cards: Vec<Card>,
    player_num: u8,
}

fn initial_cards() -> Vec<Card> {
    let mut rng = rand::rng(); // Create a thread-local RNG

    let mut cards = face_cards().collect::<Vec<Card>>();
    cards.shuffle(&mut rng);
    cards
}

fn distribute_cards(players: &mut Vec<Player>, cards: Vec<Card>) {
    for player in players.iter_mut() {
        player.cards.clear();
    }

    for (i, card) in cards.into_iter().enumerate() {
        let player_index = i % players.len();
        players[player_index].cards.push(card);
    }
}

fn init_players(mut commands: Commands) {
    // Maybe make this a resource
    let num_players = 2;
    let mut players = Vec::new();
    for player in 0..num_players {
        players.push(Player {
            cards: Vec::new(),
            player_num: player,
        });
    }

    distribute_cards(&mut players, initial_cards());

    for player in players {
        commands.spawn(player);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_players)
        .run();
}
