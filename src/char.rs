
use bevy::prelude::*;

const CHAR_SIZE : f32 = 14.0; 

pub struct CharPlugin;

#[derive(Component)]
#[require(Text2d)]
pub struct Char;

#[derive(Event)]
pub struct AddChar
{
    text: String,
    x: f32, 
    y: f32, 
    color: String
}

impl AddChar{
    pub fn new(text:String, color: String, x_i: i32, y_i: i32) -> AddChar{
        AddChar{
            text,
            color,
            x: (x_i as f32) * CHAR_SIZE,
            y: (y_i as f32) * CHAR_SIZE,
        }
    }
}


impl Plugin for CharPlugin{
    fn build(&self, app: &mut App){
        app.add_event::<AddChar>();
        app.add_systems(Update, on_add_char.run_if(on_event::<AddChar>));
    }
}




pub fn on_add_char(
    mut commands: Commands, 
    mut ev_add_char: EventReader<AddChar>
){
    for ev in ev_add_char.read() {
        let x = ev.x;
        let y = ev.y;

        let rgb_vec: Vec<u8> = ev.color.split(", ").map(|str| str.parse::<u8>().unwrap()).collect();
    
        commands.spawn((
            Char,
            TextColor(Color::srgb_u8(rgb_vec[0],rgb_vec[1],rgb_vec[2])),
            Text2d::new(ev.text.clone()),
            TextFont{
                font_size:CHAR_SIZE,
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}