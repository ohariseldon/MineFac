//! Contas da economia fechada. A esmeralda é a moeda: 1 esmeralda = `COINS_PER_EMERALD` moedas.
//!
//! Livros (saldos em memória, copiados no NBT do bloco dono a cada ciclo):
//! - **Companhia** (cofre do centro de controle): recebe do atacado (as lojas pagam pelo que a
//!   indústria entrega) e paga os salários dos trabalhadores da companhia.
//! - **Pública** (prefeitura): recebe os impostos, retidos na fonte em cada venda; paga o salário do
//!   prefeito e a assistência às famílias pobres.
//! - **Lojas** (cada comerciante): recebe das famílias e do jogador; paga o atacado à companhia e o
//!   próprio comerciante.
//! - **Famílias** (carteira no NBT de cada casa): recebe salários e assistência; gasta nas lojas.
//!
//! **Identidade contábil:** todo pagamento entre livros tira de um e põe no outro, então a soma dos
//! saldos só muda pelas bordas: esmeraldas de verdade que entram (depósito no cofre da companhia,
//! aporte na prefeitura, compras do jogador com esmeraldas) ou saem (saque da companhia). Essas
//! bordas são contadas em `BORDERS`.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Book { Company, Public, Shop }

/// Uma conta: dono, saldo e os fluxos deste turno.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Account {
    pub dimension: String,
    pub pos: [i32; 3],
    pub coins: i32,
    pub seen: u64,
    /// Fluxos do turno: [atacado, varejo, salários, impostos, recebido, assistência, saque, exportação].
    pub flow: [i32; 8],
}

/// Valor entregue pela indústria às lojas (produção a preço de atacado).
pub(crate) const MADE: usize = 0;
/// Vendas no varejo (consumo final: PIB).
pub(crate) const SOLD: usize = 1;
pub(crate) const WAGES: usize = 2;
pub(crate) const TAXES: usize = 3;
pub(crate) const RECEIVED: usize = 4;
pub(crate) const ASSIST: usize = 5;
pub(crate) const REDEEM: usize = 6;
/// Vendas ao mercado externo (entram no PIB).
pub(crate) const EXPORTS: usize = 7;

static COMPANY: Mutex<Vec<Account>> = Mutex::new(Vec::new());
static PUBLIC: Mutex<Vec<Account>> = Mutex::new(Vec::new());
static SHOP_BOOK: Mutex<Vec<Account>> = Mutex::new(Vec::new());
/// Bordas por dimensão: (esmeraldas que viraram moeda, moedas que viraram esmeralda), em moedas.
static BORDERS: Mutex<Vec<(String, i64, i64, i64)>> = Mutex::new(Vec::new());

fn book(which: Book) -> &'static Mutex<Vec<Account>> {
    match which { Book::Company => &COMPANY, Book::Public => &PUBLIC, Book::Shop => &SHOP_BOOK }
}

/// Saldo conhecido em memória, ou `None` (mundo recém-aberto: o dono carrega do NBT).
pub(crate) fn balance(which: Book, dimension: &str, pos: [i32; 3]) -> Option<i32> {
    book(which).lock().ok()?.iter().find(|account| account.dimension == dimension && account.pos == pos)
        .map(|account| account.coins)
}

/// Abre ou renova a conta do bloco dono. Na primeira vez, o saldo vem do NBT (`stored`); depois, a
/// memória manda. Devolve (saldo, fluxos do turno) e zera os fluxos quando `close_turn`.
pub(crate) fn open(which: Book, dimension: &str, pos: [i32; 3], stored: i32, now: u64, close_turn: bool) -> (i32, [i32; 8]) {
    let Ok(mut accounts) = book(which).lock() else { return (stored, [0; 8]) };
    if let Some(account) = accounts.iter_mut().find(|account| account.dimension == dimension && account.pos == pos) {
        account.seen = now;
        let flow = account.flow;
        if close_turn {
            account.flow = [0; 8];
        }
        return (account.coins, flow);
    }
    accounts.retain(|account| now.saturating_sub(account.seen) <= STALE);
    accounts.push(Account { dimension: dimension.to_owned(), pos, coins: stored, seen: now, flow: [0; 8] });
    (stored, [0; 8])
}

fn nearest<'a>(accounts: &'a mut [Account], dimension: &str, at: [i32; 3], now: u64, radius: i32) -> Option<&'a mut Account> {
    accounts.iter_mut()
        .filter(|account| account.dimension == dimension && now.saturating_sub(account.seen) <= STALE
            && within(account.pos, at, radius))
        .min_by_key(|account| (account.pos[0] - at[0]).abs() + (account.pos[2] - at[2]).abs())
}

/// Tira até `amount` moedas da conta mais próxima de `at` (a até `radius`; 0 = a conta exata).
/// Devolve quanto saiu. Quem chama põe esse valor em outro livro.
pub(crate) fn take(which: Book, dimension: &str, at: [i32; 3], radius: i32, amount: i32, kind: usize, now: u64) -> i32 {
    if amount <= 0 {
        return 0;
    }
    let Ok(mut accounts) = book(which).lock() else { return 0 };
    let Some(account) = nearest(&mut accounts, dimension, at, now, radius) else { return 0 };
    let paid = amount.min(account.coins).max(0);
    account.coins -= paid;
    account.flow[kind] += paid;
    paid
}

/// Põe `amount` moedas na conta mais próxima de `at` (a até `radius`). Devolve `false` se não há
/// conta (quem chama não tira de lugar nenhum).
pub(crate) fn put(which: Book, dimension: &str, at: [i32; 3], radius: i32, amount: i32, kind: usize, now: u64) -> bool {
    if amount <= 0 {
        return true;
    }
    let Ok(mut accounts) = book(which).lock() else { return false };
    let Some(account) = nearest(&mut accounts, dimension, at, now, radius) else { return false };
    account.coins = account.coins.saturating_add(amount);
    account.flow[kind] += amount;
    true
}

/// A companhia é uma só: o cofre mais próximo, de qualquer distância.
pub(crate) const ANYWHERE: i32 = i32::MAX / 4;

/// Há um cofre da companhia nesta dimensão?
pub(crate) fn company_open(dimension: &str, now: u64) -> bool {
    COMPANY.lock().is_ok_and(|accounts| accounts.iter().any(|account| account.dimension == dimension
        && now.saturating_sub(account.seen) <= STALE))
}

/// Imposto da fase sobre uma venda de `amount` em `at`, retido na fonte: vai direto para a
/// prefeitura que governa o lugar. Devolve o imposto (0 sem prefeitura).
pub(crate) fn withhold(dimension: &str, at: [i32; 3], amount: i32, now: u64) -> i32 {
    let Some(hall) = governing(dimension, at, now) else { return 0 };
    let tax = amount * mayor::tax_rate(hall.phase) / 100;
    if tax > 0 && put(Book::Public, dimension, hall.pos, 0, tax, TAXES, now) { tax } else { 0 }
}

/// Atacado entregue ou varejo vendido em `at`: entra nas contas da prefeitura que governa o lugar
/// (produção a preço de mercado e PIB).
pub(crate) fn record(dimension: &str, at: [i32; 3], coins: i32, kind: usize, now: u64) {
    if coins <= 0 {
        return;
    }
    let Some(hall) = governing(dimension, at, now) else { return };
    if let Ok(mut accounts) = PUBLIC.lock()
        && let Some(account) = accounts.iter_mut().find(|account| account.dimension == dimension && account.pos == hall.pos)
    {
        account.flow[kind] += coins;
    }
}

fn with_border(dimension: &str, change: impl FnOnce(&mut (String, i64, i64, i64))) {
    if let Ok(mut borders) = BORDERS.lock() {
        let index = match borders.iter().position(|(name, _, _, _)| name == dimension) {
            Some(index) => index,
            None => { borders.push((dimension.to_owned(), 0, 0, 0)); borders.len() - 1 }
        };
        change(&mut borders[index]);
    }
}

/// Moeda entrou (`coins > 0`: esmeraldas depositadas, compras do jogador, exportação) ou saiu
/// (`coins < 0`: saque) da economia.
pub(crate) fn border(dimension: &str, coins: i32) {
    with_border(dimension, |entry| if coins > 0 { entry.1 += i64::from(coins) } else { entry.2 -= i64::from(coins) });
}

/// Parte da entrada que veio do mercado externo.
pub(crate) fn exported(dimension: &str, coins: i32) {
    with_border(dimension, |entry| entry.3 += i64::from(coins.max(0)));
}

/// (entrou, saiu) nesta dimensão desde que o mundo abriu, em moedas.
pub(crate) fn borders(dimension: &str) -> (i64, i64) {
    BORDERS.lock().ok().and_then(|borders| borders.iter().find(|(name, _, _, _)| name == dimension)
        .map(|(_, emitted, redeemed, _)| (*emitted, *redeemed))).unwrap_or((0, 0))
}

/// Quanto da entrada veio de exportações.
pub(crate) fn exports(dimension: &str) -> i64 {
    BORDERS.lock().ok().and_then(|borders| borders.iter().find(|(name, _, _, _)| name == dimension)
        .map(|(_, _, _, exported)| *exported)).unwrap_or(0)
}

/// Soma dos saldos de um livro nesta dimensão.
pub(crate) fn total(which: Book, dimension: &str) -> i64 {
    book(which).lock().map(|accounts| accounts.iter().filter(|account| account.dimension == dimension)
        .map(|account| i64::from(account.coins)).sum()).unwrap_or(0)
}

/// Média móvel de um dia a partir do valor do turno.
pub(crate) fn daily(previous: i32, turn: i32) -> i32 {
    previous - previous / TURNS_PER_DAY as i32 + turn
}

/// Converte as esmeraldas de `slot` em moedas (borda de entrada).
pub(crate) fn take_emeralds(dimension: &str, entity: &mut ModBlockEntity, slot: usize) -> i32 {
    let Some(stack) = entity.items.get(slot).and_then(Option::as_ref)
        .filter(|stack| stack.item.as_ref() == WAGE && stack.extra.is_none()) else { return 0 };
    let coins = stack.count as i32 * COINS_PER_EMERALD;
    entity.items[slot] = None;
    border(dimension, coins);
    coins
}

/// O turno da sociedade começa neste ciclo do posto (ciclos de `EVERY` ticks)?
pub(crate) fn turn_edge(pos: [i32; 3], time: i64) -> bool {
    (time + i64::from((pos[0] ^ pos[2]) & 63)).rem_euclid(TURN) < EVERY
}

// ---------- Companhia (centro de controle) ----------

/// Espaços do centro de controle: cofre (depósito) e saque.
pub(crate) const COMPANY_VAULT: usize = 4;
pub(crate) const COMPANY_CASHOUT: usize = 5;
/// Botão do painel: sacar uma esmeralda da conta da companhia.
pub(crate) const CASH_OUT: i32 = 102;

/// Ciclo do cofre da companhia: depósitos, cópia no NBT e médias diárias por turno.
pub(crate) fn company_cycle(host: &mut dyn Host, pos: [i32; 3], entity: &mut ModBlockEntity, time: i64) {
    let now = defense::clock();
    let dimension = host.dimension();
    let close = turn_edge(pos, time);
    let (_, flow) = open(Book::Company, dimension, pos, number(entity, "Company"), now, close);
    let deposit = take_emeralds(dimension, entity, COMPANY_VAULT);
    put(Book::Company, dimension, pos, 0, deposit, REDEEM, now);
    if close {
        set_number(entity, "Payroll", daily(number(entity, "Payroll"), flow[WAGES]));
        set_number(entity, "Revenue", daily(number(entity, "Revenue"), flow[RECEIVED]));
    }
    set_number(entity, "Company", balance(Book::Company, dimension, pos).unwrap_or(0));
}

/// Saque: uma esmeralda da conta da companhia para o espaço de saque (borda de saída).
pub(crate) fn cash_out(dimension: &str, pos: [i32; 3], entity: &mut ModBlockEntity, now: u64) -> bool {
    if entity.items.len() <= COMPANY_CASHOUT {
        entity.items.resize(COMPANY_CASHOUT + 1, None);
    }
    if !can_add(&entity.items[COMPANY_CASHOUT], WAGE, 1)
        || balance(Book::Company, dimension, pos).unwrap_or(0) < COINS_PER_EMERALD
    {
        return false;
    }
    take(Book::Company, dimension, pos, 0, COINS_PER_EMERALD, REDEEM, now);
    add(&mut entity.items[COMPANY_CASHOUT], WAGE, 1);
    border(dimension, -COINS_PER_EMERALD);
    set_number(entity, "Company", balance(Book::Company, dimension, pos).unwrap_or(0));
    true
}

/// Linha do relatório do centro de controle sobre a companhia.
pub(crate) fn company_report(entity: &ModBlockEntity) -> Text {
    Text::tr("veio.company.report", &[&number(entity, "Company"), &number(entity, "Revenue"), &number(entity, "Payroll")])
}
