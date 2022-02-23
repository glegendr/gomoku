use std::{io, time, env, process};
use std::time::{Duration};
mod board;
use board::{Board, Input};
mod error;
use error::{FlagError, PlacementError};
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

use opengl_graphics::GlyphCache;
use opengl_graphics::*;
use std::path::Path;
use graphics::*;

fn get_human_input(_player_color: Color) -> Result<Input, PlacementError> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let vec: Vec<&str> = guess.trim().split(' ').collect();
    if vec.len() != 2 {
        return Err(PlacementError::IncorrectPlacement);
    }
    for coord in vec.iter() {
        match coord.parse::<usize>() {
            Ok(_) => (),
            _ => return Err(PlacementError::IncorrectPlacement)
        }
    }
    Ok((vec[0].parse::<usize>().unwrap(), vec[1].parse::<usize>().unwrap()))
}

fn game(board: &mut Board, players: &mut Players, trees: (&mut Option<Tree>, &mut Option<Tree>), turn_count: &mut usize, depth: usize) -> bool {
    
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
        PlayerType::Human => {
            match get_human_input(players.get_current_player().get_player_color()) {
                Ok(input) => input,
                Err(e) => {
                    println!("{}", e);
                    return false;
                }
            }
        }
        PlayerType::Bot(_) => {
            match players.get_current_player().get_player_color() {
                Color::Black => {
                    let (bot_input, bot_tree) = get_bot_input(*players, &board, trees.0, depth);
                    *trees.0 = bot_tree;
                    bot_input
                },
                Color::White => {
                    let (bot_input, bot_tree) = get_bot_input(*players, &board, trees.1, depth);
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

fn game_graphic<E: GenericEvent>(board: &Board, players: &Players, mpos: [f64; 2], event: &E, view: &View, trees: (&Option<Tree>, &Option<Tree>), turn_count: &mut usize, depth: usize) -> (Option<Option<Color>>, Option<(Board, Players, (Option<Tree>, Option<Tree>))>, Option<Input>) {
    let mut option_ret = None;
    match (board.is_finished(players.get_current_player()), players.is_finished()) {
        (_, (true, Some(color))) => {
            println!("BRAVO {:?} \"{}\"", color, color);
            return (Some(Some(color)), None, None)
        },
        ((true, None), _) => {
            println!("DRAW !");
            return (Some(None), None, None)
        },
        ((true, Some(color)), _) => {
            println!("BRAVO {:?} \"{}\"", color, color);
            return (Some(Some(color)), None, None)
        },
        _ => ()
    };
    let mut new_trees: (Option<Tree>, Option<Tree>) = (None, None);
    let input = match players.get_current_player().get_player_type() {
        PlayerType::Human => get_human_input_graphic(players.get_current_player().get_player_color(), mpos, event, view),
        PlayerType::Bot(_) => {
        let now = time::Instant::now();
        let ret: (usize, usize);
        match players.get_current_player().get_player_color() {
                Color::Black => {
                    let (bot_input, bot_tree) = get_bot_input(*players, &board, trees.0, depth);
                    new_trees.0 = bot_tree;
                    ret = bot_input;
                },
                Color::White => {
                    let (bot_input, bot_tree) = get_bot_input(*players, &board, trees.1, depth);
                    new_trees.1 = bot_tree;
                    ret = bot_input;
                },
            }
        let elapsed_time = now.elapsed();
        println!("Input took {:?}.", elapsed_time);
        ret
        },
    };
    if input.0 < board.get_size() && input.1 < board.get_size() {
        let mut new_board = board.clone();
        let mut new_players = players.clone();
        match new_board.add_value(input, &mut new_players) {
            Ok(_) => {
                *turn_count += 1;
                println!("Turn: {}", *turn_count / 2);
                new_players.next_player();
                option_ret = Some((new_board, new_players, new_trees));
            },
            Err(e) => println!("{}", e)
        }
    };
    (None, option_ret, Some(input))
}

fn get_mut_last<'a, T>(list: &'a mut Vec<T>) -> &'a mut T {
    let len = list.len() - 1;
    list.get_mut(len).unwrap()
}

fn get_last<'a, T>(list: &'a Vec<T>) -> &'a T {
    let len = list.len() - 1;
    list.get(len).unwrap()
}

fn get_last_protected<'a, T>(list: &'a Vec<T>) -> Option<&'a T> {
    let len = list.len() - 1;
    list.get(len)
}

fn print_time(us: u128) -> String {
    if us > 1000000 {
        format!("{},{} s", us / 1000000, (us % 1000000) / 1000)
    } else if us > 1000 {
        format!("{},{} ms", us / 1000, us % 1000)
    } else {
        format!("{} µs", us)
    }
}

fn calc_average(durations: &Vec<u128>) -> (u128, u128) {
    let (dur1, dur2): (Vec<u128>, Vec<u128>) = durations.iter()
        .enumerate()
        .fold((Vec::new(), Vec::new()), |mut acc, (index, duration)| {
            if index % 2 == 0 {
                acc.0.push(*duration);
            } else {
                acc.1.push(*duration);
            }
            acc
        });
    let len1: u128 = match dur1.len() {
        0 => 1,
        _ => dur1.len() as u128,
    };
    let len2: u128 = match dur2.len() {
        0 => 1,
        _ => dur2.len() as u128,
    };
    (dur1.iter().fold(0, |acc, d| acc + d) / len1, dur2.iter().fold(0, |acc, d| acc + d) / len2)
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut board: Vec<Board>;
    let mut players: Vec<Players>;
    let depth: usize;
    let visual: bool;
    match leakser(&mut args[1..]) {
        Ok((s, c, r, a, v, p1, p2, d)) => {
            board = vec![Board::new(s, a, r)];
            players = vec![Players::new(p1, p2, c, r)];
            visual = v;
            depth = d;
        },
        Err((e, f)) => {
            if e != FlagError::PrintHelper && e != FlagError::PrintRules {
                if f == usize::MAX { 
                    println!("\n{} {}", format!("error:").red(), e);
                } else {
                    println!("\n{} \'{}\' {}", format!("error:").red(), args[f + 1].yellow(), e);
                }
                println!("for more information use \"cargo run -- --help\"");
            }
            process::exit(1);
        }
    };
    let mut tree_player_1: Vec<Option<Tree>> = vec![None];
    let mut tree_player_2: Vec<Option<Tree>> = vec![None];
    let mut turn_count: usize = 1;

    match visual {
        true => {
            let mut finished: Option<Option<Color>> = None;
            let view = View::new(board.get(board.len() - 1).unwrap());
            let opengl = OpenGL::V3_2;
            let settings = WindowSettings::new("Gomoku", [view.get_window_size(), view.get_window_size()])
                .graphics_api(opengl)
                .exit_on_esc(true);
            let mut window: GlutinWindow = settings.build()
                .expect("Could not create window");
            let mut events = Events::new(EventSettings::new().lazy(true));
            let mut gl = GlGraphics::new(opengl);
            let mut mpos: [f64; 2] = [0.0; 2];
            let ref mut arrows_glyph = GlyphCache::new("assets/arrows.ttf", (), TextureSettings::new()).unwrap();
            let ref mut text_glyph = GlyphCache::new("assets/AlegreyaSansSC-ExtraBold.ttf", (), TextureSettings::new()).unwrap();
            let bravo = Texture::from_path(&Path::new("./assets/bravo.png"), &TextureSettings::new()).unwrap();
            let crown = Texture::from_path(&Path::new("./assets/crown.png"), &TextureSettings::new()).unwrap();
            let robot_black = Texture::from_path(&Path::new("./assets/robot.png"), &TextureSettings::new()).unwrap();
            let robot_white= Texture::from_path(&Path::new("./assets/robot_white.png"), &TextureSettings::new()).unwrap();
            let mut last_input: Vec<Input> = Vec::new();
            let mut start_p1 = time::Instant::now();
            let mut start_p2 = time::Instant::now();
            let mut time_p1: Duration = Duration::new(0, 0);
            let mut time_p2: Duration = Duration::new(0, 0);
            let mut time_storage: Vec<u128> = Vec::new();
            while let Some(event) = events.next(&mut window) {
                if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
                    if mpos[0] > 50.0 && mpos[0] < 150.0
                        && mpos[1] > 20.0 && mpos[1] < 70.0 {
                            let mut new_board = board.pop().unwrap();
                            let mut new_players = players.pop().unwrap();
                            new_board.reset();
                            new_players.reset();
                            board = vec![new_board];
                            players = vec![new_players];
                            last_input = vec![];
                            tree_player_1 = vec![None];
                            tree_player_2 = vec![None];
                            turn_count = 1;
                            finished = None;
                            start_p1 = time::Instant::now();
                            start_p2 = time::Instant::now();
                            time_p1 = Duration::new(0, 0);
                            time_p2 = Duration::new(0, 0);
                            time_storage = Vec::new();
                    } else if mpos[0] > 200.0 && mpos[0] < 300.0
                        && mpos[1] > 20.0 && mpos[1] < 70.0 {
                            if turn_count > 1 && get_last(&players).get_player(get_last(&players).get_current_player().get_player_color().get_inverse_color()).get_player_type() == PlayerType::Human {
                                if tree_player_1.len() > 1 {
                                    tree_player_1 = (&tree_player_1[..tree_player_1.len() - 1]).to_vec();
                                }
                                if tree_player_2.len() > 1 {
                                    tree_player_2 = (&tree_player_2[..tree_player_2.len() - 1]).to_vec();
                                }
                                board = (&board[..board.len() - 1]).to_vec();
                                last_input = (&last_input[..last_input.len() - 1]).to_vec();
                                players = (&players[..players.len() - 1]).to_vec();
                                turn_count -= 1;
                                time_storage.pop();
                                finished = None;
                            } else if turn_count > 2 {
                                if tree_player_1.len() > 1 {
                                    tree_player_1 = (&tree_player_1[..tree_player_1.len() - 1]).to_vec();
                                }
                                if tree_player_2.len() > 1 {
                                    tree_player_2 = (&tree_player_2[..tree_player_2.len() - 1]).to_vec();
                                }
                                board = (&board[..board.len() - 2]).to_vec();
                                last_input = (&last_input[..last_input.len() - 2]).to_vec();
                                players = (&players[..players.len() - 2]).to_vec();
                                turn_count -= 2;
                                time_storage.pop();
                                time_storage.pop();
                                finished = None;
                            }
                            start_p1 = time::Instant::now();
                            start_p2 = time::Instant::now();
                            time_p1 = Duration::new(0, 0);
                            time_p2 = Duration::new(0, 0);
                    } else if mpos[0] > 335.0 && mpos[0] < 375.0
                        && mpos[1] > 40.0 && mpos[1] < 90.0 {
                        players = players.iter().map(|x| {let mut ret = x.clone(); ret.change_player_type(Color::Black); ret}).collect();
                    } else if mpos[0] > 435.0 && mpos[0] < 475.0
                        && mpos[1] > 40.0 && mpos[1] < 90.0 {
                            players = players.iter().map(|x| {let mut ret = x.clone(); ret.change_player_type(Color::White); ret}).collect();
                        }
                }
                if let Some(pos) = event.mouse_cursor_args() {
                    mpos = pos
                }
                if finished.is_none() {
                    match game_graphic(get_last(&board), get_last(&players), mpos, &event, &view, (get_last(&tree_player_1), get_last(&tree_player_2)), &mut turn_count, depth) {
                        (x, Some((new_board, new_players, (new_tree_1, new_tree_2))), Some(input)) => {
                            if new_players.get_current_player().get_player_color() == Color::Black {
                                time_p2 = start_p2.elapsed();
                                time_storage.push(time_p2.as_micros());
                                start_p1 = time::Instant::now();
                            } else {
                                time_p1 = start_p1.elapsed();
                                time_storage.push(time_p1.as_micros());
                                start_p2 = time::Instant::now();
                            }
                            finished = x;
                            board.push(new_board);
                            players.push(new_players);
                            last_input.push(input);
                            if let Some(tree_1) = new_tree_1 {
                                tree_player_1.push(Some(tree_1));
                            }
                            if let Some(tree_2) = new_tree_2 {
                                tree_player_2.push(Some(tree_2));
                            }
                        }
                        (x, _, _) => finished = x
                    }
                }
                if let Some(args) = event.render_args() {
                    gl.draw(args.viewport(), |context, graphics| {
                        clear(view.get_background_color(), graphics);
                        view.draw(get_last(&board), get_last(&players), &context, graphics, mpos, finished.is_some(), get_last_protected(&last_input));
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                            "M", // Reset
                            arrows_glyph,
                            &context.draw_state,
                            context.transform
                                .trans(120.0, 30.0)
                                .flip_hv(),
                            graphics
                        ).unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                            "P", // Undo
                            arrows_glyph,
                            &context.draw_state,
                            context.transform
                                .trans(267.0, 20.0)
                                .flip_hv()
                                .rot_deg(-25.0),
                            graphics
                        ).unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                            &get_last(&players).get_player(Color::Black).get_player_captured().to_string(),
                            text_glyph,
                            &context.draw_state,
                            context.transform
                                .trans(390.0, 60.0),
                            graphics
                        ).unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                            &get_last(&players).get_player(Color::White).get_player_captured().to_string(),
                            text_glyph,
                            &context.draw_state,
                            context.transform
                                .trans(490.0, 60.0),
                            graphics
                        ).unwrap();
                        let elapsed_time = if finished.is_some() {
                            (time_p1.as_micros(), time_p2.as_micros())
                        } else if get_last(&players).get_current_player().get_player_color() == Color::Black {
                            (start_p1.elapsed().as_micros(), time_p2.as_micros())
                        } else {
                            (time_p1.as_micros(), start_p2.elapsed().as_micros())
                        };
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 12).draw(
                            &print_time(elapsed_time.0),
                            text_glyph,
                            &context.draw_state,
                            context.transform
                                .trans(575.0, 55.0),
                            graphics
                        ).unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 12).draw(
                            &print_time(calc_average(&time_storage).0),
                            text_glyph,
                            &context.draw_state,
                            context.transform
                                .trans(575.0, 75.0),
                            graphics
                        ).unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 12).draw(
                            &print_time(elapsed_time.1),
                            text_glyph,
                            &context.draw_state,
                            context.transform
                                .trans(825.0, 55.0),
                            graphics
                        ).unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 12).draw(
                            &print_time(calc_average(&time_storage).1),
                            text_glyph,
                            &context.draw_state,
                            context.transform
                                .trans(825.0, 75.0),
                            graphics
                        ).unwrap();
                        text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32).draw(
                            &format!("[ Turn: {} ]", turn_count / 2),
                            text_glyph,
                            &context.draw_state,
                            context.transform
                                .trans(650.0, 60.0),
                            graphics
                        ).unwrap();
                        if get_last(&players).get_player(Color::Black).get_player_type() == PlayerType::Human {
                            view.draw_stone(&context, graphics, view.black_color(false), [350.0, 40.0, 15.0, 15.0], 25.0); // 13
                        } else {
                            image(&robot_black, context.transform.trans(330.0, 20.0), graphics);
                        }
                        if get_last(&players).get_player(Color::White).get_player_type() == PlayerType::Human {
                            view.draw_stone(&context, graphics, view.white_color(false), [450.0, 40.0, 15.0, 15.0], 25.0); // 26.73
                        } else {
                            image(&robot_white, context.transform.trans(430.0, 20.0), graphics);
                        }
                        if finished.is_some() {
                            image(&bravo, context.transform.trans(20.0, 65.0), graphics);
                            if let Some(winner_color) = finished {
                                match winner_color {
                                    Some(Color::Black) => image(&crown, context.transform.trans(330.0, 7.0), graphics),
                                    Some(Color::White) => image(&crown, context.transform.trans(430.0, 7.0), graphics),
                                    _ => ()
                                }
                            }
                        }
                    });
                }
            }
        },
        _ => {
            loop {
                if game(get_mut_last(&mut board), get_mut_last(&mut players), (get_mut_last(&mut tree_player_1), get_mut_last(&mut tree_player_2)), &mut turn_count, depth) {
                    println!("{}", get_last(&board));
                    println!("{:?}", get_last(&players));
                    break;
                }
                println!("{}", get_last(&board));
                println!("{:?}", get_last(&players));
            }
            
        }
    }
}
