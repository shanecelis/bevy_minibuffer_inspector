use crate::{InspectorPlugins, Inspectors, utils::pretty_type_name};
use bevy_inspector_egui::{
    quick::StateInspectorPlugin,
};
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ecs::{
    prelude::{Res, ResMut, Trigger},
    schedule::Condition,
};
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
                |mut trigger: Trigger<Completed<usize>>,
                 mut states: ResMut<Inspectors<StateActs>>| {
                    if let Ok(index) = trigger.event_mut().take_result().unwrap() {
                        states.visible[index] = !states.visible[index];
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
            // acts: Acts::new([Act::new(InspectorPlugins::<StateActs>::inspector("state: ", "No states registered"))
            // .named("inspect_state")])
        }
    }
}
