use self::certus_quartz::CertusQuartzPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_scene_hook::HookPlugin;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

pub mod certus_quartz;

pub struct GameplayPlugins;

impl PluginGroup for GameplayPlugins {
  fn build(self) -> bevy::app::PluginGroupBuilder {
    info!("Adding gameplay plugins");
    PluginGroupBuilder::start::<Self>()
      .add(CertusQuartzPlugin)
      .add(LookTransformPlugin)
      .add(OrbitCameraPlugin::default())
      .add(HookPlugin)
  }
}
