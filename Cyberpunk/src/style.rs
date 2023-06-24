use bevy::prelude::*;
use crate::library::prelude::*;

//# This function constructs the Hierarchy and layout of the main menu.
pub fn create_main_menu() -> Hierarchy {

    let mut system = Hierarchy::new();

    //# Create APP widget
    let _app = Widget::new(&mut system, "App", Layout::Relative {
        relative_1: Vec2 { x: 0.0, y: 0.0 },
        relative_2: Vec2 { x: 100.0, y: 100.0 },
        ..Default::default()
    }.wrap()).unwrap();

    //# Create HANDLE in WIDGET
    let _handle = Widget::new_in(&mut system, &_app, "Handle", Layout::Window {
        relative: Vec2 { x: -5.0, y: -5.0 },
        width_relative: 110.0,
        height_relative: 110.0,
        ..Default::default()
    }.wrap()).unwrap();

    //# Create BACKGROUND in HANDLE
    let _background = Widget::new_in(&mut system, &_handle, "Background", Layout::Solid {
        width: 2560,
        height: 1440,
        scaling: Scale::Fill,
        ..Default::default()
    }.wrap()).unwrap();

    //# Create BOARD in WIDGET
    let _board = Widget::new_in(&mut system, &_app, "Board", Layout::Solid {
        width: 807,
        height: 1432,
        horizontal_anchor: -0.80,
        scaling: Scale::Fit,
        ..Default::default()
    }.wrap()).unwrap();

    //# Create nameless widget in BOARD (useful when widget is not important and is used for layout purposes only (no image, not interactive), helps with abstractions)
    //# All nameless widgets are given the name "#pNUMBER", with number being the order they were created. Nameless widgets are hidden from mapping.
    let _logo_boundary = Widget::new_in(&mut system, &_board, "", Layout::Relative {
        relative_1: Vec2 { x: -5.0, y: 70.0 },
        relative_2: Vec2 { x: 105.0, y: 85.0 },
        ..Default::default()
    }.wrap()).unwrap();

    //# Create LOGO in nameless widget and register LOGO under BOARD (it will be Board/Logo instead Board/nameless/Logo)
    let _logo = Widget::new_in(&mut system, &_board, "#p0/Logo", Layout::Solid {
        width: 681,
        height: 166,
        scaling: Scale::Fit,
        ..Default::default()
    }.wrap()).unwrap();

    //# Create LOGOSHADOW in LOGO
    let _logo_boundary = Widget::new_in(&mut system, &_logo, "LogoShadow", Layout::Relative {
        relative_1: Vec2 { x: -5.0, y: -10.0 },
        relative_2: Vec2 { x: 105.0, y: 110.0 },
        ..Default::default()
    }.wrap()).unwrap();


    //################################################################################
    //# == Button Layout ==
    //# Here we will create a ButtonList widget which will contain all the buttons.

    //# Create BUTTONLIST in BOARD
    let _button_list = Widget::new_in(&mut system, &_board, "ButtonList", Layout::Relative {
        relative_1: Vec2 { x: 17.0, y: 21.0 },
        relative_2: Vec2 { x: 82.0, y: 66.0 },
        ..Default::default()
    }.wrap()).unwrap();

    //# Create a list with names for iteration
    let button_list = ["continue", "new_game", "load_game", "settings", "additional_content", "credits", "quit_game"];
    
    //# Create buttons in BUTTONLIST
    let step = 2.0/button_list.len() as f32;        //Distribute the containers equally
    for i in 0..button_list.len() {


        //# Create a BUTTON widget that will be used as boundary for input detection only
        let button = Widget::new_in(&mut system, &_button_list, button_list[i], Layout::Solid {
            width: 532,
            height: 75,
            scaling: Scale::Fit,
            vertical_anchor: 1.0 - step * i as f32,      //Where should the container be on Y axis (range: 1.0 to -1.0)
            ..Default::default()
        }.wrap()).unwrap();

        //# Create a nameless button that we will style and animate under BUTTON widget
        let button_decor = Widget::new_in(&mut system, &button, "", Layout::Window {
            width_relative: 100.0,
            height_relative: 100.0,
            ..Default::default()
        }.wrap()).unwrap();

        //# Create a data stored in hierarchy for sharing
        //#
        //# !!! THIS IS [WIP] !!! subject to change, currently it is an ugly solution, I alredy have improvements in mind.
        //#
        let data = button_decor.fetch_mut(&mut system, "").unwrap().data_get_mut();
        *data = Option::Some(Box::new(ButtonSynchronize {..Default::default()}));
    }

    //################################################################################
    //# == Hierarchy Debug ==
    //# This will print out both "normal" and "debug" maps (It's like "ls" command on Linux). The difference is that "debug" will also print out "nameless" widgets.
    //# "Nameless" widgets are hidden because they are NOT IMPORTANT to the main functionality of the system, but are there only for layout purposes.
    //# Displaying them would be considered overwhelming.
    println!("{}", system.map_debug());
    println!("{}", system.map());

    //# Return the finished system
    system

}


//################################################################################
//# == Button Logic ==
//# These two components and systems do all the button logic. Due to Bevy ECS, there is no "clean" way of modifying values of specific entities. (At least that I am not aware of)
//# So because each of the buttons are made of a mix of 2 entities that interact between each other, I save the changes of one entity to the Hierarchy as metadata and the other
//# entity fetches that data and synchronizes itself. This way there is a direct access to data, no looping over querries and finding corresponding entity, etc.
//# Might not be as much of an ECS solution as people want but it works and it is nice and simple. Sometimes mix of both worlds is the best solution.

//# The struct that is used for synchronization through Hierarchy as a trait object (You need to implement destructurizing by yourself, might want to use "Deku" crate in the future)
//# I know its a work around right now, but maybe somebody will propose a better solution how to push and pull data from trait object
#[derive(Default)]
struct ButtonSynchronize { pub alpha: f32, pub window_x: f32, pub color_slider: f32 }
impl Data for ButtonSynchronize {
    fn get_f32s (&self) -> Vec<f32> {
        vec![self.alpha, self.window_x, self.color_slider]
    }
    fn set_f32s (&mut self, value: Vec<f32>) {
        self.alpha = value[0];
        self.window_x = value[1];
        self.color_slider = value[2];
    }
}

//# The main entitity that will interact with cursor (Hitbox)
#[derive(Component)]
pub struct MainMenuButton ();
fn button_update(mut systems: Query<&mut Hierarchy>, cursors: Query<&Cursor>, mut query: Query<(&mut Widget, &MainMenuButton)>) {
    
    //# Get Hierarchy and cursor
    let mut system = systems.get_single_mut().unwrap();
    let cursor = cursors.get_single().unwrap();

    //# Loop through all widgets in the query (MainMenuButton)
    for (widget, _) in &mut query {

        //# Check if the cursor is within the current widget boundaries
        if widget.is_within(&system, "", cursor.position_screen()).unwrap(){

            //# Fetch the nameless widget data from Hierarchy and update it (Image alpha and layout of the decoration widget)
            match widget.fetch_mut(&mut system, "#p0").unwrap().data_get_mut() {
                Option::None => (),
                Option::Some ( _box ) => {
                    _box.set_f32s(vec![0.8, 5.0, 1.0]);
                }
            }

        }
    }
}

//# The secondary entity that will get updated by the main entity
#[derive(Component)]
pub struct MainMenuButtonDecoration ();
fn button_update_decoration(mut systems: Query<&mut Hierarchy>, mut query: Query<(&Widget, &mut Sprite, &mut Children, &MainMenuButtonDecoration)>, mut text_query: Query<&mut Text>) {
    
    //# Get Hierarchy
    let mut system = systems.get_single_mut().unwrap();

    //# Loop through all widgets in the query (MainMenuButtonDecoration)
    for (widget, mut sprite, children,  _) in &mut query {

        //# Fetch the current widget data from Hierarchy and synchronize itself
        match widget.fetch_mut(&mut system, "").unwrap().data_get_mut() {
            Option::None => (),
            Option::Some ( _box ) => {

                //# Destructurize the data (In the future I want to use crate like "Deku" to do this step for you)
                let data_pull = _box.get_f32s();
                let mut alpha = data_pull[0];
                let mut window_x = data_pull[1];
                let mut color_slider = data_pull[2];

                //# Smooth alpha transition
                if alpha > 0.0 {alpha -= 0.03} else {alpha = 0.0}

                //# Smooth layout transition
                if window_x > 0.0 {window_x -= 1.0} else {window_x = 0.0}

                //# Smooth text color transition
                if color_slider > 0.0 {color_slider -= 0.03} else {color_slider = 0.0}

                //# Synchronize
                _box.set_f32s(vec![alpha, window_x, color_slider]);

                //# Update sprite, layout and text color
                sprite.color.set_a(alpha);
                let window = widget.fetch_layout_mut(&mut system, "").unwrap().expect_window_mut();
                window.relative.x = window_x;
                
                for child in &children {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        text.sections[0].style.color = Color::rgba (
                            tween(204./255., 42./255., color_slider),
                            tween(56./255., 237./255., color_slider),
                            tween(51./255., 247./255., color_slider),
                            1.5
                        )
                    }
                }

            }
        }
    }
}

//# Wrap it into plugin for code clarity
pub struct ButtonPlugin;
impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(button_update)
            .add_system(button_update_decoration);
    }
}


//################################################################################
//# == Smooth Wiggle effect ==
//# Here are just some basic systems to update widgets layout in a stacked sine wawe to immitate natural camera movement
#[derive(Component, Default)]
pub struct SmoothWiggle {
    pub x: f32,
    pub y: f32,
}
fn smooth_wiggle (mut query: Query<(&mut SmoothWiggle, &mut Transform)>) {
    for (mut smoothslider, mut transform) in &mut query {
        smoothslider.x += 0.005;
        smoothslider.y += 0.003;
        transform.translation.x = smoothslider.x.sin()*9.;
        transform.translation.y = smoothslider.y.sin()*3.;
    }
}
fn smooth_wiggle_widget (mut query: Query<(&mut SmoothWiggle, &Widget)>, mut systems: Query<&mut Hierarchy>) {
    let mut system = systems.get_single_mut().unwrap();
    for (mut smoothslider, widget) in &mut query {
        
        let pos = widget.fetch_layout_mut(&mut system, "").unwrap().expect_window_mut();
        smoothslider.x += 0.007;
        smoothslider.y += 0.002;

        pos.relative.x = -5.0 + smoothslider.x.sin()*1.3*2.;
        pos.relative.y = -5.0 + smoothslider.y.sin()*1.0*2.;
    }
}

//# Wrap it into plugin for code clarity
pub struct WigglePlugin;
impl Plugin for WigglePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(smooth_wiggle)
            .add_system(smooth_wiggle_widget);
    }
}