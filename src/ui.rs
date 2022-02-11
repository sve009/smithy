use super::game::*;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::EventPump;

use std::time::Duration;

pub enum HandlerRet {
    Exit,
    Nothing,
    Accept,
}

pub enum InventoryMode {
    BuySell,
    Select,
}

pub fn create_text<'a, T>(
    s: &str,
    tc: &'a TextureCreator<T>,
    font: &mut sdl2::ttf::Font,
    color: Color,
) -> Texture<'a> {
    let text_s = font.render(s).blended(color).unwrap();
    tc.create_texture_from_surface(&text_s).unwrap()
}

// Center a texture in a larger rect
pub fn center_text(outer_rect: Rect, texture: &Texture) -> Rect {
    // Record width + height for texture
    let w = texture.query().width;
    let h = texture.query().height;

    // Find delta width + height
    let dw = outer_rect.width() - texture.query().width;
    let dh = outer_rect.height() - texture.query().height;

    // Fail cases
    if dw < 0 || dh < 0 {
        panic!("Outer rect must be larger");
    }

    // Calculate padding offsets
    let px: i32 = (dw / 2).try_into().unwrap();
    let py: i32 = (dh / 2).try_into().unwrap();

    // Return
    Rect::new(outer_rect.x + px, outer_rect.y + py, w, h)
}

pub fn handle_selection(event_pump: &mut EventPump, active: &mut i32) -> HandlerRet {
    // Handle events
    for e in event_pump.poll_iter() {
        match e {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return HandlerRet::Exit,
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                if *active > -1 {
                    *active -= 1;
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                if *active < 3 {
                    *active += 1;
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => return HandlerRet::Accept,
            _ => (),
        }
    }
    return HandlerRet::Nothing;
}

// Display errors to user in pop-up window
pub fn display_error(game: &mut Game, message: &str) {
    // Load font
    let mut font = game
        .ttf
        .load_font("assets/SupermercadoOne-Regular.ttf", 26)
        .unwrap();

    // Prep screen

    // Backdrop
    let backdrop = Rect::new(150, 120, 300, 240);

    // Create tc
    let tc = game.canvas.texture_creator();

    // Create text
    let text = create_text(message, &tc, &mut font, Color::RGB(255, 255, 255));

    // Get text rect
    let rect = center_text(backdrop, &text);

    // Draw
    game.canvas.set_draw_color(Color::RGB(255, 255, 255));
    game.canvas.clear();

    game.canvas.set_draw_color(Color::RGB(0, 0, 0));
    game.canvas.fill_rect(backdrop).unwrap();

    game.canvas.copy(&text, None, Some(rect));
    game.canvas.present();

    // Sleep
    std::thread::sleep(Duration::new(1, 0));
}

// UI for picking a form
pub fn pick_form(game: &mut Game) -> Option<Form> {
    // Backdrop setup
    let backdrop = Rect::new(200, 160, 200, 160);

    // Possible return values
    let vals = vec![Form::Spear, Form::Axe, Form::Hammer, Form::Sword];

    // Load the font
    let mut font = game
        .ttf
        .load_font("assets/SupermercadoOne-Regular.ttf", 26)
        .unwrap();

    // Create tc
    let tc = game.canvas.texture_creator();

    // Piece of text for each form
    let mut texts = Vec::<Texture>::new();
    texts.push(create_text(
        "Spear",
        &tc,
        &mut font,
        Color::RGB(255, 255, 255),
    ));
    texts.push(create_text(
        "Axe",
        &tc,
        &mut font,
        Color::RGB(255, 255, 255),
    ));
    texts.push(create_text(
        "Hammer",
        &tc,
        &mut font,
        Color::RGB(255, 255, 255),
    ));
    texts.push(create_text(
        "Sword",
        &tc,
        &mut font,
        Color::RGB(255, 255, 255),
    ));

    // Create rects
    let mut rects = Vec::<Rect>::new();
    for i in 0..4 {
        let w = texts[i].query().width;
        let h = texts[i].query().height;

        let outer = Rect::new(200, 160 + (i as i32 * 40), 200, 40);

        rects.push(center_text(outer, &texts[i]));
    }

    // Active selection
    let mut active: i32 = -1;

    // Loop
    'go: loop {
        // Handle events
        let e = handle_selection(&mut game.event_pump, &mut active);

        match e {
            HandlerRet::Exit => break 'go,
            HandlerRet::Accept => {
                if active >= 0 {
                    return Some(vals[active as usize]);
                }
            }
            _ => (),
        };

        // Draw
        game.canvas.set_draw_color(Color::RGBA(150, 150, 150, 100));
        game.canvas.clear();

        game.canvas.set_draw_color(Color::RGB(0, 0, 0));
        game.canvas.fill_rect(backdrop);

        // Draw active
        if active >= 0 {
            game.canvas.set_draw_color(Color::RGB(50, 50, 50));
            game.canvas
                .fill_rect(Rect::new(200, 160 + (active * 40), 200, 40));
        }

        // Draw texts
        for (i, texture) in texts.iter().enumerate() {
            game.canvas.copy(texture, None, Some(rects[i])).unwrap();
        }

        // Update
        game.canvas.present();
    }
    None
}

// Display the inventory screen
pub fn display_inventory(game: &mut Game, mode: Option<InventoryMode>) -> Option<usize> {
    // State:
    let mut screen = true; // T: inventory, F: buy
    let mut active: i32 = -1; // Highlight item on list

    // Load font
    let mut font = game
        .ttf
        .load_font("assets/SupermercadoOne-Regular.ttf", 32)
        .unwrap();

    // Create tc
    let tc = game.canvas.texture_creator();

    // Label screen
    let label = create_text("Inventory:", &tc, &mut font, Color::RGB(255, 255, 255));
    let label2 = create_text("Buy:", &tc, &mut font, Color::RGB(255, 255, 255));

    // Store never changes so do it out here
    let mut store: Vec<Texture> = Vec::new();

    // Change font
    font = game
        .ttf
        .load_font("assets/SupermercadoOne-Regular.ttf", 24)
        .unwrap();

    let mut store_products: Vec<Product> = Vec::new();

    // Create store as well
    store_products.push(Product::new(Material::Iron));
    store_products.push(Product::new(Material::Steel));
    store_products.push(Product::new(Material::Bronze));
    store_products.push(Product::new(Material::Silver));
    store_products.push(Product::new(Material::Gold));

    let s1 = store_products[0].to_string();
    let s2 = store_products[1].to_string();
    let s3 = store_products[2].to_string();
    let s4 = store_products[3].to_string();
    let s5 = store_products[4].to_string();

    store.push(create_text(&s1, &tc, &mut font, Color::RGB(255, 255, 255)));
    store.push(create_text(&s2, &tc, &mut font, Color::RGB(255, 255, 255)));
    store.push(create_text(&s3, &tc, &mut font, Color::RGB(255, 255, 255)));
    store.push(create_text(&s4, &tc, &mut font, Color::RGB(255, 255, 255)));
    store.push(create_text(&s5, &tc, &mut font, Color::RGB(255, 255, 255)));

    // Draw stuff here
    'scan: loop {
        // Items in inventory
        let mut items: Vec<Texture> = Vec::new();

        // Create the textures
        for (i, item) in game.state.inventory.iter().enumerate() {
            let s = item.to_string();
            let c = match item.temp_val() {
                Temp::Under => Color::RGB(255, 255, 255),
                Temp::Over => Color::RGB(235, 204, 52),
                Temp::Perfect => Color::RGB(119, 235, 52),
            };
            items.push(create_text(&s, &tc, &mut font, c));
        }

        // Black background
        game.canvas.set_draw_color(Color::RGB(0, 0, 0));
        game.canvas.clear();

        // Tabs
        game.canvas.set_draw_color(Color::RGB(100, 100, 100));
        match screen {
            true => game.canvas.fill_rect(Rect::new(300, 0, 300, 70)).unwrap(),
            false => game.canvas.fill_rect(Rect::new(0, 0, 300, 70)).unwrap(),
        };

        // Active highlight
        match screen {
            true => {
                if active >= 0 && active < game.state.inventory.len() as i32 {
                    game.canvas.set_draw_color(Color::RGB(50, 50, 50));
                    game.canvas
                        .fill_rect(Rect::new(
                            20,
                            80 + 50 * active,
                            store[0].query().width + 40,
                            store[0].query().height,
                        ))
                        .unwrap();
                }
            }
            false => {
                if active >= 0 && active < store.len() as i32 {
                    game.canvas.set_draw_color(Color::RGB(50, 50, 50));
                    game.canvas
                        .fill_rect(Rect::new(
                            20,
                            80 + 50 * active,
                            store[0].query().width + 40,
                            store[0].query().height,
                        ))
                        .unwrap();
                }
            }
        };

        // Draw the labels
        let label_rect = Rect::new(20, 20, label.query().width, label.query().height);
        game.canvas.copy(&label, None, Some(label_rect)).unwrap();

        let label_rect2 = Rect::new(320, 20, label2.query().width, label2.query().height);
        game.canvas.copy(&label2, None, Some(label_rect2)).unwrap();

        let label3 = create_text(
            &format!("Money: {}$", game.state.money).to_owned(),
            &tc,
            &mut font,
            Color::RGB(255, 255, 255),
        );
        let label_rect3 = Rect::new(20, 350, label3.query().width, label3.query().height);
        game.canvas.copy(&label3, None, Some(label_rect3)).unwrap();

        // Draw each inventory item
        match screen {
            true => {
                for (i, item) in items.iter().enumerate() {
                    let r = Rect::new(
                        40,
                        (80 + 50 * i).try_into().unwrap(),
                        item.query().width,
                        item.query().height,
                    );
                    game.canvas.copy(item, None, Some(r)).unwrap();
                }
            }
            false => {
                for (i, item) in store.iter().enumerate() {
                    let r = Rect::new(
                        40,
                        (80 + 50 * i).try_into().unwrap(),
                        item.query().width,
                        item.query().height,
                    );
                    game.canvas.copy(item, None, Some(r)).unwrap();
                }
            }
        };

        // Event handling
        for e in game.event_pump.poll_iter() {
            match e {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'scan,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if active > -1 {
                        active -= 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if screen && active + 1 < game.state.inventory.len() as i32 {
                        active += 1;
                    } else if !screen && active < 5 {
                        active += 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Tab),
                    ..
                } => {
                    if let Some(InventoryMode::BuySell) | None = mode {
                        screen = !screen;
                        active = -1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => match mode {
                    None | Some(InventoryMode::BuySell) => {
                        if !screen
                            && active >= 0
                            && active < 5
                            && game.state.money >= store_products[active as usize].value
                            && (game.state.inventory.len() as i32)
                                < game.state.upgrades.storage_space
                        {
                            game.state.inventory.push(store_products[active as usize]);
                            game.state.money -=
                                store_products[active as usize].material.base_value();
                        } else if screen
                            && active >= 0
                            && active < game.state.inventory.len() as i32
                        {
                            game.state.money += game.state.inventory[active as usize].value;
                            game.state.inventory.remove(active as usize);
                        }
                    }
                    Some(InventoryMode::Select) => {
                        if screen && active >= 0 && active < game.state.inventory.len() as i32 {
                            return Some(active as usize);
                        }
                    }
                },
                _ => (),
            }
        }

        // Update
        game.canvas.present();

        // Sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    // Return None
    None
}
