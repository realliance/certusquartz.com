use std::f32::consts::PI;

use bevy::prelude::*;

pub struct CertusQuartzPlugin;

#[derive(Component)]
pub struct CertuzQuartz;

impl Plugin for CertusQuartzPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(rotate_certus_quartz);
  }
}

fn rotate_certus_quartz(mut query: Query<&mut Transform, With<CertuzQuartz>>, time: Res<Time>) {
  query.for_each_mut(|mut trans| {
    trans.rotate_axis(Vec3::Y, time.delta_seconds() * (2.0 * PI) / 5.0);
    trans.translation.y = (time.elapsed_seconds() * (2.0 * PI) / 3.0).sin();
  });
}
