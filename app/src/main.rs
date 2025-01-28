use clap::Parser;
use bevy::utils::HashMap;
use bevy::core_pipeline::bloom::Bloom;
use bevy_embedded_assets::*;

pub(crate) use bevy::{prelude::*, sprite::Anchor};
pub(crate) use bevy_kira_audio::prelude::*;
pub(crate) use bevy_lunex::*;
pub(crate) use vleue_kinetoscope::*;

pub(crate) use game_movies::*;
pub(crate) use game_vfx::*;


// #==========================#
// #=== MAIN APP STRUCTURE ===#

/// Launch arguments for the Bevypunk game
#[derive(Parser, Debug)]
struct Args {
    /// Flag to skip the initial intro
    #[arg(short, long)]
    skip_intro: bool,
}

/// Priority assets loaded before the game start
#[derive(Resource, Default)]
pub struct PriorityAssets {
    video: HashMap<String, Handle<AnimatedImage>>,
}

/// Different app states for the Bevypunk game
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    /// Player intro movie
    IntroMovie,
    /// The game main menu
    MainMenu,
}

fn main() -> AppExit {
    // ____________________________________
    // ----- NEW APPLICATION INSTANCE -----
    let mut app = App::new();
    let args = Args::parse();

    // Bundle all game assets into the binary and add general plugins
    app.add_plugins(EmbeddedAssetPlugin { mode: PluginMode::ReplaceDefault });
    app.add_plugins((DefaultPlugins, AnimatedImagePlugin, AudioPlugin, UiLunexPlugin));
    //app.add_plugins(UiLunexDebugPlugin::new());

    // Set the correct app state
    if !args.skip_intro {
        app.insert_state(AppState::IntroMovie);
    } else {
        app.insert_state(AppState::MainMenu);
    }

    // ___________________________________
    // ----- PRIORITY ASSET LOADING  -----
    let mut priority_assets = PriorityAssets::default();

    // Load the game intro if required
    if !args.skip_intro {
        let intro = AnimatedImageLoader::load_now_from_bytes(include_bytes!("../../assets/movies/intro_720p.webp"),"webp", &mut app).expect("Priority load failed");
        priority_assets.video.insert("intro".to_string(), intro);
    }

    app.insert_resource(priority_assets);

    // _________________________________
    // ----- START THE APPLICATION -----
    app.add_systems(Startup, spawn_camera);
    app.add_systems(OnEnter(AppState::IntroMovie), spawn_intro);
    app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu);

    app.add_plugins((VFXPlugin, MoviePlugin));

    app.run()
}


// #======================#
// #=== THE GAME LOGIC ===#

fn spawn_camera(mut commands: Commands) {
    // Spawn the camera
    commands.spawn((Camera2d, Camera { hdr: true, ..default() }, Bloom::OLD_SCHOOL, VFXBloomFlicker, UiSourceCamera::<0>));
}

fn spawn_intro(mut commands: Commands, asset_server: Res<AssetServer>, priority_assets: Res<PriorityAssets>) {
    // Create UI
    commands.spawn((
        UiLayoutRoot,
        UiFetchFromCamera::<0>,
    )).with_children(|ui| {

        // Start the intro together with music
        ui.spawn((
            UiLayout::window().full().pack(),
            Movie::play(priority_assets.video.get("intro").unwrap().clone(), asset_server.load("audio/intro.ogg")).playback(MoviePlayback::Stop)

        // Add observer that will change the state once the movie ends
        )).observe(|_: Trigger<MovieEnded>, mut next: ResMut<NextState<AppState>>, mut commands: Commands, ui: Single<Entity, With<UiLayoutRoot>>| {

            // Despawn the UI and change the state
            commands.entity(*ui).despawn_recursive();
            next.set(AppState::MainMenu);
        });
    });
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    // Start playing the music
    audio.play(asset_server.load("audio/main_menu.ogg")).looped();

    // Create UI
    commands.spawn((
        UiLayoutRoot,
        // Make the UI synchronized with camera viewport size
        UiFetchFromCamera::<0>,
    )).with_children(|ui| {

        // Spawn the background
        ui.spawn((
            // You can name your entites for easier debug
            Name::new("Background"),
            UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fill).pack(),
            Sprite::from_image(asset_server.load("images/ui/background.png")),
        ));

        // Add the panel boundary
        ui.spawn((
            UiLayout::solid().size((881.0, 1600.0)).align_x(-0.74).pack(),
        )).with_children(|ui| {

            // Spawn the panel
            ui.spawn((
                Name::new("Panel"),
                UiLayout::window().x(Rl(50.0)).anchor(Anchor::TopCenter).size(Rl(105.0)).pack(),
                Sprite::from_image(asset_server.load("images/ui/panel_menu.png")),
            ));

            // Spawn the logo boundary
            ui.spawn((
                UiLayout::window().y(Rl(11.0)).size(Rl((105.0, 20.0))).pack(),
            )).with_children(|ui| {

                // Spawn the logo
                ui.spawn((
                    Name::new("Logo"),
                    UiLayout::solid().size((1240.0, 381.0)).pack(),
                    Sprite::from_image(asset_server.load("images/ui/title.png")),
                ));
            });

            // Spawn button boundary
            ui.spawn((
                UiLayout::window().pos(Rl((22.0, 33.0))).size(Rl((55.0, 34.0))).pack(),
            )).with_children(|ui| {

                // Spawn buttons
                let gap = 3.0;
                let size = 14.0;
                let mut offset = 0.0;
                for button in ["Continue", "New Game", "Load Game", "Settings", "Additional Content", "Credits", "Quit Game"] {

                    // Spawn the button
                    ui.spawn((
                        Name::new(button),
                        UiLayout::window().y(Rl(offset)).size(Rl((100.0, size))).pack(),
                    )).with_children(|ui| {

                        // Spawn the image
                        ui.spawn((
                            // You can define layouts for multiple states
                            UiLayout::new(vec![
                                (UiBase::id(), UiLayout::window().full()),
                                (UiHover::id(), UiLayout::window().x(Rl(10.0)).full())
                            ]),
                            // Like this you can enable a state
                            UiHover::new().forward_speed(20.0).backward_speed(4.0),
                            // You can specify colors for multiple states
                            UiColor::new(vec![
                                (UiBase::id(), Color::BEVYPUNK_RED.with_alpha(0.15)),
                                (UiHover::id(), Color::BEVYPUNK_YELLOW.with_alpha(1.2))
                            ]),
                            Sprite {
                                image: asset_server.load("images/ui/components/button_symetric_sliced.png"),
                                // Here we enable sprite slicing
                                image_mode: SpriteImageMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),
                                ..default()
                            },
                        )).with_children(|ui| {

                            // Spawn the text
                            ui.spawn((
                                // For text always use window layout to position it
                                UiLayout::window().pos((Rh(40.0), Rl(50.0))).anchor(Anchor::CenterLeft).pack(),
                                UiColor::new(vec![
                                    (UiBase::id(), Color::BEVYPUNK_RED),
                                    (UiHover::id(), Color::BEVYPUNK_YELLOW.with_alpha(1.2))
                                ]),
                                UiHover::new().forward_speed(20.0).backward_speed(4.0),
                                // You can control the size of the text
                                UiTextSize::from(Rh(60.0)),
                                // You can attach text like this
                                Text2d::new(button.to_ascii_uppercase()),
                                TextFont {
                                    font: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
                                    font_size: 64.0,
                                    ..default()
                                },
                            ));

                        // Enable the transition on hover
                        }).observe(hover_set::<Pointer<Over>, true>).observe(hover_set::<Pointer<Out>, false>);
                    });

                    offset += gap + size;
                }
            });
        });
    });
}
