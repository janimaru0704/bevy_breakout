use bevy::{color::palettes::tailwind, prelude::*};

use crate::{GameState, constants};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_paddle)
            .add_systems(Update, update_paddle.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Paddle;

// パドルのスポーン
fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            constants::PADDLE_WIDTH,
            constants::PADDLE_HEIGHT,
        ))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(tailwind::EMERALD_400))),
        Transform::from_xyz(0.0, constants::PADDLE_Y, 0.0),
        Paddle,
        DespawnOnExit(GameState::Playing),
    ));
}
// パドルの更新
fn update_paddle(
    transform: Single<&mut Transform, With<Paddle>>,
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut dir = 0.0;

    if key_input.pressed(KeyCode::ArrowLeft) || key_input.pressed(KeyCode::KeyA) {
        dir -= 1.0;
    }
    if key_input.pressed(KeyCode::ArrowRight) || key_input.pressed(KeyCode::KeyD) {
        dir += 1.0;
    }

    let mut transform = transform.into_inner();
    transform.translation.x += dir * constants::PADDLE_SPEED * time.delta_secs();

    // 範囲内に収める
    let max_abs_x = (constants::WINDOW_WIDTH - constants::PADDLE_WIDTH) / 2.0;
    transform.translation.x = transform.translation.x.clamp(-max_abs_x, max_abs_x);
}
