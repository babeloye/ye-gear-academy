#![no_std]
#![allow(warnings)]
use game_session_io::*;
use gstd::{debug, exec, msg, prelude::*};

static mut SESSION: Option<Session> = None;

#[no_mangle]
extern "C" fn init() {
    let target_program_id = msg::load().expect("Unable to decode Init");
    let session = Session {
        target_program_id,
        session_status: SessionStatus::Waiting,
        start_block_height: exec::block_height(),
        attempts: 5,
    };
    unsafe {
        SESSION = Some(session);
    }
}

#[no_mangle]
extern "C" fn handle() {
    let mut session = unsafe { SESSION.as_mut().expect("Session is not initialized") }.clone();
    let action: SessionAction = msg::load().expect("Unable to decode `Action`");

    match &session.session_status {
        SessionStatus::Waiting => match action {
            SessionAction::StartGame { user } => {
                debug!("WAITING AND START GAME");
                session.start_block_height = exec::block_height();
                session.attempts = 5;

                msg::send(session.target_program_id, Action::StartGame { user }, 0)
                    .expect("Error in sending a message");
                session.session_status = SessionStatus::MessageSent;
                unsafe {
                    SESSION = Some(session);
                }
                exec::wait();
            }
            SessionAction::CheckWord { user, word } => {
                if matches!(session.session_status, SessionStatus::GameEnded { .. }) {
                    msg::reply(SessionEvent::GameError("Game has already ended".into()), 0)
                        .expect("Error in sending a reply");
                    return;
                }

                if session.attempts == 0 {
                    session.session_status = SessionStatus::GameEnded {
                        result: GameResult::Lose,
                    };
                    msg::reply(
                        SessionEvent::GameError("No more attempts left, game over".into()),
                        0,
                    )
                    .expect("Error in sending a reply");
                    return;
                }

                let current_game_status = GameStatus {
                    game_result: match &session.session_status {
                        SessionStatus::GameEnded { result } => Some(result.clone()),
                        _ => None,
                    },
                };

                if let Some(_) = current_game_status.game_result {
                    msg::reply(SessionEvent::GameStatus(current_game_status), 0)
                        .expect("Unable to reply");
                } else {
                    msg::send(
                        session.target_program_id,
                        Action::CheckWord { user, word },
                        0,
                    )
                    .expect("Error in sending a message");
                    session.attempts -= 1;
                    session.session_status = SessionStatus::MessageSent;
                    unsafe {
                        SESSION = Some(session.clone());
                    }
                    exec::wait();
                }
            }
            SessionAction::CheckGameStatus { user: _ } => {
                let current_block_height = exec::block_height();
                let block_difference =
                    current_block_height.saturating_sub(session.start_block_height);

                if block_difference >= 200 {
                    session.session_status = SessionStatus::GameEnded {
                        result: GameResult::Lose,
                    };
                    unsafe {
                        SESSION = Some(session.clone());
                    }
                    msg::reply(
                        SessionEvent::GameStatus(GameStatus {
                            game_result: Some(GameResult::Lose),
                        }),
                        0,
                    )
                    .expect("Unable to reply");
                } else {
                    let current_game_status = GameStatus {
                        game_result: match &session.session_status {
                            SessionStatus::GameEnded { result } => Some(result.clone()),
                            _ => None,
                        },
                    };
                    msg::reply(SessionEvent::GameStatus(current_game_status), 0)
                        .expect("Unable to reply");
                }
            }
        },
        SessionStatus::MessageSent => {
            debug!("MESSAGE SENT");
            msg::reply(
                SessionEvent::GameError("Message has already been sent, restart the game".into()),
                0,
            )
            .expect("Error in sending a reply");
        }
        SessionStatus::MessageReceived(event) => match event {
            Event::GameStarted { user } => {
                debug!("GAME STARTED FOR USER: {:?}", user);
                msg::send_delayed(
                    exec::program_id(),
                    SessionAction::CheckGameStatus { user: *user },
                    0,
                    200,
                )
                .expect("Failed to send delayed message");

                msg::reply(SessionEvent::GameStarted { user: *user }, 0)
                    .expect("Error in sending a reply");

                if !matches!(session.session_status, SessionStatus::GameEnded { .. }) {
                    session.session_status = SessionStatus::Waiting;
                }
                unsafe {
                    SESSION = Some(session.clone());
                }
            }
            Event::WordChecked {
                user,
                correct_positions,
                contained_in_word,
            } => {
                if correct_positions.len() == 5 {
                    session.session_status = SessionStatus::GameEnded {
                        result: GameResult::Win,
                    };
                    msg::reply(
                        SessionEvent::GameStatus(GameStatus {
                            game_result: Some(GameResult::Win),
                        }),
                        0,
                    )
                    .expect("Error in sending a reply");
                    unsafe {
                        SESSION = Some(session.clone());
                    }
                } else {
                    msg::reply(
                        SessionEvent::WordChecked {
                            user: *user,
                            correct_positions: correct_positions.clone(),
                            contained_in_word: contained_in_word.clone(),
                        },
                        0,
                    )
                    .expect("Error in sending a reply");

                    if session.attempts == 0 {
                        session.session_status = SessionStatus::GameEnded {
                            result: GameResult::Lose,
                        };
                    } else {
                        session.session_status = SessionStatus::Waiting;
                    }
                    unsafe {
                        SESSION = Some(session.clone());
                    }
                }
            }
        },
        SessionStatus::GameEnded { result } => {
            debug!("GAME ENDED: {:?}", result);
        }
    }

    unsafe {
        SESSION = Some(session);
    }
    debug!("HANDLE ENDED");
}

#[no_mangle]
extern "C" fn handle_reply() {
    let reply_to = msg::reply_to().expect("Failed to query reply_to data");
    let session = unsafe { SESSION.as_mut().expect("Session is not initialized") };
    let event: Event = msg::load().expect("Unable to decode `Event`");
    session.session_status = SessionStatus::MessageReceived(event);
    debug!("WAKING UP MESSAGE WITH ID: {:?}", reply_to);
    exec::wake(reply_to).expect("Failed to wake up the message");
}

#[no_mangle]
extern "C" fn state() {
    let session = unsafe { SESSION.as_ref().expect("Session is not initialized") };
    msg::reply(session, 0).expect("Unable to get the state");
}
