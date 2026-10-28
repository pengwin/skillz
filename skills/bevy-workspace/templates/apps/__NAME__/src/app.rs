//! Application bootstrap: composes feature plugins and orders the frame phases.

use bevy::asset::AssetPlugin;
use bevy::prelude::{App, IntoScheduleConfigs, MinimalPlugins, SpawnSystem, Startup, Update};
use bevy::scene::ScenePlugin;
use game_core::GameSet;
use game_spin::{SpinPlugin, spinner};

/// Build the headless application with every feature plugin.
pub fn build() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        ScenePlugin,
        SpinPlugin,
    ))
    .configure_sets(
        Update,
        (GameSet::Input, GameSet::Simulation, GameSet::Presentation).chain(),
    )
    // Spawn entities from scenes, not from a chain of `Commands::spawn` calls.
    .add_systems(Startup, spinner.spawn());
    app
}

/// Run a few frames of the application.
pub fn run() {
    let mut app = build();
    for _ in 0..3 {
        app.update();
    }
    eprintln!("Application ready.");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::App;
    use game_spin::Spin;
    use rstest::{fixture, rstest};

    use super::build;

    #[fixture]
    fn app() -> App {
        build()
    }

    #[rstest]
    fn application_bootstrap_spawns_the_scene(mut app: App) {
        app.update();

        let spinners = app.world_mut().query::<&Spin>().iter(app.world()).count();
        assert_eq!(spinners, 1);
    }
}
