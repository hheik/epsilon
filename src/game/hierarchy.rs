use bevy::prelude::*;

pub struct HierarchyVisualizerPlugin;

impl Plugin for HierarchyVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hierarchy_setup)
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
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(4.0),
                    left: Val::Px(4.0),
                    ..default()
                },
                ..default()
            }),
        );
}

fn hierarchy_update(
    root_query: Query<Entity, Without<Parent>>,
    child_query: Query<&Children>,
    // mut hierarchy: ResMut<Hierarchy>,
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
    child_query: &Query<&Children>,
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
