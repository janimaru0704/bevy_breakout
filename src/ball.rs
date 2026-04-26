use bevy::{color::palettes::tailwind, prelude::*};

use crate::{GameState, constants};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_ball)
            .add_systems(Update, update_ball.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
}

// ボールのスポーン
fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(constants::BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(tailwind::SKY_500))),
        Transform::from_xyz(
            0.0,
            constants::PADDLE_Y + constants::PADDLE_HEIGHT / 2.0 + constants::BALL_RADIUS,
            0.0,
        ),
        Ball {
            velocity: Vec2::new(0.0, constants::BALL_SPEED),
        },
        DespawnOnExit(GameState::Playing),
    ));
}

// ボールの更新
fn update_ball(
    ball: Single<(&mut Transform, &mut Ball)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut transform, mut ball) = ball.into_inner();

    let vx = ball.velocity.x * time.delta_secs();
    let vy = ball.velocity.y * time.delta_secs();

    // 上の壁の跳ね返り
    if transform.translation.y + vy >= constants::WINDOW_HEIGHT / 2.0 - constants::BALL_RADIUS {
        ball.velocity.y *= -1.0;
    }

    // 左右の壁の跳ね返り
    let max_abs_x = constants::WINDOW_WIDTH / 2.0 - constants::BALL_RADIUS;
    if transform.translation.x + vx <= -max_abs_x || transform.translation.x + vx >= max_abs_x {
        ball.velocity.x *= -1.0;
    }

    // 下に当たったならゲームオーバーへ遷移
    if transform.translation.y + vy <= -constants::WINDOW_HEIGHT / 2.0 + constants::BALL_RADIUS {
        next_state.set(GameState::GameOver);
    }
    transform.translation.x += ball.velocity.x * time.delta_secs();
    transform.translation.y += ball.velocity.y * time.delta_secs();
}
