use bevy::{color::palettes::tailwind, prelude::*};

use crate::GameState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ((setup_font, spawn_node), spawn_start_msg).chain())
            .add_systems(OnEnter(GameState::GameOver), spawn_gameover_msg)
            .add_systems(OnEnter(GameState::GameClear), spawn_clear_msg)
            .add_systems(Update, blink_text);
    }
}

// 画面ノードのマーカー
#[derive(Component)]
pub struct MessageNode;

// 文字点滅のマーカー
#[derive(Component)]
struct BlinkText;

// フォントを保存しておくリソース
#[derive(Resource)]
struct FontHandle(Handle<Font>);

// 文字点滅のシステム
fn blink_text(mut query: Query<&mut TextColor, With<BlinkText>>, time: Res<Time>) {
    let alpha = (time.elapsed_secs() * 2.0).sin() * 0.5 + 0.5;

    for mut color in &mut query {
        color.0.set_alpha(alpha);
    }
}

// フォントのロード
fn setup_font(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(FontHandle(assets.load("NotoSansJP-VariableFont_wght.ttf")));
}

// ノードのスポーン
fn spawn_node(mut commands: Commands) {
    commands.spawn((
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
    ));
}

// スタートメッセージのスポーン
fn spawn_start_msg(
    mut commands: Commands,
    node: Single<Entity, With<MessageNode>>,
    font: Res<FontHandle>,
) {
    println!("start msg called");
    commands.entity(node.into_inner()).with_children(|p| {
        p.spawn((
            Text::new("BREAKOUT"),
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
}

// クリア画面のテキスト表示
// ゲームクリアの文字をスポーン
fn spawn_clear_msg(
    mut commands: Commands,
    node: Single<Entity, With<MessageNode>>,
    font: Res<FontHandle>,
) {
    commands.entity(node.into_inner()).with_children(|p| {
        p.spawn((
            Text::new("GAME CLEAR!"),
            TextFont {
                font: font.0.clone(),
                font_size: 60.0,
                weight: FontWeight::EXTRA_BOLD,
                ..default()
            },
            TextColor(Color::from(tailwind::YELLOW_300)),
            DespawnOnExit(GameState::GameClear),
        ));
        p.spawn((
            Text::new("Press SPACE to Restart"),
            TextFont {
                font: font.0.clone(),
                weight: FontWeight::BOLD,
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::from(tailwind::GRAY_400)),
            DespawnOnExit(GameState::GameClear),
            BlinkText,
        ));
    });
}

// ゲームオーバーの文字をスポーン
fn spawn_gameover_msg(
    mut commands: Commands,
    node: Single<Entity, With<MessageNode>>,
    font: Res<FontHandle>,
) {
    commands.entity(node.into_inner()).with_children(|p| {
        p.spawn((
            Text::new("GAME OVER"),
            TextFont {
                font: font.0.clone(),
                font_size: 60.0,
                weight: FontWeight::EXTRA_BOLD,
                ..default()
            },
            TextColor(Color::from(tailwind::RED_600)),
            DespawnOnExit(GameState::GameOver),
        ));
        p.spawn((
            Text::new("Press SPACE to Restart"),
            TextFont {
                font: font.0.clone(),
                weight: FontWeight::BOLD,
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::from(tailwind::GRAY_400)),
            DespawnOnExit(GameState::GameOver),
            BlinkText,
        ));
    });
}
