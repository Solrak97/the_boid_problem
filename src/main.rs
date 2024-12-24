mod simulation;
mod rendering;

mod boid;

use ggez::{event, graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::Color;
use boid::Boid;
use rendering::draw_boids;


struct MainState {
    boids: Vec<Boid>,
    screen_size: (f32, f32),
}


impl MainState {

    pub fn new(screen_size: (f32, f32), boid_ammount: i32) -> Self {
        let boids = (0..boid_ammount)
            .map(|_| Boid::new(
                (
                    rand::random::<f32>() * screen_size.0,
                    rand::random::<f32>() * screen_size.1,
                ),
                (
                    rand::random::<f32>() - 0.5,
                    rand::random::<f32>() - 0.5
                )
            )
        )
            .collect();

        MainState { boids, screen_size }
    }
}



impl event::EventHandler for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for boid in &mut self.boids {
            boid.update(self.screen_size);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        draw_boids(&mut canvas, ctx, &self.boids)?;
        canvas.finish(ctx)
    }

}



fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("boids_simulation", "YourName")
        .build()
        .expect("Failed to build ggez context");

    let screen_size = (1280.0, 720.0);
    let state = MainState::new(screen_size, 30);
    event::run(ctx, event_loop, state)
}
