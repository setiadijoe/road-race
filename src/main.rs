use rusty_engine::prelude::*;
use rand::prelude::*;

struct GameState {
    health_amount: u8,
    lost: bool,
}

impl Default for GameState {
    fn default() -> Self {
        GameState { 
            health_amount: 5, 
            lost: false,
        }
    }
}

fn main() {
    let mut game = Game::new();

    // game setup goes here

    // Create hero sprite
    let hero = game.add_sprite("hero", SpritePreset::RacingCarBlue);
    hero.translation.x = -500.0;
    hero.layer = 10.0;
    hero.collision = true;

    // Play backgroung music
    game.audio_manager.
        play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // Create road line
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    // Create obstacle
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];
    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x =  thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }

    // Create health message
    let health_message = game.add_text("health_message", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Don't run any game logic if game state is ended
    if game_state.lost {
        return;
    }

    // Collect Keyboard Input
    let mut direction = 0.0;
    if engine.keyboard_state.pressed_any(&[KeyCode::W, KeyCode::Up]) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::S, KeyCode::Down]) {
        direction -= 1.0;
    }

    // Move the hero sprite
    let hero = engine.sprites.get_mut("hero").unwrap();
    hero.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    hero.rotation = direction * 0.15;
    if hero.translation.y < -360.0 || hero.translation.y > 360.0 {
        game_state.health_amount = 0;
    }

    // Move road objects
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x =  thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }

    // Deal with collission
    let health_message = engine.texts.get_mut("health_message").unwrap();
    for event in engine.collision_events.drain(..) {
        // Don't care if obstacle collide with each other or if a collistion ended
        if !event.pair.either_contains("hero") || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
    }

    // Detect if the game is end
    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game_over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}