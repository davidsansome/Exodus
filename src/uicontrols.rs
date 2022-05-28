/// This module contains UI elements and styles that are reusable throughout the program
use bevy::prelude::*;

/// The height of the Navbar
pub const NAVBAR_HEIGHT: f32 = 32.0;
/// The text used for the Navbar Back Button
pub const NAVBAR_BACK_TEXT: &str = "\u{300a}";
/// The text used for the Play Button
pub const PLAY_TEXT: &str = "\u{300b}";
/// The text used for the Delete Button
pub const DELETE_TEXT: &str = "\u{2020}";
/// The text used for the Delete Button
pub const EDIT_TEXT: &str = "E";

pub struct MenuMaterials {
    pub root: UiColor,
    pub border: UiColor,
    pub menu: UiColor,
    pub button: UiColor,
    pub button_hovered: UiColor,
    pub button_pressed: UiColor,
    pub navbar: UiColor,
    pub button_text: Color,
}

impl FromWorld for MenuMaterials {
    fn from_world(_: &mut World) -> Self {
        MenuMaterials {
            root: Color::NONE.into(),
            border: Color::rgb(0.65, 0.65, 0.65).into(),
            menu: Color::rgb(0.15, 0.15, 0.15).into(),
            button: Color::rgb(0.15, 0.15, 0.15).into(),
            button_hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            button_pressed: Color::rgb(0.35, 0.75, 0.35).into(),
            navbar: Color::rgb(0.05, 0.05, 0.05).into(),
            button_text: Color::WHITE,
        }
    }
}

fn button_system(
    materials: Res<MenuMaterials>,
    mut buttons: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut uicolor) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *uicolor = materials.button_pressed.into(),
            Interaction::Hovered => *uicolor = materials.button_hovered.into(),
            Interaction::None => *uicolor = materials.button.into(),
        }
    }
}

/// Initialize a new Button with 100% size
pub fn button(materials: &Res<MenuMaterials>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.button.clone(),
        ..Default::default()
    }
}

/// Initialize a new Button Text
pub fn button_text(asset_server: &Res<AssetServer>, materials: &Res<MenuMaterials>, label: &str) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        text: Text::with_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/PublicPixel.ttf"),
                font_size: 20.0,
                color: materials.button_text.clone(),
            },
            Default::default(),
        ),
        ..Default::default()
    };
}

///
/// Initialize a Top Menu Bar, which is a navigation bar that can be filled with navigation buttons.
/// This bar has a fixed height and a variable width.
pub fn top_menu_container(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(NAVBAR_HEIGHT)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            ..Default::default()
        },
        color: materials.navbar.into(),
        ..Default::default()
    }
}

pub fn navbar_button_container(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(NAVBAR_HEIGHT), Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.navbar.clone(),
        ..Default::default()
    }
}

pub struct UiControlsPlugin;

impl Plugin for UiControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MenuMaterials>()
            .add_system(button_system)
        ;
    }
}