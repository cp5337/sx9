//! CTAS SlotGraph Visual Demo - A few cognitive atoms interacting in 3D space
//! 
//! Homely but functional - colored cubes representing Universal Cognigraph nodes

use bevy::prelude::*;
use ctas_slotgraph_tools::*;

fn main() {
    println!("🧠 LAUNCHING VISUAL COGNIGRAPH!");
    println!("🎨 Watch cognitive atoms interact in 3D space");
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "CTAS Cognigraph - Universal Node Interactions".into(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SlotGraphPlugin)
        .add_systems(Startup, (setup_scene, setup_cognigraph_nodes))
        .add_systems(Update, (
            rotate_camera_system,
            node_interaction_system,
            update_node_colors_system,
            display_info_system,
        ))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 15.0, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -std::f32::consts::FRAC_PI_4)),
        ..default()
    });

    // Ground plane for reference
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.2),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, -2.0, 0.0),
        ..default()
    });

    info!("🎨 3D Scene setup complete");
}

fn setup_cognigraph_nodes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("🧠 Creating interactive cognitive atoms");

    // Define a few interacting nodes
    let nodes = [
        (UniversalNodeType::Source, Vec3::new(-5.0, 0.0, 0.0), Color::srgb(0.2, 1.0, 0.2), "Energy Source"),
        (UniversalNodeType::Transformer, Vec3::new(0.0, 0.0, 0.0), Color::srgb(1.0, 0.5, 0.2), "Data Transformer"),
        (UniversalNodeType::Router, Vec3::new(5.0, 0.0, 0.0), Color::srgb(0.2, 0.5, 1.0), "Traffic Router"),
        (UniversalNodeType::Monitor, Vec3::new(0.0, 5.0, 0.0), Color::srgb(1.0, 1.0, 0.2), "System Monitor"),
        (UniversalNodeType::Sink, Vec3::new(0.0, -5.0, 0.0), Color::srgb(0.8, 0.2, 0.8), "Data Sink"),
    ];

    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    for (i, (node_type, position, color, name)) in nodes.iter().enumerate() {
        let atom = CognitiveAtom::new(node_type.clone(), None, *position);
        
        let material = materials.add(StandardMaterial {
            base_color: *color,
            metallic: 0.3,
            perceptual_roughness: 0.5,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh: cube_mesh.clone(),
                material,
                transform: Transform::from_translation(*position),
                ..default()
            },
            CognitiveAtomBundle::new(atom),
            Name::new(format!("Node{}: {}", i, name)),
            NodeVisual { 
                base_color: *color,
                interaction_intensity: 0.0,
            },
        ));

        info!("✅ Spawned {} at {:?}", name, position);
    }

    info!("🔥 {} cognitive atoms ready for interaction", nodes.len());
}

#[derive(Component)]
struct NodeVisual {
    base_color: Color,
    interaction_intensity: f32,
}

fn rotate_camera_system(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
) {
    for mut transform in camera_query.iter_mut() {
        let radius = 20.0;
        let angle = time.elapsed_seconds() * 0.2;
        
        transform.translation.x = angle.cos() * radius;
        transform.translation.z = angle.sin() * radius;
        transform.translation.y = 15.0;
        
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn node_interaction_system(
    mut atom_query: Query<(&mut CognitiveAtom, &mut NodeVisual, &Transform)>,
    time: Res<Time>,
) {
    let nodes: Vec<_> = atom_query.iter().collect();
    let mut interactions = Vec::new();

    // Check for interactions between nodes
    for (i, (atom_a, _, transform_a)) in nodes.iter().enumerate() {
        for (j, (_atom_b, _, transform_b)) in nodes.iter().enumerate().skip(i + 1) {
            let distance = transform_a.translation.distance(transform_b.translation);
            
            if distance < atom_a.spatial.interaction_radius {
                let strength = (atom_a.spatial.interaction_radius - distance) / atom_a.spatial.interaction_radius;
                interactions.push((i, j, strength));
            }
        }
    }

    // Apply interactions and update visual feedback
    for (mut atom, mut visual, _) in atom_query.iter_mut() {
        // Simulate energy flow and activation
        let energy_threshold = atom.energetic.threshold;
        let current_energy = atom.energetic.generation - atom.energetic.consumption;
        
        if current_energy >= energy_threshold {
            atom.activation_state = ActivationState::Active;
            visual.interaction_intensity = (time.elapsed_seconds() * 3.0).sin().abs();
        } else {
            atom.activation_state = ActivationState::Dormant;
            visual.interaction_intensity *= 0.95; // Fade out
        }
    }
}

fn update_node_colors_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    atom_query: Query<(&CognitiveAtom, &NodeVisual, &Handle<StandardMaterial>), Changed<NodeVisual>>,
) {
    for (atom, visual, material_handle) in atom_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            // Mix base color with activation intensity
            let activation_glow = match atom.activation_state {
                ActivationState::Active => visual.interaction_intensity * 0.8,
                ActivationState::Primed => 0.3,
                _ => 0.0,
            };
            
            // Extract RGB components from Color
            let [r, g, b, _] = visual.base_color.to_srgba().to_f32_array();
            
            material.base_color = Color::srgb(
                (r + activation_glow).min(1.0),
                (g + activation_glow * 0.5).min(1.0),
                (b + activation_glow * 0.2).min(1.0),
            );
            
            // Add emissive glow for active nodes
            if matches!(atom.activation_state, ActivationState::Active) {
                material.emissive = LinearRgba::rgb(
                    activation_glow * 0.3,
                    activation_glow * 0.2,
                    activation_glow * 0.1,
                );
            } else {
                material.emissive = LinearRgba::rgb(0.0, 0.0, 0.0);
            }
        }
    }
}

fn display_info_system(
    atom_query: Query<(&CognitiveAtom, &Name)>,
    time: Res<Time>,
) {
    // Print status every 3 seconds
    if time.elapsed_seconds() as u32 % 3 == 0 && time.delta_seconds() < 0.1 {
        let active_count = atom_query.iter()
            .filter(|(atom, _)| matches!(atom.activation_state, ActivationState::Active))
            .count();
            
        println!("\n🧠 COGNIGRAPH STATUS (T+{:.1}s):", time.elapsed_seconds());
        println!("   💫 Total Atoms: {}", atom_query.iter().count());
        println!("   ⚡ Active Atoms: {}", active_count);
        
        for (atom, name) in atom_query.iter() {
            let status_icon = match atom.activation_state {
                ActivationState::Active => "🔥",
                ActivationState::Primed => "⚡",
                ActivationState::Dormant => "💤",
                _ => "❓",
            };
            
            println!("   {} {} (B{}) - Energy: {:.2}", 
                status_icon, 
                name.as_str(), 
                atom.atomic_number,
                atom.energetic.net_energy_balance()
            );
        }
    }
}
