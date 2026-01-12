use crate::{utils::pretty_type_name, InspectorPlugins, Inspectors};
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ecs::{
    prelude::{On, Res, ResMut},
    schedule::SystemCondition,
};
use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_minibuffer::{prelude::*, prompt::PromptState};
use bevy_reflect::Reflect;
use bevy_state::{prelude::in_state, state::FreelyMutableState};

/// ## Adds the 'inspect_state' act
///
/// This act toggles the visibility of registered state inspectors.
///
/// ## Usage
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_minibuffer::prelude::*;
/// use bevy_minibuffer_inspector as inspector;
/// #[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash, Reflect)]
/// enum AppState {
///     #[default]
///     A,
///     B,
///     C,
/// }
/// fn plugin(app: &mut App) {
///     app
///         .add_plugins(MinibufferPlugins)
///         .add_acts((
///             BasicActs::default(),
///             inspector::StateActs::default()
///                 .add::<AppState>()
///         ));
/// }
/// ```
pub struct StateActs {
    plugins: InspectorPlugins<Self>,
    acts: Acts,
}

impl PluginGroup for StateActs {
    fn build(self) -> PluginGroupBuilder {
        self.warn_on_unused_acts();
        self.plugins
            .warn_on_empty("No states registered with `StateActs`; consider adding some.");
        self.plugins.build()
    }
}

impl ActsPluginGroup for StateActs {
    fn acts(&self) -> &Acts {
        &self.acts
    }

    fn acts_mut(&mut self) -> &mut Acts {
        &mut self.acts
    }
}

impl StateActs {
    /// Add a state to the list of inspectors when prompted.
    pub fn add<S: FreelyMutableState + Reflect>(mut self) -> Self {
        self.plugins
            .add_inspector(pretty_type_name::<S>(), Self::add_plugin::<S>);
        self
    }

    fn add_plugin<A: FreelyMutableState + Reflect>(
        index: usize,
        inspector_plugins: &mut InspectorPlugins<Self>,
    ) {
        inspector_plugins.add_plugin(
            StateInspectorPlugin::<A>::default().run_if(
                in_state(PromptState::Visible).and(InspectorPlugins::<Self>::visible(index)),
            ),
        );
    }
}

fn inspect_state(states: Res<Inspectors<StateActs>>, mut minibuffer: Minibuffer) {
    if !states.visible.is_empty() {
        minibuffer
            .prompt_map("state: ", states.names.clone())
            .observe(
                |mut trigger: On<Completed<usize>>,
                 mut minibuffer: Minibuffer,
                 mut states: ResMut<Inspectors<StateActs>>| {
                    match trigger.event_mut().state.take_result().unwrap() {
                        Ok(index) => {
                            states.visible[index] = !states.visible[index];
                            minibuffer.clear();
                        }
                        Err(e) => {
                            minibuffer.message(format!("{e}"));
                        }
                    }
                },
            );
    } else {
        minibuffer.message("No states registered.");
    }
}

impl Default for StateActs {
    fn default() -> Self {
        Self {
            plugins: InspectorPlugins::default(),
            acts: Acts::new([Act::new(inspect_state)]),
        }
    }
}
