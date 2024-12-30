# bevy_minibuffer_inspector

Add commands to inspect the world, assets, resources, and more to
[bevy_minibuffer](https://github.com/shanecelis/bevy_minibuffer).

This crate integrates bevy_minibuffer with
[bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui) to
provide another means of invoking inspectors.

## Acts

The Minibuffer acts, i.e., commands, this crate makes available are:
- inspect_world
- inspect_resource
- inspect_asset
- inspect_state
- inspect_filter_query

They may be used _a la carte_.

### inspect_world
<img align="right" src="https://github.com/user-attachments/assets/d6a8f259-5469-4e32-93d9-aefbf54a0e5a"/>

`WorldActs` provides 'inspect_world' act.

```rust no_run
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;
use bevy_minibuffer_inspector as inspector;
fn plugin(app: &mut App) {
    app
        .add_plugins(MinibufferPlugins)
        .add_acts((
            BasicActs::default(),
            inspector::WorldActs::default(),
        ));
}
```

### inspect_resource
<img align="right" src=""/>

`ResourceActs` provides the 'inspect_resource' act. One must register the
resources that are shown at its prompt.

```rust no_run
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;
use bevy_minibuffer_inspector as inspector;
#[derive(Resource, Reflect)]
struct Configuration { verbose: bool };
fn plugin(app: &mut App) {
    app
        .add_plugins(MinibufferPlugins)
        .add_acts((
            BasicActs::default(),
            inspector::ResourceActs::default()
                .add::<Configuration>(),
        ));
}
```

### inspect_asset
<img align="right" src=""/>

`AssetActs` provides the 'inspect_asset' act. Register the assets that it
prompts for.

```rust no_run
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;
use bevy_minibuffer_inspector as inspector;
fn plugin(app: &mut App) {
    app
        .add_plugins(MinibufferPlugins)
        .add_acts((
            BasicActs::default(),
            inspector::AssetActs::default().add::<StandardMaterial>(),
        ));
}
```

### inspect_state
<img align="right" src=""/>

`StateActs` provides the 'inspect_state' act. Register the states that it
prompts for.


```rust no_run
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;
use bevy_minibuffer_inspector as inspector;
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Reflect)]
enum AppState { A, B, C }
fn plugin(app: &mut App) {
    app
        .add_plugins(MinibufferPlugins)
        .add_acts((
            BasicActs::default(),
            inspector::StateActs::default().add::<AppState>(),
        ));
}
```

### inspect_filter_query
<img align="right" src=""/>

`FilterQueryActs` provides the 'inspect_filter_query' act. Register the filters
that it prompts for.

```rust no_run
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;
use bevy_minibuffer_inspector as inspector;
fn plugin(app: &mut App) {
    app
        .add_plugins(MinibufferPlugins)
        .add_acts((
            BasicActs::default(),
            inspector::FilterQueryActs::default()
                .add::<With<Transform>>()
                .add::<With<Mesh3d>>(),
        ));
}
```

## Key Bindings

No key bindings are defined. Users are welcome to add them.

```rust no_run
use bevy_minibuffer::prelude::*;
use bevy_minibuffer_inspector as inspector;
let mut inspector_acts = inspector::WorldActs::default();
inspector_acts.acts_mut().configure("inspect_world", |mut act| {
   act.bind(keyseq! { I W });
});
```

I wonder if maybe some bindings like this would work:
- inspect_world, `I W`
- inspect_resource, `I R`
- inspect_asset, `I A`
- inspect_state, `I S`
- inspect_filter_query, `I F`

## Interaction

Here is what an interaction for 'inspect_world' might look like:

User types ':' or 'Alt-X', a black bar and prompt appear at the bottom of
the screen---that's the minibuffer. The user types 'inspect_w Tab Return' to tab
complete the 'inspect_world' act. The world inspector appears. If the user
hits the 'BackTick' (`) key, the minibuffer will disappear and so will the
inspector. Hit the 'BackTick' key again and both reappear.



## TODO
- [ ] Make aliases available for registered kinds.

## Notes

DESIGN NOTE: There may be ways to automatically register various assets,
resources, and other types but I would actually decline to do that as of now. It
can quickly make a mess, become overwhelming, and takes control out of the
user's hands.

## Visibility

Each act toggles the visibility of its inspector. However, each inspector's
visibility is tied to Minibuffer's visibility. When Minibuffer is invisible
so are its inspectors and vice versa.

NOTE: Any inspectors configured without the minibuffer module are
independent of minibuffer's influence.

## Compatibility

| bevy_minibuffer_inspector | bevy |
|---------------------------|------|
| 0.1.0                     | 0.15 |

## License

This crate is licensed under the MIT License or the Apache License 2.0.

## Acknowledgments

Many thanks to [Jakob Hellermann](https://github.com/jakobhellermann) for
[bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui)
which I am constantly reaching for to dig into my Bevy projects with. It is for
that reason it is the first integration I made for bevy_minibuffer.
