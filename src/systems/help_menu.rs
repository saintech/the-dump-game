use crate::cmtp::{DialogKind, PlayerAction, PlayerState};
use crate::engine::game;

pub fn update(world: &mut game::World) {
    let should_open_help = (world.player.state == PlayerState::MakingTurn)
        && (world.player.action == PlayerAction::OpenHelp);
    if !should_open_help {
        return;
    }
    let msg = "           Как играть\n\
               \n\
               \n\
               Сохранить и выйти...Esc\n\
               Обзор...............мышью\n\
               Взять, спуститься...Enter\n\
               Инвентарь...........I\n\
               Характеристики......C\n\
               Выбросить предмет...D\n\
               Передвижение........стрелки, Home,\n\
               \x20                   End, Page Up,\n\
               \x20                   Page Down,\n\
               \x20                   Numpad";
    world.add_dialog_box(DialogKind::MessageBox, String::from(msg), vec![], 36);
}
