use crate::AppState;
use bevy::{prelude::*, app::AppExit};

pub struct MainMenuPlugin;

struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

struct MenuMaterials {
    root: UiColor,
    border: UiColor,
    menu: UiColor,
    button: UiColor,
    button_text: Color,
}

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => {
                    app_state
                        .set(AppState::LiveSim)
                        .expect("Couldn't switch state to InGame");
                }
                MenuButton::Quit => exit.send(AppExit),
            };
        }
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MenuMaterials {
                root: Color::NONE.into(),
                border: Color::rgb(0.65, 0.65, 0.65).into(),
                menu: Color::rgb(0.15, 0.15, 0.15).into(),
                button: Color::rgb(0.15, 0.15, 0.15).into(),
                button_text: Color::WHITE,
            })
            .add_system(button_press_system)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup));
    }
}

fn root(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.root,
        ..Default::default()
    }
}

fn border(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: Rect::all(Val::Px(8.0)),
            ..Default::default()
        },
        color: materials.border,
        ..Default::default()
    }
}

fn menu_background(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        color: materials.menu,
        ..Default::default()
    }
}

fn button(materials: &Res<MenuMaterials>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.button,
        ..Default::default()
    }
}

fn button_text(asset_server: &Res<AssetServer>, materials: &Res<MenuMaterials>, label: &str) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::with_section(
            label,
            TextStyle {
                font: asset_server.load("Gidole-Regular.ttf"),
                font_size: 30.0,
                color: materials.button_text.clone(),
            },
            Default::default(),
        ),
        ..Default::default()
    };
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<MenuMaterials>,
    mut windows: ResMut<Windows>
) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(border(&materials))
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(menu_background(&materials))
                        .with_children(|parent| {
                            parent.spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&asset_server, &materials, "Simulate"));
                                })
                                .insert(MenuButton::Play);
                            parent.spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&asset_server, &materials, "Quit"));
                                })
                                .insert(MenuButton::Quit);
                        });
                });
        })
        .id();

    commands.insert_resource(MainMenuData {
        camera_entity,
        ui_root,
    });
    
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(false);
    window.set_cursor_visibility(true);
}

fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}