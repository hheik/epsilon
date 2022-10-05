use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub struct HierarchyVisualizerPlugin;

impl Plugin for HierarchyVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hierarchy_setup)
            .add_system(mouse_scroll)
            .add_system(hierarchy_update);
    }
}

pub struct HierarchyNode {
    pub entity: Entity,
    pub depth: usize,
}

pub struct Hierarchy {
    pub nodes: Vec<HierarchyNode>,
}

#[derive(Default, Component)]
pub struct HierarchyVisualizer;

fn hierarchy_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/venice_classic.ttf");
    commands
        .spawn()
        .insert(Name::new("hierarchy visualizer"))
        .insert_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                size: Size::new(Val::Undefined, Val::Percent(100.0)),
                overflow: Overflow::Hidden,
                ..default()
            },
            color: Color::rgba(0.1, 0.1, 0.1, 0.5).into(),
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn()
                .insert(Name::new("scrolling"))
                .insert(ScrollingList::default())
                .insert_bundle(NodeBundle {
                    style: Style {
                        flex_grow: 1.0,
                        margin: UiRect {
                            top: Val::Px(4.0),
                            left: Val::Px(8.0),
                            bottom: Val::Px(12.0),
                            right: Val::Px(4.0),
                        },
                        ..default()
                    },
                    color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..default()
                })
                .with_children(|builder| {
                    builder
                        .spawn()
                        .insert(Name::new("text"))
                        .insert(HierarchyVisualizer)
                        .insert_bundle(
                            TextBundle::from_section(
                                "",
                                TextStyle {
                                    font: font.clone(),
                                    color: Color::WHITE,
                                    font_size: 13.0,
                                },
                            )
                            .with_style(Style {
                                align_self: AlignSelf::FlexEnd,
                                ..default()
                            }),
                        );
                });
        });
}

fn hierarchy_update(
    root_query: Query<Entity, Without<Parent>>,
    child_query: Query<&Children, Without<HierarchyVisualizer>>,
    mut visualizer_query: Query<(&mut Text, &mut Visibility), With<HierarchyVisualizer>>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
) {
    if let Ok((mut text, mut visibility)) = visualizer_query.get_single_mut() {
        if input.just_pressed(KeyCode::P) {
            visibility.is_visible = !visibility.is_visible;
        }
        if !visibility.is_visible {
            return;
        }
        let font = asset_server.load("fonts/venice_classic.ttf");
        let mut nodes: Vec<HierarchyNode> = vec![];
        for root_entity in root_query.iter() {
            traverse_hierarchy(root_entity, 0, &child_query, &mut nodes);
        }
        let mut sections: Vec<TextSection> = Vec::with_capacity(nodes.len() * 2);
        for node in nodes.iter() {
            let mut value = "[entity]";
            if let Ok(name) = name_query.get(node.entity) {
                value = name.as_str();
            };
            sections.push(TextSection {
                value: format!("{}{} ", "    ".repeat(node.depth), value),
                style: TextStyle {
                    font: font.clone(),
                    color: Color::WHITE,
                    font_size: 13.0,
                },
            });
            sections.push(TextSection {
                value: format!("{}.{}\n", node.entity.id(), node.entity.generation()),
                style: TextStyle {
                    font: font.clone(),
                    color: Color::GRAY,
                    font_size: 13.0,
                },
            });
        }
        text.sections = sections;
    }
}

fn traverse_hierarchy(
    current: Entity,
    depth: usize,
    child_query: &Query<&Children, Without<HierarchyVisualizer>>,
    nodes: &mut Vec<HierarchyNode>,
) {
    nodes.push(HierarchyNode {
        entity: current,
        depth,
    });
    if let Ok(children) = child_query.get(current) {
        for child in children {
            traverse_hierarchy(*child, depth + 1, child_query, nodes);
        }
    }
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 200.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
