use bevy::prelude::*;

mod playing_cards;

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


    let card_width = 0.59;
    let card_height = 0.89;
    let card_mesh = meshes.add(Rectangle::new(card_width, card_height));

    let a_card = playing_cards::PlayingCard::Standard(
        playing_cards::Suite::Clubs,
        playing_cards::Rank::Ace,
    );

    let base_color: Handle<Image> = asset_server.load(
        &playing_cards::front_texture_path(a_card)
    );
    let back_color: Handle<Image> = asset_server.load(
        &playing_cards::back_texture_path()
    );

    let front_material = materials.add(StandardMaterial {
        base_color_texture: Some(base_color.clone()),
        ..Default::default()
    });
    let back_material = materials.add(StandardMaterial {
        base_color_texture: Some(back_color.clone()),
        ..Default::default()
    });

    commands.spawn((
        Rotating { speed: 1.0 },
        Transform::from_xyz(0.0, 0.0, 0.0),
        children![
            (
                Mesh3d(card_mesh.clone()),
                MeshMaterial3d(front_material.clone()),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ),
            (
                Mesh3d(card_mesh.clone()),
                MeshMaterial3d(back_material.clone()),
                Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)
            ),
    )
        ],
    ));

}

#[derive(Component)]
struct Rotating{
    speed: f32,
}



fn rotation(
    time: Res<Time>,
    mut query: Query<(&Rotating, &mut Transform)>,
) {
    for (rotating, mut transform) in query.iter_mut() {
        transform.rotate_y(rotating.speed * time.delta_secs());
    }
}

