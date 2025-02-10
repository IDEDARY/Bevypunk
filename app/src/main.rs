use std::time::Duration;

use clap::Parser;
use bevy::{core_pipeline::bloom::Bloom, render::view::RenderLayers};

pub(crate) use bevy::{prelude::*, sprite::Anchor};
pub(crate) use bevy_kira_audio::prelude::*;
pub(crate) use bevy_lunex::*;
pub(crate) use vleue_kinetoscope::*;

pub(crate) use game_loading::*;
pub(crate) use game_movies::*;
pub(crate) use game_preferences::*;
pub(crate) use game_vfx::*;


// #==========================#
// #=== MAIN APP STRUCTURE ===#

/// Launch arguments for the Bevypunk game
#[derive(Parser, Debug)]
struct Args {
    /// Flag to skip the initial intro
    #[arg(short, long)]
    skip_intro: bool,

    /// If to launch with low ram expectations
    #[arg(short, long)]
    lowram: bool,
}

/// Different app states for the Bevypunk game
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    /// Player intro movie
    IntroMovie,
    /// The game main menu
    MainMenu,
    /// The game creation
    NewGame,
    /// The game settings
    Settings,
}

fn main() -> AppExit {

    // ----- NEW APPLICATION INSTANCE -----

    let mut app = App::new();
    let args = Args::parse();

    // Add all Bevy plugins
    app.add_plugins(BevyPlugins);
    //app.add_plugins(UiLunexDebugPlugin::new());

    // Set the correct app state
    app.insert_state(if args.skip_intro { AppState::MainMenu } else { AppState::IntroMovie });


    // ----- PRIORITY ASSET LOADING  -----

    let mut priority_assets = PriorityAssets::default();

    // Load the game intro if required
    if !args.skip_intro {
        let intro = AnimatedImageLoader::load_now_from_bytes(
            if args.lowram { include_bytes!("../../assets/movies/intro_720p.webp") } else { include_bytes!("../../assets/movies/intro_1080p.webp") },
            "webp", &mut app).expect("Priority load failed");
        priority_assets.video.insert("intro".to_string(), intro);
    }

    app.insert_resource(priority_assets);
    app.add_systems(PreStartup, preload);


    // ----- START THE APPLICATION -----

    app.add_systems(Startup, spawn_camera);
    app.add_systems(OnEnter(AppState::IntroMovie), IntroScene::spawn).add_systems(OnExit(AppState::IntroMovie), despawn_scene::<IntroScene>);
    app.add_systems(OnEnter(AppState::MainMenu), MainMenuScene::spawn).add_systems(OnExit(AppState::MainMenu), despawn_scene::<MainMenuScene>);
    app.add_systems(OnEnter(AppState::NewGame), NewGameScene::spawn).add_systems(OnExit(AppState::NewGame), despawn_scene::<NewGameScene>);
    app.add_systems(OnEnter(AppState::Settings), SettingsScene::spawn).add_systems(OnExit(AppState::Settings), despawn_scene::<SettingsScene>);

    app.add_plugins((VFXPlugin, MoviePlugin));

    app.run()
}


// #======================#
// #=== THE GAME LOGIC ===#

/// This system is run in PreStartup. It locks some assets from being freed when not used.
fn preload(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn an entity with these assets, so that Bevy does not unload these assets when nobody is using them
    commands.spawn(AssetLock { assets: vec![
        asset_server.load_folder("fonts").untyped(),
        asset_server.load_folder("images/ui").untyped()
    ]});
    // This is good to reduce poping-in of important assets, such as UI, fonts, etc.
    commands.spawn(AssetLock { assets: vec![
        asset_server.load::<AudioSource>("audio/intro.ogg").untyped(),
        asset_server.load::<AudioSource>("audio/main_menu.ogg").untyped(),
    ]});
}

/// This system spawns & setups the basic camera with cursor
fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>, mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>) {
    // Spawn the camera
    commands.spawn((
        Camera2d, Camera { hdr: true, ..default() }, Bloom::OLD_SCHOOL, VFXBloomFlicker, UiSourceCamera::<0>, Transform::from_translation(Vec3::Z * 1000.0),
    )).with_children(|cam| {

        // Spawn cursor
        /* cam.spawn ((
            Cursor2d::new()
                .set_index(bevy::window::SystemCursorIcon::Default, 0, (14.0, 14.0))
                .set_index(bevy::window::SystemCursorIcon::Pointer, 1, (10.0, 12.0))
                .set_index(bevy::window::SystemCursorIcon::Grab, 2, (40.0, 40.0)),

            // Change the scale
            Transform::from_scale(Vec3::new(0.45, 0.45, 1.0)),

            // Change the sprite
            Sprite {
                image: asset_server.load("images/ui/cursor.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout.add(TextureAtlasLayout::from_grid(UVec2::splat(80), 3, 1, None, None)),
                    index: 0,
                }),
                color: Color::BEVYPUNK_YELLOW.with_alpha(1.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
        )); */
/*
        // Spawn cursor
        cam.spawn ((
            Cursor2d::new()
                .set_index(bevy::window::SystemCursorIcon::Default, 0, (14.0, 14.0))
                .set_index(bevy::window::SystemCursorIcon::Pointer, 1, (10.0, 12.0))
                .set_index(bevy::window::SystemCursorIcon::Grab, 2, (40.0, 40.0)),

            // Change the scale
            Transform::from_scale(Vec3::new(0.45, 0.45, 1.0)),

            // Change the sprite
            Sprite {
                image: asset_server.load("images/ui/cursor.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout.add(TextureAtlasLayout::from_grid(UVec2::splat(80), 3, 1, None, None)),
                    index: 0,
                }),
                color: Color::BEVYPUNK_YELLOW.with_alpha(1.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            GamepadCursor::new(),
        ));

        // Spawn cursor
        cam.spawn ((
            Cursor2d::new()
                .set_index(bevy::window::SystemCursorIcon::Default, 0, (14.0, 14.0))
                .set_index(bevy::window::SystemCursorIcon::Pointer, 1, (10.0, 12.0))
                .set_index(bevy::window::SystemCursorIcon::Grab, 2, (40.0, 40.0)),

            // Change the scale
            Transform::from_scale(Vec3::new(0.45, 0.45, 1.0)),

            // Change the sprite
            Sprite {
                image: asset_server.load("images/ui/cursor.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout.add(TextureAtlasLayout::from_grid(UVec2::splat(80), 3, 1, None, None)),
                    index: 0,
                }),
                color: Color::BEVYPUNK_YELLOW.with_alpha(1.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            GamepadCursor::new(),
        ));

        // Spawn cursor
        cam.spawn ((
            Cursor2d::new()
                .set_index(bevy::window::SystemCursorIcon::Default, 0, (14.0, 14.0))
                .set_index(bevy::window::SystemCursorIcon::Pointer, 1, (10.0, 12.0))
                .set_index(bevy::window::SystemCursorIcon::Grab, 2, (40.0, 40.0)),

            // Change the scale
            Transform::from_scale(Vec3::new(0.45, 0.45, 1.0)),

            // Change the sprite
            Sprite {
                image: asset_server.load("images/ui/cursor.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout.add(TextureAtlasLayout::from_grid(UVec2::splat(80), 3, 1, None, None)),
                    index: 0,
                }),
                color: Color::BEVYPUNK_YELLOW.with_alpha(1.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            GamepadCursor::new(),
        ));

        // Spawn cursor
        cam.spawn ((
            Cursor2d::new()
                .set_index(bevy::window::SystemCursorIcon::Default, 0, (14.0, 14.0))
                .set_index(bevy::window::SystemCursorIcon::Pointer, 1, (10.0, 12.0))
                .set_index(bevy::window::SystemCursorIcon::Grab, 2, (40.0, 40.0)),

            // Change the scale
            Transform::from_scale(Vec3::new(0.45, 0.45, 1.0)),

            // Change the sprite
            Sprite {
                image: asset_server.load("images/ui/cursor.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout.add(TextureAtlasLayout::from_grid(UVec2::splat(80), 3, 1, None, None)),
                    index: 0,
                }),
                color: Color::BEVYPUNK_YELLOW.with_alpha(1.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            GamepadCursor::new(),
        ));

        // Spawn cursor
        cam.spawn ((
            Cursor2d::new()
                .set_index(bevy::window::SystemCursorIcon::Default, 0, (14.0, 14.0))
                .set_index(bevy::window::SystemCursorIcon::Pointer, 1, (10.0, 12.0))
                .set_index(bevy::window::SystemCursorIcon::Grab, 2, (40.0, 40.0)),

            // Change the scale
            Transform::from_scale(Vec3::new(0.45, 0.45, 1.0)),

            // Change the sprite
            Sprite {
                image: asset_server.load("images/ui/cursor.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas_layout.add(TextureAtlasLayout::from_grid(UVec2::splat(80), 3, 1, None, None)),
                    index: 0,
                }),
                color: Color::BEVYPUNK_YELLOW.with_alpha(1.0),
                anchor: Anchor::TopLeft,
                ..default()
            },
            GamepadCursor::new(),
        ));
 */
    });
}

/// This is a generic system that will despawn all entities with attached component S.
fn despawn_scene<S: Component>(mut commands: Commands, query: Query<Entity, With<S>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}


#[derive(Component)]
struct IntroScene;
impl IntroScene {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, priority_assets: Res<PriorityAssets>) {
        // Create UI
        commands.spawn((
            UiLayoutRoot,
            // Make the UI synchronized with camera viewport size
            UiFetchFromCamera::<0>,
            // A scene marker for later mass scene despawn, not UI related
            IntroScene
        )).with_children(|ui| {
    
            // Start the intro together with music
            ui.spawn((
                UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fill).pack(),
                Movie::play(priority_assets.video.get("intro").unwrap().clone(), asset_server.load("audio/intro.ogg")).playback(MoviePlayback::Stop)
    
            // Add observer that will change the state once the movie ends
            )).observe(|_: Trigger<MovieEnded>, mut next: ResMut<NextState<AppState>>| {
                next.set(AppState::MainMenu);
            });
        });
    }
}


#[derive(Component)]
struct MainMenuScene;
impl MainMenuScene {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
        // Start playing the music
        audio.play(asset_server.load("audio/main_menu.ogg")).looped().fade_in(AudioTween::new(Duration::new(2, 0), AudioEasing::OutPowf(2.0)));
    
        // Create UI
        commands.spawn((
            UiLayoutRoot,
            // Make the UI synchronized with camera viewport size
            UiFetchFromCamera::<0>,
            // A scene marker for later mass scene despawn, not UI related
            MainMenuScene
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
                        let mut button_entity = ui.spawn((
                            Name::new(button),
                            UiLayout::window().y(Rl(offset)).size(Rl((100.0, size))).pack(),
                        ));
                        button_entity.with_children(|ui| {
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
                                // Make sure it does not cover the bounding zone of parent
                                PickingBehavior::IGNORE,
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
                                    // Make sure it does not cover the bounding zone of parent
                                    PickingBehavior::IGNORE,
                                ));
                            });
    
                        // Enable the transition on hover
                        }).observe(hover_set::<Pointer<Over>, true>).observe(hover_set::<Pointer<Out>, false>);
                        
                        // Assign a functionality to the buttons
                        match button {
                            "New Game" => {
                                button_entity.observe(|_: Trigger<Pointer<Click>>, mut next: ResMut<NextState<AppState>>| {
                                    // Change the state to settings
                                    next.set(AppState::NewGame);
                                });
                            },
                            "Settings" => {
                                button_entity.observe(|_: Trigger<Pointer<Click>>, mut next: ResMut<NextState<AppState>>| {
                                    // Change the state to settings
                                    next.set(AppState::Settings);
                                });
                            },
                            "Quit Game" => {
                                button_entity.observe(|_: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
                                    // Close the app
                                    exit.send(AppExit::Success);
                                });
                            },
                            _ => {
                                button_entity.observe(|c_trigger: Trigger<Pointer<Click>>, c_button: Query<NameOrEntity, With<UiLayout>>| {
                                    info!("Clicked: {}", c_button.get(c_trigger.entity()).unwrap());
                                });
                            }
                        }
                        
                        offset += gap + size;
                    }
                });
            });
        });
    }
}


#[derive(Component)]
struct NewGameScene;
impl NewGameScene {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Create UI
        commands.spawn((
            UiLayoutRoot,
            // Make the UI synchronized with camera viewport size
            UiFetchFromCamera::<0>,
            // A scene marker for later mass scene despawn, not UI related
            NewGameScene
        )).with_children(|ui| {
    
            // Spawn the background
            ui.spawn((
                Name::new("Background"),
                UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fill).pack(),
                Sprite::from_image(asset_server.load("images/ui/background.png")),
            ));
        });
    }
}


#[derive(Component)]
struct SettingsScene;
impl SettingsScene {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
        
        // Create the transparent render texture
        let image_handle = images.add(Image::clear_render_texture());
        
        // Create embedd camera that will render to the texture
        let texture_camera = commands.spawn((
            Camera2d, Camera::clear_render_to(image_handle.clone()),
            // This filters out all the 
            RenderLayers::layer(1),
            // A scene marker for later mass scene despawn, not UI related
            SettingsScene
        )).id();
        
        // Create UI
        commands.spawn((
            UiLayoutRoot,
            // Make the UI synchronized with camera viewport size
            UiFetchFromCamera::<0>,
            // A scene marker for later mass scene despawn, not UI related
            SettingsScene
        )).with_children(|ui| {
    
            // Spawn the background
            ui.spawn((
                Name::new("Background"),
                UiLayout::solid().size((1920.0, 1080.0)).scaling(Scaling::Fill).pack(),
                Sprite::from_image(asset_server.load("images/ui/background.png")),
            ));
    
            // Spawn the settings content
            ui.spawn((
                UiLayout::solid().size((3.0, 3.0)).align_y(-1.0).pack(),
            )).with_children(|ui| {
    
                // Spawn the tab bar
                ui.spawn((
                    UiLayout::window().size(Rl((100.0, 8.0))).pack(),
                )).with_children(|ui| {
    
                    // Spawn left chevron
                    ui.spawn((
                        Name::new("Chevron Left"),
                        UiLayout::window().pos(Rl((5.0, 50.0))).anchor(Anchor::Center).size(Rh(35.0)).pack(),
                        Sprite::from_image(asset_server.load("images/ui/components/chevron_left.png")),
                        UiHover::new().forward_speed(20.0).backward_speed(20.0).curve(|v| v.round()),
                        UiColor::new(vec![
                            (UiBase::id(), Color::BEVYPUNK_RED),
                            (UiHover::id(), Color::BEVYPUNK_BLUE.with_alpha(1.2))
                        ]),
                    )).observe(hover_set::<Pointer<Over>, true>).observe(hover_set::<Pointer<Out>, false>);
    
                    // Spawn right chevron
                    ui.spawn((
                        Name::new("Chevron Right"),
                        UiLayout::window().pos(Rl((95.0, 50.0))).anchor(Anchor::Center).size(Rh(35.0)).pack(),
                        Sprite::from_image(asset_server.load("images/ui/components/chevron_right.png")),
                        UiHover::new().forward_speed(20.0).backward_speed(20.0).curve(|v| v.round()),
                        UiColor::new(vec![
                            (UiBase::id(), Color::BEVYPUNK_RED),
                            (UiHover::id(), Color::BEVYPUNK_BLUE.with_alpha(1.2))
                        ]),
                    )).observe(hover_set::<Pointer<Over>, true>).observe(hover_set::<Pointer<Out>, false>);
    
                    // Spawn the control bar
                    ui.spawn((
                        UiLayout::window().x(Rl(10.0)).size(Rl((80.0, 100.0))).pack(),
                    )).with_children(|ui| {
                        
                        let categories = ["Controls", "Sound", "Graphics", "Window"];
                        let pos = 100.0 / categories.len() as f32;
                        for (i, category) in categories.into_iter().enumerate() {
    
                            // Spawn the button
                            ui.spawn((
                                Name::new(category),
                                UiLayout::window().x(Rl(pos * i as f32)).size(Rl((pos, 100.0))).pack(),
                            )).with_children(|ui| {
                                
                                // Spawn the background
                                ui.spawn((
                                    UiLayout::window().full().y(Rl(10.0)).height(Rl(80.0)).pack(),
                                    UiHover::new().forward_speed(20.0).backward_speed(5.0),
                                    UiColor::new(vec![
                                        (UiBase::id(), Color::BEVYPUNK_RED.with_alpha(0.0)),
                                        (UiHover::id(), Color::BEVYPUNK_RED.with_alpha(0.4))
                                    ]),
                                    Sprite {
                                        image: asset_server.load("images/ui/components/button_symetric_sliced.png"),
                                        image_mode: SpriteImageMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),
                                        ..default()
                                    },
                                    PickingBehavior::IGNORE,
                                )).with_children(|ui| {
    
                                    // Spawn the text
                                    ui.spawn((
                                        UiLayout::window().pos(Rl(50.0)).anchor(Anchor::Center).pack(),
                                        UiColor::new(vec![
                                            (UiBase::id(), Color::BEVYPUNK_RED),
                                            (UiHover::id(), Color::BEVYPUNK_BLUE.with_alpha(1.2))
                                        ]),
                                        UiHover::new().forward_speed(20.0).backward_speed(20.0).curve(|v| v.round()),
                                        UiTextSize::from(Rh(50.0)),
                                        Text2d::new(category.to_ascii_uppercase()),
                                        TextFont {
                                            font: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
                                            font_size: 64.0,
                                            ..default()
                                        },
                                        PickingBehavior::IGNORE,
                                    ));
                                });
                            
                            // Add the observers
                            }).observe(hover_set::<Pointer<Over>, true>).observe(hover_set::<Pointer<Out>, false>);
                        }
        
                    });
    
                });

                // Spawn the Bevy UI embedd
                ui.spawn((
                    UiLayout::boundary().y1(Rl(10.0)).pos2(Rl(100.0)).pack(),
                    Sprite::from_image(image_handle),
                ));

            });
        });

        // The Bevy UI nodes must be here to work
        commands.spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            // Render this ui to our new camera
            TargetCamera(texture_camera),
            // A scene marker for later mass scene despawn, not UI related
            SettingsScene
        )).with_children(|parent| {
            parent.spawn((
                Text::new("This is a Bevy UI"),
                TextFont {
                    font_size: 64.0,
                    font: asset_server.load("fonts/rajdhani/Rajdhani-Medium.ttf"),
                    ..default()
                },
                TextColor::WHITE,
            ));
        });

    }
}
