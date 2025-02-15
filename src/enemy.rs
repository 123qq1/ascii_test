use bevy::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, fs::File};

use crate::char::AddChar;

const TEST_ENEMIES_PATH : &str = "./assets/enemies/forest.json";
const TEST_LAYOUT_PATH : &str = "./assets/layouts/test_layout.json";

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

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, (load_enemies,load_layouts));
        app.add_systems(Update, spawn_test);
    }
}


fn spawn_test(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_add_char: EventWriter<AddChar>,
    layouts: Res<LayoutDict>,
    enemies: Res<EnemyDict>,
){
    if keys.just_pressed(KeyCode::Space){


        let mut batch : Vec<AddChar> = Vec::new();

        let l_dict = &layouts.test;
        let e_dict = &enemies.forest;

        for l in l_dict {
            let e = e_dict.get(&l.id).unwrap();
            batch.push(AddChar::new(
                e.char.clone(),
                e.color.clone(),
                l.x,
                l.y,
            ));
        }

        if !batch.is_empty(){ ev_add_char.send_batch(batch);}
    }
}


fn load_enemies(
    mut commands: Commands, 
){

    let file = File::open(TEST_ENEMIES_PATH).unwrap();
    let forest : HashMap<String,EnemyData> = serde_json::from_reader(file).unwrap();
    commands.insert_resource(EnemyDict{forest});
}

fn load_layouts(
    mut commands: Commands,
){

    let file = File::open(TEST_LAYOUT_PATH).unwrap();
    let test : Vec<LayoutData> = serde_json::from_reader(file).unwrap();
    commands.insert_resource(LayoutDict{test});
}