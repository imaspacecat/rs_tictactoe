use std::io;
use crate::game::Game;
use crate::game::GameState::Playing;

mod board;
mod game;
mod player;


// converts a single integer from 1-9 into a 2d array position (x, y)
fn process_input(i: usize) -> (usize, usize) {
    let n = i - 1;
    ((n - n % 3) / 3, n % 3)
}

fn main() {
    let mut game = Game::new();
    let mut input = String::new();
    print!("{}[2J", 27 as char);
    println!("{}'s turn\nplease input a number between 1-9 corresponding to the desired square:",
             game.current_player);
    println!("{}", game.board);
    while game.get_game_status() == Playing {
        io::stdin().read_line(&mut input).expect("failed to read line");
        let i_input: usize = input.trim().parse().unwrap_or(0);
        input.clear();
        if !(1..=9).contains(&i_input) {
            println!("please enter a number from 1-9");
            continue;
        }

        let (x, y) = process_input(i_input);

        print!("{}[2J", 27 as char);
        match game.do_turn(x, y) {
            Ok(()) => (),
            Err(err) => println!("{}", err)
        }

        println!("{}", game.board);
        println!("{}'s turn\nplease input a number between 1-9 corresponding to the desired square:",
                 game.current_player);
    }

    println!("{} won!", game.current_player.opposite())
}
