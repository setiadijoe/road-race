use rusty_engine::prelude::*;
use roadrace::*;

fn main() {
    let mut game = Game::new();

    // Create hero sprite
    let hero = game.add_sprite(ID_PLAYER_SPRITE, SpritePreset::RacingCarBlue);
    hero.translation.x = -500.0;
    hero.layer = 10.0;
    hero.collision = true;

    // Play backgroung music
    game.audio_manager.
        play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    create_sprites(&mut game);

    // Create health message
    let health_message = game.add_text(ID_HEALTH_TEXT, "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}



fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Don't run any game logic if game state is ended
    if game_state.lost {
        return;
    }

    player_movement_logic(engine, game_state);
    road_movement_logic(engine, game_state);
    collision_logic(engine, game_state);

    // Detect if the game is end
    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game_over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}
