use bevy_app::{App, Plugin};
use bevy_ecs::{
    prelude::{Res, ResMut},
    schedule::Condition,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_minibuffer::{prelude::*, prompt::PromptState};
use bevy_reflect::Reflect;
use bevy_state::app::AppExtStates;
use bevy_state::prelude::in_state;
use bevy_state::prelude::{NextState, State, States};

/// Is the prompt visible?
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
enum WorldInspectorState {
    /// Invisible
    #[default]
    Invisible,
    /// Visible
    Visible,
}

/// ## Adds the 'inspect_world' act
///
/// This act toggles the visibility of the world inspector.
///
/// ## Usage
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_minibuffer::prelude::*;
/// use bevy_minibuffer_inspector as inspector;
/// fn plugin(app: &mut App) {
///     app
///         .add_plugins(MinibufferPlugins)
///         .add_acts((
///             BasicActs::default(),
///             inspector::WorldActs::default(),
///         ));
/// }
/// ```
pub struct WorldActs {
    acts: Acts,
}

impl ActsPlugin for WorldActs {
    fn acts(&self) -> &Acts {
        &self.acts
    }
    fn acts_mut(&mut self) -> &mut Acts {
        &mut self.acts
    }
}

fn inspect_world(
    state: Res<State<WorldInspectorState>>,
    mut next_state: ResMut<NextState<WorldInspectorState>>,
    mut minibuffer: Minibuffer,
) {
    use WorldInspectorState::*;
    let state = match state.get() {
        Invisible => Visible,
        Visible => Invisible,
    };
    next_state.set(state);
    minibuffer.clear()
}

impl Default for WorldActs {
    fn default() -> Self {
        Self {
            acts: Acts::new([Act::new(inspect_world)]),
        }
    }
}

impl Plugin for WorldActs {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            WorldInspectorPlugin::default()
                .run_if(in_state(PromptState::Visible).and(in_state(WorldInspectorState::Visible))),
        )
        .init_state::<WorldInspectorState>();
        self.warn_on_unused_acts();
    }
}
