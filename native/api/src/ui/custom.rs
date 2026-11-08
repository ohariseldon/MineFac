//! Telas com desenho próprio (centro de controle, prefeitura, lar e monitor) dentro da moldura
//! do kit: o item e o nome no alto, os slots com o fantasma do que vai neles e o inventário.
//! O conteúdo de cada uma fica no módulo dono dos dados.
use super::*;
use super::screens::{self, View};
use super::spec::Kind;
use horizon_client::mod_api::ScreenDrawEvent;

pub(crate) fn draw(host: &dyn Host, event: &mut ScreenDrawEvent<'_>) -> bool {
    let Some(view) = View::new(event.block, event.data) else { return false; };
    if view.spec.kind != Kind::Custom { return false; }
    let slots: Vec<[f32; 2]> = presentation::screen(view.block)
        .map(|screen| screen.slots.into_iter().flatten().collect())
        .unwrap_or_default();
    let mut pen = Pen::from_event(host, event);
    screens::frame(&mut pen);
    // O centro tem abas: cabeçalho de uma linha só para caberem as seis linhas.
    let center = view.block == workforce::CONTROL_CENTER;
    screens::header(&mut pen, &view, !center);
    for [x, y] in &slots {
        pen.slot(*x, *y);
    }
    match view.block {
        workforce::CONTROL_CENTER => workforce::center::draw(&mut pen, &view),
        workforce::TOWN_HALL => workforce::society::draw_hall(&mut pen, &view),
        workforce::HOME => workforce::home::draw(&mut pen, &view),
        industry::MONITOR => industry::draw_monitor(&mut pen, &view),
        _ => {}
    }
    screens::inventory(&mut pen);
    true
}
