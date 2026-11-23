//! The edge light side editor (`EdgeEditorGUI`): sneak-clicking an edge light opens a small screen with
//! one switch per side, and each press sends the four switches to the server, which writes them to the block
//! (`UpdateEdgeLightPacket`, `EdgeLight.updateShape`).
//!
//! The server opens the screen with a payload carrying the block and its sides; the client draws the screen
//! over the world and takes its clicks in the screen hooks, then sends from the next client tick (the screen
//! hooks only read the client).

use std::sync::{Arc, Mutex};

use mod_api::{
    BlockUpdateFlags, ClientHost, DrawContext, Flow, IPayloadContext, IPayloadHandler,
    PayloadBytes, PayloadHandleError, PayloadId, PlayerRef, ScreenMouseButtonPressedPreEvent,
    ScreenRenderPostEvent, ServerHost,
};

use crate::features::{property_of, with_property};

pub const OPEN: &str = "simplylight:open_edge_editor";
pub const UPDATE: &str = "simplylight:update_edge_light";
const SCREEN: &str = "simplylight";

/// The sides in the order the packet's bits use: north 1, east 2, south 4, west 8.
const SIDES: [(&str, &str, u8); 4] = [
    ("north", "simplylight.gui.north", 1),
    ("east", "simplylight.gui.east", 2),
    ("south", "simplylight.gui.south", 4),
    ("west", "simplylight.gui.west", 8),
];

/// The panel's size in GUI pixels (`small_editor.png`).
const PANEL: [f32; 2] = [128.0, 70.0];
const SWITCH: [f32; 2] = [32.0, 16.0];
const EXIT: [f32; 4] = [92.0, 50.0, 32.0, 16.0];
const SPRITE_PANEL: &str = "simplylight:small_editor";
const SPRITE_SWITCH: &str = "simplylight:edge_switch";

/// How far from an edge light a player may edit it (the interaction range).
const REACH: f64 = 8.0;

/// The light being edited and its four sides, in the packet's bits.
#[derive(Clone, Copy)]
struct Open {
    pos: [i32; 3],
    bits: u8,
}

enum Action {
    Send,
    Close,
}

#[derive(Default)]
struct Client {
    open: Option<Open>,
    /// Where the last frame put the panel's corner.
    at: [f32; 2],
    actions: Vec<Action>,
}

static CLIENT: Mutex<Client> = Mutex::new(Client {
    open: None,
    at: [0.0; 2],
    actions: Vec::new(),
});

fn encode(pos: [i32; 3], bits: u8) -> PayloadBytes {
    let mut out = Vec::with_capacity(13);
    for c in pos {
        out.extend_from_slice(&c.to_be_bytes());
    }
    out.push(bits);
    PayloadBytes(out)
}

fn decode(bytes: &[u8]) -> Option<Open> {
    if bytes.len() != 13 {
        return None;
    }
    let at = |i: usize| i32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
    Some(Open {
        pos: [at(0), at(4), at(8)],
        bits: bytes[12],
    })
}

/// An edge light's sides as the packet's bits, from its state text.
pub fn bits_of(state: &str) -> u8 {
    SIDES
        .iter()
        .filter(|(side, _, _)| property_of(state, side) == Some("true"))
        .fold(0, |bits, (_, _, bit)| bits | bit)
}

/// `EdgeLight.useWithoutItem`: a sneaking player opens the editor, which the server tells their client to show.
pub fn open_for(host: &mut dyn ServerHost, player: PlayerRef, pos: [i32; 3], state: &str) {
    if player.entity().is_crouching(&*host) == Some(true) {
        let _ = player.send_payload(host, &PayloadId(OPEN.into()), encode(pos, bits_of(state)));
    }
}

pub struct OpenHandler;

impl IPayloadHandler for OpenHandler {
    fn handle(
        &self,
        payload: PayloadBytes,
        context: &mut dyn IPayloadContext,
    ) -> Result<(), PayloadHandleError> {
        let Some(open) = decode(&payload.0) else {
            return Ok(());
        };
        if let Ok(host) = context.client_host()
            && host.mod_screen_open(SCREEN)
        {
            CLIENT.lock().unwrap_or_else(|e| e.into_inner()).open = Some(open);
        }
        Ok(())
    }
}

/// `UpdateEdgeLightPacket.handle`: the four sides written to the block, when it is an edge light in reach.
pub struct UpdateHandler;

impl IPayloadHandler for UpdateHandler {
    fn handle(
        &self,
        payload: PayloadBytes,
        context: &mut dyn IPayloadContext,
    ) -> Result<(), PayloadHandleError> {
        let (Some(open), Ok(player)) = (decode(&payload.0), context.player()) else {
            return Ok(());
        };
        if let Ok(host) = context.server_host() {
            update(host, player, open);
        }
        Ok(())
    }
}

fn update(host: &mut dyn ServerHost, player: PlayerRef, open: Open) {
    let Some(level) = player
        .dimension(&*host)
        .and_then(|d| host.level(&d.to_string()))
    else {
        return;
    };
    let Some(here) = player.entity().position(&*host) else {
        return;
    };
    let centre = [
        f64::from(open.pos[0]) + 0.5,
        f64::from(open.pos[1]) + 0.5,
        f64::from(open.pos[2]) + 0.5,
    ];
    if (0..3).map(|i| (here[i] - centre[i]).powi(2)).sum::<f64>() > REACH * REACH {
        return;
    }
    let Some(state) = level.block_state(&*host, open.pos) else {
        return;
    };
    let Some(mut text) = host.block_state_string(state) else {
        return;
    };
    if !text.starts_with("simplylight:edge_light") {
        return;
    }
    for (side, _, bit) in SIDES {
        let Some(next) = with_property(
            &text,
            side,
            if open.bits & bit != 0 {
                "true"
            } else {
                "false"
            },
        ) else {
            return;
        };
        text = next;
    }
    if let Some(next) = host.block_state_id(&text) {
        // `setBlockAndUpdate`.
        let _ = level.set_block(
            host,
            open.pos,
            next,
            BlockUpdateFlags::NEIGHBORS | BlockUpdateFlags::CLIENTS,
        );
    }
}

/// The panel's top left corner in a draw of `size`.
fn corner(size: [f32; 2]) -> [f32; 2] {
    [
        ((size[0] - PANEL[0]) / 2.0).floor(),
        ((size[1] - PANEL[1]) / 2.0).floor(),
    ]
}

fn switch_rect(corner: [f32; 2], row: usize) -> [f32; 4] {
    [
        corner[0] + 4.0,
        corner[1] + 3.0 + 16.0 * row as f32,
        SWITCH[0],
        SWITCH[1],
    ]
}

fn inside(rect: [f32; 4], at: [f32; 2]) -> bool {
    at[0] >= rect[0] && at[0] < rect[0] + rect[2] && at[1] >= rect[1] && at[1] < rect[1] + rect[3]
}

/// The compass word for a yaw (`Direction.fromYRot`, horizontal).
fn facing_key(yaw: f32) -> &'static str {
    match (((yaw / 90.0) + 0.5).floor() as i32).rem_euclid(4) {
        0 => "simplylight.gui.south",
        1 => "simplylight.gui.west",
        2 => "simplylight.gui.north",
        _ => "simplylight.gui.east",
    }
}

fn is_ours(host: &dyn ClientHost) -> bool {
    host.mod_screen() == Some(SCREEN)
}

/// Draws the panel, its four switches, the facing and the exit button.
pub fn render(host: &dyn ClientHost, event: &mut ScreenRenderPostEvent<'_>) {
    if !is_ours(host) {
        return;
    }
    let draw: &mut dyn DrawContext = &mut *event.draw;
    let at = corner(draw.size());
    let open = {
        let mut client = CLIENT.lock().unwrap_or_else(|e| e.into_inner());
        client.at = at;
        match client.open {
            Some(open) => open,
            None => return,
        }
    };
    let text = |key: &str| host.translate(key).unwrap_or_else(|| key.to_owned());
    draw.sprite(
        SPRITE_PANEL,
        [0.0, 0.0, PANEL[0], PANEL[1]],
        [at[0], at[1], PANEL[0], PANEL[1]],
        None,
    );
    for (row, (_, key, bit)) in SIDES.iter().enumerate() {
        let rect = switch_rect(at, row);
        let on = if open.bits & bit != 0 { 16.0 } else { 0.0 };
        draw.sprite(SPRITE_SWITCH, [0.0, on, SWITCH[0], SWITCH[1]], rect, None);
        draw.text(
            &text(key),
            [rect[0] + 34.0, rect[1] + 4.0],
            0xFF40_4040,
            false,
        );
    }
    let yaw = host
        .local_player()
        .and_then(|p| p.entity().rotation(host))
        .map_or(0.0, |r| r[0]);
    let facing = text("simplylight.gui.facing");
    let direction = text(facing_key(yaw));
    for (row, line) in [facing, direction].iter().enumerate() {
        let x = at[0] + PANEL[0] - 4.0 - draw.text_width(line);
        draw.text(
            line,
            [x, at[1] + 8.0 + 10.0 * row as f32],
            0xFF40_4040,
            false,
        );
    }
    let exit = [at[0] + EXIT[0], at[1] + EXIT[1], EXIT[2], EXIT[3]];
    let hover = inside(exit, event.screen.mouse);
    if let Some(label) = host.translate("simplylight.gui.exit") {
        let _ = draw.widget_button(exit, &mod_api::Text::literal(label), true, hover, None);
    }
}

/// A click on a switch flips it and queues the send; a click on exit queues the close.
pub fn click(host: &dyn ClientHost, event: &mut ScreenMouseButtonPressedPreEvent<'_>) -> Flow {
    if !is_ours(host) || event.button != 0 {
        return Flow::Continue;
    }
    let mut client = CLIENT.lock().unwrap_or_else(|e| e.into_inner());
    let at = client.at;
    let Some(open) = client.open.as_mut() else {
        return Flow::Continue;
    };
    let mouse = event.screen.mouse;
    if let Some(row) = (0..4).find(|&row| inside(switch_rect(at, row), mouse)) {
        open.bits ^= SIDES[row].2;
        client.actions.push(Action::Send);
    } else if inside([at[0] + EXIT[0], at[1] + EXIT[1], EXIT[2], EXIT[3]], mouse) {
        client.actions.push(Action::Close);
    }
    Flow::Continue
}

/// Sends what the screen hooks queued.
pub fn tick(host: &mut dyn ClientHost) {
    let (open, actions) = {
        let mut client = CLIENT.lock().unwrap_or_else(|e| e.into_inner());
        (client.open, std::mem::take(&mut client.actions))
    };
    for action in actions {
        match action {
            Action::Send => {
                if let Some(open) = open {
                    let _ = host.send_payload_to_server(
                        &PayloadId(UPDATE.into()),
                        encode(open.pos, open.bits),
                    );
                }
            }
            Action::Close => host.mod_screen_close(SCREEN),
        }
    }
    if host.mod_screen() != Some(SCREEN) {
        CLIENT.lock().unwrap_or_else(|e| e.into_inner()).open = None;
    }
}

pub fn handlers() -> (Arc<dyn IPayloadHandler>, Arc<dyn IPayloadHandler>) {
    (Arc::new(OpenHandler), Arc::new(UpdateHandler))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_packet_round_trips() {
        let open = decode(&encode([-3, 64, 1_000_000], 0b1010).0).unwrap();
        assert_eq!((open.pos, open.bits), ([-3, 64, 1_000_000], 0b1010));
        assert!(decode(&[0; 12]).is_none());
    }

    #[test]
    fn sides_become_bits() {
        let state = "simplylight:edge_light[east=true,north=false,south=true,west=false]";
        assert_eq!(bits_of(state), 2 | 4);
    }

    #[test]
    fn facing_follows_the_yaw() {
        assert_eq!(facing_key(0.0), "simplylight.gui.south");
        assert_eq!(facing_key(90.0), "simplylight.gui.west");
        assert_eq!(facing_key(180.0), "simplylight.gui.north");
        assert_eq!(facing_key(-90.0), "simplylight.gui.east");
    }
}
