use crate::player::*;
use console_engine::pixel;


pub fn draw_window_frame(
    engine: &mut console_engine::ConsoleEngine, 
    w: i32, h: i32, 
    map: &Vec<String>,
    player: &mut Player
) {

    let frame_char = pixel::pxl('█');
    // let mut screen = Screen::new(w as u32, h as u32);

    engine.line(0, 0, w, 0, frame_char);
    engine.line(0, h, w, h, frame_char);
    engine.line(0, 0, 0, h, frame_char);
    engine.line(w, 0, w, h, frame_char);

    let visible_map = get_map_subregion(player, w - 2, h - 2, map);
    for (i, line) in visible_map.iter().enumerate() {
        engine.print(1, (i + 1) as i32, line);
    }
    
    engine.print(1, 1, format!("{:?}", (player.x, player.y)).as_str());

    // screen.draw();
}


pub fn get_map_subregion(
    player: &mut Player,
    w: i32, h: i32,
    map: &Vec<String>
) -> Vec<String> 
{
    let map_height = map.len() as i32;
    let map_width = map[0].len() as i32;

    // Calculer les limites de la région visible
    let start_y = (player.y - h / 2).max(0);
    let end_y = (player.y + h / 2).min(map_height - 1);

    let start_x = (player.x - w / 2).max(0);
    let end_x = (player.x + w / 2).min(map_width - 1);

    // Extraire les lignes visibles
    map[start_y as usize..=end_y as usize]
        .iter()
        .map(|line| {
            let visible_line = &line[start_x as usize..=end_x as usize];
            visible_line.to_string()
        })
        .collect()
}
