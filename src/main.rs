use std::{env, process::exit};
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
    -wh <SIZE> change the height of the world
    -ww <SIZE> change the width of the world
    -sh <SIZE> change the height of the screen
    -sw <SIZE> change the width of the screen
    -t set the world to be a taurus
";

const VERSION_MESSAGE: &str = "running intelligent snakes lpha version, subject to constant changes";

fn main() {
    // start logic, argument parsing
    let mut args = env::args();
    let _runpath = args.next().unwrap();
    let mut filepath: String = String::new();
    
    // define overwritable by an option world arguments
    let mut player = false;
    let mut worldheight = 256;
    let mut worldwidth = 256;
    let mut screenheight = 128;
    let mut screenwidth = 128;
    let mut taurus = false;

    // read arguments
    'argread: while let Some(arg) = args.next() {
        match arg.as_str() {
            "-v" => { println!("{}", VERSION_MESSAGE) },
            "-h" => { println!("{}", HELP_MESSAGE); exit(0); },
            "-p" => { player = true; },
            "-wh" => { worldheight  = args.next().expect("Expected argument after option -wh").parse().expect("Expected an integer value"); },            
            "-ww" => { worldwidth   = args.next().expect("Expected argument after option -ww").parse().expect("Expected an integer value"); },
            "-sh" => { screenheight = args.next().expect("Expected argument after option -sh").parse().expect("Expected an integer value"); },
            "-sw" => { screenwidth  = args.next().expect("Expected argument after option -sw").parse().expect("Expected an integer value"); },
            "-t" => { taurus = true; }
            _ => { filepath = arg; break 'argread }
        }
    }
    // detect filepath not being the last argument but an unknown parameter
    if let Some(arg) = args.next() {
        println!("unknown argument: '{}' or '{}'", filepath, arg);
        panic!("Usage: intelligentsnakes [OPTIONS] <FILENAME>, see more using intelligentsnakes -h");
    }

    // detect screen size bigger than world size
    if screenheight > worldheight || screenwidth > worldwidth {
        panic!("screen is bigger than the world in some dimension");
    }

    // setup gameloop, gamestate and terminal
    let context = BTermBuilder::simple(screenwidth, screenheight).unwrap()
        .with_title("intelligent snakes")
        .with_fps_cap(140.)
        .build().expect("error setting up bterm context");
    let gs = State::new(player, worldheight, worldwidth, screenheight, screenwidth, taurus);
    main_loop(context, gs).unwrap();
}