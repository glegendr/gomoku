use std::{io, time, env, process};
mod board;
use board::{Board, Input, Tile};
mod error;
use error::{FlagError};
mod color;
use color::{Color};
mod players;
use players::*;
mod algo;
use algo::{get_bot_input, Tree};
mod leakser;
use leakser::{leakser};
mod heuristic;
mod matching_cases;
mod view;
use view::{View};

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate colored;

use piston::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use graphics::{clear};
use colored::Colorize;


fn get_human_input(_player_color: Color) -> Input {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let vec: Vec<i32> = guess.trim().split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    (vec[0] as usize, vec[1] as usize)
}

fn game(board: &mut Board, players: &mut Players, trees: (&mut Option<Tree>, &mut Option<Tree>), turn_count: &mut usize) -> bool{
    
    match (board.is_finished(players.get_current_player()), players.is_finished()) {
        (_, (true, Some(color))) => {
            println!("BRAVO {:?} \"{}\"", color, color);
            return true;
        },
        ((true, None), _) => {
            println!("DRAW !");
            return true;
        },
        ((true, Some(color)), _) => {
            println!("BRAVO {:?} \"{}\"", color, color);
            return true;
        },
        _ => ()
    };
    
    let now = time::Instant::now();
    let input = match players.get_current_player().get_player_type() {
        PlayerType::Human => get_human_input(players.get_current_player().get_player_color()),
        PlayerType::Bot => {
            match players.get_current_player().get_player_color() {
                Color::Black => {
                    let (bot_input, bot_tree) = get_bot_input(&players, &board, trees.0);
                    *trees.0 = bot_tree;
                    bot_input
                },
                Color::White => {
                    let (bot_input, bot_tree) = get_bot_input(&players, &board, trees.1);
                    *trees.1 = bot_tree;
                    bot_input
                },
            }
        },
    };
    let elapsed_time = now.elapsed();
    println!("Input took {:?}.", elapsed_time);
    match board.add_value(input, players) {
        Ok(_) => {
            *turn_count += 1;
            println!("Turn: {}", *turn_count / 2);
            players.next_player()
        },
        Err(e) => println!("{}", e)
    };
    false
}


fn get_human_input_graphic<E: GenericEvent>(_player_color: Color, mpos: [f64; 2], event: &E, view: &View) -> Input {
    if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
        if mpos[0] > view.get_grid_start() && mpos[0] < view.get_grid_end()
            && mpos[1] > view.get_grid_start() && mpos[1] < view.get_grid_end() {
            return (
                (mpos[0] as usize - view.get_grid_start() as usize) / view.get_cell_size() as usize,
                (mpos[1] as usize - view.get_grid_start() as usize) / view.get_cell_size() as usize
            )
        }
    }
    (usize::MAX, usize::MAX)
}

fn game_graphic<E: GenericEvent>(board: &mut Board, players: &mut Players, mpos: [f64; 2], event: &E, view: &View, trees: (&mut Option<Tree>, &mut Option<Tree>), turn_count: &mut usize) -> bool {
    match (board.is_finished(players.get_current_player()), players.is_finished()) {
        (_, (true, Some(color))) => {
            println!("BRAVO {:?} \"{}\"", color, color);
            // process::exit(1);
            return true
        },
        ((true, None), _) => {
            println!("DRAW !");
            return true
            // process::exit(1);
        },
        ((true, Some(color)), _) => {
            println!("BRAVO {:?} \"{}\"", color, color);
            return true
            // process::exit(1);    
        },
        _ => ()
    };
    let input = match players.get_current_player().get_player_type() {
        PlayerType::Human => get_human_input_graphic(players.get_current_player().get_player_color(), mpos, event, view),
        PlayerType::Bot => {
        let now = time::Instant::now();
        let ret: (usize, usize);
        match players.get_current_player().get_player_color() {
                Color::Black => {
                    let (bot_input, bot_tree) = get_bot_input(&players, &board, trees.0);
                    *trees.0 = bot_tree;
                    ret = bot_input;
                },
                Color::White => {
                    let (bot_input, bot_tree) = get_bot_input(&players, &board, trees.1);
                    *trees.1 = bot_tree;
                    ret = bot_input;
                },
            }
        let elapsed_time = now.elapsed();
        println!("Input took {:?}.", elapsed_time);
        ret
        },
    };
    if input.0 < board.get_size() && input.1 < board.get_size() {
        match board.add_value(input, players) {
            Ok(_) => {
                *turn_count += 1;
                println!("Turn: {}", *turn_count / 2);
                players.next_player()
            },
            Err(e) => println!("{}", e)
        }
    };
    false
}


fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut board: Board;
    let mut players: Players;
    let visual: bool;
    match leakser(&mut args[1..]) {
        Ok((s, c, r, a, v, p1, p2)) => {
            board = Board::new(s, a, r);
            players = Players::new(p1, p2, c, r);
            visual = v;
        },
        Err((e, f)) => {
            if f == usize::MAX {
                println!("\n{} {}", format!("error:").red(), e);
            } else {
                println!("\n{} \'{}\' {}", format!("error:").red(), args[f + 1].yellow(), e);
            }
            if e != FlagError::PrintHelper && e != FlagError::PrintRules {
                println!("for more information use \"cargo run -- --help\"");
            }
            process::exit(1);
        }
    };
    let mut tree_player_1: Option<Tree> = None;
    let mut tree_player_2: Option<Tree> = None;
    let mut turn_count: usize = 1;

    match visual {
        true => {
            let mut finished = false;
            let view = View::new(&board);
            let opengl = OpenGL::V3_2;
            let settings = WindowSettings::new("Gomoku", [view.get_window_size(), view.get_window_size()])
                .graphics_api(opengl)
                .exit_on_esc(true);
            let mut window: GlutinWindow = settings.build()
                .expect("Could not create window");
            let mut events = Events::new(EventSettings::new().lazy(true));
            let mut gl = GlGraphics::new(opengl);
            let mut mpos: [f64; 2] = [0.0; 2];
            while let Some(event) = events.next(&mut window) {
                if let Some(pos) = event.mouse_cursor_args() {
                    mpos = pos
                }
                if !finished {
                    finished = game_graphic(&mut board, &mut players, mpos, &event, &view, (&mut tree_player_1, &mut tree_player_2), &mut turn_count);
                }
                if let Some(args) = event.render_args() {
                    gl.draw(args.viewport(), |context, graphics| {
                        clear(view.get_background_color(), graphics);
                        view.draw(&board, &players, &context, graphics, mpos)
                    });
                }
            }
        },
        _ => {
            loop {
                println!("{}", board);
                if game(&mut board, &mut players, (&mut tree_player_1, &mut tree_player_2), &mut turn_count) {
                    println!("{}", board);
                    println!("{:?}", players);
                    break;
                }
            }
            
        }
    }
}
