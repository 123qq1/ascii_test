use bevy::prelude::*;

mod char;
mod enemy;

const CAM_SCALE : f32 = 1.2;
const CAM_HEIGHT : f32 = -150.0;

fn main() {

    App::new()

    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window{
            title: String::from("Ascii World"),
            resolution: (800.,600.).into(),
            ..Default::default()
        }),
        ..Default::default()
    }))

    .insert_resource(ClearColor(Color::BLACK))

    .add_plugins(char::CharPlugin)
    .add_plugins(enemy::EnemyPlugin)

    .add_systems(Startup, setup)

    .run();

}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn((
        Camera2d,
        OrthographicProjection{
            scale: CAM_SCALE,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(0.0,CAM_HEIGHT,0.0),
        MainCamera
    ));
}
