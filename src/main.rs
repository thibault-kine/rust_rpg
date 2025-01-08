#[macro_use]
extern crate lazy_static;

pub mod graphics;
use graphics::*;

pub mod player;
use player::*;

pub mod map_handler;
use map_handler::*;

use std::u32;
use std::sync::Mutex;
use termion;
use console_engine::{
    pixel,
    Color,
    KeyCode
};


lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player {
        x: 0,
        y: 0,
        icon: '@'
    });
}


fn main() {

    // Get terminal size
    let (_w, _h) = termion::terminal_size().unwrap();
    let (w, h) = (_w as u32, _h as u32);
    let (true_w, true_h) = ((w - 1) as i32, (h - 1) as i32);


    let mut engine = 
        console_engine::
        ConsoleEngine::
        init(w, h, 60).unwrap();

    let mut player = PLAYER.lock().unwrap();
    *player = Player {
        x: true_w / 2,
        y: true_h / 2,
        icon: '@'
    };

    let map = load_map("maps/map.txt");
    
    
    loop {
        engine.wait_frame();
        engine.clear_screen();

        draw_window_frame(&mut engine,true_w, true_h,  &map, &mut player);

        handle_input(&mut engine, &mut player);

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }

        engine.draw();
    }
}


fn handle_input(engine: &mut console_engine::ConsoleEngine, player: &mut Player) {
    
    // MOVEMENT
    if  engine.is_key_pressed(KeyCode::Char('z')) || 
        engine.is_key_pressed(KeyCode::Up) {
        player.y -= 1; 
    }
    if  engine.is_key_pressed(KeyCode::Char('s')) ||
        engine.is_key_pressed(KeyCode::Down) {
        player.y += 1;
    }
    if  engine.is_key_pressed(KeyCode::Char('q')) || 
        engine.is_key_pressed(KeyCode::Left) {
        player.x -= 1;
    }
    if  engine.is_key_pressed(KeyCode::Char('d')) || 
        engine.is_key_pressed(KeyCode::Right) {
        player.x += 1;
    }

    engine.set_pxl(player.x, player.y, pixel::pxl_fg(player.icon, Color::Green));
}
