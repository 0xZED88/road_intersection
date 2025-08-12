use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 800;
const VEHICLE_SIZE: u32 = 40;
const VEHICLE_SPEED: i32 = 2;
const ROAD_WIDTH: u32 = 140;
const LANE_WIDTH: u32 = 40;
const CENTER_X: i32 = (WINDOW_WIDTH / 2) as i32;
const CENTER_Y: i32 = (WINDOW_HEIGHT / 2) as i32;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone)]
struct Vehicle {
    x: f32,
    y: f32,
    direction: Direction,
    color: Color,
}

impl Vehicle {
    fn new(direction: Direction) -> Self {
        let (x, y) = match direction {
            Direction::North => (
                CENTER_X as f32 - LANE_WIDTH as f32 / 2.0, 
                WINDOW_HEIGHT as f32 - 50.0
            ),
            Direction::South => (
                CENTER_X as f32 + LANE_WIDTH as f32 / 2.0, 
                50.0
            ),
            Direction::East => (
                50.0, 
                CENTER_Y as f32 - LANE_WIDTH as f32 / 2.0
            ),
            Direction::West => (
                WINDOW_WIDTH as f32 - 50.0, 
                CENTER_Y as f32 + LANE_WIDTH as f32 / 2.0
            ),
        };
////////////////////////cars////////////////
        let color = match direction {
            Direction::North => Color::RGB(255, 100, 100),
            Direction::South => Color::RGB(100, 255, 100),
            Direction::East => Color::RGB(100, 100, 255),
            Direction::West => Color::RGB(255, 255, 100),
        };

        Self { x, y, direction, color }
    }

    fn update(&mut self) {
        match self.direction {
            Direction::North => self.y -= VEHICLE_SPEED as f32,
            Direction::South => self.y += VEHICLE_SPEED as f32,
            Direction::East => self.x += VEHICLE_SPEED as f32,
            Direction::West => self.x -= VEHICLE_SPEED as f32,
        }
    }

    fn is_off_screen(&self) -> bool {
        self.x < -50.0 || self.x > WINDOW_WIDTH as f32 + 50.0 ||
        self.y < -50.0 || self.y > WINDOW_HEIGHT as f32 + 50.0
    }

    fn get_rect(&self) -> Rect {
        Rect::new(
            self.x as i32 - VEHICLE_SIZE as i32 / 2,
            self.y as i32 - VEHICLE_SIZE as i32 / 2,
            VEHICLE_SIZE,
            VEHICLE_SIZE,
        )
    }
}

struct TrafficSimulation {
    vehicles: Vec<Vehicle>,
    spawn_cooldown: u32,
}

impl TrafficSimulation {
    fn new() -> Self {
        Self {
            vehicles: Vec::new(),
            spawn_cooldown: 0,
        }
    }

    fn spawn_vehicle(&mut self, direction: Direction) {
        if self.spawn_cooldown == 0 {
            self.vehicles.push(Vehicle::new(direction));
            self.spawn_cooldown = 20;
        }
    }

    fn update(&mut self) {
        if self.spawn_cooldown > 0 {
            self.spawn_cooldown -= 1;
        }

        for vehicle in &mut self.vehicles {
            vehicle.update();
        }

        self.vehicles.retain(|vehicle| !vehicle.is_off_screen());
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        self.draw_roads(canvas)?;

        for vehicle in &self.vehicles {
            canvas.set_draw_color(vehicle.color);
            canvas.fill_rect(vehicle.get_rect())?;
        }

        canvas.present();
        Ok(())
    }

    fn draw_roads(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let road_color = Color::RGB(0, 0, 0);
        let line_color = Color::RGB(255, 255, 255);
        let center_color = Color::RGB(0, 0, 0);

        canvas.set_draw_color(road_color);
        canvas.fill_rect(Rect::new(
            0,
            CENTER_Y - ROAD_WIDTH as i32 / 2,
            WINDOW_WIDTH,
            ROAD_WIDTH,
        ))?;

        canvas.set_draw_color(road_color);
        canvas.fill_rect(Rect::new(
            CENTER_X - ROAD_WIDTH as i32 / 2,
            0,
            ROAD_WIDTH,
            WINDOW_HEIGHT,
        ))?;

        canvas.set_draw_color(center_color);
        canvas.fill_rect(Rect::new(
            CENTER_X - ROAD_WIDTH as i32 / 2,
            CENTER_Y - ROAD_WIDTH as i32 / 2,
            ROAD_WIDTH,
            ROAD_WIDTH,
        ))?;

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
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Traffic Simulation", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut simulation = TrafficSimulation::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Escape => break 'running,
                        Keycode::Up => simulation.spawn_vehicle(Direction::North),
                        Keycode::Down => simulation.spawn_vehicle(Direction::South),
                        Keycode::Right => simulation.spawn_vehicle(Direction::East),
                        Keycode::Left => simulation.spawn_vehicle(Direction::West),
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        simulation.update();
        simulation.render(&mut canvas)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
