#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::prelude::*;

use crate::library::prelude::*;

use super::ui_container::{Container, PositionLayout};


//===========================================================================


#[derive(Component, Default)]
pub struct Hierarchy {
    pub width: f32,
    pub height: f32,
    branch: Branch,
}
impl Hierarchy {
    pub fn new () -> Hierarchy {
        let mut branch = Branch::new();
        branch.container.position_layout_set(Layout::Relative {
            relative_1: Vec2 { x: 0.0, y: 0.0 },
            relative_2: Vec2 { x: 100.0, y: 100.0 },
            ..Default::default()
        }.wrap());

        Hierarchy {
            width: 0.0,
            height: 0.0,
            branch,
        }
    }
    pub fn update (&mut self) {
        self.branch.cascade_update(Vec2::default(), self.width, self.height);
    }
    pub fn map (&self) -> String {
        let mut string = String::from("#HIERARCHY");
        string = self.branch.map(string, 0);
        string
    }
    pub fn map_debug (&self) -> String {
        let mut string = String::from("#HIERARCHY");
        string = self.branch.map_debug(string, 0);
        string
    }
    
    pub (in crate::library) fn expose (&self) -> & Branch {
        & self.branch
    }
    pub (in crate::library) fn expose_mut (&mut self) -> &mut Branch {
        &mut self.branch
    }
    
}

pub fn hierarchy_update(mut query: Query<&mut Hierarchy>, mut windows: Query<&mut Window>) {
    let window = windows.get_single_mut().unwrap();
    for mut system in &mut query {
        system.width = window.resolution.width();
        system.height = window.resolution.height();

        system.update();
    }
}


//===========================================================================


#[derive(Default)]
pub struct Branch {
    name: String,                                                                                                                           //Caches name for debug
    level: f32,                                                                                                                             //How deep it is located (For highlighting)
    container: Container,
    data: Option<Box<dyn Data + Send + Sync>>,
    //active: bool,

    pernament: Vec<Branch>,
    removable: HashMap<usize, Branch>,
    register: HashMap<String, String>,
}
impl Branch {
    //#USER EXPOSED CONTROL
    pub fn data_get (&self) -> &Option<Box<dyn Data + Send + Sync>> {                                                                
        &self.data
    }
    pub fn data_get_mut (&mut self) -> &mut Option<Box<dyn Data + Send + Sync>> {                                                                
        &mut self.data
    }
    pub fn container_get (&self) -> &Container {                                                                
        &self.container
    }
    pub fn container_get_mut (&mut self) -> &mut Container {                                                                
        &mut self.container
    }

    //#LIBRARY RECURSION CALLS
    pub (in crate::library) fn map (&self, mut string: String, level: u32) -> String {                                                      //This will cascade map registered branches
        for x in self.register.iter(){
            if let Ok (widget) = self.borrow_chain_checked(x.1){
                string += "\n  ";
                for _x in 0..level {
                    string += "|    ";
                };
                string += "|-> ";
                string += x.0;
                string = widget.map(string, level + 1);
            }
        }
        string
    }
    pub (in crate::library) fn map_debug (&self, mut string: String, level: u32) -> String {                                                //This will cascade map all branches with debug mode
        let mut done_widgets: HashMap<String, bool> = HashMap::new();
        for x in self.register.iter(){
            match self.borrow_chain_checked(x.1){
                Ok (widget) => {
                    string += "\n  ";
                    for _x in 0..level {
                        string += "|    ";
                    }
                    string += "|-> ";
                    string += x.0;
                    string += " (";
                    string += x.1;
                    string += ")";
                    string = widget.map_debug(string, level + 1);
                    done_widgets.insert(x.1.to_string(), true);
                },
                Err(..) => {
                    string += "\n  ";
                    for _x in 0..level {
                        string += "|    ";
                    }
                    string += "|-> ";
                    string += x.0;
                    string += " #[! Dangling pointer !]";
                },
            }
        }
        for i in 0..self.pernament.len() {
            if done_widgets.contains_key( &("#p".to_string() + &i.to_string())) {
                continue;
            }
            string += "\n  ";
            for _x in 0..level {
                string += "|    ";
            }
            string += "|-> #p";
            string += &i.to_string();
            string = self.pernament[i].map_debug(string, level + 1);
        }
        for x in self.removable.iter(){
            if done_widgets.contains_key( &("#r".to_string() + &x.0.to_string())) {
                continue;
            }
            string += "\n  ";
            for _x in 0..level {
                string += "|    ";
            }
            string += "|-> #r";
            string += &x.0.to_string();
            string = x.1.map_debug(string, level + 1);
        }
        string
    }
    pub (in crate::library) fn cascade_update (&mut self, point: Vec2, width: f32, height: f32) {                                           //This will cascade update all branches
        self.container.update(point, width, height);
        for i in 0..self.pernament.len() {
            let pos = self.container.position_get();
            self.pernament[i].cascade_update(pos.point_1, pos.width, pos.height);
        }
        for x in self.removable.iter_mut(){
            let pos = self.container.position_get();
            x.1.cascade_update(pos.point_1, pos.width, pos.height);
        }
    }

    //#LIBRARY MECHANISMS
    fn new () -> Branch {
        Branch {
            name: String::new(),
            level: 0.0,
            container: Container::new(),
            data: Option::None,

            pernament: Vec::new(),
            removable: HashMap::new(),
            register: HashMap::new(),
        }
    }

    pub (in crate::library) fn create_simple (&mut self, removable: bool, position: PositionLayout) -> String {                              //This creates unnamed Branch in one of the 2 registers and return string with ABSOLUTE local path
        if !removable {
            let ukey = self.pernament.len();
            let mut branch = Branch::new();
            branch.container.position_layout_set(position);
            self.pernament.push(branch);
            String::from("#p") + &ukey.to_string()
        } else {
            let mut ukey = 0;
            loop {
                if !self.removable.contains_key(&ukey) {break;};
                ukey += 1;
            };
            let mut branch = Branch::new();
            branch.container.position_layout_set(position);
            self.removable.insert(ukey, branch);
            String::from("#r") + &ukey.to_string()
        }
    }
    pub (in crate::library) fn create_simple_checked (&mut self, key: &str, position: PositionLayout) -> Result<String, String> {            //This decides if Branch should be removable or not and also checks for key collision and returns ABSOLUTE/RELATIVE local path
        if key.is_empty() {
            Result::Ok(self.create_simple(false, position))
        } else {
            match self.register.get(key){
                None => {
                    let path = self.create_simple(true, position);
                    self.register_path(String::from(key), path);
                    Result::Ok(String::from(key))
                },
                Some (..) => Result::Err(format!("The key '{}' is already in use!", &key).to_string()),
            }
        }
    }

    pub (in crate::library) fn register_path (&mut self, key: String, path: String){                                                         //This registers ABSOLUTE PATH for a key
        self.register.insert(key, path);
    }

    pub (in crate::library) fn translate_simple (&self, key: &str) -> Result<String, String> {                                               //This can take ONLY RELATIVE and return ABSOLUTE
        match self.register.get(key) {
            Some (value) => Result::Ok(String::from(value)),
            None => Result::Err(format!("The key '{}' is not in the register!", &key).to_string()),
        }
    }
    pub (in crate::library) fn translate_simple_checked (&self, key: &str) -> Result<String, String> {                                       //This can take RELATIVE/ABSOLUTE and return ABSOLUTE
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => Result::Ok(key.to_owned()),
                _ => self.translate_simple(key),
            }
            None => Result::Err(String::from("There is no key!")),
        }
    }
    pub (in crate::library) fn translate_chain (&self, keypath: &str) -> Result<String, String> {                                            //This can take chained RELATIVE path and return ABSOLUTE
        match keypath.split_once('/') {
            None => {
                self.translate_simple(keypath)
            },
            Some (tuple) => match self.translate_simple(tuple.0) {
                Ok (new_key) => match self.borrow_simple(&new_key) {
                    Ok (borrowed_widget) => match borrowed_widget.translate_chain(tuple.1) {
                        Ok (path_result) => Result::Ok(new_key.to_owned() + "/" + &path_result),
                        Err (message) => Result::Err(message),
                    },
                    Err (message) => Result::Err(message),
                },
                Err (message) => Result::Err(message),
            },
        }
    }
    pub (in crate::library) fn translate_chain_checked (&self, keypath: &str) -> Result<String, String> {                                    //This can take chained RELATIVE/ABSOLUTE path and return ABSOLUTE
        match keypath.split_once('/') {
            None => {
                self.translate_simple_checked(keypath)
            },
            Some (tuple) => match self.translate_simple_checked(tuple.0) {
                Ok (new_key) => match self.borrow_simple_checked(&new_key) {
                    Ok (borrowed_widget) => match borrowed_widget.translate_chain_checked(tuple.1) {
                        Ok (path_result) => Result::Ok(new_key.to_owned() + "/" + &path_result),
                        Err (message) => Result::Err(message),
                    },
                    Err (message) => Result::Err(message),
                },
                Err (message) => Result::Err(message),
            },
        }
    }

    pub (in crate::library) fn borrow_simple (&self, path: &str) -> Result<&Branch, String> {                                                //This can take ONLY ABSOLUTE and return reference
        match path.chars().nth(1) {
            Some (value) => {
                match value {
                    'p' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                if index >= self.pernament.len() {
                                    return Result::Err(format!("Pernament branch with index '{}' does not exist!", &index).to_string());
                                };
                                Result::Ok(&self.pernament[index])
                            },
                            Err (..) => Result::Err(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    'r' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                match self.removable.get(&index) {
                                    Some (widget) => {
                                        Result::Ok(widget)
                                    },
                                    None => Result::Err(format!("Removable branch with path '{}' does not exist!", &index).to_string()),
                                }
                            },
                            Err (..) => Result::Err(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    _ => Result::Err(format!("The second character '{}' in '{}' needs to be either 'r' or 'p' (Stands for storage stack)!", &value, &path).to_string()),
                }
            },
            None => Result::Err(format!("Path '{}' is missing information (Example: #r12)!", &path).to_string()),
        }
    }
    pub (in crate::library) fn borrow_simple_checked (&self, key: &str) -> Result<&Branch, String> {                                         //This can take RELATIVE/ABSOLUTE and return reference
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => self.borrow_simple(key),
                _ => match self.translate_simple(key){
                    Ok (new_key) => self.borrow_chain_checked(&new_key),
                    Err (message) => Result::Err(message),
                },
            }
            None => Result::Err(String::from("There is no key!")),
        }
    }
    pub (in crate::library) fn borrow_chain (&self, path: &str) -> Result<&Branch, String> {                                                 //This can take chained ABSOLUTE path and return reference
        match path.split_once('/') {
            None => {
                self.borrow_simple(path)
            },
            Some (tuple) => match self.borrow_simple(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.borrow_chain(tuple.1),
                Err (message) => Result::Err(message),
            },
        }
    }
    pub (in crate::library) fn borrow_chain_checked (&self, keypath: &str) -> Result<&Branch, String> {                                      //This can take chained ABSOLUTE/RELATIVE path and return reference
        match keypath.split_once('/') {
            None => {
                self.borrow_simple_checked(keypath)
            },
            Some (tuple) => match self.borrow_simple_checked(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.borrow_chain_checked(tuple.1),
                Err (message) => Result::Err(message),
            },
        }
    }

    pub (in crate::library) fn borrow_simple_mut (&mut self, path: &str) -> Result<&mut Branch, String> {                                    //This can take ONLY ABSOLUTE and return MUT reference
        match path.chars().nth(1) {
            Some (value) => {
                match value {
                    'p' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                if index >= self.pernament.len() {
                                    return Result::Err(format!("Pernament branch with index '{}' does not exist!", &index).to_string());
                                };
                                Result::Ok(&mut self.pernament[index])
                            },
                            Err (..) => Result::Err(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    'r' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                match self.removable.get_mut(&index) {
                                    Some (widget) => {
                                        Result::Ok(widget)
                                    },
                                    None => Result::Err(format!("Removable branch with path '{}' does not exist!", &index).to_string()),
                                }
                            },
                            Err (..) => Result::Err(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    _ => Result::Err(format!("The second character '{}' in '{}' needs to be either 'r' or 'p' (Stands for storage stack)!", &value, &path).to_string()),
                }
            },
            None => Result::Err(format!("Path '{}' is missing information (Example: #r12)!", &path).to_string()),
        }
    }
    pub (in crate::library) fn borrow_simple_checked_mut (&mut self, key: &str) -> Result<&mut Branch, String> {                             //This can take RELATIVE/ABSOLUTE and return MUT reference
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => self.borrow_simple_mut(key),
                _ => match self.translate_simple(key){
                    Ok (new_key) => self.borrow_chain_checked_mut(&new_key),
                    Err (message) => Result::Err(message),
                },
            }
            None => Result::Err(String::from("There is no key!")),
        }
    }
    pub (in crate::library) fn borrow_chain_mut (&mut self, path: &str) -> Result<&mut Branch, String> {                                     //This can take chained ABSOLUTE path and return MUT reference
        match path.split_once('/') {
            None => {
                self.borrow_simple_mut(path)
            },
            Some (tuple) => match self.borrow_simple_mut(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.borrow_chain_mut(tuple.1),
                Err (message) => Result::Err(message),
            },
        }
    }
    pub (in crate::library) fn borrow_chain_checked_mut (&mut self, keypath: &str) -> Result<&mut Branch, String> {                          //This can take chained ABSOLUTE/RELATIVE path and return MUT reference
        match keypath.split_once('/') {
            None => {
                self.borrow_simple_checked_mut(keypath)
            },
            Some (tuple) => match self.borrow_simple_checked_mut(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.borrow_chain_checked_mut(tuple.1),
                Err (message) => Result::Err(message),
            },
        }
    }

    pub (in crate::library) fn check_simple (&self, path: &str) -> bool {                                                                    //This can take ONLY ABSOLUTE and return reference
        match path.chars().nth(1) {
            Some (value) => {
                match value {
                    'p' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                if index >= self.pernament.len() {
                                    return false;
                                };
                                true
                            },
                            Err (..) => false,
                        }
                    },
                    'r' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                match self.removable.get(&index) {
                                    Some (widget) => true,
                                    None => false,
                                }
                            },
                            Err (..) => false,
                        }
                    },
                    _ => false,
                }
            },
            None => false,
        }
    }
    pub (in crate::library) fn check_simple_checked (&self, key: &str) -> bool {                                                             //This can take RELATIVE/ABSOLUTE and return reference
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => self.check_simple(key),
                _ => match self.translate_simple(key){
                    Ok (new_key) => self.check_chain_checked(&new_key),
                    Err (message) => false,
                },
            }
            None => false,
        }
    }
    pub (in crate::library) fn check_chain (&self, path: &str) -> bool {                                                                     //This can take chained ABSOLUTE path and return reference
        match path.split_once('/') {
            None => {
                self.check_simple(path)
            },
            Some (tuple) => match self.borrow_simple(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.check_chain(tuple.1),
                Err (..) => false,
            },
        }
    }
    pub (in crate::library) fn check_chain_checked (&self, keypath: &str) -> bool {                                                          //This can take chained ABSOLUTE/RELATIVE path and return reference
        match keypath.split_once('/') {
            None => {
                self.check_simple_checked(keypath)
            },
            Some (tuple) => match self.borrow_simple_checked(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.check_chain_checked(tuple.1),
                Err (..) => false,
            },
        }
    }

    pub (in crate::library) fn destroy_simple (&mut self, path: &str) -> Outcome {                                                           //This can take ONLY ABSOLUTE and return Option if the destruction succeded
        match path.chars().nth(1) {
            Some (value) => {
                match value {
                    'p' => Outcome::Fail(String::from("Widgets with no name are supposed to be permanent and cannot be destroyed directly!")),
                    'r' => {
                        match str::parse::<usize>(&path[2..]) {
                            Ok (index) => {
                                if !self.removable.contains_key(&index) {
                                    return Outcome::Fail(format!("Removable branch with key '{}' does not exist!", &index).to_string());
                                }
                                self.removable.remove(&index);
                                Outcome::Pass
                            },
                            Err (..) => Outcome::Fail(format!("The path '{}' is not a valid number!", &path).to_string()),
                        }
                    },
                    _ => Outcome::Fail(format!("The second character '{}' in '{}' needs to be either 'r' or 'p' (Stands for storage stack)!", &value, &path).to_string()),
                }
            },
            None => Outcome::Fail(format!("Path '{}' is missing information (Example: #r12)!", &path).to_string()),
        }
    }
    pub (in crate::library) fn destroy_simple_checked (&mut self, key: &str) -> Outcome {                                                    //This can take RELATIVE/ABSOLUTE and return Option if the destruction succeded
        match key.chars().next() {
            Some (_char) => match _char {
                '#' => self.destroy_simple(key),
                _ => match self.translate_simple(key){
                    Ok (new_key) => self.destroy_chain(&new_key),
                    Err (message) => Outcome::Fail(message),
                },
            }
            None => Outcome::Fail(String::from("There is no key!")),
        }
    }
    pub (in crate::library) fn destroy_chain (&mut self, path: &str) -> Outcome {                                                            //This can take chained ABSOLUTE path and return Option if the destruction succeded
        match path.split_once('/') {
            None => {
                self.destroy_simple(path)
            },
            Some (tuple) => match self.borrow_simple_mut(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.destroy_chain(tuple.1),
                Err (message) => Outcome::Fail(message),
            },
        }
    }
    pub (in crate::library) fn destroy_chain_checked (&mut self, keypath: &str) -> Outcome {                                                 //This can take chained ABSOLUTE/RELATIVE path and return Option if the destruction succeded
        match keypath.split_once('/') {
            None => {
                self.destroy_simple_checked(keypath)
            },
            Some (tuple) => match self.borrow_simple_checked_mut(tuple.0) {
                Ok (borrowed_widget) => borrowed_widget.destroy_simple_checked(tuple.1),
                Err (message) => Outcome::Fail(message),
            },
        }
    }

    pub (in crate::library) fn remove_simple_checked (&mut self, key: &str) -> Outcome {                                                     //This can take ONLY RELATIVE and return Option if the widget was destroyed and removed from register
        if self.register.contains_key(key) {
            match self.destroy_chain_checked(key) {
                Outcome::Pass => {
                    self.register.remove(key);
                    Outcome::Pass
                },
                Outcome::Fail (message) => Outcome::Fail(message),
            }
        } else {
            Outcome::Fail(format!("Widget registered as '{}' does not exist!", &key).to_string())
        }
    }
    
}


//===========================================================================
pub fn tween (value_1: f32, value_2: f32, slide: f32) -> f32 {
    let diff = value_2 - value_1;
    value_1 + diff * slide
}

pub trait Data {
    fn get_f32 (&self) -> f32 {
        0.0
    }
    fn get_vec2 (&self) -> Vec2 {
        Vec2::default()
    }
    fn get_vec3 (&self) -> Vec3 {
        Vec3::default()
    }
    fn get_vec4 (&self) -> Vec4 {
        Vec4::default()
    }
    fn get_bool (&self) -> bool {
        false
    }
    fn get_string (&self) -> String {
        String::new()
    }

    fn get_f32s (&self) -> Vec<f32> {
        Vec::new()
    }
    fn get_vec2s (&self) -> Vec<Vec2> {
        Vec::new()
    }
    fn get_vec3s (&self) -> Vec<Vec3> {
        Vec::new()
    }
    fn get_vec4s (&self) -> Vec<Vec4> {
        Vec::new()
    }
    fn get_bools (&self) -> Vec<bool> {
        Vec::new()
    }
    fn get_strings (&self) -> Vec<String> {
        Vec::new()
    }
    
    fn get_buffer (&self) -> Vec<u8> {
        Vec::new()
    }

    fn set_f32 (&mut self, value: f32) {}
    fn set_vec2 (&mut self, value: Vec2) {}
    fn set_vec3 (&mut self, value: Vec3) {}
    fn set_vec4 (&mut self, value: Vec4) {}
    fn set_bool (&mut self, value: bool) {}
    fn set_string (&mut self, value: String) {}

    fn set_f32s (&mut self, value: Vec<f32>) {}
    fn set_vec2s (&mut self, value: Vec<Vec2>) {}
    fn set_vec3s (&mut self, value: Vec<Vec3>) {}
    fn set_vec4s (&mut self, value: Vec<Vec4>) {}
    fn set_bools (&mut self, value: Vec<bool>) {}
    fn set_strings (&mut self, value: Vec<String>) {}

    fn set_buffer (&mut self, value: Vec<u8>) {}
}