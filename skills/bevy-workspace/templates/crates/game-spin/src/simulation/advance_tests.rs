use core::time::Duration;

use bevy::ecs::message::Messages;
use bevy::prelude::{App, Update};
use bevy::time::{TimePlugin, TimeUpdateStrategy};
use rstest::{fixture, rstest};

use super::advance_spin;
use crate::data::{Spin, SpinSpeed};
use crate::messaging::FullTurn;

#[fixture]
fn app() -> App {
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs(1)))
        .add_message::<FullTurn>()
        .add_systems(Update, advance_spin);
    app
}

#[rstest]
#[case(2.0, false)]
#[case(20.0, true)]
fn spin_advances_without_any_rendering(
    mut app: App,
    #[case] speed: f32,
    #[case] completes_turn: bool,
) {
    let entity = app
        .world_mut()
        .spawn((Spin::default(), SpinSpeed(speed)))
        .id();

    // Virtual time clamps each frame to 250 ms, so run enough frames to cross a full turn.
    for _ in 0..4 {
        app.update();
    }

    let spin = app.world().get::<Spin>(entity);
    assert!(spin.is_some_and(|spin| spin.angle > 0.0));
    let turns = app.world().resource::<Messages<FullTurn>>().len();
    assert_eq!(turns > 0, completes_turn);
}
