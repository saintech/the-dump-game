use crate::cfg;
use crate::cmtp::{PlayerAction, PlayerState};
use crate::engine::game;
use tcod::input;

pub fn update(world: &mut game::World) {
    use input::KeyCode::*;
    world.player.action = match world.player.state {
        PlayerState::InDialog => match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, input::Event::Key(key))) => match (key.code, key.printable) {
                (Escape, _) => PlayerAction::Cancel,
                (Text, printable) => printable_to_action(printable),
                _ => PlayerAction::None,
            },
            _ => PlayerAction::None,
        },

        PlayerState::MakingTurn | PlayerState::TargetingTile(_) => {
            match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
                Some((_, input::Event::Key(key))) => match (key.code, key.printable) {
                    (Escape, _) => PlayerAction::Cancel,
                    (Up, _) | (Number8, _) => PlayerAction::GoToUp,
                    (Down, _) | (NumPad2, _) => PlayerAction::GoToDown,
                    (Left, _) | (NumPad4, _) => PlayerAction::GoToLeft,
                    (Right, _) | (NumPad6, _) => PlayerAction::GoToRight,
                    (Home, _) | (NumPad7, _) => PlayerAction::GoToUpLeft,
                    (PageUp, _) | (NumPad9, _) => PlayerAction::GoToUpRight,
                    (End, _) | (NumPad1, _) => PlayerAction::GoToDownLeft,
                    (PageDown, _) | (NumPad3, _) => PlayerAction::GoToDownRight,
                    (NumPad5, _) => PlayerAction::SkipTurn,
                    (Enter, _) => PlayerAction::InteractWithMap,
                    (F1, _) => PlayerAction::OpenHelp,
                    (Text, 'i') => PlayerAction::OpenInventory,
                    (Text, 'c') => PlayerAction::OpenCharInfo,
                    (Text, 'd') => PlayerAction::DropItem,
                    _ => PlayerAction::None,
                },
                Some((_, input::Event::Mouse(m))) => match (
                    m.lbutton_pressed,
                    m.rbutton_pressed,
                    to_map_coordinates(m.cx, m.cy),
                ) {
                    (false, true, ..) => PlayerAction::Cancel,
                    (false, false, (x, y)) => PlayerAction::LookAt(x as i32, y as i32),
                    (true, _, (x, y)) => PlayerAction::ClickAt(x as i32, y as i32),
                },
                _ => PlayerAction::None,
            }
        }
    }
}

fn printable_to_action(key: char) -> PlayerAction {
    b"123456789abcdefghijklmnopqrstuvwxyz"
        .iter()
        .position(|&val| val as char == key)
        .map_or(PlayerAction::None, |v| PlayerAction::SelectMenuItem(v))
}

fn to_map_coordinates(x: isize, y: isize) -> (isize, isize) {
    const MAP_VIEW_WIDTH: i32 = cfg::SCREEN_WIDTH - cfg::PANEL_WIDTH;
    const MAP_OFFSET_X: i32 = (MAP_VIEW_WIDTH - cfg::MAP_WIDTH) / 2;
    const MAP_OFFSET_Y: i32 = (cfg::SCREEN_HEIGHT - cfg::MAP_HEIGHT) / 2;
    (x - MAP_OFFSET_X as isize, y - MAP_OFFSET_Y as isize)
}
