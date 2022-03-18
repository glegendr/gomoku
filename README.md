# Gomoku
42 gomoku in Rust

## Dependencies

The only dependencie you need is Rust.     
To get it, use `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` or check [Rust web site](https://www.rust-lang.org/tools/install)
## Project

To start the project use:
 ```
 git clone https://github.com/glegendr/gomoku.git; cd gomoku; cargo run --release -- -v
```
## Game
The rules are simple, two players take turns placing stones of their color on an intersection of the board.      
The game ends when one player manages to align five stones or capture 10 opponent's stones.      
The game is played on a 19x19 Goban, without limit to the number of stones.      
A player that manages to align five stones only wins if the opponent can not break this alignment by capturing a pair.      
To capture a pair of your opponent's stones and remove them from the board, flank them with your own stones.      
Example:      
```
. . . .
X O O A
. . . .
```     
In this scenario, by playing in A, X captures the O pair and removes the stones from the game.      
The now-free intersections can be played on as if they were never occupied.      
It is forbidden to play a move that introduces two free-three alignments, which would guarantee a win by alignment.      
A free-three is an alignement of three stones that, if not immediately blocked, allows for an indefendable alignment of four stones      
Example:      
```
. . . . .
. X . . .
. . X . .
. . . X .
. . . . .
```     
A double-three is a move that introduces two simultaneous free-three alignments. This is an indefendable scenario.      
Example:      
```
. . . . . . . .
. X . . . . . .
. . X . . . . .
. . . . . . . .
. . . . A X X .
```     
In this scenario, by playing in A, X would introduce a double-three, therefore this is a forbidden move.      
}