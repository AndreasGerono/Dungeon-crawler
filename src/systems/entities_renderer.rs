#![warn(clippy::all, clippy::pedantic)]

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(FieldOfView)]
#[read_component(Render)]
#[read_component(Player)]
pub fn render_entities(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    // get all entieties with Point and Render components
    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visable_tiles.contains(pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });
    // Why 5000?
    draw_batch.submit(5000).expect("Barch error");
}
