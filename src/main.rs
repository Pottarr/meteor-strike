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
    North,
    South,
    East,
    West
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
        match direction {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
            _ => {}
        }
    }

    fn player_score(&mut self) {
        self.score += 1;
    }
}

struct Meteor {
    id: u8,
    x: u16,
    y: u16
}

impl Meteor {
    fn new(meteor_id: u8, max_x: u16, max_y: u16) -> Meteor {
        let mut rng = rand::thread_rng();
        let meteor_x = rng.gen_range(1..=max_x);
        let meteor_y = rng.gen_range(1..=max_y);
        Meteor {
            id: meteor_id,
            x: meteor_x,
            y: meteor_y,
        }
    }
}

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
    let max_x: u16 = 40;
    let max_y: u16 = 45;

    loop {
        let mut meteor_vec: Vec<Meteor> = Vec::new();
        if meteor_vec.len() < 10 {
            meteor_vec.push(Meteor::new(meteor_vec.len() as u8 + 1, max_x, max_y))
        }
        stdout.execute(Clear(ClearType::All));
        
        show_entity(0, 0, &format!("Score: {}", player.score), Color::White);
        show_entity(player.x, player.y, "|", Color::Blue);
        
        for i in &meteor_vec {
            show_entity(i.x, i.y, "@", Color::Red);
        }

        if let Ok(true) = event::poll(Duration::from_millis(30)) {
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
                        player.move_player(Direction::North);
                    }
                    KeyCode::Char('s') => {
                        player.move_player(Direction::South);
                    }
                    KeyCode::Char('d') => {
                        player.move_player(Direction::East);
                    }
                    KeyCode::Char('a') => {
                        player.move_player(Direction::West);
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        stdout.flush();
        player.score += 1;
        thread::sleep(Duration::from_millis(30));
    }

}
