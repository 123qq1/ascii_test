use bevy::prelude::*;

const CHAR_SIZE : f32 = 14.0; 

pub struct CharPlugin;

#[derive(Component)]
#[require(Text2d)]
pub struct Char;

impl Plugin for CharPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, test_chars);
    }
}

fn test_chars(
    mut commands: Commands
){
    let v = vec![12.,22.,32.,42.,52.,62.];
    let v2 = v.clone();

    for x in &v {
        for y in &v2 {
            add_char(&mut commands, "@", *x, *y);
        }
    }
}


pub fn add_char(commands: &mut Commands ,_text: &'static str,x: f32,y: f32){
    commands.spawn((
        Char,
        Text2d::new(_text),
        TextFont{
            font_size:CHAR_SIZE,
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
    ));
}