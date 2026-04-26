use bevy::{color::palettes::tailwind, prelude::*};

use crate::{GameState, constants};

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_blocks);
    }
}

#[derive(Component)]
pub struct Block;

// ブロックのスポーン
fn spawn_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 全体の横幅を計算
    let total_width = constants::BLOCK_COLUMNS as f32
        * (constants::BLOCK_WIDTH + constants::BLOCK_GAP)
        - constants::BLOCK_GAP;

    // 左端の開始位置
    let start_x = -total_width / 2.0 + constants::BLOCK_WIDTH / 2.0;

    for row in 0..constants::BLOCK_ROWS {
        for col in 0..constants::BLOCK_COLUMNS {
            // 座標を計算
            let block_pos = Vec3::new(
                start_x + col as f32 * (constants::BLOCK_WIDTH + constants::BLOCK_GAP),
                constants::WINDOW_HEIGHT / 2.0
                    - constants::BLOCK_TOP_OFFSET
                    - row as f32 * (constants::BLOCK_HEIGHT + constants::BLOCK_GAP),
                0.0,
            );

            // スポーン
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(
                    constants::BLOCK_WIDTH,
                    constants::BLOCK_HEIGHT,
                ))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(tailwind::FUCHSIA_600))),
                Transform::from_translation(block_pos),
                Block,
                DespawnOnExit(GameState::Playing),
            ));
        }
    }
}
