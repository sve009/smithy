use super::game::*;
use super::ui::*;
use crate::handle_events;
use crate::Controls;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;

use sdl2::image::LoadTexture;

use rand::Rng;

use std::path::Path;
use std::time::Duration;

pub struct Bangs<'a> {
    image: sdl2::render::Texture<'a>,
    rect: Rect,
}

pub struct BangSpawner {
    countdown: i32,
}

fn update_note(n: &mut Bangs) -> bool {
    n.rect.y = n.rect.y + 5;

    if n.rect.y >= 480 {
        return true;
    }

    false
}

fn find_in_vec(v: &Vec<Bangs>, r: Rect) -> i64 {
    for (i, note) in v.iter().enumerate() {
        if let Some(_) = note.rect.intersection(r) {
            return i as i64;
        }
    }
    -1
}

fn spawn<'a, T>(
    bangs: &mut Vec<Bangs<'a>>,
    spawner: &mut BangSpawner,
    texture_creator: &'a TextureCreator<T>,
) {
    if spawner.countdown > 0 {
        spawner.countdown -= 1;
        return;
    }

    let mut rng = rand::thread_rng();

    let recs = vec![
        Rect::new(24, -60, 120, 60),
        Rect::new(168, -60, 120, 60),
        Rect::new(312, -60, 120, 60),
        Rect::new(456, -60, 120, 60),
    ];

    let i: usize = rng.gen_range(0..4);

    let t = match i {
        0 => texture_creator
            .load_texture(Path::new("assets/BangViolet.png"))
            .unwrap(),
        1 => texture_creator
            .load_texture(Path::new("assets/BangRed.png"))
            .unwrap(),
        2 => texture_creator
            .load_texture(Path::new("assets/BangBlue.png"))
            .unwrap(),
        3 => texture_creator
            .load_texture(Path::new("assets/BangYellow.png"))
            .unwrap(),
        _ => panic!("This should never happen, generated num outside range"),
    };

    bangs.push(Bangs {
        image: t,
        rect: recs[i],
    });

    spawner.countdown = 15;
}

fn exit_anvil<T>(
    points: i32,
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<T>,
    font: &mut sdl2::ttf::Font,
) {
    let s = &points.to_string();

    let line1 = create_text(
        "You scored:",
        texture_creator,
        font,
        Color::RGB(255, 255, 255),
    );
    let line2 = create_text(s, texture_creator, font, Color::RGB(255, 255, 255));

    let w1 = line1.query().width;
    let h1 = line1.query().height;
    let w2 = line2.query().width;
    let h2 = line2.query().height;

    let x1 = (600 - w1) / 2;
    let x2 = (600 - w2) / 2;
    let y1 = (480 - h1 - h2) / 3;
    let y2 = 2 * ((480 - h1 - h2) / 3);

    let r1 = Rect::new(x1 as i32, y1 as i32, w1, h1);
    let r2 = Rect::new(x2 as i32, y2 as i32, w2, h2);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.copy(&line1, None, Some(r1)).unwrap();
    canvas.copy(&line2, None, Some(r2)).unwrap();

    canvas.present();

    ::std::thread::sleep(Duration::new(1, 0));
    println!("You scored: {}", points);
}

pub fn run_anvil(game: &mut Game) {
    // Run minigame

    // Set up texture creator + font
    let texture_creator = game.canvas.texture_creator();

    // Pick item
    let index = match display_inventory(game, Some(InventoryMode::Select)) {
        Some(x) => x,
        None => return,
    };

    // Multiplier for additional value
    // TODO: Let player know that white can't be used if selected
    let mult = match game.state.inventory[index].temp_val() {
        Temp::Under => {
            display_error(game, "Item not hot enough");
            return;
        }
        Temp::Perfect => 1.5,
        Temp::Over => 1f32,
    };

    // Pick form
    let form = pick_form(game);

    // Handle stuff
    let form = match form {
        None => return,
        Some(x) => x,
    };

    // Load Bangs
    let bang = texture_creator
        .load_texture(Path::new("assets/Bang.png"))
        .unwrap();

    let mut spawner = BangSpawner { countdown: 0 };

    let mut cs = Controls::new();

    // Receptacles
    let r1 = Rect::new(24, 390, 120, 60);
    let r2 = Rect::new(168, 390, 120, 60);
    let r3 = Rect::new(312, 390, 120, 60);
    let r4 = Rect::new(456, 390, 120, 60);

    // Notes
    let mut notes = Vec::<Bangs>::new();

    // Points
    let mut points = 0;

    // Game loop
    'running: loop {
        // Clear game.canvas
        game.canvas.set_draw_color(Color::RGB(255, 255, 255));
        game.canvas.clear();

        // Handle events
        if !handle_events(&mut cs, &mut game.event_pump) {
            break;
        }

        // Update notes
        for mut note in notes.iter_mut() {
            if update_note(&mut note) {
                break 'running;
            }
        }

        // Handle logic
        if cs.left {
            let i = find_in_vec(&notes, r1);
            if i >= 0 {
                let val = notes[i as usize].rect.y - r1.y;
                let val = if val < 0 { val * -1 } else { val };
                points += 100 - val;
                notes.remove(i as usize);
                cs.left = false;
            } else {
                break;
            }
        }
        if cs.up {
            let i = find_in_vec(&notes, r2);
            if i >= 0 {
                let val = notes[i as usize].rect.y - r2.y;
                let val = if val < 0 { val * -1 } else { val };
                points += 100 - val;
                notes.remove(i as usize);
                cs.up = false;
            } else {
                break;
            }
        }
        if cs.down {
            let i = find_in_vec(&notes, r3);
            if i >= 0 {
                let val = notes[i as usize].rect.y - r3.y;
                let val = if val < 0 { val * -1 } else { val };
                points += 100 - val;
                notes.remove(i as usize);
                cs.down = false;
            } else {
                break;
            }
        }
        if cs.right {
            let i = find_in_vec(&notes, r4);
            if i >= 0 {
                let val = notes[i as usize].rect.y - r4.y;
                let val = if val < 0 { val * -1 } else { val };
                points += 100 - val;
                notes.remove(i as usize);
                cs.right = false;
            } else {
                break;
            }
        }

        // Draw notes
        for note in &notes {
            //game.canvas.fill_rect(note.rect);
            game.canvas.copy(&note.image, None, note.rect).unwrap();
        }

        // Spawn notes
        spawn(&mut notes, &mut spawner, &texture_creator);

        // Draw receptacles
        game.canvas.copy(&bang, None, r1).unwrap();
        game.canvas.copy(&bang, None, r2).unwrap();
        game.canvas.copy(&bang, None, r3).unwrap();
        game.canvas.copy(&bang, None, r4).unwrap();

        // Update game.canvas
        game.canvas.present();

        // Sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // Update value + form + location (incorrect for rn)
    game.state.inventory[index].value = (mult * (points as f32)) as i32;
    game.state.inventory[index].form = form;
    game.state.inventory[index].location = Location::Storage;

    // Create font to pass to exit_anvil
    let mut font = game
        .ttf
        .load_font("assets/SupermercadoOne-Regular.ttf", 32)
        .unwrap();
    exit_anvil(points, &mut game.canvas, &texture_creator, &mut font);
}
