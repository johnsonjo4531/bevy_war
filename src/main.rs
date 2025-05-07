use bevy::{platform::collections::HashMap, prelude::*};
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

fn image_name(card: &Card) -> String {
    let suit = card.0.clone();
    let card = card.1.as_str();
    format!("kenney_boardgame-pack/PNG/Cards/card{}{}.png", suit, card).to_string()
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

#[derive(Component, Default, Clone)]
struct PlayerNum(u8);

impl PartialEq for PlayerNum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for PlayerNum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayerNum {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl Eq for PlayerNum {}

#[derive(Component, Debug, Clone)]
#[require(PlayerNum)]
struct Player {
    cards: Vec<Card>,
}

fn initial_cards() -> Vec<Card> {
    let mut rng = rand::rng(); // Create a thread-local RNG

    let mut cards = face_cards().collect::<Vec<Card>>();
    cards.shuffle(&mut rng);
    cards
}

fn init_players(mut commands: Commands) {
    // Maybe make this a resource
    let num_players = 2;
    let mut players = Vec::new();
    for player in 0..num_players {
        players.push(Player { cards: Vec::new() });
    }

    for (player_num, player) in players.into_iter().enumerate() {
        commands.spawn((player, PlayerNum((player_num + 1) as u8)));
    }
}

fn init_pot(mut commands: Commands) {
    commands.spawn(Pot { cards: Vec::new() });
}

fn reset_players(mut players_query: Query<&mut Player>) {
    let mut players = Vec::new();
    for player in players_query.iter_mut() {
        players.push(player);
    }

    for player in players.iter_mut() {
        player.cards.clear();
    }

    for (i, card) in initial_cards().into_iter().enumerate() {
        let player_index = i % players.len();
        players[player_index].cards.push(card);
    }
}

fn reset_pot(mut pot: Query<&mut Pot>) -> Result {
    let mut pot = pot.single_mut()?;
    pot.cards.clear();
    Ok(())
}

fn reset_cards_in_play(mut cards: Query<&mut CardsInPlay>) {
    for mut cards in cards.iter_mut() {
        cards.cards.clear();
    }
}

#[derive(Component)]
#[require(PlayerNum)]
struct PlayerArea;

#[derive(Component)]
#[require(PlayerNum)]
struct PlayerCardNum;

#[derive(Component)]
#[require(ImageNode)]
struct DeckArea;

#[derive(Component, Default)]
#[require(ImageNode)]
struct CurrentCardArea;

#[derive(Component)]
#[require(CurrentCardArea, PlayerNum)]
struct CardsInPlay {
    cards: Vec<Card>,
}

#[derive(Component)]
struct Pot {
    cards: Vec<Card>,
}

#[derive(Component)]
#[require(TextSpan)]
struct StatusText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Stretch,
            ..default()
        },
        BackgroundColor::from(Color::srgb(0.3, 0.3, 0.3)),
        children![
            spawn_player_ui(
                font.clone(),
                asset_server.load("kenney_boardgame-pack/PNG/Cards/cardBack_blue5.png"),
                FlexDirection::RowReverse,
                PlayerNum(2),
            ),
            (
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![(
                    Text(String::from("")),
                    TextFont {
                        font: font.clone(),
                        font_size: 60.,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                    children![(
                        StatusText,
                        TextSpan(String::from("")),
                        TextFont {
                            font: font.clone(),
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    )]
                )]
            ),
            spawn_player_ui(
                font,
                asset_server.load("kenney_boardgame-pack/PNG/Cards/cardBack_green5.png"),
                FlexDirection::Row,
                PlayerNum(1),
            )
        ],
    ));
}

fn spawn_player_ui(
    font: Handle<Font>,
    back_image: Handle<Image>,
    flex_direction: FlexDirection,
    player_num: PlayerNum,
) -> impl Bundle {
    (
        // Play Area
        Node {
            margin: UiRect::all(Val::Px(10.0)),
            flex_direction,
            column_gap: Val::Px(20.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Px(140.0),
                    height: Val::Px(190.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ImageNode {
                    image: back_image,
                    ..default()
                },
                DeckArea,
                player_num.clone()
            ),
            (
                Node {
                    width: Val::Px(140.),
                    height: Val::Px(190.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ImageNode { ..default() },
                CurrentCardArea,
                CardsInPlay { cards: Vec::new() },
                player_num.clone()
            ),
            (
                Node {
                    width: Val::Px(140.),
                    height: Val::Px(190.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Text(format!(
                            "Player {}",
                            match player_num {
                                PlayerNum(num) => format!("{}", num),
                            }
                        )),
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::all(Val::Px(10.)),
                            ..default()
                        },
                    ),
                    (
                        Text(String::from("Cards: ")),
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::all(Val::Px(10.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        children![(
                            TextSpan(String::from("?")),
                            TextFont {
                                font: font.clone(),
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            PlayerCardNum,
                            player_num,
                        )],
                    )
                ]
            ),
        ],
    )
}

fn update_card_count(
    player: Query<(&Player, &PlayerNum)>,
    mut player_text: Query<(&mut TextSpan, &PlayerNum), With<PlayerCardNum>>,
) {
    for player in player
        .iter()
        .sort::<&PlayerNum>()
        .zip(player_text.iter_mut().sort::<&PlayerNum>())
    {
        let (player, text) = player;
        let (player, _) = player;
        let (mut text, _) = text;

        *text = TextSpan(format!("{}", player.cards.len()));
    }
}

fn display_current_card(
    player: Query<(&CardsInPlay, &PlayerNum)>,
    mut card_image: Query<(&mut ImageNode, &PlayerNum), With<CurrentCardArea>>,
    asset_server: Res<AssetServer>,
) {
    for player in player
        .iter()
        .sort::<&PlayerNum>()
        .zip(card_image.iter_mut().sort::<&PlayerNum>())
    {
        let (player, image) = player;
        let (player, _) = player;
        let (mut image, _) = image;
        if let Some(card) = player.cards.first() {
            *image = ImageNode {
                image: asset_server.load(image_name(card)),
                ..default()
            }
        }
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum RoundState {
    #[default]
    Begin,

    GameStart,

    Draw,
    Outcome,

    GameOver,
}

fn advance_round(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<RoundState>>,
    mut next_state: ResMut<NextState<RoundState>>,
) {
    if keys.just_pressed(KeyCode::Space)
        || (!state.get().eq(&RoundState::GameOver)
            && (keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight)))
    {
        next_state.set(match state.get() {
            RoundState::Begin => RoundState::GameStart,
            RoundState::GameStart => RoundState::Draw,
            RoundState::Draw => RoundState::Outcome,
            RoundState::Outcome => RoundState::Draw,
            RoundState::GameOver => RoundState::GameStart,
        });
    }
}

fn draw(
    mut player: Query<(&mut Player, &PlayerNum)>,
    mut cards: Query<(&mut CardsInPlay, &PlayerNum)>,
) {
    for player in player
        .iter_mut()
        .sort::<&PlayerNum>()
        .zip(cards.iter_mut().sort::<&PlayerNum>())
    {
        let (player, cards) = player;
        let (mut player, _) = player;
        let (mut cards, _) = cards;
        if player.cards.is_empty() {
            break;
        }
        cards.cards.splice(0..0, player.cards.splice(0..1, []));
    }
}

fn check_battle(
    mut player: Query<(&mut Player, &PlayerNum)>,
    mut cards: Query<(&mut CardsInPlay, &PlayerNum)>,
    mut pot: Query<&mut Pot>,
    mut status: Query<&mut TextSpan, With<StatusText>>,
) -> Result {
    let mut pot = pot.single_mut()?;
    let mut status = status.single_mut()?;
    let mut greatest = 0;
    let mut winner = 0;
    let mut is_draw = true;
    for cards in cards.iter_mut().sort::<&PlayerNum>() {
        let (mut cards, player_num) = cards;
        if let Some(card) = cards.cards.first() {
            if card.2 > greatest {
                greatest = card.2;
                winner = player_num.0;
                is_draw = false;
            } else if card.2 == greatest {
                is_draw = true;
            }
        }
        pot.cards.append(&mut cards.cards);
        cards.cards = Vec::new();
    }
    if is_draw {
        *status = TextSpan(String::from("Draw!"));
        return Ok(());
    };
    *status = TextSpan(format!("Player {} win!", winner));
    for player in player.iter_mut().sort::<&PlayerNum>() {
        let (mut player, player_num) = player;
        if player_num.0 == winner {
            // shuffle the cards after a battle should allow the game to end sooner and hopefully
            // not go on forever.
            let mut rng = rand::rng();
            pot.cards.shuffle(&mut rng);
            player.cards.append(&mut pot.cards);
            pot.cards = Vec::new();
        }
    }
    Ok(())
}

fn hide_outcome(mut status: Query<&mut TextSpan, With<StatusText>>) -> Result {
    let mut status = status.single_mut()?;

    *status = TextSpan(String::from(""));
    Ok(())
}

fn check_end(mut players: Query<&Player>, mut next_state: ResMut<NextState<RoundState>>) {
    let zero_card_players = players
        .iter()
        .filter(|player| player.cards.is_empty())
        .collect::<Vec<&Player>>();
    if zero_card_players.len() >= players.iter().len() - 1 {
        next_state.set(RoundState::GameOver);
    }
}

fn show_winner(
    players: Query<(&Player, &PlayerNum)>,
    state: Res<State<RoundState>>,
    mut next_state: ResMut<NextState<RoundState>>,
    mut status: Query<&mut TextSpan, With<StatusText>>,
) -> Result {
    if !state.get().eq(&RoundState::GameOver) {
        return Ok(());
    }
    let mut status = status.single_mut()?;
    let zero_card_players = players
        .iter()
        .sort_by::<&Player>(|p1, p2| p1.cards.len().cmp(&p2.cards.len()).reverse())
        .filter(|(player, _)| !player.cards.is_empty())
        .collect::<Vec<(&Player, &PlayerNum)>>();
    if let Some((_winner, player_num)) = zero_card_players.first() {
        *status = TextSpan(format!("Player {} wins! Play again?", player_num.0));
    }
    Ok(())
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("War!"),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (init_players, setup_ui, init_pot))
        .add_systems(
            Update,
            (update_card_count, display_current_card, advance_round),
        )
        .add_systems(
            OnEnter(RoundState::GameStart),
            (reset_pot, reset_players, reset_cards_in_play),
        )
        .add_systems(OnEnter(RoundState::Draw), (hide_outcome, draw))
        .add_systems(OnEnter(RoundState::Outcome), (check_battle))
        .add_systems(OnExit(RoundState::Outcome), check_end)
        .add_systems(OnEnter(RoundState::GameOver), show_winner)
        .init_state::<RoundState>()
        .run();
}
