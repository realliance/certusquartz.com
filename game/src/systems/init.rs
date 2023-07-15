use bevy::prelude::*;

use bevy_scene_hook::{HookedSceneBundle, SceneHook};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

use super::gameplay::certus_quartz::CertuzQuartz;

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
  let camera_pos = Vec3::new(10.0, 2.0, 10.0);

  commands.spawn(Camera3dBundle::default()).insert(OrbitCameraBundle::new(
    OrbitCameraController {
      mouse_rotate_sensitivity: Vec2::splat(0.5),
      mouse_translate_sensitivity: Vec2::splat(0.0),
      ..Default::default()
    },
    camera_pos.clone(),
    Vec3::ZERO,
    Vec3::Y,
  ));

  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 5000.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_translation(camera_pos).looking_at(Vec3::ZERO, Vec3::Y),
    ..default()
  });

  commands.spawn(HookedSceneBundle {
    scene: SceneBundle {
      scene: asset_server.load("CertusQuartz.gltf#Scene0"),
      ..default()
    },
    hook: SceneHook::new(|entity, cmds| {
      match entity.get::<Name>().map(|t| t.as_str()) {
        Some("CertusQuartz") => cmds.insert(CertuzQuartz),
        _ => cmds,
      };
    }),
  });
}
