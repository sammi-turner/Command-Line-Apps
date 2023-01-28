mod rbp;
mod yahtzee;

use rbp::{seed, vt_open, vt_close};
use yahtzee::{Game};

fn main() {
    seed();
    vt_open();

    let mut game = Game {
        dice:[0; 7],
        scores:[0; 13]
    };

    Game::main_loop(&mut game);
    vt_close("\n     PRESS ANY KEY TO CLOSE");
}
