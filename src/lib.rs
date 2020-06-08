use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::{
  distributions::{Distribution, Standard},
  Rng,
};

const CSI: &str = "\x1b[";
const MINSIZEX: u16 = 10;
const MINSIZEY: u16 = 10;

pub fn get_key(dev_state: &DeviceState) -> Option<Keycode> {
        let keys: Vec<Keycode> = dev_state.get_keys();
        for key in keys.iter() {
            match key {
                Keycode::A |
                Keycode::D |
                Keycode::W |
                Keycode::Escape => {
                    return Some(key.clone())
                }
                _ => ()
            }
        }
        return None
}

pub fn get_console_size() -> Option<Pair> {
  if let Some((terminal_size::Width(x), terminal_size::Height(y)))
    = terminal_size::terminal_size() {
    if x > MINSIZEX*2 && y > MINSIZEY{
      Some(Pair{x: (x/2) as usize, y: (y-1) as usize})
    } else {
      None
    }
  } else {
    None
  }
}

#[derive(Copy, Clone)]
pub struct Pair {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
enum Direction {
  Up,
  Left,
  Right,
  Down,
}

impl Distribution<Direction> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
      match rng.gen_range(0, 4) {
          0 => Direction::Up,
          1 => Direction::Left,
          2 => Direction::Right,
          _ => Direction::Down,
      }
  }
}

#[derive(Copy, Clone)]
enum Object {
  Border,
  Snake,
  Head,
  GameOverHead,
  Food,
  Empty,
}

impl Object {
  fn color(&self) -> String {
    match self {
      Object::Border => CSI.to_owned() + "47;1m  " + CSI + "0m",
      Object::Snake => CSI.to_owned() + "47;1m  " + CSI + "0m",
      Object::Head => CSI.to_owned() + "42;1m  " + CSI + "0m",
      Object::GameOverHead => CSI.to_owned() + "41;1m  " + CSI + "0m",
      Object::Food => CSI.to_owned() + "41;1m  " + CSI + "0m",
      Object::Empty => CSI.to_owned() + "40m  " + CSI + "0m",
    }
  }
}

pub struct Game {
    snake: Vec<Pair>,
    dir: Direction,
    food: Pair,
    grid: Pair,
    game_over: bool,
}


impl Game {
  pub fn new(grid: Pair) -> Game {
    let mut game = Game{
      snake: vec![Pair{x: grid.x/2, y: grid.y/2}],
      dir: rand::random(),
      food: Pair{x: 1, y: 1},
      grid,
      game_over: false,
    };
    game.next_food();
    game
  }


  pub fn next_food(&mut self){
    let mut vars: Vec<Pair> = Vec::new();
    for y in 1..self.grid.y-1 {
      'xloop: for x in 1..self.grid.x-1 {
        for p in &self.snake {
          if p.x == x && p.y == y {
            continue 'xloop;
          }
          vars.push(Pair{x, y})
        }
      }
    }
    self.food = vars[
      rand::thread_rng().gen_range(0, vars.len())
    ];
    
  }

  pub fn change_dir(&mut self, key: Keycode){
    match (key, self.dir) {
      (Keycode::D, Direction::Left) |
      (Keycode::A, Direction::Right) => self.dir = Direction::Up,

      (Keycode::D, Direction::Up) |
      (Keycode::A, Direction::Down) => self.dir = Direction::Right,

      (Keycode::D, Direction::Right) |
      (Keycode::A, Direction::Left) => self.dir = Direction::Down,

      (Keycode::D, Direction::Down) |
      (Keycode::A, Direction::Up) => self.dir = Direction::Left,

      (_, _) => ()
    }
  }

  pub fn next_iter(&mut self) {
    let mut head = self.snake.last().unwrap().clone();
    match self.dir {
      Direction::Up => head.y -= 1,
      Direction::Right => head.x += 1,
      Direction::Down => head.y += 1,
      Direction::Left => head.x -= 1,
    }
    if head.x == self.food.x && head.y == self.food.y {
      self.next_food();
    } else {
      self.snake.remove(0);
    }
    if head.x == 0 || head.x == self.grid.x-1 ||
      head.y == 0 || head.y == self.grid.y-1 {
      self.game_over = true;
    }
    if self.snake.len() > 4 {
      for i in 0..self.snake.len()-4 {
        if self.snake[i].x == head.x &&
        self.snake[i].y == head.y {
          self.game_over = true;
          break
        }
      }
    }
    self.snake.push(head);
  }

  pub fn render(&self) {
    let mut screen = vec![
      vec![Object::Empty; self.grid.x]; self.grid.y
    ];

    for y in 1..self.grid.y - 1 {
      screen[y][0] = Object::Border;
      screen[y][self.grid.x-1] = Object::Border;
    }
    screen[0] = vec![Object::Border; self.grid.x];
    screen[self.grid.y-1] = vec![Object::Border; self.grid.x];

    for part in &self.snake {
      screen[part.y][part.x] = Object::Snake
    }
    if self.game_over {
      screen[self.snake.last().unwrap().y]
        [self.snake.last().unwrap().x] = Object::GameOverHead;
    } else {
      screen[self.snake.last().unwrap().y]
        [self.snake.last().unwrap().x] = Object::Head;
    }
    screen[self.food.y][self.food.x] = Object::Food;
    let mut frame = CSI.to_owned() + "2J"
      + CSI + "1;1H" + CSI + "?25l";
    for str in screen {
      for cell in str {
        frame += &cell.color()[..];
      }
      frame += "\n";
    }
    print!("{}", frame);
    print!("{}1;1H", CSI);
  }

  pub fn is_over(&self) -> bool {
    self.game_over
  }
}
