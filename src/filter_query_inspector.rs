use crate::{utils::pretty_type_name, InspectorPlugins, Inspectors};
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ecs::{
    prelude::{Res, ResMut, On},
    query::QueryFilter,
    schedule::SystemCondition,
};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_minibuffer::{prelude::*, prompt::PromptState};
use bevy_state::prelude::in_state;

/// ## Adds the 'inspect_filter_query' act
///
/// This act toggles the visibility of the added filter query filters.
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
///             inspector::FilterQueryActs::default()
///                 .add::<With<Transform>>()
///                 .add::<With<Mesh3d>>()
///         ));
/// }
/// ```
pub struct FilterQueryActs {
    plugins: InspectorPlugins<Self>,
    acts: Acts,
}

impl ActsPluginGroup for FilterQueryActs {
    fn acts(&self) -> &Acts {
        &self.acts
    }

    fn acts_mut(&mut self) -> &mut Acts {
        &mut self.acts
    }
}

impl FilterQueryActs {
    /// Add a query filter.
    pub fn add<A: QueryFilter + 'static>(mut self) -> Self {
        self.plugins.add_inspector(
            pretty_type_name::<A>(),
            Self::filter_query_inspector_plugin::<A>,
        );
        self
    }

    fn filter_query_inspector_plugin<A: QueryFilter + 'static>(
        index: usize,
        inspector_plugins: &mut InspectorPlugins<Self>,
    ) {
        inspector_plugins.add_plugin(
            FilterQueryInspectorPlugin::<A>::default().run_if(
                in_state(PromptState::Visible).and(InspectorPlugins::<Self>::visible(index)),
            ),
        );
    }
}

impl Default for FilterQueryActs {
    fn default() -> Self {
        Self {
            plugins: InspectorPlugins::default(),
            acts: Acts::new([Act::new(inspect_filter_query)]),
        }
    }
}

fn inspect_filter_query(filters: Res<Inspectors<FilterQueryActs>>, mut minibuffer: Minibuffer) {
    if !filters.visible.is_empty() {
        minibuffer
            .prompt_map("filter query: ", filters.names.clone())
            .observe(
                |mut trigger: On<Completed<usize>>,
                 mut minibuffer: Minibuffer,
                 mut filters: ResMut<Inspectors<FilterQueryActs>>| {
                    match trigger.event_mut().state.take_result().unwrap() {
                        Ok(index) => {
                            filters.visible[index] = !filters.visible[index];
                            minibuffer.clear();
                        }
                        Err(e) => {
                            minibuffer.message(format!("{e}"));
                        }
                    }
                },
            );
    } else {
        minibuffer.message("No filter queries registered.");
    }
}

impl PluginGroup for FilterQueryActs {
    fn build(self) -> PluginGroupBuilder {
        self.warn_on_unused_acts();
        self.plugins.warn_on_empty(
            "No filter queries registered with `FilterQueryActs`; consider adding some.",
        );
        self.plugins.build()
    }
}
