use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::window::WindowResolution;

// ウィンドウサイズ
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn main() {
    // 初期化
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                title: "ブロック崩し".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(
            Startup,
            (setup_font, setup_clear_color, setup, spawn_blocks).chain(),
        )
        .add_systems(Update, wait_for_start.run_if(in_state(GameState::PreGame)))
        .add_systems(
            Update,
            (
                (update_ball, update_paddle),
                (check_collision_paddle, check_collision_block),
                check_clear,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), show_gameover_msg)
        .add_systems(OnEnter(GameState::GameClear), show_clear_msg)
        .add_systems(Update, blink_text)
        .run();
}

fn setup_font(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(FontHandle(assets.load("NotoSansJP-VariableFont_wght.ttf")));
}

fn setup_clear_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::BLACK;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    font: Res<FontHandle>,
) {
    commands.spawn(Camera2d);

    // タイトル画面用の文字をスポーン
    commands
        .spawn((
            MessageNode,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("ブロック崩し"),
                TextFont {
                    font: font.0.clone(),
                    weight: FontWeight::EXTRA_BOLD,
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::from(tailwind::YELLOW_500)),
                DespawnOnExit(GameState::PreGame),
            ));
            p.spawn((
                Text::new("Press SPACE to Start"),
                TextFont {
                    font: font.0.clone(),
                    weight: FontWeight::BOLD,
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::from(tailwind::GRAY_400)),
                DespawnOnExit(GameState::PreGame),
                BlinkText,
            ));
        });

    // ボールをスポーン
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(tailwind::SKY_500))),
        Transform::from_xyz(0.0, PADDLE_Y + PADDLE_HEIGHT / 2.0 + BALL_RADIUS, 0.0),
        Ball {
            velocity: Vec2::new(0.0, BALL_SPEED),
        },
    ));

    // パドルをスポーン
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(tailwind::EMERALD_400))),
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
        Paddle,
    ));
}

// ブロックをスポーン
fn spawn_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 全体の横幅を計算
    let total_width = BLOCK_COLUMNS as f32 * (BLOCK_WIDTH + BLOCK_GAP) - BLOCK_GAP;

    // 左端の開始位置
    let start_x = -total_width / 2.0 + BLOCK_WIDTH / 2.0;

    for row in 0..BLOCK_ROWS {
        for col in 0..BLOCK_COLUMNS {
            // 座標を計算
            let block_pos = Vec3::new(
                start_x + col as f32 * (BLOCK_WIDTH + BLOCK_GAP),
                WINDOW_HEIGHT / 2.0 - BLOCK_TOP_OFFSET - row as f32 * (BLOCK_HEIGHT + BLOCK_GAP),
                0.0,
            );

            // スポーン
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(BLOCK_WIDTH, BLOCK_HEIGHT))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(tailwind::FUCHSIA_600))),
                Transform::from_translation(block_pos),
                Block,
            ));
        }
    }
}

#[derive(Resource)]
struct FontHandle(Handle<Font>);

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

// ゲームオーバーの文字をスポーン
fn show_gameover_msg(
    mut commands: Commands,
    node: Single<Entity, With<MessageNode>>,
    font: Res<FontHandle>,
) {
    let node = node.into_inner();

    commands.entity(node).with_children(|p| {
        p.spawn((
            Text::new("GAME OVER"),
            TextFont {
                font: font.0.clone(),
                font_size: 60.0,
                weight: FontWeight::EXTRA_BOLD,
                ..default()
            },
            TextColor(Color::from(tailwind::RED_600)),
        ));
    });
}

// 画面ノードのマーカー
#[derive(Component)]
struct MessageNode;

// 文字点滅のマーカー
#[derive(Component)]
struct BlinkText;

// 文字点滅のシステム
fn blink_text(mut query: Query<&mut TextColor, With<BlinkText>>, time: Res<Time>) {
    let alpha = (time.elapsed_secs() * 2.0).sin() * 0.5 + 0.5;

    for mut color in &mut query {
        color.0.set_alpha(alpha);
    }
}

// ボール
const BALL_RADIUS: f32 = 16.0;
const BALL_SPEED: f32 = 400.0;

#[derive(Component)]
struct Ball {
    velocity: Vec2,
}

fn update_ball(
    ball: Single<(&mut Transform, &mut Ball)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut transform, mut ball) = ball.into_inner();

    let vx = ball.velocity.x * time.delta_secs();
    let vy = ball.velocity.y * time.delta_secs();

    // 上の壁の跳ね返り
    if transform.translation.y + vy >= WINDOW_HEIGHT / 2.0 - BALL_RADIUS {
        ball.velocity.y *= -1.0;
    }

    // 左右の壁の跳ね返り
    let max_abs_x = WINDOW_WIDTH / 2.0 - BALL_RADIUS;
    if transform.translation.x + vx <= -max_abs_x || transform.translation.x + vx >= max_abs_x {
        ball.velocity.x *= -1.0;
    }

    // 下に当たったならゲームオーバーへ遷移
    if transform.translation.y + vy <= -WINDOW_HEIGHT / 2.0 + BALL_RADIUS {
        next_state.set(GameState::GameOver);
    }
    transform.translation.x += ball.velocity.x * time.delta_secs();
    transform.translation.y += ball.velocity.y * time.delta_secs();
}

// パドル
const PADDLE_WIDTH: f32 = 120.0;
const PADDLE_HEIGHT: f32 = 10.0;

const PADDLE_Y: f32 = -WINDOW_HEIGHT / 2.0 + 20.0;

const PADDLE_SPEED: f32 = 500.0;

#[derive(Component)]
struct Paddle;

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
    transform.translation.x += dir * PADDLE_SPEED * time.delta_secs();

    // 範囲内に収める
    let max_abs_x = (WINDOW_WIDTH - PADDLE_WIDTH) / 2.0;
    transform.translation.x = transform.translation.x.clamp(-max_abs_x, max_abs_x);
}

// パドルとの衝突判定
fn check_collision_paddle(
    ball: Single<(&mut Ball, &Transform)>,
    paddle: Single<&Transform, With<Paddle>>,
) {
    let (mut ball, ball_transform) = ball.into_inner();
    let paddle_transform = paddle.into_inner();

    // 2次元座標を得る
    let ball_pos = ball_transform.translation.truncate();
    let paddle_pos = paddle_transform.translation.truncate();

    // ボールとパドルの一番近い点. ボールの座標をパドル内領域に切り詰めることで得る
    let closest_point = Vec2::new(
        ball_transform.translation.x.clamp(
            paddle_transform.translation.x - PADDLE_WIDTH / 2.0,
            paddle_transform.translation.x + PADDLE_WIDTH / 2.0,
        ),
        ball_transform.translation.y.clamp(
            paddle_transform.translation.y - PADDLE_HEIGHT / 2.0,
            paddle_transform.translation.y + PADDLE_HEIGHT / 2.0,
        ),
    );

    // 近い点との距離が半径以下なら衝突
    if ball_pos.distance(closest_point) <= BALL_RADIUS {
        // パドルのどのあたりに当たったか
        let offset = (ball_pos.x - paddle_pos.x) / (PADDLE_WIDTH / 2.0);

        // 反射角. 最大は60degに指定
        let max_angle = std::f32::consts::PI / 3.0;
        let bounce_angle = offset * max_angle;

        // 速度を更新
        ball.velocity = Vec2::new(
            BALL_SPEED * bounce_angle.sin(),
            BALL_SPEED * bounce_angle.cos(),
        );
    }
}

// ブロックとの衝突判定
fn check_collision_block(
    mut commands: Commands,
    ball: Single<(&mut Ball, &Transform)>,
    block_query: Query<(Entity, &Transform), With<Block>>,
) {
    let (mut ball, ball_transform) = ball.into_inner();
    let ball_pos = ball_transform.translation.truncate();

    for (block_entity, block_transform) in &block_query {
        let block_pos = block_transform.translation.truncate();

        // ブロック内の最接近点を探す
        let closest_point = Vec2::new(
            ball_pos.x.clamp(
                block_pos.x - BLOCK_WIDTH / 2.0,
                block_pos.x + BLOCK_WIDTH / 2.0,
            ),
            ball_pos.y.clamp(
                block_pos.y - BLOCK_HEIGHT / 2.0,
                block_pos.y + BLOCK_HEIGHT / 2.0,
            ),
        );

        // 衝突しているなら
        if ball_pos.distance(closest_point) <= BALL_RADIUS {
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

// ブロック
const BLOCK_WIDTH: f32 = 80.0;
const BLOCK_HEIGHT: f32 = 40.0;

const BLOCK_GAP: f32 = 5.0;
const BLOCK_TOP_OFFSET: f32 = 30.0;

const BLOCK_ROWS: u32 = 6;
const BLOCK_COLUMNS: u32 = 9;

#[derive(Component)]
struct Block;

// 勝利判定
fn check_clear(block_query: Query<(), With<Block>>, mut next_state: ResMut<NextState<GameState>>) {
    if block_query.is_empty() {
        next_state.set(GameState::GameClear);
    }
}

// クリア画面のテキスト表示
// ゲームオーバーの文字をスポーン
fn show_clear_msg(
    mut commands: Commands,
    node: Single<Entity, With<MessageNode>>,
    font: Res<FontHandle>,
) {
    let node = node.into_inner();

    commands.entity(node).with_children(|p| {
        p.spawn((
            Text::new("GAME CLEAR!"),
            TextFont {
                font: font.0.clone(),
                font_size: 60.0,
                weight: FontWeight::EXTRA_BOLD,
                ..default()
            },
            TextColor(Color::from(tailwind::YELLOW_300)),
            BlinkText,
        ));
    });
}
