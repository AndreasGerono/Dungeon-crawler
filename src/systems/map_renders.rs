#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
    #[resource] theme: &Option<Box<dyn MapTheme>>,
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0); // draw on 1st console
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if Map::in_bounds(pt)
                && (player_fov.visable_tiles.contains(&pt)
                    || map.revealed_tiles[get_idx(x, y)])
            {
                let tint = if player_fov.visable_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARK_GRAY
                };
                let idx = get_idx(x, y);
                
                let glyph = match theme {
                    Some(theme) => theme.tile_render(map.tiles[idx]),
                    _ => panic!("theme is not defined!"),
                };

                draw_batch.set(
                    pt - offset,
                    ColorPair::new(tint, BLACK),
                    glyph,
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
