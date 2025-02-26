use bevy::{app::PluginGroupBuilder, prelude::*, render::{settings::{PowerPreference, RenderCreation, WgpuSettings}, RenderPlugin}, window::{PresentMode, WindowMode, WindowResolution}};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_kira_audio::AudioPlugin;
use bevy_lunex::UiLunexPlugin;
use vleue_kinetoscope::AnimatedImagePlugin;


/// Plugin group implementing minimal default logic.
pub struct BevyPlugins;
impl PluginGroup for BevyPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Embedd all assets into the binary
        builder = builder.add(EmbeddedAssetPlugin { mode: PluginMode::ReplaceDefault });

        // Add default plugins
        builder = builder.add_group(DefaultPlugins);

        // Set window plugin
        builder = builder.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevypunk".into(),
                //mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                mode: WindowMode::Windowed,
                present_mode: PresentMode::AutoVsync,
                resolution: WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        });

        // Set render plugin to pick high performance GPU
        builder = builder.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(
                WgpuSettings {
                    power_preference: PowerPreference::HighPerformance,
                    ..default()
                }
            ),
            ..default()
        });

        // Disable the buildin pointers
        builder = builder.set(PointerInputPlugin { 
            is_mouse_enabled: true,
            is_touch_enabled: true,
        });

        // Add 3rd-party Bevy plugins
        builder = builder.add(AnimatedImagePlugin).add(AudioPlugin).add(UiLunexPlugin);

        // Return the plugin group
        builder
    }
}
