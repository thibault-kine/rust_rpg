#[macro_use]
extern crate lazy_static;

use std::u32;
use std::sync::Mutex;
use termion;
use console_engine::{
    pixel, Color, KeyCode
};


struct Player {
    x: i32,
    y: i32,
    icon: char,
}

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

    {
        let mut player = PLAYER.lock().unwrap();
        *player = Player {
            x: true_w / 2,
            y: true_h / 2,
            icon: '@'
        };
    
    
    loop {
        engine.wait_frame();
        engine.clear_screen();

        draw_window_frame(true_w, true_h, &mut engine, &mut player);

        handle_input(&mut engine, &mut player);

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }

        engine.draw();
    }

    }
}


fn handle_input(engine: &mut console_engine::ConsoleEngine, player: &mut Player) {
    
    if engine.is_key_pressed(KeyCode::Char('z')) {
       player.y -= 1; 
    }
    if engine.is_key_pressed(KeyCode::Char('s')) {
        player.y += 1;
    }
    if engine.is_key_pressed(KeyCode::Char('q')) {
        player.x -= 1;
    }
    if engine.is_key_pressed(KeyCode::Char('d')) {
        player.x += 1;
    }

    engine.set_pxl(player.x, player.y, pixel::pxl_fg(player.icon, Color::Green));
}


fn draw_window_frame(w: i32, h: i32, engine: &mut console_engine::ConsoleEngine, player: &mut Player) {

    let frame_char = pixel::pxl('█');

    engine.line(0, 0, w, 0, frame_char);
    engine.line(0, h, w, h, frame_char);
    engine.line(0, 0, 0, h, frame_char);
    engine.line(w, 0, w, h, frame_char);

    engine.print(1, 1, format!("{:?}", (player.x, player.y)).as_str());
}

