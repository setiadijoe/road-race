use rusty_engine::prelude::*;
use rand::prelude::*;

pub struct GameState {
    pub health_amount: u8,
    pub lost: bool,
}

impl Default for GameState {
    fn default() -> Self {
        GameState { 
            health_amount: 5, 
            lost: false,
        }
    }
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
const CARS_SPEED: f32 = 200.0;

const SCALE_BARRIER: f32 = 0.7;
const WINDOW_MIN_Y: f32 = -360.0;
const WINDOW_MAX_Y: f32 = 360.0;
const WINDOW_MIN_X: f32 = -675.0;
const WINDOW_MAX_X: f32 = 675.0;
const WINDOW_MIN_OBSTACLE_GENERATION: f32 = 700.0;
const WINDOW_MAX_OBSTACLE_GENERATION: f32 = 1800.0;

pub const ID_PLAYER_SPRITE: &str = "hero";
const ID_ROAD_LINE_SPRITE: &str = "roadline";
const ID_BARRIER: &str = "barrier";
const ID_CARS: &str = "car";
const ID_OBSTACLE: &str = "obstacle";
pub const ID_HEALTH_TEXT: &str = "health";

pub fn create_sprites(game: &mut Game<GameState>) {
    // Create road
    for i in 0..10 {
        let road_line = game.add_sprite(format!("{ID_ROAD_LINE_SPRITE}{}", i), SpritePreset::RacingBarrierWhite);
        road_line.scale = 0.1;
        road_line.translation.x = -600.0 + 150.0 * i as f32;
    }

    // Create top barrier
    for i in 0..10 {
        let top_barrier = game.add_sprite(format!("{ID_BARRIER}_top{}", i), SpritePreset::RacingBarrierWhite);
        top_barrier.layer = 1.0;
        top_barrier.scale = SCALE_BARRIER;
        top_barrier.translation.x = WINDOW_MIN_X + 150.0 * i as f32;
        top_barrier.translation.y = WINDOW_MAX_Y;
        top_barrier.collision = true;
    }

    // Create low barrier
    for i in 0..10 {
        let low_barrier = game.add_sprite(format!("{ID_BARRIER}_low{}", i), SpritePreset::RacingBarrierWhite);
        low_barrier.layer = 1.0;
        low_barrier.scale = SCALE_BARRIER;
        low_barrier.translation.x = WINDOW_MIN_X + 150.0 * i as f32;
        low_barrier.translation.y = WINDOW_MIN_Y;
        low_barrier.collision = true;
    }

    // Create cars
    let car_presets = vec![
        SpritePreset::RacingCarBlack,
        SpritePreset::RacingCarGreen,
        SpritePreset::RacingCarRed,
        SpritePreset::RacingCarYellow,
    ];

    for (i, preset) in car_presets.into_iter().enumerate() {
        let car = game.add_sprite(format!("{ID_CARS}{}", i), preset);
        car.layer = 5.0;
        car.collision = true;
        car.translation.x = thread_rng().gen_range(WINDOW_MIN_OBSTACLE_GENERATION..WINDOW_MAX_OBSTACLE_GENERATION);
        car.translation.y = thread_rng().gen_range((WINDOW_MIN_Y + 60.0)..(WINDOW_MAX_Y - 60.0));
    }

    // Create obstacles
    let car_presets = vec![
        SpritePreset::RollingHoleStart,
        SpritePreset::RollingHoleEnd,
        SpritePreset::RollingHoleStart,
        SpritePreset::RollingHoleEnd,
    ];
    for (i, preset) in car_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("{ID_OBSTACLE}{}", i), preset);
        obstacle.layer = 2.0;
        obstacle.collision = true;
        obstacle.translation.x =
            thread_rng().gen_range(WINDOW_MIN_OBSTACLE_GENERATION..WINDOW_MAX_OBSTACLE_GENERATION);
        obstacle.translation.y =
            thread_rng().gen_range((WINDOW_MIN_Y + 60.0)..(WINDOW_MAX_Y - 60.0));
    }
}

pub fn player_movement_logic(engine: &mut Engine, _game_state: &mut GameState) {
    let player = engine.sprites.get_mut(ID_PLAYER_SPRITE).unwrap();
    let mut direction = 0.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        direction += 1.0;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        direction -= 1.0;
    }

    if player.translation.y < WINDOW_MAX_Y - 40.0 && player.translation.y > WINDOW_MIN_Y + 40.0 {
        player.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
        player.rotation = direction * 0.15;
    }
}

pub fn road_movement_logic(engine: &mut Engine, _game_state: &mut GameState) {
    for sprite in engine.sprites.values_mut() {
        // Road and barrier movement
        if sprite.label.starts_with(ID_ROAD_LINE_SPRITE) || sprite.label.starts_with(ID_BARRIER)
        {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < WINDOW_MIN_X {
                sprite.translation.x += WINDOW_MAX_X * 2.0;
            }
        }
        // Car movement
        if sprite.label.starts_with(ID_CARS) {
            sprite.translation.x -= CARS_SPEED * engine.delta_f32;
            if sprite.translation.x < WINDOW_MIN_X - 200.0 {
                sprite.translation.x = thread_rng().
                    gen_range(WINDOW_MIN_OBSTACLE_GENERATION..WINDOW_MAX_OBSTACLE_GENERATION);
                sprite.translation.y = thread_rng().
                    gen_range((WINDOW_MIN_Y + 60.0)..(WINDOW_MAX_Y - 60.0));
            }
        }
        // Obstacle movement
        if sprite.label.starts_with(ID_OBSTACLE) {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < WINDOW_MIN_X - 200.0 {
                sprite.translation.x = thread_rng().
                    gen_range(WINDOW_MIN_OBSTACLE_GENERATION..WINDOW_MAX_OBSTACLE_GENERATION);
                sprite.translation.y = thread_rng().
                    gen_range((WINDOW_MIN_Y + 60.0)..(WINDOW_MAX_Y - 60.0));
            }
        }
    }
}

pub fn collision_logic(engine: &mut Engine, game_state: &mut GameState) {
    let health_message = engine.texts.get_mut(ID_HEALTH_TEXT).unwrap();
    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains(ID_PLAYER_SPRITE) || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 1.0);
        }
    }
}
