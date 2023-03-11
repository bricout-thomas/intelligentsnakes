use std::env;
use bracket_terminal::prelude::*;

mod game; use game::State;

const HELP_MESSAGE: &str = "
Usage: intelligentsnakes [OPTIONS] <FILENAME>

A game with the original snake game feel and evolution!
using bracket_terminal as \"game engine\"
doesn't suppot reading state from file yet

Options:
    -h display this help message
    -v get current version
    -p spawn human controllable player
";

const VERSION_MESSAGE: &str = "running intelligent snakes lpha version, subject to constant changes";

fn main() {
    // start logic, argument parsing
    let mut args = env::args();
    let _runpath = args.next().unwrap();
    let mut filepath: String = String::new();
    let mut player = false;
    // read arguments
    'argread: while let Some(arg) = args.next() {
        match arg.as_str() {
            "-v" => { println!("{}", VERSION_MESSAGE) },
            "-h" => { println!("{}", HELP_MESSAGE) },
            "-p" => { player = true; },
            _ => { filepath = arg; break 'argread }
        }
    }
    // detect filepath not being the last argument but an unknown parameter
    if let Some(arg) = args.next() {
        println!("unknown argument: '{}' or '{}'", filepath, arg);
        panic!("Usage: intelligentsnakes [OPTIONS] <FILENAME>, see more using intelligentsnakes -h");
    }

    // setup gameloop, gamestate and terminal
    let context = BTermBuilder::simple80x50()
        .with_title("intelligent snakes")
        .with_fps_cap(10.)
        .build().expect("error setting up bterm context");
    let gs = State::new(player);
    main_loop(context, gs).unwrap();
}