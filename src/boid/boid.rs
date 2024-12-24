pub struct Boid {
    pub position: (f32, f32),
    pub velocity: (f32, f32),
    pub max_speed: f32,
    pub perception_radius: f32,
}


impl Boid {

    pub fn new(position: (f32, f32), velocity: (f32, f32)) -> Self {
        Self {
            position,
            velocity,
            max_speed: 5.0,
            perception_radius: 50.0,
        }
    }


    // I'll need the edges to impl the wrap logic on the screen
    pub fn update(&mut self, screen_size: (f32, f32)){
        
    }


    pub fn apply_rules(){

    }
}