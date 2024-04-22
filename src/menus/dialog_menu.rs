


use bevy::{prelude::*, utils::HashMap};

/*
use crate::{appstate::AppState,
     world_interaction::dialog::DialogRunner,
      file_system_interaction::{asset_loading::DialogAssets, dialog::Dialog}};
*/
//use super::ui_menu_state::{UiMenuState, UiMenuType};

use super::{element_linker::{UiElementLinker}   };
/*

when this is active, render it and slow down or freeze other aspects of the game 


*/
pub struct DialogMenuPlugin;

impl Plugin for DialogMenuPlugin {
    fn build(&self, app: &mut App) {
      
        
        app
  //  .add_systems(OnEnter(AppState::Playing), spawn_dialog_menu )
    
    .add_systems(Update, update_dialog_menu_visibility.run_if(any_with_component::<DialogMenuComponent>) )
    
    .add_systems(Update, update_dialog_menu_state.run_if(any_with_component::<DialogMenuComponent>) )
      .add_systems(Update, update_dialog_menu_contents.run_if(any_with_component::<DialogMenuComponent> ) )
    
    ;
    
    }
}

 
//resource or component??
#[derive(Component,Default)]
pub struct DialogMenuComponent{
    pub body: Option<String>,
    pub choices: Option<Vec<String>>
    // 
    
    

    //pub menu_requested_active: bool ,
    //pub menu_is_active: bool, 



   // pub root_node_entity: Option<Entity>
}

fn update_dialog_menu_visibility(
    mut menu_query: Query<(Entity,&mut Visibility) , With<DialogMenuComponent>>,
    
    
  //  dialog_runner: Query<&DialogRunner>
    //ui_menu_state: Res<State<UiMenuState>> 
){    
    if let Ok((menu_entity,mut visibility)) = menu_query.get_single_mut(){
        
         let visible =  true  ;
                   
                   
          *visibility = match visible {
              true =>  Visibility::Visible,
              false =>  Visibility::Hidden
          }; 
                   
            
    } 
}


/*

Read joy axes for this --
*/

//https://taintedcoders.com/bevy/ui/#displaying-some-text
pub fn spawn_dialog_menu(
    mut commands: Commands
    
    ) {
    let text = "Dialog";
    
    let mut elements_map = HashMap::new();
    
    
    let root_node = commands.spawn(
        NodeBundle{
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            
            visibility: Visibility::Hidden,  
            
            ..default()
        }       
        
        ).id();
        
         

    let title_node = commands.spawn(
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the alignment of the Text
          .with_text_justify(JustifyText::Left)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        })
    ).id();
    
    
     let container_node = commands.spawn(
            NodeBundle {
                                style: Style {
    
                                    width: Val::Px(600.0),
                                    height: Val::Px(140.0),
                                    position_type: PositionType::Absolute,
                             
                                    ..Style::DEFAULT
                                     },
                                background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                                ..default()
                            }
        )
            .add_child(title_node) 
        .id();
        
        
    elements_map.insert("title".into(), title_node);
    
    commands.entity(root_node)
    .insert( DialogMenuComponent::default() )
    .insert(  UiElementLinker {
        elements_map
    } ) 
    .add_child(container_node);

    
    
}


 
fn update_dialog_menu_state(
    mut menu_state_query: Query<(Entity,&mut DialogMenuComponent)  >, 
     
 //    dialog_assets_resource: Res<DialogAssets>,
//     dialogs: Res<Assets<Dialog>>,
     
//    dialog_runner_query: Query<&DialogRunner> 
){    
    if let Ok((menu_entity,mut menu_state)) = menu_state_query.get_single_mut(){
  //  if let Ok(dialog_runner) = dialog_runner_query.get_single(){
        
        //let current_dialog_node_name = &dialog_runner. node ;
      //  let current_dialog_node_step_index = &dialog_runner. current_node_index ;
        
   //    if  let Some(dialog_node_handle) = dialog_assets_resource.nodes.get(  current_dialog_node_name ){ 
     //        if let Some(dialog) = dialogs.get(dialog_node_handle){  
                //    println!("found dialog from handle to update menu state.. "); 
                    
                 //  if  let Some(current_step_data) = dialog.steps.get( *current_dialog_node_step_index ) {  
                       //  let body = &current_step_data.body;
                       
                          menu_state.body = Some(  "title body ".into() );
                          
                 //  }
                 
         //    } 
   //    }
    //}
    } 
    
}
 

fn update_dialog_menu_contents(
     menu_query: Query<(Entity,& DialogMenuComponent,&UiElementLinker), Changed<DialogMenuComponent>  >, 
     
     mut text_node_query: Query<&mut Text> 
    
){    
    if let Ok((menu_entity,dialog_menu,element_linker)) = menu_query.get_single(){
        
        let current_body = &dialog_menu.body;
        
        if let Some(title_element) = element_linker.elements_map.get( "title" ) { 
             if  let Ok(mut title_node) = text_node_query.get_mut( *title_element  ){
            
          //        title_node.set_value(  current_body.clone() ) ;
                    title_node.sections[0].value =   current_body.clone() .unwrap_or(" ? ".into());
             } 
        }
       
        
    } 
    
}

/*


do this kinda but in bevy ui..

#[sysfail(log(level = "error"))]
fn display_interaction_prompt(
    interaction_opportunity: Res<InteractionOpportunity>,
    mut dialogue_runner: Query<&mut DialogueRunner>,
    mut egui_contexts: EguiContexts,
    actions: Query<&ActionState<PlayerAction>>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    dialog_target_query: Query<&DialogTarget>,
    mut freeze: ResMut<ActionsFrozen>,
) -> Result<()> {
    let Some(opportunity) = interaction_opportunity.0 else {
        return Ok(());
    };
    let dialog_target = dialog_target_query.get(opportunity)?;
    let window = primary_windows
        .get_single()
        .context("Failed to get primary window")?;
    egui::Window::new("Interaction")
        .collapsible(false)
        .title_bar(false)
        .auto_sized()
        .fixed_pos(egui::Pos2::new(window.width() / 2., window.height() / 2.))
        .show(egui_contexts.ctx_mut(), |ui| {
            ui.label("E: Talk");
        });
    for actions in actions.iter() {
        if actions.just_pressed(PlayerAction::InteractAttack) {
            let mut dialogue_runner = dialogue_runner.single_mut();
            dialogue_runner.start_node(&dialog_target.node);
            freeze.freeze();
        }
    }
    Ok(())
}

*/