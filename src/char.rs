use std::{collections::HashMap, env::current_dir, fs::File};

use bevy::prelude::*;
use serde::Deserialize;

const CHAR_SIZE : f32 = 14.0; 
const TEST_ENEMIES_PATH : &str = "./assets/enemies/forest.json";
const TEST_LAYOUT_PATH : &str = "./assets/layout/test_layout.json";

pub struct CharPlugin;

#[derive(Component)]
#[require(Text2d)]
pub struct Char;

#[derive(Resource)]
pub struct EnemyDict{
    forest: HashMap<String,Enemy>,
}

#[derive(Deserialize, Debug)]
pub struct Enemy{
    char: String,
    color: String,
}

impl Plugin for CharPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, test_chars);
    }
}

fn test_chars(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    let v = vec![12.,22.,32.,42.,52.,62.];
    let v2 = v.clone();

    for x in &v {
        for y in &v2 {
            add_char(&mut commands, "@", *x, *y);
        }
    }

    load_enemies(&mut commands);
}

fn load_enemies(
    commands: &mut Commands, 
){

    let file = File::open(TEST_ENEMIES_PATH).unwrap();
    let e : HashMap<String,Enemy> = serde_json::from_reader(file).unwrap();
    commands.insert_resource(EnemyDict{forest:e});
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