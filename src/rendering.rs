use super::boid::prelude::*;
use ggez::{graphics::{self, Canvas}, Context, GameResult};




pub fn draw_boids(canvas: &mut Canvas, ctx: &mut Context, boids: &[Boid]) -> GameResult {

    for boid in boids {

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [boid.position.0, boid.position.1],
            5.0, // Boid size
            0.1,
            graphics::Color::WHITE,
        )?;

        graphics::draw(canvas, &circle, graphics::DrawParam::default());
    }

    Ok(())

}
