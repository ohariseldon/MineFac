"""Simulador do equilíbrio econômico do Veio (sociedade + comércio exterior).

Reproduz, dia a dia, as fórmulas de `native/api/src/workforce/society/` com os mesmos números
(salário, custo de vida, atacado, imposto, saturação, frete, contratos, importação, teto do preço
local) para verificar o balanceamento em cenários típicos, sem abrir o jogo.

    python tools/sim_economy.py            # tabela dos cenários
    python tools/sim_economy.py --check    # falha (código 1) se um invariante de equilíbrio quebrar

Modelo (um dia = 20 turnos): a companhia produz bens por dia; vende às lojas no atacado quando o
atacado rende mais que a exportação, senão exporta; famílias recebem salário e compram comida pelo
preço local; a loja ajusta o preço pela cobertura do estoque (com teto na paridade de importação se
houver conexão) e importa quando falta comida. Simplificações: um produto de comida (pão) e um
industrial por cenário; produtividade dada em unidades por dia.
"""
import math
import sys

# ----- Parâmetros (iguais aos do código Rust) -----
COINS_PER_EMERALD = 60
TURNS = 20
LIVING_COST = 40                    # society::LIVING_COST
WAGE_DAY = 2 * LIVING_COST          # society::WAGE_PER_TURN x 20
FOOD_POINTS_DAY = 20                # FOOD_PER_TURN x 20
WHOLESALE = 0.75                    # market::WHOLESALE
IMPORT_FREIGHT, IMPORT_TARIFF, IMPORT_MARGIN = 0.20, 0.10, 0.10
PREMIUM = 0.50
SCRAP = 0.25
TAX = {1: 0.04, 2: 0.08, 3: 0.12, 4: 0.16}
TIER_FACTOR = {0: 1.0, 1: 1.5, 2: 2.0, 3: 3.0, 4: 4.0}
ABSORB = {0: 256, 1: 512, 2: 512, 3: 256, 4: 128}
SHOP_RESERVE = 600                  # society::SHOP_RESERVE
SAVINGS_DAYS = 2                    # society::SAVINGS_DAYS
FREIGHT = {"port": (4, 6, 9, 2), "station": (2, 12, 6, 1)}  # base, por pilha, pilhas, turnos
PRODUCTS = {  # item: (valor base, camada, pontos de comida)
    "raw_iron": (2, 0, 0), "wheat": (1, 0, 1), "bread": (6, 1, 6), "iron_plate": (4, 1, 0),
    "gear": (7, 2, 0), "circuit": (14, 3, 0), "speed_module": (40, 4, 0),
}


def reference(item):
    base, tier, _ = PRODUCTS[item]
    return base * TIER_FACTOR[tier]


def spot(item, pressure, index=1.0):
    base, tier, _ = PRODUCTS[item]
    full = reference(item) * index
    return max(full * ABSORB[tier] / (ABSORB[tier] + pressure), full * SCRAP)


def freight_per_unit(connection):
    base, stack, stacks, _ = FREIGHT[connection]
    return (base + stack * stacks) / (64 * stacks)


def export_net(item, pressure, phase, connection="port"):
    """O que a companhia recebe por unidade exportada (depois de imposto e frete)."""
    return spot(item, pressure) * (1 - TAX[phase]) - freight_per_unit(connection)


def import_parity(item):
    return reference(item) * (1 + IMPORT_FREIGHT + IMPORT_TARIFF)


def ceiling(item, phase=1):
    """Teto do preço local: custo de importar + margem, já contando o imposto da venda."""
    return import_parity(item) * (1 + IMPORT_MARGIN) / (1 - TAX[phase])


def daily_export_revenue(item, per_day, days=10):
    """Receita bruta por dia em regime, exportando `per_day` unidades por dia à vista."""
    recent = 0.0
    revenue = 0.0
    for _ in range(days):
        revenue = 0.0
        for unit in range(per_day):
            revenue += spot(item, recent / 2 + unit)
        recent = recent * 0.6 + per_day * 0.4
    return revenue


def simulate(name, people, workers, bread_per_day, industrial, connected, phase, days=12):
    """Um cenário: devolve as médias dos últimos dias."""
    company, shop, public, wallets = 1200.0, 0.0, 0.0, people * 40.0
    payable = 0.0
    price_factor = 1.0
    demand_day = people * FOOD_POINTS_DAY / 6
    stock = 0.0
    fed_days = 0
    log = {}
    recent = {"bread": 0.0, industrial[0]: 0.0} if industrial else {"bread": 0.0}
    for day in range(days):
        company -= workers * WAGE_DAY
        wallets += workers * WAGE_DAY
        retail = reference("bread") * price_factor
        wholesale = retail * WHOLESALE
        # A companhia escolhe: atacado local ou exportação, o que render mais por unidade.
        exported = 0
        local_supply = bread_per_day
        if connected and export_net("bread", recent["bread"] / 2, phase) > wholesale * (1 - TAX[phase]):
            local_supply = min(bread_per_day, demand_day * 0.5)
            exported = bread_per_day - local_supply
        stock += local_supply
        payable += local_supply * wholesale
        # Famílias: compram o que precisam se tiverem dinheiro e estoque.
        want = demand_day
        affordable = wallets / max(retail, 0.01)
        bought = min(want, stock, affordable)
        short = want - bought
        imported = 0.0
        if short > 0 and connected:
            landed = import_parity("bread")
            imported = min(short, max(shop, 0) / landed)
            shop -= imported * landed
            public += imported * reference("bread") * IMPORT_TARIFF
            stock += imported
            bought = min(want, stock, affordable)
            short = want - bought
        stock -= bought
        spent = bought * retail
        wallets -= spent
        shop += spent * (1 - TAX[phase])
        public += spent * TAX[phase]
        # A loja paga o atacado (fiado) com o caixa; a companhia recebe menos o imposto na fonte.
        settled = min(payable, max(shop, 0))
        payable -= settled
        shop -= settled
        company += settled * (1 - TAX[phase])
        public += settled * TAX[phase]
        fed_days += bought / want
        # O comerciante leva o lucro acima da reserva de giro para a família (o dinheiro circula).
        profit = max(shop - SHOP_RESERVE, 0) * 0.88
        shop -= profit
        wallets += profit
        # Poupança acima de 2 dias de custo de vida vira compra de bens (que a companhia fornece).
        surplus = max(wallets - people * LIVING_COST * SAVINGS_DAYS, 0)
        goods = surplus * (1 - 0.75 ** TURNS)
        wallets -= goods
        shop += goods * (1 - TAX[phase]) - goods * WHOLESALE
        company += goods * WHOLESALE * (1 - TAX[phase])
        public += goods * TAX[phase] + goods * WHOLESALE * TAX[phase]
        # Exportação: pão excedente e o produto industrial.
        export_revenue = 0.0
        if connected:
            for item, amount in [("bread", int(exported))] + ([industrial] if industrial else []):
                for unit in range(int(amount)):
                    export_revenue += export_net(item, recent[item] / 2 + unit, phase)
                recent[item] = recent[item] * 0.6 + amount * 0.4
        company += export_revenue
        # Preço local: cobertura do estoque frente à procura, um quarto do caminho por dia.
        cover = max(stock, 0.25) / demand_day
        target = min(max(1 / math.sqrt(max(cover, 0.25)) + (0.25 if short > 0 else 0), 0.5), 2.0)
        price_factor += (target - price_factor) / 4
        if connected:
            price_factor = min(price_factor, ceiling("bread", phase) / reference("bread"))
        if day == days - 4:
            company_start = company
        if day >= days - 3:
            for key, value in [("bread_price", retail), ("fed", bought / want), ("company_day", 0.0),
                               ("export_day", export_revenue), ("import_units", imported)]:
                log[key] = log.get(key, 0) + value / 3
    log["company_end"] = company
    log["company_day"] = (company - company_start) / 3
    log["money"] = company + shop + public + wallets
    return name, log


SCENARIOS = [
    # nome, pessoas, trabalhadores, pão/dia, (produto industrial, unidades/dia), conexão, fase
    ("Povoado sem conexão", 12, 6, 50, None, False, 1),
    ("Vila com porto", 24, 12, 100, ("iron_plate", 400), True, 2),
    ("Cidade industrial", 48, 24, 160, ("gear", 600), True, 3),
    ("Vila sem comida (importa)", 30, 15, 30, ("iron_plate", 200), True, 2),
]


def invariants():
    """Regras que o balanceamento precisa respeitar. Devolve a lista de falhas."""
    failures = []
    def check(ok, text):
        if not ok:
            failures.append(text)
    # 1. Um salário sustenta duas pessoas a preços de referência.
    bread_cost_day = FOOD_POINTS_DAY / PRODUCTS["bread"][2] * reference("bread")
    check(WAGE_DAY >= 2 * (bread_cost_day + 10), "salário não cobre duas pessoas")
    # 2. Importar nunca dá prejuízo ao comerciante: o teto, menos o imposto, cobre o custo.
    for phase in TAX:
        check(ceiling("bread", phase) * (1 - TAX[phase]) > import_parity("bread"),
              f"importar dá prejuízo na fase {phase}")
    # 3. No teto, vender à vila compete com exportar (o mercado local não fica sem comida).
    check(ceiling("bread", 2) * WHOLESALE >= export_net("bread", 0, 2) * 0.95,
          "no teto, o atacado local perde para a exportação: a vila passaria fome")
    # 4. Frete pesa mais em matéria-prima que em componentes.
    share = lambda item: freight_per_unit("station") / reference(item)
    check(share("raw_iron") > 5 * share("gear"), "frete não diferencia valor agregado")
    # 5. Valor agregado: bread (2 trigos) vale muito mais que os 2 trigos.
    check(reference("bread") >= 4 * 2 * reference("wheat"), "processar comida não compensa")
    # 6. Saturação: inundar dá retorno decrescente, mas nunca abaixo da sucata.
    r1, r2 = daily_export_revenue("iron_plate", 256), daily_export_revenue("iron_plate", 1024)
    check(r2 > r1 and r2 < 4 * r1, "saturação sem retorno decrescente")
    # 6b. Exportar o volume de absorção rende pelo menos metade do valor cheio (não é só sucata).
    for item in PRODUCTS:
        a = ABSORB[PRODUCTS[item][1]]
        check(daily_export_revenue(item, a) >= 0.5 * a * reference(item), f"saturação dura demais em {item}")
    check(spot("iron_plate", 10**6) >= reference("iron_plate") * SCRAP - 1e-9, "sucata abaixo do piso")
    # 7. Contrato paga mais que o mercado à vista.
    check(reference("gear") * (1 + PREMIUM) > spot("gear", 0), "contrato não compensa")
    return failures


def main():
    print(f"Salário {WAGE_DAY}/dia · custo de vida {LIVING_COST}/pessoa · pão {reference('bread'):.1f}"
          f" (importação {import_parity('bread'):.1f}, teto fase 1 {ceiling('bread', 1):.1f} a fase 4 {ceiling('bread', 4):.1f})")
    print(f"Frete por unidade: porto {freight_per_unit('port'):.3f}, estação {freight_per_unit('station'):.3f}")
    for item in PRODUCTS:
        print(f"  {item:13} ref {reference(item):6.1f}  256/dia → {daily_export_revenue(item, 256):8.0f}"
              f"  1024/dia → {daily_export_revenue(item, 1024):8.0f}")
    print()
    print(f"{'cenário':28} {'pão':>6} {'comida':>7} {'export/dia':>11} {'importa':>8} {'lucro/dia':>10}")
    for scenario in SCENARIOS:
        name, log = simulate(*scenario)
        print(f"{name:28} {log['bread_price']:6.1f} {log['fed']*100:6.0f}% {log['export_day']:11.0f}"
              f" {log['import_units']:8.1f} {log['company_day']:10.0f}")
    failures = invariants()
    for failure in failures:
        print("FALHA:", failure)
    if "--check" in sys.argv and failures:
        sys.exit(1)
    if not failures:
        print("Invariantes de equilíbrio: ok")


if __name__ == "__main__":
    main()
