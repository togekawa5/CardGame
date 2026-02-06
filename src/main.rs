use bevy::{prelude::*};

mod playing_cards;
mod card_token;
use crate::card_token::CardToken;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.add_systems(Update, rotation);
    app.run();
}

fn setup
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 3.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y)
    ));

    commands.spawn(PointLight {
        intensity: 1500.0,
        shadows_enabled: true,
        ..Default::default()
    });


    
    let a_card = playing_cards::PlayingCard::Standard(
        playing_cards::Suite::Clubs,
        playing_cards::Rank::Ace,
    );
    let size = a_card.size();
    let card_mesh = meshes.add(Rectangle::new(size.0, size.1));
    let base_color: Handle<Image> = asset_server.load(
        &a_card.front_texture_path()
    );
    let back_color: Handle<Image> = asset_server.load(
        &a_card.back_texture_path()
    );
    let front_material = materials.add(StandardMaterial {
        base_color_texture: Some(base_color.clone()),
        ..Default::default()
    });
    let back_material = materials.add(StandardMaterial {
        base_color_texture: Some(back_color.clone()),
        ..Default::default()
    });

    spawn_card(
        &mut commands,
        card_mesh.clone(),
        front_material.clone(),
        back_material.clone(),
    );

    let random_card = playing_cards::PlayingCard::from(24);
    let random_card_color = asset_server.load(
        &random_card.front_texture_path()
    );
    let random_front_mat = materials.add(StandardMaterial {
        base_color_texture: Some(random_card_color.clone()),
        ..Default::default()
    });
    let card2_id = spawn_card(&mut commands, card_mesh.clone(), random_front_mat.clone(), back_material.clone());
    commands.entity(card2_id).insert(Transform::from_xyz(-0.7, 0.0, 0.0));

}

#[derive(Component)]
struct Rotating{
    speed: f32,
}

fn spawn_card(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    front_material: Handle<StandardMaterial>,
    back_material: Handle<StandardMaterial>,
) -> Entity {
    commands.spawn((
        Rotating { speed: 1.0 },
        Transform::from_xyz(0.0, 0.0, 0.0),
        children![
            (
                Mesh3d(mesh.clone()),
                MeshMaterial3d(front_material.clone()),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ),
            (
                Mesh3d(mesh.clone()),
                MeshMaterial3d(back_material.clone()),
                Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)
            ),
    )
        ],
    )).id()
}



fn rotation(
    time: Res<Time>,
    mut query: Query<(&Rotating, &mut Transform)>,
) {
    for (rotating, mut transform) in query.iter_mut() {
        transform.rotate_y(rotating.speed * time.delta_secs());
    }
}

