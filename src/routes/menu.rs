use std::borrow::Borrow;
use bevy_lunex::prelude::*;
use bevy::prelude::*;

use crate::UiComponent;
use crate::components as ui;


#[derive(Default)]
pub struct Menu;
impl UiComponent for Menu {
    fn construct(self, commands: &mut Commands, asset_server: &Res<AssetServer>, tree: &mut UiTree, path: impl Borrow<str>) -> Result<(), LunexError> {

        let menu = RelativeLayout::new().build(tree, "Menu")?;

        let image = SolidLayout::new()
            .with_scaling(SolidScale::Fill)
            .with_size(2560.0, 1440.0)
            .build(tree, menu.end("Background-Image"))?;
        commands.spawn(ImageElementBundle::new(image, ImageParams::default().with_depth(-0.5), asset_server.load("images/main_menu/screen_10.png"), Vec2::new(1920.0, 1080.0)));
        
        let board = SolidLayout::new()
            .with_size(807.0, 1432.0)
            .with_horizontal_anchor(-0.8)
            .build(tree, menu.end("Board"))?;
        commands.spawn(ImageElementBundle::new(&board, ImageParams::default(), asset_server.load("images/main_menu/board.png"), Vec2::new(807.0, 1432.0)));
        
        let boundary = RelativeLayout::new()
            .with_rel_1(Vec2::new(-5.0, 12.0))
            .with_rel_2(Vec2::new(105.0, 32.0))
            .build(tree, board.end("boundary"))?;

        let logo = SolidLayout::new()
            .with_size(1240.0, 381.0)
            .build(tree, boundary.end("Logo"))?;
        commands.spawn(ImageElementBundle::new(logo, ImageParams::default(), asset_server.load("images/main_menu/bevypunk2.png"), Vec2::new(1240.0, 381.0)));


        // Build buttons

        let list = RelativeLayout::new()
            .with_rel_1(Vec2::new(17.0, 33.0))
            .with_rel_2(Vec2::new(82.0, 79.0))
            .build(tree, board.end("list"))?;


        let mut segment = GridSegment::new();
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "CONTINUE"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "NEW GAME"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "LOAD GAME"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "SETTINGS"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "ADDITIONAL CONTENT"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "CREDITS"));
        segment.add_cell(GridCell::named(Vec2::new(10.0, 10.0), "QUIT GAME"));
        let widget_list = segment.add_gaps(1.0).build_in(tree, list, GridOrientation::Vertical)?;

        for x in widget_list {
            ui::Button::new(x.name()).construct(commands, asset_server, tree, x.end("Button"))?;
        }

        println!("{}", tree.tree());
        Ok(())
    }
}

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app;
    }
}