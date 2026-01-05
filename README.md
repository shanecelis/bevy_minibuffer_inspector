# bevy_minibuffer_inspector

Inspect bevy worlds, assets, resources, and more with
[bevy_minibuffer](https://github.com/shanecelis/bevy_minibuffer) and
[bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui).

This crate merely adapts a gamedev console to invoke bevy-inspector-egui's
inspectors. The inspectors themselves are a wonder of that crate.

## Acts

This crate exposes the following Minibuffer acts, i.e., commands:
- inspect_world,
- inspect_resource,
- inspect_asset,
- inspect_state,
- and inspect_filter_query.

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

There is no configuration required for `WorldActs` unless you want to add a key
binding.

```rust no_run
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;
use bevy_minibuffer_inspector as inspector;

fn plugin(app: &mut App) {
    app
        .add_plugins(MinibufferPlugins)
        .add_acts((
            BasicActs::default(),
            inspector::WorldActs::default()
                .configure("inspect_world", |mut act| {
                    act.bind(keyseq! { I W });
                }),
        ));
}
```

### inspect_resource
<img align="right" src="https://github.com/user-attachments/assets/4ea741f3-6224-4421-a844-6dc3a21e406e"/>

`ResourceActs` provides the 'inspect_resource' act. Register the resources that
it prompts for. If no resources are registered, a warning will be emitted in the
logs and the 'inspect_resource' act will report that there are no resources
available when run.

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
<img align="right" src="https://github.com/user-attachments/assets/b85ccf90-c9de-4298-b645-3fdd88ff3636"/>

`AssetActs` provides the 'inspect_asset' act. Register the assets that it
prompts for. If no assets are registered, a warning will be emitted in the
logs and the 'inspect_asset' act will report that there are no assets
available when run.


```rust no_run
use bevy::prelude::*;
use bevy_minibuffer::prelude::*;
use bevy_minibuffer_inspector as inspector;
fn plugin(app: &mut App) {
    app
        .add_plugins(MinibufferPlugins)
        .add_acts((
            BasicActs::default(),
            inspector::AssetActs::default()
                .add::<StandardMaterial>(),
        ));
}
```

### inspect_state
<img align="right" src="https://github.com/user-attachments/assets/7324be62-87b9-4f36-94c7-db62c979195d"/>

`StateActs` provides the 'inspect_state' act. Register the states that it
prompts for. If no states are registered, a warning will be emitted in the
logs and the 'inspect_state' act will report that there are no states
available when run.


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
            inspector::StateActs::default()
                .add::<AppState>(),
        ));
}
```

### inspect_filter_query
<img align="right" src="https://github.com/user-attachments/assets/723b60a9-a9f0-4983-a4cf-31acf0f88dc8"/>

`FilterQueryActs` provides the 'inspect_filter_query' act. Register the filters
that it prompts for. If no filter queries are registered, a warning will be
emitted in the logs and the 'inspect_filter querie' act will report that there
are no filter queries available when run.

This is probably one of the most useful ways to get at exactly what one's
interested in.

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

| bevy_minibuffer_inspector | bevy_minibuffer | bevy |
|---------------------------|-----------------|------|
| 0.3.0                     | 0.5             | 0.17 |
| 0.2.0                     | 0.4             | 0.16 |
| 0.1.0                     | 0.3             | 0.15 |

## License

This crate is licensed under the MIT License or the Apache License 2.0.

## Acknowledgments

Thanks to [Jakob Hellermann](https://github.com/jakobhellermann) for
[bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui)
which I often reach for to dig into my own Bevy projects. That is the reason
this is the first integration I made for bevy_minibuffer.
