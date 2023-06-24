#![allow(dead_code)]
#![allow(unused_variables)]

use crate::library::prelude::*;
use bevy::prelude::*;

use super::ui_container::{Position, PositionLayout};
use super::ui_core::Branch;

//===========================================================================

#[derive(Component)]
pub struct Widget {
    pub path: String,
}
impl Widget {

    pub fn fetch<'a> (&'a self, system: &'a  Hierarchy, key: &str) -> Result<&Branch, String> {
        let mut extra_path = String::from(&self.path);
        if !key.is_empty() { extra_path += "/";extra_path += key;}
        match system.expose().borrow_chain_checked(&extra_path){
            Ok (branch) => Result::Ok(branch),
            Err (message) => Err(format!("Fetch failed, could not find '{}' #REASON: {}", &extra_path, message).to_string()),
        }
    }
    pub fn fetch_mut<'a> (&'a self, system: &'a mut Hierarchy, key: &str) -> Result<&mut Branch, String> {
        let mut extra_path = String::from(&self.path);
        if !key.is_empty() { extra_path += "/";extra_path += key;}
        match system.expose_mut().borrow_chain_checked_mut(&extra_path){
            Ok (branch) => Result::Ok(branch),
            Err (message) => Err(format!("Fetch failed, could not find '{}' #REASON: {}", &extra_path, message).to_string()),
        }
    }
    
    pub fn fetch_layout<'a> (&'a self, system: &'a  Hierarchy, key: &str) -> Result<&PositionLayout, String> {
        match self.fetch(system, key){
            Ok (branch) => Result::Ok(branch.container_get().position_layout_get()),
            Err (message) => Err(message),
        }
    }
    pub fn fetch_layout_mut<'a> (&'a self, system: &'a mut Hierarchy, key: &str) -> Result<&mut PositionLayout, String> {
        match self.fetch_mut(system, key){
            Ok (branch) => Result::Ok(branch.container_get_mut().position_layout_get_mut()),
            Err (message) => Err(message),
        }
    }
    pub fn fetch_data<'a> (&'a self, system: &'a  Hierarchy, key: &str) -> Result<&Option<Box<dyn Data + Send + Sync>>, String> {
        match self.fetch(system, key){
            Ok (branch) => Result::Ok(branch.data_get()),
            Err (message) => Err(message),
        }
    }
    pub fn fetch_data_mut<'a> (&'a self, system: &'a mut Hierarchy, key: &str) -> Result<&mut Option<Box<dyn Data + Send + Sync>>, String> {
        match self.fetch_mut(system, key){
            Ok (branch) => Result::Ok(branch.data_get_mut()),
            Err (message) => Err(message),
        }
    }
    pub fn fetch_position<'a> (&'a self, system: &'a Hierarchy, key: &str) -> Result<&Position, String> {
        match self.fetch(&system, key) {
            Ok (branch) => Result::Ok(&branch.container_get().position_get()),
            Err (message) => Result::Err(message),
        }
    }
    pub fn fetch_position_mut<'a> (&'a self, system: &'a mut Hierarchy, key: &str) -> Result<&mut Position, String> {
        match self.fetch_mut(system, key) {
            Ok (branch) => Result::Ok(branch.container_get_mut().position_get_mut()),
            Err (message) => Result::Err(message),
        }
    }
    pub fn is_within (&self, system: &Hierarchy, key: &str, point: &Vec2) -> Result<bool, String> {
        match self.fetch_position(&system, key) {
            Ok (position) => Result::Ok((point.x > position.point_1.x && point.x < position.point_2.x) && (point.y > position.point_1.y && point.y < position.point_2.y)),
            Err (message) => Result::Err(message),
        }
    }
    //add is cursor_within + depth


    fn from_path (path: &str) -> Widget {
        Widget { 
            path: path.to_string(),
        }
    }

    pub fn new(system: &mut Hierarchy, key: &str, position: PositionLayout) -> Result<Widget, String> {
        match system.expose_mut().create_simple_checked(key, position) {
            Ok (new_key) => Result::Ok(Widget::from_path(&new_key)),
            Err (message) => Err(String::from("UNABLE TO CREATE WIDGET! #Error: ") + &message),
        }
    }
    pub fn new_in(system: &mut Hierarchy, widget: &Widget, key: &str, position: PositionLayout) -> Result <Widget, String> {
        match key.split_once('/') {
            None => {
                match system.expose_mut().borrow_chain_checked_mut(&widget.path){
                    Ok (reference) => match reference.create_simple_checked(key, position) {
                        Ok (new_key) => Result::Ok(Widget::from_path(&(String::new() + &widget.path + "/"+ &new_key))),
                        Err (message) => Result::Err(message),
                    },
                    Err (message) => Err(String::from("WIDGET '") + &widget.path + "' NOT FOUND! #Error: "+ &message),
                }
            },
            Some (tuple) => {
                let mut path = String::new();
                let tuple1 = MString::split_last(key,"/");       // xxx/xxx/xxx - yyy
                let is_rooted:bool = match tuple1.0.split_once('/'){             // xxx - xxx/xxx
                    Some (tuple2) => if tuple2.0 == "#ROOT" {path += tuple2.1;true} else {path += &tuple1.0;false},
                    None =>  if tuple1.0 == "#ROOT" {true} else {path += &tuple1.0;false},
                };

                if is_rooted {
                    if path.is_empty() {
                        Result::Err(String::from("THIS KEY IS ILLEGAL!"))
                    } else {
                        let source = match system.expose_mut().translate_chain_checked(&path){
                            Ok (source) => source,
                            Err (message) => return Result::Err(message),
                        };
                        let substring = match system.expose_mut().translate_chain_checked(&widget.path){
                            Ok (substring) => substring,
                            Err (message) => return Result::Err(message),
                        };

                        let _path = MString::subtract_void(&source, &substring);

                        let new_key: String = match system.expose_mut().borrow_chain_mut(&source) {
                            Ok (set_widget) => {
                                set_widget.create_simple(true, position)
                            },
                            Err (message) => return Result::Err(message),
                        };
                        match system.expose_mut().borrow_chain_mut(&substring) {
                            Ok (register_widget) => {
                                register_widget.register_path(String::from(&tuple1.1), _path + "/" + &new_key)
                            },
                            Err (message) => return Result::Err(message),
                        };

                        Result::Ok(Widget::from_path(&(String::new() + &widget.path + "/" + &tuple1.1)))
                    }
                } else {
                    let mut new_local_path = String::from(&path) + "/";
                    match system.expose_mut().borrow_chain_checked_mut(&(String::from(&widget.path)+ "/" + &path)) {
                        Ok (set_widget) => {
                            new_local_path += &set_widget.create_simple(true, position);
                        },
                        Err (message) => return Err(String::from("CRASHED ON MAKING NEW BRANCH! #Error: ") + &message),
                    };
                    match system.expose_mut().borrow_chain_checked_mut(&widget.path) {
                        Ok (register_widget) => {
                            register_widget.register_path(String::from(&tuple1.1), new_local_path.clone())
                        },
                        Err (message) => return Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
                    };
                    Result::Ok(Widget::from_path(&(String::new() + &widget.path + "/" + &tuple1.1)))
                }
            },
        }
    }
    
    //OLD, REQUIRES REWORK
    fn map (&self, system: & Hierarchy) -> Result<String, String> {
        match system.expose().borrow_chain_checked(&self.path){
            Ok (reference) => {
                let list: Vec<&str> =  self.path.split('/').collect();
                let mut string = String::from(list[list.len()-1]);
                string = reference.map(string, 0);
                Ok(string)
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    fn map_debug (&self, system: & Hierarchy) -> Result<String, String> {
        match system.expose().borrow_chain_checked(&self.path){
            Ok (reference) => {
                let list: Vec<&str> =  self.path.split('/').collect();
                let mut string = String::from(list[list.len()-1]);
                string = reference.map_debug(string, 0);
                Ok(string)
            },
            Err (message) => Err(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    fn destroy (&self, system: &mut Hierarchy, path : &str) -> Outcome {
        match system.expose_mut().borrow_chain_checked_mut(&self.path){
            Ok (reference) => {
                reference.destroy_chain_checked(path)
            },
            Err (message) => Outcome::Fail(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }
    fn remove (&self, system: &mut Hierarchy, key : &str) -> Outcome {
        match system.expose_mut().borrow_chain_checked_mut(&self.path){
            Ok (reference) => {
                reference.remove_simple_checked(key)
            },
            Err (message) => Outcome::Fail(String::from("WIDGET NOT FOUND! #Error: ") + &message),
        }
    }


}