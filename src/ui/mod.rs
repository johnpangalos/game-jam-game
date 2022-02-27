use bevy::{
    math::{Rect, Size},
    prelude::{
        App, Button, ButtonBundle, Changed, ChildBuilder, Children, Color, Component, Handle,
        Plugin, Query, TextBundle, With,
    },
    text::{Font, Text, TextSection, TextStyle},
    ui::{AlignItems, AlignSelf, Interaction, JustifyContent, PositionType, Style, UiColor, Val},
};

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]

pub struct DrawCardButton;

pub fn button(position: Rect<Val>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            position,
            ..Default::default()
        },
        color: NORMAL_BUTTON.into(),
        ..Default::default()
    }
}

pub fn button_text(value: &'static str, font: Handle<Font>) -> impl FnOnce(&mut ChildBuilder) {
    |parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                value.to_string(),
                TextStyle {
                    font,
                    font_size: 28.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        });
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub struct DescriptionOptions {
    pub position: Rect<Val>,
    pub title_font: Handle<Font>,
    pub definition_font: Handle<Font>,
}

pub fn description(title: &str, definition: &str, options: DescriptionOptions) -> TextBundle {
    TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: options.position,
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: title.to_string(),
                    style: TextStyle {
                        font: options.title_font,
                        font_size: 28.0,
                        ..Default::default()
                    },
                },
                TextSection {
                    value: definition.to_string(),
                    style: TextStyle {
                        font: options.definition_font,
                        font_size: 28.0,
                        ..Default::default()
                    },
                },
            ],
            ..Default::default()
        },

        ..Default::default()
    }
}

pub struct CustomUiPlugin;

impl Plugin for CustomUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_system);
    }
}
