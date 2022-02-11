use sdl2::image;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct Game {
    pub sdl_context: sdl2::Sdl,
    pub image_context: sdl2::image::Sdl2ImageContext,
    pub ttf: sdl2::ttf::Sdl2TtfContext,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Start sdl2 image
        let image_context = image::init(image::InitFlag::PNG).unwrap();

        // Start sdl2 ttf
        let ttf = sdl2::ttf::init().unwrap();

        // Get window
        let window = video_subsystem
            .window("rust-sdl2 demo", 600, 480)
            .position_centered()
            .build()
            .unwrap();

        // Get canvas
        let canvas: Canvas<Window> = window.into_canvas().build().unwrap();

        // Set up event pump
        let event_pump = sdl_context.event_pump().unwrap();

        Game {
            sdl_context,
            image_context,
            ttf,
            canvas,
            event_pump,
            state: GameState::new(),
        }
    }
}

pub enum MenuLevel {
    Main,
    Game,
    Furnace,
    Anvil,
    Inventory,
    Shop,
}

pub enum Temp {
    Perfect,
    Over,
    Under,
}

#[derive(Clone, Copy)]
pub enum Material {
    Iron,
    Steel,
    Bronze,
    Silver,
    Gold,
}

impl Material {
    pub fn to_string(&self) -> String {
        match self {
            Material::Iron => String::from("Iron"),
            Material::Steel => String::from("Steel"),
            Material::Bronze => String::from("Bronze"),
            Material::Silver => String::from("Silver"),
            Material::Gold => String::from("Gold"),
        }
    }
    pub fn base_value(&self) -> i32 {
        match self {
            Material::Iron => 100,
            Material::Steel => 400,
            Material::Bronze => 100,
            Material::Silver => 300,
            Material::Gold => 500,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Form {
    Bar,
    Spear,
    Axe,
    Hammer,
    Sword,
}

impl Form {
    pub fn to_string(&self) -> String {
        match self {
            Form::Bar => String::from("Bar"),
            Form::Spear => String::from("Spear"),
            Form::Axe => String::from("Axe"),
            Form::Hammer => String::from("Hammer"),
            Form::Sword => String::from("Sword"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Storage,
    Forge,
    Anvil,
}

impl Location {
    pub fn to_string(&self) -> String {
        match self {
            Location::Storage => String::new(),
            Location::Forge => String::from("<Forge>"),
            Location::Anvil => String::from("<Anvil>"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Product {
    pub material: Material,
    pub form: Form,
    pub location: Location,
    pub value: i32,
    pub temp: i32,
}

impl Product {
    pub fn new(m: Material) -> Product {
        let v = m.base_value();

        Product {
            material: m,
            form: Form::Bar,
            location: Location::Storage,
            value: v,
            temp: 70,
        }
    }
    pub fn to_string(&self) -> String {
        let mut s: String = String::from(self.material.to_string());
        s.push_str(" ");
        s.push_str(&self.form.to_string());
        s.push_str(" ");
        s.push_str(&self.location.to_string());
        s.push_str(": ");
        s.push_str(&self.value.to_string());
        s.push_str("$");

        s
    }
    pub fn temp_val(&self) -> Temp {
        match self.material {
            Material::Iron => {
                if self.temp < 2400 {
                    return Temp::Under;
                } else if self.temp > 2600 {
                    return Temp::Over;
                } else {
                    return Temp::Perfect;
                }
            }
            Material::Steel => {
                if self.temp < 2100 {
                    return Temp::Under;
                } else if self.temp > 2300 {
                    return Temp::Over;
                } else {
                    return Temp::Perfect;
                }
            }
            Material::Bronze => {
                if self.temp < 1100 {
                    return Temp::Under;
                } else if self.temp > 1300 {
                    return Temp::Over;
                } else {
                    return Temp::Perfect;
                }
            }
            Material::Silver => {
                if self.temp < 1050 {
                    return Temp::Under;
                } else if self.temp > 1250 {
                    return Temp::Over;
                } else {
                    return Temp::Perfect;
                }
            }
            Material::Gold => {
                if self.temp < 1150 {
                    return Temp::Under;
                } else if self.temp > 1350 {
                    return Temp::Over;
                } else {
                    return Temp::Perfect;
                }
            }
        }
    }
}

pub struct Upgrades {
    pub forge_space: i32,
    pub storage_space: i32,
}

pub struct GameState {
    pub inventory: Vec<Product>,
    pub money: i32,
    pub reputation: i32,
    pub upgrades: Upgrades,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            inventory: vec![],
            money: 100,
            reputation: 0,
            upgrades: Upgrades {
                forge_space: 1,
                storage_space: 5,
            },
        }
    }
}
