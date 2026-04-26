use bevy::prelude::*;
use bevy::window::WindowResolution;

mod ball;
mod block;
mod collision;
mod constants;
mod paddle;
mod ui;

fn main() {
    // 初期化
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(
                    constants::WINDOW_WIDTH as u32,
                    constants::WINDOW_HEIGHT as u32,
                ),
                title: "BREAKOUT".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins((
            ui::UIPlugin,
            paddle::PaddlePlugin,
            ball::BallPlugin,
            block::BlockPlugin,
            collision::CollisionPlugin,
        ))
        .add_systems(Startup, (setup_clear_color, setup).chain())
        .add_systems(
            Update,
            wait_for_start.run_if(
                in_state(GameState::PreGame)
                    .or(in_state(GameState::GameOver))
                    .or(in_state(GameState::GameClear)),
            ),
        )
        .run();
}

fn setup_clear_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::BLACK;
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// ゲーム状態
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    PreGame,
    Playing,
    GameOver,
    GameClear,
}

// ゲームスタートを待つ
fn wait_for_start(
    key_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if key_input.pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}
