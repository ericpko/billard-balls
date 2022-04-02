use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{SIM_WIDTH, SIM_HEIGHT};

const N_BILLARDS: u32 = 20;
const RESTITUTION: f32 = 1.0;
const GRAVITY: f32 = -9.8;
const _DT: f64 = 1.0 / 60.0;


#[derive(Component)]
struct Billard;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct Radius(f32);


pub struct BillardsPlugin;

impl Plugin for BillardsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_billards)
            .add_system(handle_billard_collisions)
            .add_system(handle_wall_collisions)
            .add_system(symplectic_euler);
    }
}


fn init_billards(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..N_BILLARDS {
        let radius = 0.05 + rng.gen::<f32>() * 0.1;
        let mass = PI * radius * radius;
        let pos = Vec2::new(
            rng.gen::<f32>() * SIM_WIDTH,
            rng.gen::<f32>() * SIM_HEIGHT,
        );
        let vel = Vec2::new(
            rng.gen::<f32>() * 2.0 - 1.0,
            rng.gen::<f32>() * 2.0 - 1.0,
        );
        let color = Color::rgb(
            rng.gen::<f32>(),
            rng.gen::<f32>(),
            rng.gen::<f32>(),
        );

        let mesh_bundle = MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius,
            }))
            .into(),
            transform: Transform {
                translation: Vec3::new(pos.x, pos.y, 1.0),
                ..Default::default()
            },
            material: materials.add(ColorMaterial::from(color)),
            ..Default::default()
        };

        #[allow(unused)]
        let billard = commands
            .spawn_bundle(mesh_bundle)
            .insert(Position(pos))
            .insert(Velocity(vel))
            .insert(Mass(mass))
            .insert(Radius(radius))
            .insert(Billard)
            .insert(Name::new("Billard"))
            .id();
    }
}

fn handle_billard_collisions(
    mut query: Query<(&Mass, &Radius, &mut Position, &mut Velocity)>,
) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(m1, r1, mut p1, mut v1), (m2, r2, mut p2, mut v2)]) = iter.fetch_next() {
        let dir = p2.0 - p1.0;
        let len = dir.length();
        if len == 0.0 || len > r1.0 + r2.0 {
            continue;
        }
        let dir = dir.normalize();

        let corr = (r1.0 + r2.0 - len) / 2.0;
        p1.0 += dir * -corr;
        p2.0 += dir * corr;

        let v1_com = v1.0.dot(dir);
        let v2_com = v2.0.dot(dir);

        let v1_com_new = (m1.0 * v1_com + m2.0 * v2_com - m2.0 * (v1_com - v2_com) * RESTITUTION) / (m1.0 + m2.0);
        let v2_com_new = (m1.0 * v1_com + m2.0 * v2_com - m1.0 * (v2_com - v1_com) * RESTITUTION) / (m1.0 + m2.0);

        v1.0 += dir * (v1_com_new - v1_com);
        v2.0 += dir * (v2_com_new - v2_com);
    }
}

fn handle_wall_collisions(
    mut query: Query<(&Radius, &mut Position, &mut Velocity), With<Billard>>,
) {
    for (r, mut p, mut v) in query.iter_mut() {
        if p.0.x - r.0 < 0.0 {
            p.0.x = 0.0 + r.0;
            v.0.x = -v.0.x;
        } else if p.0.x + r.0 > SIM_WIDTH {
            p.0.x = SIM_WIDTH - r.0;
            v.0.x = -v.0.x;
        }
        if p.0.y - r.0 < 0.0 {
            p.0.y = 0.0 + r.0;
            v.0.y = -v.0.y;
        } else if p.0.y + r.0 > SIM_HEIGHT {
            p.0.y = SIM_HEIGHT - r.0;
            v.0.y = -v.0.y;
        }
    }
}

fn symplectic_euler(
    time: Res<Time>,
    mut query: Query<(&mut Position, &mut Velocity, &mut Transform), With<Billard>>,
) {
    for (mut p, mut v, mut t) in query.iter_mut() {
        v.0 += Vec2::ZERO * -GRAVITY * time.delta_seconds();
        p.0 += v.0 * time.delta_seconds(); 
        t.translation = Vec3::new(p.0.x, p.0.y, 1.0);
    }
}
