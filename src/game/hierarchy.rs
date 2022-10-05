use bevy::{
    ecs::query::WorldQuery,
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
pub struct HierarchyVisualizer {
    pub selected: Option<Entity>,
}

#[derive(Default, Component)]
pub struct IgnoreHierarchy;

fn hierarchy_setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Name::new("hierarchy visualizer"))
        .insert(IgnoreHierarchy)
        .insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Undefined, Val::Percent(100.0)),
                overflow: Overflow::Hidden,
                ..default()
            },
            color: Color::rgba(0.1, 0.1, 0.1, 1.0).into(),
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn()
                .insert(Name::new("hierarchy"))
                .insert(ScrollingList::default())
                .insert(HierarchyVisualizer::default())
                .insert_bundle(NodeBundle {
                    style: Style {
                        flex_grow: 1.0,
                        margin: UiRect {
                            top: Val::Px(4.0),
                            left: Val::Px(8.0),
                            bottom: Val::Px(12.0),
                            right: Val::Px(12.0),
                        },
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    color: Color::rgba(0.5, 0.0, 0.0, 0.0).into(),
                    ..default()
                });
        });
}

fn hierarchy_update(
    mut commands: Commands,
    root_query: Query<Entity, (Without<Parent>, Without<IgnoreHierarchy>)>,
    child_query: Query<&Children, Without<IgnoreHierarchy>>,
    mut visualizer_query: Query<(Entity, &mut HierarchyVisualizer, &mut Visibility)>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
) {
    if let Ok((entity, mut visualizer, mut visibility)) = visualizer_query.get_single_mut() {
        if input.just_pressed(KeyCode::P) {
            visibility.is_visible = !visibility.is_visible;
        }
        if !visibility.is_visible {
            return;
        }
        let font: Handle<Font> = asset_server.load("fonts/open_sans_medium.ttf");

        commands.entity(entity).despawn_descendants();

        let mut nodes: Vec<HierarchyNode> = vec![];
        for root_entity in root_query.iter() {
            traverse_hierarchy(root_entity, 0, &child_query, &mut nodes);
        }

        let mut select_direction = 0;
        for pressed in input.get_just_pressed() {
            match pressed {
                KeyCode::Down => select_direction += 1,
                KeyCode::Up => select_direction -= 1,
                KeyCode::PageDown => select_direction += 10,
                KeyCode::PageUp => select_direction -= 10,
                KeyCode::End => select_direction += i32::MAX / 4,
                KeyCode::Home => select_direction -= i32::MAX / 4,
                _ => (),
            }
        }

        if select_direction != 0 {
            if let Some(selected) = visualizer.selected {
                if nodes
                    .iter()
                    .position(|node| node.entity == selected)
                    .is_none()
                {
                    visualizer.selected = None;
                }
            }
            match visualizer.selected {
                Some(selected) => {
                    let current_index = nodes
                        .iter()
                        .position(|node| node.entity == selected)
                        .unwrap();
                    if current_index == 0 && select_direction < 0 {
                        visualizer.selected = None;
                    } else {
                        let index = (current_index as i32 + select_direction)
                            .clamp(0, nodes.len() as i32 - 1)
                            as usize;
                        visualizer.selected = Some(nodes[index].entity);
                    }
                }
                None => {
                    let index = (-1 + select_direction).clamp(0, nodes.len() as i32 - 1) as usize;
                    visualizer.selected = Some(nodes[index].entity);
                }
            }
        }

        commands.entity(entity).with_children(|builder| {
            for node in nodes.iter() {
                let selected =
                    visualizer.selected.is_some() && visualizer.selected.unwrap() == node.entity;
                let mut name = "[entity]";
                if let Ok(ent_name) = name_query.get(node.entity) {
                    name = ent_name.as_str();
                };
                let font_size = 16.0;

                let mut sections: Vec<TextSection> = Vec::with_capacity(nodes.len() * 2);
                sections.push(TextSection {
                    value: format!("{} ", name),
                    style: TextStyle {
                        font: font.clone(),
                        color: Color::WHITE,
                        font_size,
                    },
                });
                sections.push(TextSection {
                    value: format!("{}.{}", node.entity.id(), node.entity.generation()),
                    style: TextStyle {
                        font: font.clone(),
                        color: Color::GRAY,
                        font_size,
                    },
                });
                builder
                    .spawn()
                    .insert_bundle(NodeBundle {
                        color: if selected {
                            Color::rgba(0.3, 0.3, 0.3, 0.5).into()
                        } else {
                            Color::rgba(0.0, 0.0, 0.0, 0.0).into()
                        },
                        ..default()
                    })
                    .insert_bundle(TextBundle::from_sections(sections).with_style(Style {
                        margin: UiRect {
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            left: Val::Px(24.0 * node.depth as f32),
                            ..default()
                        },
                        size: Size::new(Val::Undefined, Val::Px(font_size)),
                        flex_shrink: 0.0,
                        ..default()
                    }));
            }
        });
    }
}

fn traverse_hierarchy<T>(
    current: Entity,
    depth: usize,
    child_query: &Query<&Children, T>,
    nodes: &mut Vec<HierarchyNode>,
) where
    T: WorldQuery,
{
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
