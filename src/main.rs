mod vector2;
use vector2::Vector2;

use ggez::{conf, event, graphics::{self, Color}, Context, ContextBuilder, GameError, GameResult, timer};
use std::path;
use std::f32::consts::PI as PI;

const SCREEN_SIZE: (f32, f32) = (1200.0, 900.0);
const EDGE: f32 = 25.0;
const LEFT_EDGE: f32 = 125.0;

const DESIRED_FPS: u32 = 30;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VeichleType {
    Eagle,
    Rat,
}

#[derive(Clone)]
struct Veichle {
    veichle_type: VeichleType,
    scale: [f32; 2],
    rotation: f32,
    position: Vector2,
    target_list: Vec<VeichleType>,
    target: Option<Vector2>,
    velocity: Vector2,
    acceleration: Vector2,
    sensory_range: f32,
    sensory_angle: f32, // Angle of vision conde in radians
    max_speed: f32,
    max_force: f32,
}

impl Veichle {
    fn new(veichle_type: VeichleType) -> Self {
        let (_target_list, _scale, _sensory_range, _sensory_angle, _max_speed, _max_force, _pos) = Veichle::get_stats(veichle_type);
        Veichle {
            veichle_type: veichle_type,
            rotation: 0.0,
            scale: _scale,
            position: _pos,
            target_list: _target_list,
            target: None,
            velocity: Vector2::new(),
            acceleration: Vector2::new(),
            sensory_range: _sensory_range,
            sensory_angle: _sensory_angle,
            max_speed: _max_speed,
            max_force: _max_force,
        }
    }

    pub fn apply_force(&mut self) {
        self.velocity = self.velocity + self.acceleration;
        self.position = self.position + self.velocity;
        self.acceleration = 0.0 * self.acceleration;        
        self.target = None;
        self.rotation = self.velocity.angle();
    }

    pub fn steer(&mut self) {
        let current = self.velocity;
        let pos: (f32, f32) = (self.position.x, self.position.y);
        let desired: Vector2 = match pos {
            (_x, _) if _x < LEFT_EDGE => Vector2::from(self.max_speed, current.y),
            (_x, _) if _x > SCREEN_SIZE.0 - EDGE => Vector2::from(-self.max_speed, current.y),
            (_, _y) if _y < EDGE => Vector2::from(current.x, self.max_speed),
            (_, _y) if _y > SCREEN_SIZE.1 - EDGE => Vector2::from(current.x, -self.max_speed),
            (_, _) => self.max_speed * self.get_desired().normalized()
        };

        self.acceleration = self.max_force * (desired - current).normalized();
    }

    fn get_desired(&self) -> Vector2 {
        match self.veichle_type {
            VeichleType::Eagle => {
                match self.target {
                    Some(_target) => {
                        _target
                    },
                    None => {
                        Vector2::from(1000.0, 600.0) - self.position
                    }
                }
            },
            VeichleType::Rat => {
                match self.target {
                    Some(_target) => {
                        -1.0 * _target
                    },
                    None => {
                        Vector2::random()
                    }
                }
            },
        }
    }

    fn get_target(&mut self, entities: &Vec<Veichle>) {
        let mut min_dist: f32 = std::f32::INFINITY;
        let mut min_dist_vec: Option<Vector2> = None;
        for entity in entities {
            let distance_vec: Vector2 = match self.target_list.iter().find(|_target| _target == &&entity.veichle_type ) {
                Some(_target) => (entity.position - self.position),
                None => continue
            };
            if distance_vec.normal() < min_dist {
                min_dist = distance_vec.normal();
                min_dist_vec = Some(distance_vec);
            }
        }

        match min_dist_vec {
            Some(_vec) => {
                let angle = _vec.angle_to(Vector2::from_angle(self.rotation));
                if min_dist < self.sensory_range 
                    && angle < self.sensory_angle {
                    self.target = min_dist_vec;
                }
            },
            None => ()
        }
    }

    fn get_stats(veichle_type: VeichleType) -> (Vec<VeichleType>, [f32; 2], f32, f32, f32, f32, Vector2) {
        match veichle_type {
            VeichleType::Eagle => {
                let targets = vec![VeichleType::Rat];
                let scale = [0.5, 0.5];
                let sensory_range = 150.0;
                let sensory_angle = PI;
                let max_speed = 6.0;
                let max_force = 2.0;
                let pos = Vector2::from(200.0, 200.0);
                (targets, scale, sensory_range, sensory_angle, max_speed, max_force, pos)
            },
            VeichleType::Rat => {
                let targets = vec![VeichleType::Eagle];
                let scale = [0.3, 0.3];
                let sensory_range = 150.0;
                let sensory_angle = PI;
                let max_speed = 5.0;
                let max_force = 5.0;
                let pos = Vector2::from(1000.0, 600.0);
                (targets, scale, sensory_range, sensory_angle, max_speed, max_force, pos)
            }
        }
    }
}

struct AppState {
    static_sprites: Vec<graphics::Image>,
    veichle_sprites: Vec<(VeichleType, graphics::Image)>,
    veichles: Vec<Veichle>,
}


impl AppState {
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        Ok(AppState {
            static_sprites: AppState::load_static_sprites(ctx),
            veichle_sprites: AppState::load_veichle_sprites(ctx),
            veichles: vec![Veichle::new(VeichleType::Eagle), Veichle::new(VeichleType::Rat)],
        })
    }

    fn load_static_sprites(ctx: &mut Context) -> Vec<graphics::Image> {
        vec![
            graphics::Image::new(ctx, "/background.png".to_string()).unwrap(),
            graphics::Image::new(ctx, "/tree.png".to_string()).unwrap(),
        ]
    }

    fn load_veichle_sprites(ctx: &mut Context) -> Vec<(VeichleType, graphics::Image)> {
        vec![
            (VeichleType::Eagle, graphics::Image::new(ctx, "/eagle.png".to_string()).unwrap()),
            (VeichleType::Rat, graphics::Image::new(ctx, "/rat.png".to_string()).unwrap()),
        ]
    }
}

impl event::EventHandler<GameError> for AppState {
    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let entities = self.veichles.clone();
            for i in &mut self.veichles {
                i.get_target(&entities);
                i.steer();
                i.apply_force();
            }
        }
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        graphics::draw(
            ctx,
            &self.static_sprites[0],
            graphics::DrawParam::default()
                .scale([0.6, 0.6])
        ).expect("Failed to draw background");
        graphics::draw(
            ctx,
            &self.static_sprites[1],
            graphics::DrawParam::default()
                .scale([0.7, 0.7])
                .dest([500.0, 100.0])
        ).expect("Failed to draw background");
        let rectangle = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::stroke(5.0), 
            graphics::Rect::new(
                LEFT_EDGE,
                EDGE,
                SCREEN_SIZE.0 - EDGE - LEFT_EDGE,
                SCREEN_SIZE.1 - 2.0 * EDGE,
            ),
            graphics::Color::RED,
        ).expect("Failed to create button");
        graphics::draw(ctx, &rectangle, graphics::DrawParam::default())
            .expect("Failed to draw bar");

        for i in &self.veichles {
            let sprite = match self.veichle_sprites.iter().find(|x| x.0 == i.veichle_type) {
                Some(x) => x.1.clone(),
                _ => panic!("Failed to find sprite")
            };
            graphics::draw(
                ctx,
                &sprite,
                graphics::DrawParam::default()
                    .scale(i.scale)
                    .dest([i.position.x, i.position.y])
                    .rotation(i.rotation)
                    .offset([0.5, 0.5])
            ).expect("Failed to draw background");
        }

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new("forest_sim", "Emil")
        .add_resource_path(resource_dir) // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()
                .title("Forest Sim") // Set window title "Chess"
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false), // Fixate window size
        )
        .modules(conf::ModuleConf::default().audio(false));
    let (mut contex, event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state) // Run window event loop
}