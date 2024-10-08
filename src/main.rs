use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, KeyCode, KeyEvent},
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use rand::Rng;
use std::{
     io::{stdout, Write}, time::{Duration, Instant}
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

    fn new(max_x: u16, max_y: u16) -> Player {
        Player {
            alive: true,
            x: (max_x + 1) / 2,
            y: (max_y + 1) /2,
            score: 0
        }
    }

    fn move_player(&mut self, direction: Direction, max_x: u16, max_y: u16) {
        match direction {
            Direction::North => {
                if self.y >= 1 {
                    self.y -= 1;
                } else {
                    self.y = 1;
                }
            }
            Direction::East => {
                if self.x <= max_x{
                    self.x += 1;
                } else {
                    self.x = max_x;
                }
            },
            Direction::South => {
                if self.y <= max_y {
                    self.y += 1;
                } else {
                    self.y = max_y
                }
            }
            Direction::West => {
                if self.x >= 1 {
                    self.x -= 1;
                } else {
                    self.x = 1
                }
            }
        }
    }

    fn add_score(&mut self) {
        self.score += 1;
    }
}


struct Meteor {
    x: u16,
    y: u16
}

impl Meteor {
    fn new(max_x: u16, max_y: u16) -> Meteor {
        let mut rng = rand::thread_rng();
        let meteor_x = rng.gen_range(1..=max_x);
        let meteor_y = rng.gen_range(1..=max_y);
        Meteor {
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
    stdout.execute(Hide).unwrap();
    terminal::enable_raw_mode().unwrap();
    
    let max_x: u16 = 35;
    let max_y: u16 = 21;

    let mut player = Player::new(max_x, max_y);    
    
    let mut meteor_vec: Vec<Meteor> = Vec::new();
    
    let mut score_time = Instant::now();
    let mut summon_met_time = Instant::now();
    let mut met_time = Instant::now();
    
    loop {

        if player.alive == false {
            stdout.execute(Clear(ClearType::All)).unwrap();
            show_entity(5, 10, "You DEAD ", Color::DarkRed);
            show_entity(5, 11, &format!("Score: {}", player.score), Color::White);
            break;
        }
        if player.score == 60 {
            stdout.execute(Clear(ClearType::All)).unwrap();
            show_entity(5, 10, "You SURVIVED ", Color::Cyan);
            show_entity(5, 11, &format!("Score: {}", player.score), Color::White);
            break;
        }

        if summon_met_time.elapsed() >= Duration::new(1, 0) {
            meteor_vec.push(Meteor::new(max_x, max_y));
            summon_met_time = Instant::now();
        }
        
        
        for met in &mut meteor_vec {
            if player.x == met.x && player.y == met.y {
                player.alive = false;
            }
            
            if met_time.elapsed() >= Duration::new(0, 0) && met_time.elapsed() <= Duration::new(1, 0) {
                show_entity(met.x, met.y, "@", Color::Red);
            }
            met_time = Instant::now();
        }
        
        show_entity(player.x, player.y, "■", Color::Blue);
        
        show_entity(0, 0, &format!("Score: {}", player.score), Color::White);
        
        if let Ok(true) = event::poll(Duration::from_millis(1)) {
            if let Ok(event::Event::Key(KeyEvent { code, .. })) = event::read() {
                match code {
                    KeyCode::Char('w') => {
                        player.move_player(Direction::North, max_x, max_y);
                    }
                    KeyCode::Char('s') => {
                        player.move_player(Direction::South, max_x, max_y);
                    }
                    KeyCode::Char('d') => {
                        player.move_player(Direction::East, max_x,  max_y);
                    }
                    KeyCode::Char('a') => {
                        player.move_player(Direction::West, max_x, max_y);
                    }
                    KeyCode::Esc => {
                        stdout.execute(Clear(ClearType::All)).unwrap();
                        show_entity(5, 10, "You QUIT", Color::DarkRed);
                        show_entity(5, 11, &format!("Score: {}", player.score), Color::White);
                        break;
                        
                    }
                    
                    _ => {}
                }
            }
        }
        
        
        stdout.flush().unwrap();
        
        if score_time.elapsed() >= Duration::new(1,0) {
            player.add_score();
            score_time = Instant::now();
        }
        stdout.execute(Clear(ClearType::All)).unwrap();
    }

}

// Github: https://github.com/Pottarr/meteor-strike