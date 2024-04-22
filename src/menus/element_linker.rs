
use bevy::{prelude::*, utils::HashMap};

//could i use enum instead of string maybe ?


#[derive(Component,Default)]
pub struct UiElementLinker{
    pub elements_map: HashMap<String, Entity>   //maps strings to the sub-entity 
}
