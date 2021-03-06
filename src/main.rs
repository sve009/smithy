extern crate sdl2;

mod anvil;
mod game;
mod ui;

use anvil::*;
use game::*;
use ui::*;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::EventPump;

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
        Controls {
            up: false,
            down: false,
            left: false,
            right: false,
            enter: false,
        }
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
            Event::Quit { .. } => {
                return false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                controls.up = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                controls.down = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                controls.left = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                controls.right = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => {
                controls.enter = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => {
                controls.up = false;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            } => {
                controls.down = false;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            } => {
                controls.left = false;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => {
                controls.right = false;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Return),
                ..
            } => {
                controls.enter = false;
            }
            _ => {}
        }
    }
    true
}

pub fn move_to_furnace(game: &mut Game) {
    let num_items = game
        .state
        .inventory
        .iter()
        .filter(|x| x.location == game::Location::Forge)
        .count() as i32;

    // Again probably add some error handling in the future
    if num_items >= game.state.upgrades.forge_space {
        display_error(game, "Not enough furnace space");
        return;
    }
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
            _ => (),
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

    let anvil_rect = Rect::new(389, 288, 120, 120);
    let forge_rect = Rect::new(319, -59, 201, 219);
    let desk_rect = Rect::new(69, 152, 125, 250);
    let mut p_rect = Rect::new(224, 178, 120, 120);

    game.canvas.set_draw_color(Color::RGB(0, 255, 255));
    game.canvas.clear();
    game.canvas.present();

    // Track number of frames run and whether to keep running
    let mut i = 0;
    let mut days = 0;
    let mut run = true;
    while run {
        // Handle time system
        i += 1;
        if i >= 3600 {
            i = 0;
            days += 1;

            // End condition (for now)
            if days == 5 {
                let m = game.state.money;
                continue_screen(&mut game, vec!["Two weeks have passed", &format!("You made {}$", m)]);
                return;
            }

            // Alert user
            display_error(&mut game, "A day has passed");
        }

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
                // Run anvil minigame
                if run_anvil(&mut game) {
                    // Hammering always takes 1/3 day
                    i += 20 * 60;
                }
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
