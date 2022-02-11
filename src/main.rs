extern crate sdl2;

mod game;
mod ui;
mod anvil;

use game::*;
use ui::*;
use anvil::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::EventPump;
use sdl2::render::Texture;

use std::path::Path;
use std::time::Duration;

pub struct Controls {
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
  pub enter: bool,
}

impl Controls {
  pub fn new() -> Controls {
    Controls { up: false, down: false, left: false, right: false, enter: false }
  }
}

fn max(x: i32, y: i32) -> i32 {
  if x > y {
    return x;
  } else {
    return y;
  }
}

fn min(x: i32, y: i32) -> i32 {
  if y > x {
    return x;
  } else {
    return y;
  }
}

pub fn update_player_rect(c: &Controls, r: &mut Rect) {
  if c.up {
    r.y -= 7;
  }
  if c.down {
    r.y += 7;
  }
  if c.left {
    r.x -= 7;
  }
  if c.right {
    r.x += 7;
  }

  // Clamp
  r.y = max(r.y, 60);
  r.x = min(r.x, 560);
  r.x = max(r.x, 0);
  r.y = min(r.y, 440);
}

fn handle_events(controls: &mut Controls, event_pump: &mut EventPump) -> bool {
  for event in event_pump.poll_iter() {
    match event {
      Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          return false;
        },
        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
          controls.up = true;
        },
        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
          controls.down = true;
        },
        Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
          controls.left = true;
        },
        Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
          controls.right = true;
        },
        Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
          controls.enter = true;
        },
        Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
          controls.up = false;
        },
        Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
          controls.down = false;
        },
        Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
          controls.left = false;
        },
        Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
          controls.right = false;
        },
        Event::KeyUp { keycode: Some(Keycode::Return), .. } => {
          controls.enter = false;
        },
        _ => {}
    }
  }
  true
}

pub fn move_to_furnace(game: &mut Game) {
  // Pick what to move
  let ret = display_inventory(game, Some(InventoryMode::Select));
  
  // Move it
  if let Some(i) = ret {
    game.state.inventory[i].location = Location::Forge;
  }
}

pub fn update_temp(game: &mut Game) {
  for mut item in game.state.inventory.iter_mut() {
      match item.location {
        Location::Forge => item.temp += 1,
        _ => ()
      }
  }
}

pub fn main() {
  let mut controls = Controls::new();

  let mut game = Game::new();

  let tc = game.canvas.texture_creator();

  let floor: Texture = tc.load_texture(Path::new("assets/Floor.png")).unwrap();
  let anvil: Texture = tc.load_texture(Path::new("assets/Anvil.png")).unwrap();
  let forge: Texture = tc.load_texture(Path::new("assets/Forge.png")).unwrap();
  let desk: Texture = tc.load_texture(Path::new("assets/Desk.png")).unwrap();
  let p: Texture = tc.load_texture(Path::new("assets/Player.png")).unwrap();


  let anvil_rect = Rect::new(400, 300, 120, 120);
  let forge_rect = Rect::new(250, -100, 200, 300);
  let desk_rect = Rect::new(15, 160, 150, 300);
  let mut p_rect = Rect::new(280, 220, 120, 120);

  game.canvas.set_draw_color(Color::RGB(0, 255, 255));
  game.canvas.clear();
  game.canvas.present();

  // Track number of frames run and whether to keep running
  let mut i = 0;
  let mut run = true;
  while run {
    i = (i + 1) % 255;
    game.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    game.canvas.clear();

    // Update player
    update_player_rect(&controls, &mut p_rect);

    // Update items
    update_temp(&mut game);

    // Handle events
    run = handle_events(&mut controls, &mut game.event_pump); 

    // Interact button
    if controls.enter {
      controls.enter = false;
      if let Some(_) = p_rect.intersection(anvil_rect) {
        // Load font

        // Temp until I change run_anvil
        run_anvil(&mut game); 
        
      } else if let Some(_) = p_rect.intersection(desk_rect) {
        // Test inventory
        display_inventory(&mut game, None);      
      } else if let Some(_) = p_rect.intersection(forge_rect) {
        // Choose what to put in
        move_to_furnace(&mut game);
      }
        
    }

    // Draw images
    game.canvas.copy(&floor, None, None).unwrap();
    game.canvas.copy(&anvil, None, Some(anvil_rect)).unwrap();
    game.canvas.copy(&forge, None, Some(forge_rect)).unwrap();
    game.canvas.copy(&desk, None, Some(desk_rect)).unwrap();
    game.canvas.copy(&p, None, Some(p_rect)).unwrap();

    // Borrow error? (Rect implements Copy)
    game.canvas.copy(&p, None, Some(p_rect)).unwrap();

    // Update
    game.canvas.present();

    // Sleep
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
