use crossterm::{
    cursor::{MoveTo, Hide, Show},
    event::{self, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use rand::Rng;
use std::{
    collections::HashSet,
    error::Error,
    io::{stdout, Write},
    thread,
    time::Duration,
};

enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}

struct Player {
    alive: bool,
    x: u16,
    y: u16,
    score: u16
}

impl Player {

    fn new() -> Player {
        Player {
            alive: true,
            x: 10,
            y: 10,
            score: 0
        }
    }

    fn move_player(&mut self, direction: Direction) {
        
    }

    fn player_score(&mut self) {
        self.score += 1;
    }
}

// struct Meteor {
//     id: u8,
//     x: u16,
//     y: u16
// }

// impl Meteor {
//     fn new(screen_x, screen_y) {
//         let rng = rand::thread_rng();
//         let meteor_x = rng.gen_range(1..=screen_x);
//         let meteor_y = rng.gen_range(1..=screen_y);

//     }
// }

fn show_entity(x: u16, y: u16, entity: &str, color: Color) {
    let mut stdout = stdout();
     stdout.execute(MoveTo(x, y)).unwrap();
     stdout.execute(SetForegroundColor(color)).unwrap();
     print!("{}", entity);
     stdout.execute(ResetColor).unwrap();
}

fn main() {
    let mut stdout = stdout();
    stdout.execute(Hide);
    terminal::enable_raw_mode();

    let mut player = Player::new();    


    loop {
        stdout.execute(Clear(ClearType::All));

        show_entity(0, 0, &format!("Score: {}", player.score), Color::White);

        if let Ok(true) = event::poll(Duration::from_millis(50)) {
            if let Ok(event::Event::Key(KeyEvent { code, .. })) = event::read() {
                match code {
                    // KeyCode::Char('a') => {
                    //     if pos_x > 1 {
                    //         pos_x -= 1;
                    //     }
                    // }
                    // KeyCode::Char('d') => {
                    //     if pos_x < MAX_X {
                    //         pos_x += 1;
                    //     }
                    // }
                    // KeyCode::Char(' ') => {
                    //     if !bullet.0 {
                    //         bullet = (true, pos_x, pos_y - 1);
                    //     }
                    // }
                    KeyCode::Char('w') => {
                        player.move_player(Direction::NORTH);
                    }
                    KeyCode::Char('s') => {
                        player.move_player(Direction::SOUTH);
                    }
                    KeyCode::Char('d') => {
                        player.move_player(Direction::EAST);
                    }
                    KeyCode::Char('a') => {
                        player.move_player(Direction::WEST);
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        player.score += 1;
        thread::sleep(Duration::from_millis(1000));
    }

}
