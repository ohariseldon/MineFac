//! Contas separadas: a da **companhia** (o jogador, no centro de controle) e a **pública** (a
//! prefeitura). As duas são moedas em memória, copiadas no NBT do bloco dono a cada ciclo.
//!
//! - **Companhia:** esmeraldas no cofre do centro de controle viram moedas. Paga o salário de quem
//!   trabalha nos postos do jogador e os impostos. Recebe nas lojas (esmeraldas no caixa).
//! - **Pública:** recebe os impostos (e aportes voluntários no cofre da prefeitura). Paga o salário
//!   do prefeito, a manutenção dos poços e a assistência às famílias mais pobres.
//!
//! Fluxos do turno (produção, vendas, salários, impostos...) são somados aqui e viram médias
//! diárias no turno do dono. Nada é guardado por evento.
use super::*;

/// Uma conta: dono, saldo e o que entrou/saiu neste turno.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Account {
    pub dimension: String,
    pub pos: [i32; 3],
    pub coins: i32,
    pub seen: u64,
    /// Fluxos do turno: [produção, vendas, salários, impostos, (livre), assistência].
    pub flow: [i32; 6],
}

pub(crate) const MADE: usize = 0;
pub(crate) const SOLD: usize = 1;
pub(crate) const WAGES: usize = 2;
pub(crate) const TAXES: usize = 3;
pub(crate) const ASSIST: usize = 5;

static COMPANY: Mutex<Vec<Account>> = Mutex::new(Vec::new());
static PUBLIC: Mutex<Vec<Account>> = Mutex::new(Vec::new());

fn book(public: bool) -> &'static Mutex<Vec<Account>> {
    if public { &PUBLIC } else { &COMPANY }
}

/// Saldo conhecido em memória, ou `None` (mundo recém-aberto: o dono carrega do NBT).
pub(crate) fn balance(public: bool, dimension: &str, pos: [i32; 3]) -> Option<i32> {
    book(public).lock().ok()?.iter().find(|account| account.dimension == dimension && account.pos == pos)
        .map(|account| account.coins)
}

/// Abre ou renova a conta do bloco dono, com o saldo `coins`. Devolve os fluxos do turno e os zera
/// quando `close_turn`.
pub(crate) fn touch(public: bool, dimension: &str, pos: [i32; 3], coins: i32, now: u64, close_turn: bool) -> [i32; 6] {
    let Ok(mut accounts) = book(public).lock() else { return [0; 6] };
    if let Some(account) = accounts.iter_mut().find(|account| account.dimension == dimension && account.pos == pos) {
        account.coins = coins;
        account.seen = now;
        let flow = account.flow;
        if close_turn {
            account.flow = [0; 6];
        }
        return flow;
    }
    accounts.retain(|account| now.saturating_sub(account.seen) <= STALE);
    accounts.push(Account { dimension: dimension.to_owned(), pos, coins, seen: now, flow: [0; 6] });
    [0; 6]
}

fn nearest<'a>(accounts: &'a mut [Account], dimension: &str, at: [i32; 3], now: u64, radius: i32) -> Option<&'a mut Account> {
    accounts.iter_mut()
        .filter(|account| account.dimension == dimension && now.saturating_sub(account.seen) <= STALE
            && within(account.pos, at, radius))
        .min_by_key(|account| (account.pos[0] - at[0]).abs() + (account.pos[2] - at[2]).abs())
}

/// Tira até `amount` moedas da conta mais próxima de `at` (a até `radius`), anotando o fluxo `kind`.
/// Devolve quanto saiu.
pub(crate) fn pay(public: bool, dimension: &str, at: [i32; 3], radius: i32, amount: i32, kind: usize, now: u64) -> i32 {
    if amount <= 0 {
        return 0;
    }
    let Ok(mut accounts) = book(public).lock() else { return 0 };
    let Some(account) = nearest(&mut accounts, dimension, at, now, radius) else { return 0 };
    let paid = amount.min(account.coins).max(0);
    account.coins -= paid;
    account.flow[kind] += paid;
    paid
}

/// Produção (tarefas) ou vendas em `at`: entram no PIB da prefeitura que governa o lugar e são a
/// base do imposto do turno.
pub(crate) fn record(dimension: &str, at: [i32; 3], coins: i32, commerce: bool, now: u64) {
    if coins <= 0 {
        return;
    }
    let Some(hall) = governing(dimension, at, now) else { return };
    if let Ok(mut accounts) = PUBLIC.lock()
        && let Some(account) = accounts.iter_mut().find(|account| account.dimension == dimension && account.pos == hall.pos)
    {
        account.flow[if commerce { SOLD } else { MADE }] += coins;
    }
}

/// Média móvel de um dia a partir do valor do turno.
pub(crate) fn daily(previous: i32, turn: i32) -> i32 {
    previous - previous / TURNS_PER_DAY as i32 + turn
}

/// Converte as esmeraldas de `slot` em moedas.
pub(crate) fn take_emeralds(entity: &mut ModBlockEntity, slot: usize) -> i32 {
    let Some(stack) = entity.items.get(slot).and_then(Option::as_ref)
        .filter(|stack| stack.item.as_ref() == WAGE && stack.extra.is_none()) else { return 0 };
    let coins = stack.count as i32 * COINS_PER_EMERALD;
    entity.items[slot] = None;
    coins
}

// ---------- Companhia (centro de controle) ----------

/// Espaço do cofre da companhia no centro de controle.
pub(crate) const COMPANY_VAULT: usize = 4;

/// Ciclo do cofre da companhia: depósitos, cópia no NBT e médias diárias por turno.
pub(crate) fn company_cycle(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let now = defense::clock();
    let dimension = host.dimension();
    let mut coins = balance(false, dimension, pos).unwrap_or_else(|| number(entity, "Company"));
    coins = coins.saturating_add(take_emeralds(entity, COMPANY_VAULT));
    let close = turn_edge(pos, time);
    let flow = touch(false, dimension, pos, coins, now, close);
    if close {
        set_number(entity, "Payroll", daily(number(entity, "Payroll"), flow[WAGES]));
        set_number(entity, "Taxes", daily(number(entity, "Taxes"), flow[TAXES]));
    }
    set_number(entity, "Company", coins);
}

/// O turno da sociedade começa neste ciclo do posto (ciclos de `EVERY` ticks)?
pub(crate) fn turn_edge(pos: [i32; 3], time: i64) -> bool {
    (time + i64::from((pos[0] ^ pos[2]) & 63)).rem_euclid(TURN) < EVERY
}

/// Linhas do relatório do centro de controle sobre a companhia.
pub(crate) fn company_report(entity: &ModBlockEntity) -> Text {
    Text::tr("veio.company.report", &[&number(entity, "Company"), &number(entity, "Payroll"), &number(entity, "Taxes")])
}
