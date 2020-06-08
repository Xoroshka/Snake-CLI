use device_query::{DeviceState, Keycode};
use std::{thread, time};
mod lib;
const ITERMS: u64 = 800;
const KEYMS: u64 = 75;

fn main() {
  let mut game = lib::Game::new(20, 20);
  let dev_state = DeviceState::new();
  let delay = time::Duration::from_millis(KEYMS/2);
  loop {
    let mut key = Keycode::W;
    for _i in 0..ITERMS/KEYMS {
      thread::sleep(delay);
      if let Some(res) = lib::get_key(&dev_state) {
        key = res;
      } else {()}
      thread::sleep(delay);
    }
    if key == Keycode::Escape {
      break;
    }
    game.change_dir(key);
    game.next_iter();
    if game.is_over() {
      print!("GAME OVER");
      break;
    }
    game.render();
  }
}
