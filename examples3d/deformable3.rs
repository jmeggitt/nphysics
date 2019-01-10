extern crate nalgebra as na;
extern crate ncollide3d;
extern crate nphysics3d;
extern crate nphysics_testbed3d;

use std::sync::Arc;
use na::{Isometry3, Point3, Vector3};
use ncollide3d::shape::{Cuboid, ShapeHandle, TriMesh};
use nphysics3d::object::{BodyPartHandle, Material, DeformableVolumeDesc, ColliderDesc};
use nphysics3d::world::World;
use nphysics3d::volumetric::Volumetric;
use ncollide3d::transformation::ToTriMesh;
use nphysics_testbed3d::Testbed;

fn main() {
    /*
     * World
     */
    let mut world = World::new();
    world.set_gravity(Vector3::new(0.0, -9.81, 0.0));

    /*
     * Ground.
     */
    let ground_size = 50.0;
    let ground = ShapeHandle::new(Cuboid::new(Vector3::repeat(ground_size)));

    let _ = ColliderDesc::new(ground)
        .with_translation(Vector3::y() * (-ground_size - 1.0))
        .build(&mut world);


    let ground_size = 3.0;
    let obstacle = ShapeHandle::new(Cuboid::new(Vector3::new(0.02, 0.02, ground_size)));

    let mut obstacle_desc = ColliderDesc::new(obstacle);

    let _ = obstacle_desc
        .set_translation(Vector3::new(0.4, -0.01, 0.0))
        .build(&mut world);

    let _ = obstacle_desc
        .set_translation(Vector3::new(-0.4, -0.01, 0.0))
        .build(&mut world);

    /*
     * Create the deformable body and a collider for its boundary.
     */
    let _ = DeformableVolumeDesc::cube(50, 1, 1)
        .with_scale(Vector3::new(1.0, 0.1, 0.1))
        .with_translation(Vector3::y() * 0.1)
        .with_young_modulus(1.0e3)
        .with_mass_damping(0.2)
        .with_boundary_trimesh_collider(true)
        .build(&mut world);

    /*
     * Set up the testbed.
     */
    let mut testbed = Testbed::new(world);
    // testbed.hide_performance_counters();
    testbed.look_at(Point3::new(0.0, 0.0, 2.0), Point3::new(0.0, 0.0, 0.0));
    testbed.run();
}
