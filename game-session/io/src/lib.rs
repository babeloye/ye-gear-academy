#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, PartialEq};

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<SessionAction, SessionEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<Session>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionAction {
    StartGame { user: ActorId },
    CheckWord { user: ActorId, word: String },
    CheckGameStatus { user: ActorId },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionEvent {
    GameStarted {
        user: ActorId,
    },
    WordChecked {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
    GameStatus(GameStatus),
    GameError(String),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Session {
    pub target_program_id: ActorId,
    pub session_status: SessionStatus,
    pub start_block_height: u32,
    pub attempts: u8,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct GameStatus {
    pub game_result: Option<GameResult>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum GameResult {
    Win,
    Lose,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum SessionStatus {
    Waiting,
    MessageSent,
    MessageReceived(Event),
    GameEnded { result: GameResult },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Action {
    StartGame { user: ActorId },
    CheckWord { user: ActorId, word: String },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum Event {
    GameStarted {
        user: ActorId,
    },
    WordChecked {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    pub session_status: SessionStatus,
    pub attempts_remaining: u8,
    pub game_result: Option<GameResult>,
    pub last_action_block: u32,
}

impl From<&Session> for GameState {
    fn from(session: &Session) -> Self {
        GameState {
            session_status: session.session_status.clone(),
            attempts_remaining: session.attempts,
            game_result: match &session.session_status {
                SessionStatus::GameEnded { result } => Some(result.clone()),
                _ => None,
            },
            last_action_block: session.start_block_height,
        }
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct GameMetrics {
    pub total_games_played: u32,
    pub total_wins: u32,
    pub total_losses: u32,
    pub average_attempts: u32,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StateQuery {
    GetGameState,
    GetGameMetrics,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StateReply {
    GameState(GameState),
    GameMetrics(GameMetrics),
}

impl Session {
    pub fn handle_state_query(&self, query: StateQuery) -> StateReply {
        match query {
            StateQuery::GetGameState => StateReply::GameState(self.into()),
            StateQuery::GetGameMetrics => StateReply::GameMetrics(GameMetrics {
                total_games_played: 0,
                total_wins: 0,
                total_losses: 0,
                average_attempts: 0,
            }),
        }
    }
}
