#![no_std]

use game_session_io::*;
use gtest::{Program, ProgramBuilder, System};

const USER1: u64 = 10;
const SESSION_PROGRAM_ID: u64 = 1;
const WORDLE_PROGRAM_ID: u64 = 2;

#[test]
fn test_game_start() {
    let system = System::new();
    system.init_logger();

    let session_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(SESSION_PROGRAM_ID)
            .build(&system);

    let wordle_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    system.mint_to(USER1, 12345678987654321258);
    wordle_program.send_bytes(USER1, []);
    system.run_next_block();
    session_program.send(USER1, wordle_program.id());
    system.run_next_block();
    session_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "helle".into(),
        },
    );
    system.run_next_block();

    let state: Session = session_program
        .read_state(())
        .expect("Failed to read state");
    assert_eq!(state.session_status, SessionStatus::Waiting);
}

#[test]
fn test_game_check() {
    let system = System::new();
    system.init_logger();

    let session_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(SESSION_PROGRAM_ID)
            .build(&system);

    let wordle_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);
    system.mint_to(USER1, 12345678987654321258);

    wordle_program.send_bytes(USER1, []);
    system.run_next_block();
    session_program.send(USER1, wordle_program.id());
    system.run_next_block();
    session_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    system.run_next_block();

    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "homan".into(),
        },
    );
    system.run_next_block();

    let state: Session = session_program
        .read_state(())
        .expect("Failed to read state");
    assert_eq!(
        state.session_status,
        SessionStatus::GameEnded {
            result: GameResult::Lose
        }
    );
}

#[test]
fn test_game_win() {
    let system = System::new();
    system.init_logger();

    let session_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(SESSION_PROGRAM_ID)
            .build(&system);

    let wordle_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    system.mint_to(USER1, 12345678987654321258);
    wordle_program.send_bytes(USER1, []);
    system.run_next_block();
    session_program.send(USER1, wordle_program.id());
    system.run_next_block();
    session_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    system.run_next_block();

    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "human".into(),
        },
    );
    system.run_next_block();

    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "horse".into(),
        },
    );
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "house".into(),
        },
    );
    system.run_next_block();

    let state: Session = session_program
        .read_state(())
        .expect("Failed to read state");
    assert_eq!(
        state.session_status,
        SessionStatus::GameEnded {
            result: GameResult::Win
        }
    );
}

#[test]
fn test_timeout() {
    let system = System::new();
    system.init_logger();

    let session_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/game_session.opt.wasm")
            .with_id(SESSION_PROGRAM_ID)
            .build(&system);

    let wordle_program: Program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/debug/wordle.opt.wasm")
            .with_id(WORDLE_PROGRAM_ID)
            .build(&system);

    system.mint_to(USER1, 12345678987654321258);
    wordle_program.send_bytes(USER1, []);
    system.run_next_block();
    session_program.send(USER1, wordle_program.id());
    system.run_next_block();
    session_program.send(USER1, SessionAction::StartGame { user: USER1.into() });
    system.run_next_block();
    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "hello".into(),
        },
    );
    system.run_next_block();
    system.run_to_block(210);

    session_program.send(
        USER1,
        SessionAction::CheckWord {
            user: USER1.into(),
            word: "hello".into(),
        },
    );

    let state: Session = session_program.read_state(()).unwrap();
    assert_eq!(
        state.session_status,
        SessionStatus::GameEnded {
            result: GameResult::Lose
        }
    );
}
