use std::collections::VecDeque;
use bevy::{input::mouse::MouseMotion, window::{CursorGrabMode, PrimaryWindow}};

use crate::*;

pub const SMOOTH_SAMPLES_LEN: usize = 3;

#[derive(Resource, Default)]
pub struct MouseCapture {
    focus: bool,
    pub delta: Vec2,
    delta_buffer: VecDeque<Vec2>,
}
fn update_mouse_capture(mut mouse_motion_events: EventReader<MouseMotion>, mut capture: ResMut<MouseCapture>) {
    let mut delta: Vec2 = mouse_motion_events.read().map(|e| e.delta).sum();
    if !capture.focus { delta = Vec2::ZERO }
    while capture.delta_buffer.len() >= SMOOTH_SAMPLES_LEN { capture.delta_buffer.pop_front(); }
    capture.delta_buffer.push_back(delta);
    capture.delta = capture.delta_buffer.iter().fold(Vec2::ZERO, |sum, &vec| sum + vec) / capture.delta_buffer.len() as f32;
}
fn update_mouse_capture_focus(mut windows: Query<&mut Window, With<PrimaryWindow>>, capture: Res<MouseCapture>) {
    if capture.is_changed() {
        if let Ok(mut window) = windows.get_single_mut() {
            if capture.focus {
                window.cursor.grab_mode = CursorGrabMode::Confined;
                window.cursor.visible = false;
            } else {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
        }
    }
}
fn switch_mouse_capture_focus(keyboard_input: Res<ButtonInput<KeyCode>>, mut capture: ResMut<MouseCapture>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        capture.focus = !capture.focus;
    }
}


// #=====================#
// #=== MODULE PLUGIN ===#

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MouseCapture::default())
            .add_systems(Update, (update_mouse_capture, update_mouse_capture_focus, switch_mouse_capture_focus));
    }
}