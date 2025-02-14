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
    forest: HashMap<String,EnemyData>,
}

#[derive(Resource)]
pub struct LayoutDict{
    test: Vec<LayoutData>,
}

#[derive(Deserialize)]
pub struct EnemyData{
    char: String,
    color: String,
}

#[derive(Deserialize)]
pub struct LayoutData{
    id: String,
    x:i32,
    y:i32,
}

impl Plugin for CharPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, test_chars);
    }
}

fn test_chars(
    mut commands: Commands,
){


    load_enemies(&mut commands);
    load_layouts(&mut commands);
}

fn load_enemies(
    commands: &mut Commands, 
){

    let file = File::open(TEST_ENEMIES_PATH).unwrap();
    let forest : HashMap<String,EnemyData> = serde_json::from_reader(file).unwrap();
    commands.insert_resource(EnemyDict{forest});
}

fn load_layouts(
    commands: &mut Commands,
){

    let file = File::open(TEST_LAYOUT_PATH).unwrap();
    let test : Vec<LayoutData> = serde_json::from_reader(file).unwrap();
    commands.insert_resource(LayoutDict{test});
}


pub fn add_char(commands: &mut Commands, text: &'static str, x: f32, y: f32, color: String){

    let rgb_vec: Vec<u8> = color.split(", ").map(|str| str.parse::<u8>().unwrap()).collect();

    commands.spawn((
        Char,
        TextColor(Color::srgb_u8(rgb_vec[0],rgb_vec[1],rgb_vec[2])),
        Text2d::new(text),
        TextFont{
            font_size:CHAR_SIZE,
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
    ));
}