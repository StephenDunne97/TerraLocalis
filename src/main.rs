use ggez::{
    Context, 
    ContextBuilder,
    GameResult,
    graphics::{self, Color, Rect, DrawMode},
    event::{self, EventHandler, KeyCode, KeyMods},
    error::GameError,
    timer,
};
use glam::Vec2;
use std::time::Duration;
use std::f32::consts::PI;

type Point2 = Vec2;
type Vector2 = Vec2;

const SCREEN_WIDTH: f32 = 1600.; //  pixels
const SCREEN_HEIGHT: f32 = 1600.; //  pixels
const BOX_WIDTH: f32 = 100.; // pixels
const BOX_LENGTH: f32 = 200.; // pixels
const BOX_TIP_LEN: f32 = 20.; // in pixels
const MOVEMENT_SPEED: f32 = 110. / 180. * PI; // in radians/second
const MAX_ANGLE: f32 = 75. / 180. * PI; // in radians

#[derive(Debug)]
struct InputState {
    to_turn: f32,
    started: bool,
}
impl Default for InputState {
    fn default() -> Self {
        InputState {
            to_turn: 0.,
            started: false,
        }
    }
}

struct Screen {
    box_across_offset: f32, // in pixels
    direction: f32,         // in radians
    previous_frame_time: Duration,
    period_in_sec: f32, // in seconds
    input: InputState,
}
impl Screen {
    pub fn new(_ctx: &mut Context) -> Screen {
        // Load/create resources such as images here.
        Screen {
            box_across_offset: 0.,
            direction: 0.,
            previous_frame_time: Duration::from_secs(0),
            period_in_sec: 0.,
            input: InputState::default(),
        }
    }
    pub fn steer(&mut self, side: f32){
        if side == 0. {
            return;
        }
        self.direction += MOVEMENT_SPEED * self.period_in_sec * side;
        if self.direction > MAX_ANGLE {
            self.direction = MAX_ANGLE;
        } else if self.direction < -MAX_ANGLE {
            self.direction = -MAX_ANGLE;
        }
    }
}
impl EventHandler<GameError> for Screen {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(_ctx, DESIRED_FPS) {
            // Update game objects here.
            let now = timer::time_since_start(_ctx);
            self.period_in_sec = now.as_secs() as f32 + now.subsec_nanos() as f32 / 1_000_000_000.0;
            self.previous_frame_time = now;
            self.steer(self.input.to_turn);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        let square = graphics::MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect {
                    x: -BOX_WIDTH / 2.,
                    y: BOX_TIP_LEN,
                    w: BOX_WIDTH,
                    h: BOX_LENGTH,
                },
                [1., 0., 1., 1.].into(),
            )?
            .build(ctx)?;

        let dest = Point2::new(100.0, 100.0);

        graphics::draw(ctx, &square, (dest, 0.0, Color::WHITE))?;
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ){
        match keycode {
            KeyCode::D => {
                self.input.to_turn = -1.0;
            }
            KeyCode::A => {
                self.input.to_turn = 1.0;
            }
            _ => (),
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::A | KeyCode::D => {
                self.input.to_turn = 0.0;
            }
            _ => (),
        }
    }

}
fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Terra Localis")
        .build()
        .expect("Could not create a context in ggez");
    let my_game = Screen::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}



