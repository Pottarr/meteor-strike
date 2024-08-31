use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, KeyCode, KeyEvent},
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use rand::Rng;
use std::{
     io::{stdout, Write}, thread, time::{Duration, Instant}
};

enum Direction {
    North,
    South,
    East,
    West
}

// enum MeteorState {
//     Ground,
//     Falling
// }

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
    // id: u8,
    x: u16,
    y: u16,
    // state: MeteorState
}

impl Meteor {
    // fn new(meteor_id: u8, max_x: u16, max_y: u16) -> Meteor {
    fn new(max_x: u16, max_y: u16) -> Meteor {
        let mut rng = rand::thread_rng();
        let meteor_x = rng.gen_range(1..=max_x);
        let meteor_y = rng.gen_range(1..=max_y);
        Meteor {
            // id: meteor_id,
            x: meteor_x,
            y: meteor_y,
            // state: MeteorState::Falling
        }
    }
    // fn falling(&mut self, mut met_time: Instant) {
    //     if met_time.elapsed() >= Duration::new(0, 0) && met_time.elapsed() <= Duration::new(1, 0) {
    //         show_entity(self.x, self.y, "@", Color::White);
    //     } else if met_time.elapsed() >= Duration::new(1, 0) && met_time.elapsed() <= Duration::new(2, 0) {
    //         show_entity(self.x, self.y, "@", Color::Yellow);
    //     } else if met_time.elapsed() >= Duration::new(2, 0) && met_time.elapsed() <= Duration::new(5, 0) {
    //         show_entity(self.x, self.y, "@", Color::Red);
    //     } else {}
    //     met_time = Instant::now();
    //     // thread::sleep(Duration::from_secs(3));
    // }
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
    
    let mut player = Player::new();    
    let max_x: u16 = 40;
    let max_y: u16 = 45;
    
    let mut meteor_vec: Vec<Meteor> = Vec::new();
    
    let mut score_time = Instant::now();
    let mut summon_met_time = Instant::now();
    let mut met_time = Instant::now();
    
    loop {

        if player.alive == false || player.score == 60 {
            break;
        }

        if summon_met_time.elapsed() >= Duration::new(1, 0) {
            meteor_vec.push(Meteor::new(max_x, max_y));
            // meteor_vec.push(Meteor::new(meteor_vec.len() as u8 + 1, max_x, max_y));
            summon_met_time = Instant::now();
        }
        
        stdout.execute(Clear(ClearType::All)).unwrap();
        
        for met in &mut meteor_vec {
            if player.x == met.x && player.y == met.y {
                player.alive = false;
            }
            
            if met_time.elapsed() >= Duration::new(0, 0) && met_time.elapsed() <= Duration::new(1, 0) {
                show_entity(met.x, met.y, "@", Color::Red);
            }
            // } else if met_time.elapsed() >= Duration::new(1, 0) && met_time.elapsed() <= Duration::new(2, 0) {
                //     show_entity(met.x, met.y, "@", Color::Yellow);
                // } else if met_time.elapsed() >= Duration::new(2, 0) && met_time.elapsed() <= Duration::new(5, 0) {
                    //     show_entity(met.x, met.y, "@", Color::Red);
                    // } else {}
                    met_time = Instant::now();
                    
                    // met.falling(met_time);
                    // met_time = Instant::now();
                }
                
                show_entity(0, 0, &format!("Score: {}", player.score), Color::White);
        
        show_entity(0, 0, &format!("Score: {}", player.score), Color::White);
        show_entity(player.x, player.y, "|", Color::Blue);
        
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
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        
        
        stdout.flush().unwrap();
        
        if score_time.elapsed() >= Duration::new(1,0) {
            player.add_score();
            score_time = Instant::now();
        }
    }
    thread::sleep(Duration::from_millis(1000));
    stdout.execute(Clear(ClearType::All)).unwrap();
    
    // let start = Instant::now();
    
}
