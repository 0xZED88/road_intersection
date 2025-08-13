use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 800;
const VEHICLE_SIZE: u32 = 40;
const VEHICLE_SPEED: f32 = 2.0;
const ROAD_WIDTH: u32 = 120;
const LANE_WIDTH: u32 = 30;
const LIGHT_SIZE: u32 = 30;
const CENTER_X: i32 = (WINDOW_WIDTH / 2) as i32;
const CENTER_Y: i32 = (WINDOW_HEIGHT / 2) as i32;

#[derive(Clone)]
struct Vehicle {
    x: f32,
    y: f32,
    direction: String,          
    initial_direction: String,  
    route: String,            
    color: Color,
}

impl Vehicle {
    fn new(direction: &str) -> Self {
        // let (x, y) = match direction {
        //     "up" => (CENTER_X as f32 - LANE_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32),
        //     "down" => (CENTER_X as f32 + LANE_WIDTH as f32 / 2.0, 0.0),
        //     "right" => (0.0, CENTER_Y as f32 + LANE_WIDTH as f32 / 2.0),
        //     "left" => (WINDOW_WIDTH as f32, CENTER_Y as f32 - LANE_WIDTH as f32 / 2.0),
        //     _ => (0.0, 0.0),
        // };

        let (x, y) = match direction {
            "up" => (CENTER_X as f32 - LANE_WIDTH as f32, WINDOW_HEIGHT as f32),
            "down" => (CENTER_X as f32 + LANE_WIDTH as f32, 0.0),
            "right" => (0.0, CENTER_Y as f32 + LANE_WIDTH as f32),
            "left" => (WINDOW_WIDTH as f32, CENTER_Y as f32 - LANE_WIDTH as f32),
            _ => (0.0, 0.0),
        };

        let routes = ["Straight", "TurnLeft", "TurnRight"];
        let route_str = routes[rand::rng().random_range(0..routes.len())];

        let color = match route_str {
            "Straight" => Color::RGB(200, 200, 200),
            "TurnLeft" => Color::RGB(100, 255, 100),
            "TurnRight" => Color::RGB(100, 100, 255),
            _ => Color::RGB(255, 255, 255), 
        };

        Self {
            x,
            y,
            direction: direction.to_string(),
            initial_direction: direction.to_string(),
            route: route_str.to_string(),
            color,
        }
    }

    fn update(&mut self) {
        let y_to_up = CENTER_Y as f32 + LANE_WIDTH as f32 / 2.0;
        let y_to_down = CENTER_Y as f32 - LANE_WIDTH as f32 / 2.0;
        let x_to_right = CENTER_X as f32 - LANE_WIDTH as f32 / 2.0;
        let x_to_left = CENTER_X as f32 + LANE_WIDTH as f32 / 2.0;

        match self.initial_direction.as_str() {
            "up" => {
                if self.route == "TurnLeft" && self.y <= y_to_up {
                    self.direction = "right".to_string();
                } else if self.route == "TurnRight" && self.y <= y_to_down {
                    self.direction = "left".to_string();
                }
            },
            "down" => {
                if self.route == "TurnLeft" && self.y >= y_to_down {
                    self.direction = "left".to_string();
                } else if self.route == "TurnRight" && self.y >= y_to_up {
                    self.direction = "right".to_string();
                }
            },
            "right" => {
                if self.route == "TurnLeft" && self.x >= x_to_right {
                    self.direction = "up".to_string();
                } else if self.route == "TurnRight" && self.x >= x_to_left {
                    self.direction = "down".to_string();
                }
            },
            "left" => {
                if self.route == "TurnLeft" && self.x <= x_to_left {
                    self.direction = "down".to_string();
                } else if self.route == "TurnRight" && self.x <= x_to_right {
                    self.direction = "up".to_string();
                }
            },
            _ => {}
        }

        match self.direction.as_str() {
            "up" => self.y -= VEHICLE_SPEED,
            "down" => self.y += VEHICLE_SPEED,
            "right" => self.x += VEHICLE_SPEED,
            "left" => self.x -= VEHICLE_SPEED,
            _ => {}
        }
    }

    fn is_off_screen(&self) -> bool {
        self.x < -50.0
            || self.x > WINDOW_WIDTH as f32 + 50.0
            || self.y < -50.0
            || self.y > WINDOW_HEIGHT as f32 + 50.0
    }

    fn get_rect(&self) -> Rect {
        Rect::new(
            self.x as i32 - (VEHICLE_SIZE / 2) as i32,
            self.y as i32 - (VEHICLE_SIZE / 2) as i32,
            VEHICLE_SIZE,
            VEHICLE_SIZE,
        )
    }
}

struct TrafficSimulation {
    vehicles: Vec<Vehicle>,
}

fn now_in_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX EPOCH!")
        .as_millis()
}

impl TrafficSimulation {
    fn new() -> Self {
        Self {
            vehicles: Vec::new(),
        }
    }

    fn spawn_vehicle(&mut self, direction: &str) {
        self.vehicles.push(Vehicle::new(direction));
    }

    fn update(&mut self) {
        for vehicle in &mut self.vehicles {
            vehicle.update();
        }
        self.vehicles.retain(|vehicle| !vehicle.is_off_screen());
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        self.draw_roads(canvas)?;
        self.draw_traffic_lights(canvas)?;

        for vehicle in &self.vehicles {
            canvas.set_draw_color(vehicle.color);
            canvas.fill_rect(vehicle.get_rect())?;
        }

        canvas.present();
        Ok(())
    }

    fn draw_roads(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let line_color = Color::RGB(255, 255, 255);

        canvas.set_draw_color(line_color);
        for x in (0..WINDOW_WIDTH).step_by(30) {
            canvas.fill_rect(Rect::new(x as i32, CENTER_Y - 2, 15, 4))?;
        }
        for y in (0..WINDOW_HEIGHT).step_by(30) {
            canvas.fill_rect(Rect::new(CENTER_X - 2, y as i32, 4, 15))?;
        }

        canvas.set_draw_color(line_color);
        canvas.draw_line(
            (0, CENTER_Y - ROAD_WIDTH as i32 / 2),
            (WINDOW_WIDTH as i32, CENTER_Y - ROAD_WIDTH as i32 / 2),
        )?;
        canvas.draw_line(
            (0, CENTER_Y + ROAD_WIDTH as i32 / 2),
            (WINDOW_WIDTH as i32, CENTER_Y + ROAD_WIDTH as i32 / 2),
        )?;
        canvas.draw_line(
            (CENTER_X - ROAD_WIDTH as i32 / 2, 0),
            (CENTER_X - ROAD_WIDTH as i32 / 2, WINDOW_HEIGHT as i32),
        )?;
        canvas.draw_line(
            (CENTER_X + ROAD_WIDTH as i32 / 2, 0),
            (CENTER_X + ROAD_WIDTH as i32 / 2, WINDOW_HEIGHT as i32),
        )?;

        Ok(())
    }

    fn draw_traffic_lights(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let half_road = ROAD_WIDTH as i32 / 2;

        let lights = [
            Rect::new(
                CENTER_X - half_road - LIGHT_SIZE as i32,
                CENTER_Y - half_road - LIGHT_SIZE as i32,
                LIGHT_SIZE,
                LIGHT_SIZE,
            ),
            Rect::new(
                CENTER_X + half_road,
                CENTER_Y - half_road - LIGHT_SIZE as i32,
                LIGHT_SIZE,
                LIGHT_SIZE,
            ),
            Rect::new(
                CENTER_X - half_road - LIGHT_SIZE as i32,
                CENTER_Y + half_road,
                LIGHT_SIZE,
                LIGHT_SIZE,
            ),
            Rect::new(
                CENTER_X + half_road,
                CENTER_Y + half_road,
                LIGHT_SIZE,
                LIGHT_SIZE,
            ),
        ];

        for light in &lights {
            canvas.fill_rect(*light)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("hhhh", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("Could not create window");

    let mut canvas = window.into_canvas().build().expect("Could not create canvas");
    let mut event_pump = sdl_context.event_pump()?;
    let mut simulation = TrafficSimulation::new();

    let delay: u128 = 1000;
    let mut last_spawn_time: [u128; 4] = [0, 0, 0, 0];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let current_time = now_in_millis();

                    let (direction, timer_index) = match keycode {
                        Keycode::Up => ("up", 0),
                        Keycode::Down => ("down", 1),
                        Keycode::Right => ("right", 2),
                        Keycode::Left => ("left", 3),
                        Keycode::Escape => break 'running,
                        _ => continue,
                    };

                    if current_time - last_spawn_time[timer_index] > delay {
                        simulation.spawn_vehicle(direction);
                        last_spawn_time[timer_index] = current_time;
                    }
                }
                _ => {}
            }
        }

        simulation.update();
        simulation.render(&mut canvas)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}