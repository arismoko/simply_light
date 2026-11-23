//! The lights' behaviour beyond the illuminant switch: the three-block lamp post, the edge
//! lights' placement, and the shift-detail tooltips.
//!
//! Upstream: `blocks/LampPost.java` (placement, `setPlacedBy`, `onRemove`, light from the top
//! block only), `blocks/EdgeLight.java` (`getStateForPlacement`) and `blocks/BaseBlockItem.java`
//! (`appendHoverText`). The rules are pure functions at the top, tested by `cargo test`.

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use mod_api::{
    self, AttachmentCodec, AttachmentType, BlockChangedEvent, BlockPart, BlockState,
    BlockUpdateFlags, ClientHost, Compound, Host, ItemPart, LevelHost, LevelRef, LevelSide,
    LightEmissionEvent, PlayerLoggedInEvent, Registrar, ServerHost, SetPlacedByEvent,
    StateForPlacementEvent, Tag, Text, TooltipEvent,
};

/// The 15 dye colours; white is the bare name.
pub const COLOURS: [&str; 15] = [
    "light_gray",
    "gray",
    "black",
    "brown",
    "red",
    "orange",
    "yellow",
    "lime",
    "green",
    "cyan",
    "light_blue",
    "blue",
    "purple",
    "magenta",
    "pink",
];

/// `{base}` and `{base}_{colour}` for every colour, in upstream's order.
pub fn coloured(base: &str) -> Vec<String> {
    std::iter::once(base.to_owned())
        .chain(COLOURS.iter().map(|colour| format!("{base}_{colour}")))
        .collect()
}

/// `LampPost`'s light: 15 for the top block, 0 for the others (`lightLevel`).
pub fn post_light(position: &str) -> u8 {
    if position == "top" { 15 } else { 0 }
}

/// The other blocks of a post, as offsets from the block at `position`
/// (`LampPost.onRemove`): the two others in the column of three.
pub fn post_column(position: &str) -> [i32; 2] {
    match position {
        "top" => [-1, -2],
        "middle" => [-1, 1],
        _ => [1, 2],
    }
}

/// The property value in a state's text form (`block[a=b,c=d]`).
pub fn property_of<'a>(state: &'a str, key: &str) -> Option<&'a str> {
    let body = state.split_once('[')?.1.strip_suffix(']')?;
    body.split(',')
        .find_map(|pair| pair.strip_prefix(key)?.strip_prefix('='))
}

/// A state's text with one property replaced.
pub fn with_property(state: &str, key: &str, value: &str) -> Option<String> {
    let old = property_of(state, key)?;
    Some(state.replacen(&format!("{key}={old}"), &format!("{key}={value}"), 1))
}

/// The tooltip base a block's item uses (`BaseBlockItem.getTooltipBase`): a coloured light reads
/// its white sibling's text.
pub fn tooltip_base(tag: &str) -> Option<&'static str> {
    Some(match tag {
        "any_bulb" => "lightbulb",
        "any_edge_light" => "edge_light",
        "any_edge_light_top" => "edge_light_top",
        "any_fixture" => "wall_lamp",
        "any_lamp_off" => "illuminant_block",
        "any_lamp_on" => "illuminant_block_on",
        "any_panel" => "illuminant_panel",
        "any_post" => "lamp_post",
        "any_rod" => "rodlamp",
        "any_slab" => "illuminant_slab",
        _ => return None,
    })
}

/// The recipe version a player has been given (`RecipeUnlocker`'s persistent `simplylight_unlocked`).
static UNLOCKED: AttachmentType<i32> = AttachmentType {
    namespace: "simplylight",
    name: "recipes_unlocked",
    default: || 0,
    codec: Some(AttachmentCodec {
        encode: |value| {
            let mut out = Compound::new();
            out.insert("version".into(), Tag::Int(*value));
            Ok(out)
        },
        decode: |saved| match saved.get(&"version".into()) {
            Some(Tag::Int(value)) => Ok(*value),
            _ => Err(mod_api::AttachmentError::Decode),
        },
    }),
    copy_on_death: true,
    sync: false,
};

/// The version of the recipe set; raising it gives players the new recipes at their next join.
const RECIPE_VERSION: i32 = 4;

/// `RecipeUnlocker.onPlayerLoggedIn`: a player who has not been given this version of the recipes
/// gets all of them.
pub fn on_player_logged_in(host: &mut dyn ServerHost, event: &mut PlayerLoggedInEvent<'_>) {
    let Some(player) = host.player(event.player) else {
        return;
    };
    let holder = mod_api::Holder::Player(event.player);
    if UNLOCKED
        .get(host.attachments(), holder)
        .is_ok_and(|seen| seen >= RECIPE_VERSION)
    {
        return;
    }
    if player
        .award_recipes(host, crate::recipes::RECIPES)
        .is_some()
    {
        let _ = UNLOCKED.set(host.attachments(), holder, RECIPE_VERSION);
    }
}

const POST_BASE: &str = "simplylight:lamp_post";

/// The three blocks of one lamp post: light by position, placement and the column.
pub struct Post;

#[mod_api::sunrise_part]
impl BlockPart for Post {
    /// Only the top block emits.
    fn light_emission(&self, host: &dyn Host, event: &mut LightEmissionEvent) -> Option<u8> {
        let state = host.block_state_string(event.state)?;
        Some(post_light(property_of(&state, "position")?))
    }

    /// `LampPost.getStateForPlacement`: room for the two blocks above, else no placement.
    fn state_for_placement(
        &self,
        level: LevelSide,
        event: &mut StateForPlacementEvent<'_>,
    ) -> Option<Option<BlockState>> {
        let (LevelSide::Server(level) | LevelSide::Client(level)) = level;
        let pos = [
            event.context.pos.x,
            event.context.pos.y,
            event.context.pos.z,
        ];
        let replaceable = |dy: i32| {
            level
                .block_state([pos[0], pos[1] + dy, pos[2]])
                .and_then(|state| level.block_states().replaceable(state))
                .unwrap_or(false)
        };
        if replaceable(1) && replaceable(2) {
            Some(Some(event.state))
        } else {
            Some(None)
        }
    }

    /// `LampPost.setPlacedBy`: the middle and top blocks.
    fn set_placed_by(&self, host: &mut dyn ServerHost, event: &mut SetPlacedByEvent<'_>) {
        let Some(text) = host.block_state_string(event.state) else {
            return;
        };
        for (dy, position) in [(1, "middle"), (2, "top")] {
            let Some(next) = with_property(&text, "position", position) else {
                continue;
            };
            let Some(state) = host.block_state_id(&next) else {
                continue;
            };
            let at = [event.pos[0], event.pos[1] + dy, event.pos[2]];
            let _ = event.level.set_block(
                host,
                at,
                state,
                BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS,
            );
        }
    }
}

/// An edge light: its sides follow the sturdy blocks around it when it is placed
/// (`EdgeLight.getStateForPlacement`).
pub struct Edge;

/// (property, neighbour offset, the neighbour's face that touches the light).
const SIDES: [(&str, [i32; 3], mod_api::BlockFace); 4] = [
    ("north", [0, 0, -1], mod_api::BlockFace::South),
    ("south", [0, 0, 1], mod_api::BlockFace::North),
    ("east", [1, 0, 0], mod_api::BlockFace::West),
    ("west", [-1, 0, 0], mod_api::BlockFace::East),
];

#[mod_api::sunrise_part]
impl BlockPart for Edge {
    fn state_for_placement(
        &self,
        level: LevelSide,
        event: &mut StateForPlacementEvent<'_>,
    ) -> Option<Option<BlockState>> {
        let (LevelSide::Server(level) | LevelSide::Client(level)) = level;
        let pos = [
            event.context.pos.x,
            event.context.pos.y,
            event.context.pos.z,
        ];
        let mut state = event.state;
        for (property, offset, face) in SIDES {
            let at = [pos[0] + offset[0], pos[1] + offset[1], pos[2] + offset[2]];
            let supported = level
                .block_face_sturdy(at, face, mod_api::SupportType::Center)
                .unwrap_or(false);
            state = level.state_with_property(
                state,
                property,
                if supported { "true" } else { "false" },
            )?;
        }
        Some(Some(state))
    }
}

fn styled(key: &str, colour: &str, italic: bool) -> Text {
    let mut style = mod_api::text::TextStyle::default();
    style.color = Some(colour.to_owned());
    style.italic = italic.then_some(true);
    Text::translate(key, vec![]).with_style(style)
}

/// A light's shift-detail tooltip (`BaseBlockItem.appendHoverText`).
pub struct Info {
    /// The tooltip base: `block.simplylight.<base>.info`.
    pub base: &'static str,
}

#[mod_api::sunrise_part]
impl ItemPart for Info {
    fn tooltip(&self, host: &dyn ClientHost, event: &mut TooltipEvent<'_>) {
        let gray = 0xAAAAAA;
        let key = format!("block.simplylight.{}.info", self.base);
        let shift = host.is_key_down("key.keyboard.left.shift")
            || host.is_key_down("key.keyboard.right.shift");
        if shift && host.translate(&key).is_some() {
            event
                .lines
                .push((Text::translate(key.clone(), vec![]), gray));
            let second = format!("{key}2");
            if host.translate(&second).is_some() {
                let redstone = matches!(self.base, "illuminant_block" | "illuminant_block_on");
                let args = if redstone {
                    vec![styled("simplylight.redstone", "dark_red", false)]
                } else {
                    vec![]
                };
                event.lines.push((Text::translate(second, args), gray));
            }
        } else {
            let key_name = styled("simplylight.key.shift", "gold", true);
            event
                .lines
                .push((Text::translate("simplylight.shift", vec![key_name]), gray));
        }
    }
}

/// Lamp post blocks that left a column, to clear their partners after the tick
/// (`LampPost.onRemove`).
#[derive(Default)]
struct Torn {
    by_level: BTreeMap<&'static str, Vec<([i32; 3], BlockState)>>,
}

fn torn() -> std::sync::MutexGuard<'static, Torn> {
    static TORN: OnceLock<Mutex<Torn>> = OnceLock::new();
    TORN.get_or_init(|| Mutex::new(Torn::default()))
        .lock()
        .expect("the torn post lock is never held across a panic")
}

/// A block was committed: a lamp post block that is no longer one leaves its column behind.
pub fn on_block_changed(level: &dyn LevelHost, event: &mut BlockChangedEvent) {
    let Some(old) = level.state_block(event.old) else {
        return;
    };
    if !old.starts_with(POST_BASE) || level.state_block(event.new) == Some(old) {
        return;
    }
    torn()
        .by_level
        .entry(event.dimension.key().id())
        .or_default()
        .push(([event.pos.x, event.pos.y, event.pos.z], event.old));
}

/// After the level's tick: remove the partners of every post block that was removed.
pub fn on_level_tick_post(host: &mut dyn ServerHost, level: LevelRef) {
    if !level.in_place(&*host) {
        return;
    }
    let Some(dimension) = level.dimension().id(&*host) else {
        return;
    };
    let Some(torn) = torn().by_level.remove(dimension) else {
        return;
    };
    let Some(air) = host.block_state_id("minecraft:air") else {
        return;
    };
    for (pos, old) in torn {
        let Some(text) = host.block_state_string(old) else {
            continue;
        };
        let Some(position) = property_of(&text, "position") else {
            continue;
        };
        for dy in post_column(position) {
            let at = [pos[0], pos[1] + dy, pos[2]];
            let is_post = level
                .block(&*host, at)
                .and_then(|block| block.id(&*host))
                .is_some_and(|id| id.starts_with(POST_BASE));
            if is_post {
                let _ = level.set_block(
                    host,
                    at,
                    air,
                    BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS,
                );
            }
        }
    }
}

/// Attach the parts.
pub fn declare(r: &mut dyn Registrar) {
    let _ = r.register_attachment_type("simplylight:recipes_unlocked", false);
    for id in coloured("lamp_post") {
        let block = r.block_key(&format!("simplylight:{id}"));
        r.block_part(block, Post);
    }
    for base in ["edge_light", "edge_light_top"] {
        let ids = if base == "edge_light" {
            coloured("edge_light")
        } else {
            std::iter::once("edge_light_top".to_owned())
                .chain(COLOURS.iter().map(|c| format!("edge_light_top_{c}")))
                .collect()
        };
        for id in ids {
            let block = r.block_key(&format!("simplylight:{id}"));
            r.block_part(block, Edge);
        }
    }
    for tag in [
        "any_bulb",
        "any_edge_light",
        "any_edge_light_top",
        "any_fixture",
        "any_lamp_off",
        "any_lamp_on",
        "any_panel",
        "any_post",
        "any_rod",
        "any_slab",
    ] {
        if let Some(base) = tooltip_base(tag) {
            r.item_part_tag(&format!("simplylight:{tag}"), Info { base });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_top_of_a_post_emits() {
        assert_eq!(post_light("top"), 15);
        assert_eq!(post_light("middle"), 0);
        assert_eq!(post_light("bottom"), 0);
    }

    #[test]
    fn a_removed_block_takes_the_rest_of_its_column() {
        assert_eq!(post_column("bottom"), [1, 2]);
        assert_eq!(post_column("middle"), [-1, 1]);
        assert_eq!(post_column("top"), [-1, -2]);
    }

    #[test]
    fn properties_read_and_rewrite_in_state_text() {
        let text = "simplylight:lamp_post[position=bottom,waterlogged=false]";
        assert_eq!(property_of(text, "position"), Some("bottom"));
        assert_eq!(
            with_property(text, "position", "top").as_deref(),
            Some("simplylight:lamp_post[position=top,waterlogged=false]")
        );
        assert_eq!(property_of("simplylight:rodlamp", "axis"), None);
    }

    #[test]
    fn colours_come_in_upstream_order_with_white_first() {
        let posts = coloured("lamp_post");
        assert_eq!(posts.len(), 16);
        assert_eq!(posts[0], "lamp_post");
        assert_eq!(posts[1], "lamp_post_light_gray");
    }

    #[test]
    fn tags_map_to_their_white_sibling_text() {
        assert_eq!(tooltip_base("any_fixture"), Some("wall_lamp"));
        assert_eq!(tooltip_base("any_lamp_on"), Some("illuminant_block_on"));
        assert_eq!(tooltip_base("any_nothing"), None);
    }
}
