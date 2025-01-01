use crate::{utils::pretty_type_name, InspectorPlugins, Inspectors};
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ecs::{
    prelude::{Res, ResMut, Resource, Trigger},
    schedule::Condition,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_minibuffer::{prelude::*, prompt::PromptState};
use bevy_reflect::Reflect;
use bevy_state::prelude::in_state;

/// ## Adds the 'inspect_resource' act
///
/// This act toggles the visibility of resource inspectors that were added.
///
/// ## Usage
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_minibuffer::prelude::*;
/// use bevy_minibuffer_inspector as inspector;
/// #[derive(Resource, Reflect)]
/// struct R1;
/// #[derive(Resource, Reflect)]
/// struct R2;
/// fn plugin(app: &mut App) {
///     app
///         .add_plugins(MinibufferPlugins)
///         .add_acts((
///             BasicActs::default(),
///             inspector::ResourceActs::default()
///                 .add::<R1>()
///                 .add::<R2>()
///         ));
/// }
/// ```
pub struct ResourceActs {
    plugins: InspectorPlugins<Self>,
    acts: Acts,
}

impl ActsPluginGroup for ResourceActs {
    fn acts(&self) -> &Acts {
        &self.acts
    }

    fn acts_mut(&mut self) -> &mut Acts {
        &mut self.acts
    }
}

impl ResourceActs {
    /// Add a resource to the list of resources when prompted.
    pub fn add<R: Resource + Reflect>(mut self) -> Self {
        self.plugins.add_inspector(
            pretty_type_name::<R>(),
            Self::resource_inspector_plugin::<R>,
        );
        self
    }

    fn resource_inspector_plugin<R: Resource + Reflect>(
        index: usize,
        inspector_plugins: &mut InspectorPlugins<Self>,
    ) {
        inspector_plugins.add_plugin(
            ResourceInspectorPlugin::<R>::default().run_if(
                in_state(PromptState::Visible).and(InspectorPlugins::<Self>::visible(index)),
            ),
        );
    }
}

impl Default for ResourceActs {
    fn default() -> Self {
        Self {
            plugins: InspectorPlugins::default(),
            acts: Acts::new([Act::new(inspect_resource)]),
        }
    }
}

fn inspect_resource(resources: Res<Inspectors<ResourceActs>>, mut minibuffer: Minibuffer) {
    if !resources.visible.is_empty() {
        minibuffer
            .prompt_map("resource: ", resources.names.clone())
            .observe(
                |mut trigger: Trigger<Completed<usize>>,
                 mut minibuffer: Minibuffer,
                 mut resources: ResMut<Inspectors<ResourceActs>>| {
                    match trigger.event_mut().take_result().unwrap() {
                        Ok(index) => {
                            resources.visible[index] = !resources.visible[index];
                            minibuffer.clear();
                        }
                        Err(e) => {
                            minibuffer.message(format!("{e}"));
                        }
                    }
                },
            );
    } else {
        minibuffer.message("No resource inspectors available.");
    }
}

impl PluginGroup for ResourceActs {
    fn build(self) -> PluginGroupBuilder {
        self.warn_on_unused_acts();
        self.plugins
            .warn_on_empty("No resources registered with `ResourceActs`; consider adding some.");
        self.plugins.build()
    }
}
