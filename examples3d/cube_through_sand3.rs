use crate::helper;
use na::{point, vector, DMatrix};
use rapier3d::math::Point;
use rapier3d::prelude::{
    ColliderBuilder, ColliderSet, ImpulseJointSet, MultibodyJointSet, RigidBodySet,
};
use rapier_testbed3d::{Testbed, TestbedApp};
use sparkl3d::prelude::*;
use sparkl3d::third_party::rapier::MpmTestbedPlugin;

pub fn init_world(testbed: &mut Testbed) {
    /*
     * World
     */
    let mut models = ParticleModelSet::new();
    let mut particles = ParticleSet::new();

    let cell_width = 0.025;
    let mut colliders = ColliderSet::new();
    let ground_height = cell_width * 10.0;
    let ground_half_side = 20.0;

    let (nx, ny) = (40, 40);
    let mut heigths = DMatrix::zeros(nx + 1, ny + 1);

    for i in 0..=nx {
        for j in 0..=ny {
            heigths[(i, j)] = -(i as f32 * std::f32::consts::PI / (nx as f32)).sin();
        }
    }
    colliders.insert(
        ColliderBuilder::heightfield(
            heigths.into(),
            vector![ground_half_side * 2.0, 10.0, ground_half_side * 2.0],
        )
        .translation(vector![0.0, 10.0, 0.0])
        .sensor(true)
        .build(),
    );
    colliders.insert(
        ColliderBuilder::cuboid(10.0, 0.11, 5.0)
            .translation(vector![0.0, 1.5, 5.0])
            .sensor(true)
            .build(),
    );
    // colliders.insert(
    //     ColliderBuilder::capsule_y(2.0, 1.0)
    //         .translation(vector![10.0, 4.0, 5.0])
    //         .sensor(true)
    //         .build(),
    // );
    // colliders.insert(
    //     ColliderBuilder::triangle(
    //         Point::new(6.0, 2.0, 4.0),
    //         Point::new(12.0, 2.0, 4.0),
    //         Point::new(10.0, 8.0, 4.0),
    //     )
    //     .sensor(true)
    //     .translation(vector![-5.0, 0.0, 1.0])
    //     .build(),
    // );

    const NU: Real = 0.2;
    const E: Real = 1.0e7;

    let plasticity = DruckerPragerPlasticity::new(E, NU);
    let sand_model = models.insert(ParticleModel::with_plasticity(
        CorotatedLinearElasticity::new(E, NU),
        plasticity,
    ));
    let block_model = models.insert(ParticleModel::new(CorotatedLinearElasticity::new(E, NU)));
    let cell_width = 0.2;
    let particle_rad = cell_width / 4.0;
    let sand_particles = helper::cube_particles(
        point![0.0, cell_width * 3.0 + 2.0, 0.0],
        100, // 40,
        50,  // 100,
        50,  // 40,
        sand_model,
        particle_rad,
        2700.0,
        false,
    );

    let mut block_particles = helper::cube_particles(
        point![-10.0, cell_width * 3.0 + 2.0, 0.0],
        25, // 40,
        25, // 100,
        25, // 40,
        block_model,
        particle_rad,
        2700.0,
        false,
    );

    for p in &mut block_particles {
        p.kinematic_vel = Some(vector![10.0, 0.0, 0.0]);
    }

    log::info!(
        "Num particles: {}",
        sand_particles.len() + block_particles.len()
    );

    particles.insert_batch(sand_particles);
    particles.insert_batch(block_particles);

    let bodies = RigidBodySet::new();
    let impulse_joints = ImpulseJointSet::new();
    let multibody_joints = MultibodyJointSet::new();

    let mut plugin = MpmTestbedPlugin::new(models, particles, cell_width);
    testbed.add_plugin(plugin);
    testbed.set_world(bodies, colliders, impulse_joints, multibody_joints);
    testbed.integration_parameters_mut().dt = 1.0 / 60.0;
    testbed.look_at(
        Point::new(ground_half_side, 4.0, ground_half_side + 20.0),
        point![ground_half_side, 1.0, ground_half_side],
    );
}

fn main() {
    let testbed = TestbedApp::from_builders(0, vec![("Elasticity", init_world)]);
    testbed.run()
}
