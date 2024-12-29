# bevy_minibuffer_inspector

Add commands to inspect the world, assets, resources, and more to
[bevy_minibuffer](https://github.com/shanecelis/bevy_minibuffer).

This crate integrates bevy_minibuffer with
[bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui) to
provide another means of invoking inspectors.

## Acts

The Minibuffer acts, i.e., commands this module makes available are:
- inspect_world
- inspect_resource
- inspect_asset
- inspect_state
- inspect_filter_query

They may be used _a la carte_.

## Key Bindings

No key bindings have been defined. Users are welcome to add them.

```no_run
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

## Configuration

The `WorldActs` provides 'inspect_world' act and it is the only one that does
not require any type registration.

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

### Type Registration

Each of the other acts do expect type registrations. For instance, the
`AssetActs` provides 'inspect_asset' but expects registration of
what assets it should prompt for when the act is invoked. A warning will be
emitted if no types have been registered.

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
                .add::<StandardMaterial>()
        ));
}
```

DESIGN NOTE: There may be ways to automatically register various assets,
resources, and other types but I would actually decline to do that as of now. It
can quickly make a mess, become overwhelming, and takes control out of the
user's hands.

## Visibility

Each act toggles the visibility of an inspector. However, each inspector's
visibility is tied to minibuffer's visibility. When minibuffer is invisible
so are its inspectors and vice versa.

NOTE: Any inspectors configured without the minibuffer module are
independent of minibuffer's influence, so that's one escape hatch to this
behavior.

# Compatibility

| bevy_minibuffer_inspector | bevy |
|---------------------------|------|
| 0.1.0                     | 0.15 |

# License

This crate is licensed under the MIT License or the Apache License 2.0.
