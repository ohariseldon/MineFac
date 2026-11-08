use super::*;
use horizon_client::mod_api::{ScreenDrawEvent, ScreenTooltipEvent, Text, TooltipEvent};

pub(super) fn draw_labels(host: &dyn Host, event: &mut ScreenDrawEvent<'_>) {
    let [x, y] = event.origin;
    // Botão de ajuda no canto, em relevo como os botões vanilla.
    host.fill(event.batch, [x + 177.0, y + 4.0, 12.0, 12.0], 0xff000000);
    host.fill(event.batch, [x + 178.0, y + 5.0, 10.0, 10.0], 0xffa8a8a8);
    host.fill(event.batch, [x + 178.0, y + 5.0, 9.0, 1.0], 0xffffffff);
    host.fill(event.batch, [x + 178.0, y + 5.0, 1.0, 9.0], 0xffffffff);
    host.text(event.batch, "?", [x + 181.0, y + 6.0], presentation::INK, false);
    let Some(layout) = presentation::screen(event.block) else { return; };
    let roles: &[&str] = if let Some(roles) = production::roles(event.block).or_else(|| workforce::roles(event.block)).or_else(|| freight::roles(event.block)).or_else(|| milestones::owns(event.block).then(milestones::roles)) { roles } else { match event.block {
        DRILL | DRILL_MK2 => &["output"],
        GENERATOR => &["fuel"],
        BELT | FAST_BELT | SPLITTER | lift::LIFT => &[],
        MERGER => &["left", "right"],
        INTEGRATOR | ALLOY | ELECTRONICS => &["input1", "input2", "output"],
        HUB => &["input1", "input2", "output1", "output2"],
        HARVESTER => &["fertilizer", "output", "", ""],
        SOLAR | BATTERY | TESLA => &[],
        _ => &["input", "output"],
    } };
    for (index, (slot, role)) in layout.slots.iter().zip(roles).enumerate() {
        let Some([sx, sy]) = slot else { continue; };
        if role.is_empty() {
            continue;
        }
        let label = Text::tr(&format!("veio.guide.role.{role}"), &[]).plain();
        let width = host.text_width(&label);
        // Um rótulo seguido de papéis vazios cobre um grupo de três slots: centraliza no do meio.
        let grouped = roles.get(index + 1).is_some_and(|next| next.is_empty());
        let center = if grouped { sx + 26.0 } else { sx + 8.0 };
        host.text(event.batch, &label, [x + center - width / 2.0, y + sy + 20.0], presentation::SOFT_INK, false);
    }
}

pub(super) fn screen_tooltip(event: &mut ScreenTooltipEvent<'_>) {
    let Some(name) = event.block.strip_prefix("veio:") else { return; };
    let Some(layout) = presentation::screen(event.block) else { return; };
    let [x, y] = event.at;
    if (8.0..186.0).contains(&x) && (23.0..34.0).contains(&y) && event.block != HUB && !milestones::owns(event.block) {
        let status = event.data.first().copied().unwrap_or(0);
        event.lines.push(Text::tr(&operations::advice_key(status), &[]).plain());
        return;
    }
    if (176.0..190.0).contains(&x) && (3.0..17.0).contains(&y) {
        if let Some(profile) = operations::profile(event.block) {
            event.lines.push(Text::tr(profile.sector.translation_key(), &[]).plain());
        }
        for prefix in ["veio.help", "veio.guide.make"] {
            let text = Text::tr(&format!("{prefix}.{name}"), &[]).plain();
            event.lines.extend(text.lines().map(str::to_owned));
        }
        event.lines.push(Text::tr("veio.guide.tip", &[]).plain());
        return;
    }
    for (i, slot) in layout.slots.iter().enumerate() {
        if let Some([sx, sy]) = slot
            && (*sx..*sx + 16.0).contains(&x) && (*sy..*sy + 16.0).contains(&y)
        {
            event.lines.push(Text::tr(&format!("veio.slothelp.{name}.{i}"), &[]).plain());
            break;
        }
    }
}

pub(super) fn tooltip(event: &mut TooltipEvent<'_>) {
    if let Some(name) = event.stack.item.strip_prefix("veio:") {
        let key = format!("veio.help.{name}");
        let text = Text::tr(&key, &[]).plain();
        if text != key {
            for line in text.lines() {
                event.lines.push((Text::literal(line), 0xaaaaaa));
            }
        } else {
            let key = format!("veio.componenthelp.{name}");
            let text = Text::tr(&key, &[]).plain();
            if text != key {
                event.lines.push((Text::literal(text), 0xaaaaaa));
            }
        }
    } else if event.stack.extra.as_deref().and_then(|data| data.written.as_ref())
        .is_some_and(|book| book.author == "Veio")
    {
        event.lines.push((Text::tr("veio.guide.open", &[]), 0xffc857));
    }
}
