use bevy::prelude::*;

use crate::{GameState, ball, block, constants, paddle};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((check_collision_paddle, check_collision_block), check_clear)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

// パドルとの衝突判定
fn check_collision_paddle(
    ball: Single<(&mut ball::Ball, &Transform)>,
    paddle: Single<&Transform, With<paddle::Paddle>>,
) {
    let (mut ball, ball_transform) = ball.into_inner();
    let paddle_transform = paddle.into_inner();

    // 2次元座標を得る
    let ball_pos = ball_transform.translation.truncate();
    let paddle_pos = paddle_transform.translation.truncate();

    // ボールとパドルの一番近い点. ボールの座標をパドル内領域に切り詰めることで得る
    let closest_point = Vec2::new(
        ball_transform.translation.x.clamp(
            paddle_transform.translation.x - constants::PADDLE_WIDTH / 2.0,
            paddle_transform.translation.x + constants::PADDLE_WIDTH / 2.0,
        ),
        ball_transform.translation.y.clamp(
            paddle_transform.translation.y - constants::PADDLE_HEIGHT / 2.0,
            paddle_transform.translation.y + constants::PADDLE_HEIGHT / 2.0,
        ),
    );

    // 近い点との距離が半径以下なら衝突
    if ball_pos.distance(closest_point) <= constants::BALL_RADIUS {
        // パドルのどのあたりに当たったか
        let offset = (ball_pos.x - paddle_pos.x) / (constants::PADDLE_WIDTH / 2.0);

        // 反射角. 最大は60degに指定
        let max_angle = std::f32::consts::PI / 3.0;
        let bounce_angle = offset * max_angle;

        // 速度を更新
        ball.velocity = Vec2::new(
            constants::BALL_SPEED * bounce_angle.sin(),
            constants::BALL_SPEED * bounce_angle.cos(),
        );
    }
}

// ブロックとの衝突判定
fn check_collision_block(
    mut commands: Commands,
    ball: Single<(&mut ball::Ball, &Transform)>,
    block_query: Query<(Entity, &Transform), With<block::Block>>,
) {
    let (mut ball, ball_transform) = ball.into_inner();
    let ball_pos = ball_transform.translation.truncate();

    for (block_entity, block_transform) in &block_query {
        let block_pos = block_transform.translation.truncate();

        // ブロック内の最接近点を探す
        let closest_point = Vec2::new(
            ball_pos.x.clamp(
                block_pos.x - constants::BLOCK_WIDTH / 2.0,
                block_pos.x + constants::BLOCK_WIDTH / 2.0,
            ),
            ball_pos.y.clamp(
                block_pos.y - constants::BLOCK_HEIGHT / 2.0,
                block_pos.y + constants::BLOCK_HEIGHT / 2.0,
            ),
        );

        // 衝突しているなら
        if ball_pos.distance(closest_point) <= constants::BALL_RADIUS {
            // ブロックを削除
            commands.entity(block_entity).despawn();

            // 衝突方向を決定
            let relative_vec = ball_pos - closest_point;
            // xの差が大きいなら左右から衝突, そうでないなら上下から衝突
            if relative_vec.x.abs() > relative_vec.y.abs() {
                ball.velocity.x *= -1.0;
            } else {
                ball.velocity.y *= -1.0;
            }
            break;
        }
    }
}

// 勝利判定
fn check_clear(
    block_query: Query<(), With<block::Block>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if block_query.is_empty() {
        next_state.set(GameState::GameClear);
    }
}
