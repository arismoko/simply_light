//! Simply Light — the one `impl Hooks` at the crate root.
//!
//! The 160 light blocks are content in `data/` and `assets/` (block declarations on vanilla
//! templates, their states, recipes and tags). The code here is the one behaviour content can't
//! say: the illuminant blocks' redstone switch (upstream `LampBlock`), as a block part on the 32
//! illuminant blocks.
//!
//! [`runtime`] is the port's census model of the content, checked by `cargo test` alone.

pub mod editor;
pub mod features;
pub mod recipes;
pub mod runtime;

use mod_api::{
    self, BlockChangedEvent, BlockPart, ClientHost, ClientTickPostEvent, Flow, Hooks, HostSide,
    IsSignalSourceEvent, LevelHost, LevelTickPostEvent, LevelWriteHost, NeighborChangedEvent,
    PayloadId, PlayerLoggedInEvent, Registrar, RegistrationContext, ScreenInitPostEvent,
    ScreenMouseButtonPressedPreEvent, ScreenRenderBackgroundEvent, ServerHost, SetPlacedByEvent,
};

/// The 16 colours in upstream's registration order; white is the bare `illuminant_block`.
const COLOURS: [&str; 16] = [
    "white",
    "orange",
    "magenta",
    "light_blue",
    "yellow",
    "lime",
    "pink",
    "gray",
    "light_gray",
    "cyan",
    "purple",
    "blue",
    "brown",
    "green",
    "red",
    "black",
];

/// The illuminant block of `colour`, plain (lit by redstone) or `_on` (dimmed by redstone).
fn illuminant_id(colour: &str, inverted: bool) -> String {
    let base = if colour == "white" {
        "simplylight:illuminant_block".to_owned()
    } else {
        format!("simplylight:illuminant_{colour}_block")
    };
    if inverted { base + "_on" } else { base }
}

/// `LampBlock`'s rule: a lamp shows its default (`on` for the `_on` blocks, off for the others)
/// unless a neighbour powers it, which inverts it.
fn lit(inverted: bool, powered: bool) -> bool {
    inverted != powered
}

/// Vanilla's `Block.UPDATE_ALL` (`UPDATE_NEIGHBORS | UPDATE_CLIENTS`).
const UPDATE_ALL: i32 = 3;

fn on_value(on: bool) -> &'static str {
    if on { "true" } else { "false" }
}

/// The illuminant blocks' redstone switch (`LampBlock.setPlacedBy`, `neighborChanged`,
/// `isSignalSource`). `inverted` is the block's default `on`.
struct Lamp {
    inverted: bool,
}

#[mod_api::sunrise_part]
impl BlockPart for Lamp {
    /// `LampBlock.setPlacedBy`: a lamp placed beside a powered block takes its other state
    /// at once (`setBlockAndUpdate`, as `set_block_property` writes).
    fn set_placed_by(&self, host: &mut dyn ServerHost, event: &mut SetPlacedByEvent<'_>) {
        if event
            .level
            .best_neighbor_signal(&*host, event.pos)
            .is_some_and(|s| s > 0)
        {
            host.set_block_property(event.pos, "on", on_value(lit(self.inverted, true)));
        }
    }

    /// `LampBlock.neighborChanged`: the lamp takes its new state at once (`setBlockAndUpdate`,
    /// `Block.UPDATE_ALL`: neighbours and clients).
    fn neighbor_changed(&self, level: &mut dyn LevelWriteHost, event: &mut NeighborChangedEvent) {
        let pos = [event.pos.x, event.pos.y, event.pos.z];
        if let Some(on) = level.has_neighbor_signal(pos) {
            let target = on_value(lit(self.inverted, on));
            if level.state_with_property(event.state, "on", target) != Some(event.state) {
                level.set_block_property(pos, "on", target, UPDATE_ALL);
            }
        }
    }

    /// `LampBlock.isSignalSource` is true, so redstone dust beside a lamp turns toward it.
    fn is_signal_source(&self, _: &dyn LevelHost, event: &mut IsSignalSourceEvent) {
        event.source = true;
    }
}

#[derive(Default)]
struct Mod;

#[mod_api::sunrise_mod]
impl Hooks for Mod {
    /// Attach the switch to the 32 illuminant blocks.
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        for colour in COLOURS {
            for inverted in [false, true] {
                let block = r.block_key(&illuminant_id(colour, inverted));
                r.block_part(block, Lamp { inverted });
            }
        }
        features::declare(r);
        let (open, update) = editor::handlers();
        r.payload_handlers()
            .registrar("1")
            .optional()
            .play_to_client(PayloadId(editor::OPEN.into()), Some(open))
            .play_to_server(PayloadId(editor::UPDATE.into()), update);
    }

    /// `SimplyLight.PackFinders`: the optional full-block connected-texture pack.
    fn add_pack_finders(
        &self,
        _: &mut dyn mod_api::PackDiscoveryContext,
        event: &mut mod_api::AddPackFindersEvent<'_>,
    ) {
        if let Ok(location) = mod_api::Identifier::parse(
            "simplylight:assets/simplylight/resourcepacks/optional_fullblock_ctm",
        ) {
            let title = mod_api::Text::translate("simplylight.pack.fullblock_ctm", vec![]);
            let _ = event.packs.add_pack_finders(mod_api::PackRegistration::new(
                location,
                mod_api::PackType::ClientResources,
                title,
            ));
        }
    }

    /// The editor screen over the world.
    fn screen_render_background(
        &self,
        host: &dyn ClientHost,
        event: &mut ScreenRenderBackgroundEvent<'_>,
    ) {
        editor::render(host, event);
    }

    /// The editor's exit button.
    fn screen_init_post(&self, _: &dyn ClientHost, event: &mut ScreenInitPostEvent<'_>) {
        editor::init(event);
    }

    /// A click on one of the editor's switches.
    fn screen_mouse_button_pressed_pre(
        &self,
        host: &dyn ClientHost,
        event: &mut ScreenMouseButtonPressedPreEvent<'_>,
    ) -> Flow {
        editor::click(host, event)
    }

    /// What the editor's clicks queued goes out.
    fn client_tick_post(&self, host: &mut dyn ClientHost, _: &mut ClientTickPostEvent) {
        editor::tick(host);
    }

    /// `RecipeUnlocker`: a new player gets the mod's recipes.
    fn player_logged_in(&self, host: &mut dyn ServerHost, event: &mut PlayerLoggedInEvent<'_>) {
        features::on_player_logged_in(host, event);
    }

    /// A lamp post block that was removed takes its column with it.
    fn block_changed(&self, level: &dyn LevelHost, event: &mut BlockChangedEvent) {
        features::on_block_changed(level, event);
    }

    /// After the level's tick, in the level that ticked: clear the columns of removed posts.
    fn level_tick_post(&self, host: HostSide<'_>, event: &mut LevelTickPostEvent) {
        if let HostSide::Server(host) = host {
            features::on_level_tick_post(host, event.level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redstone_inverts_each_lamp_kind() {
        assert!(!lit(false, false));
        assert!(lit(false, true));
        assert!(lit(true, false));
        assert!(!lit(true, true));
    }

    #[test]
    fn ids_follow_upstream_names() {
        assert_eq!(
            illuminant_id("white", false),
            "simplylight:illuminant_block"
        );
        assert_eq!(
            illuminant_id("white", true),
            "simplylight:illuminant_block_on"
        );
        assert_eq!(
            illuminant_id("light_blue", true),
            "simplylight:illuminant_light_blue_block_on"
        );
    }
}
