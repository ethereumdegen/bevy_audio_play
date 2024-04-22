//! This example illustrates the various features of Bevy UI.

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    input::{keyboard::KeyboardInput, mouse::{MouseScrollUnit, MouseWheel}, ButtonState},
    prelude::*,
    winit::WinitSettings,
};
use bevy_kira_audio::{Audio, AudioPlugin,AudioControl};
 

mod menus;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AudioPlugin))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
      //  .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        
        .add_systems(Startup, menus::dialog_menu::spawn_dialog_menu)

        .add_systems(Update, listen_for_inputs) 
        
        //.add_plugins( menus::dialog_menu::DialogMenuPlugin )
        
    //    .add_systems(Update, mouse_scroll)
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
    
    ) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    
      
}


//on key press, play sound 
fn listen_for_inputs(
    mut key_evt_reader: EventReader<KeyboardInput>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
    ){
    for evt in key_evt_reader.read() {
        if evt.state == ButtonState::Pressed {
        
            match evt.key_code {

                KeyCode::KeyF => {

                    println!("playing sound ");

                    audio.play(asset_server.load("sounds/walk_sfx2.ogg")).looped();

                }


                _ => {} 

             
        }   
        }
         
    


    }

}




/*
#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

*/