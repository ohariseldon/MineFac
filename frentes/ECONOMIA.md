# Equilíbrio da economia (0.25)

Este documento explica **como a economia do Veio funciona em números** e por que os parâmetros são
os que são. O desenho dos sistemas está em [SOCIEDADE.md](SOCIEDADE.md); aqui fica o balanceamento.

Tudo é verificável sem abrir o jogo:

```bash
python tools/sim_economy.py
```

```bash
python tools/sim_economy.py --check
```

O simulador reproduz as fórmulas do código (`native/api/src/workforce/society/`) dia a dia. As mesmas
regras de equilíbrio são testes do jogo (`balance_*` em `workforce/tests.rs`) e fazem parte de
`tools/check_project.py`: quem mudar um número e quebrar o equilíbrio fica sabendo na hora.

## Princípios

1. **Uma âncora de preço.** O valor de referência de um produto é `valor base x complexidade`
   (matéria-prima 1x, processado 1,5x, componente 2x, eletrônico 3x, módulo 4x). É o mesmo dentro e
   fora da vila: o pão vale 9 na loja e 9 (x índice do dia) na bolsa. Antes da 0.25, o pão valia 6 na
   loja e 9 lá fora, então exportar sempre ganhava do mercado local e a vila passava fome.
2. **Salário a partir do custo de vida.** Uma pessoa gasta 40 moedas por dia: 20 pontos de comida
   (3,3 pães x 9 = 30) e 20 de conforto (bens: ~10). Um salário sustenta duas pessoas: **80 por dia
   (4 por turno)**. Uma família de 4 com 2 trabalhadores fecha a conta; com 1, precisa de assistência.
3. **O preço local vive entre dois limites.** Com uma conexão externa:
   - **teto = custo de importar + 10% de margem, já contando o imposto da fase**. Importar nunca dá
     prejuízo ao comerciante, e a inflação para no teto;
   - **piso, na prática = o que a exportação paga**: abaixo disso, a companhia prefere exportar, a loja
     fica sem estoque, a cobertura cai e o preço sobe.
   No teto, o atacado local (75%) rende tanto quanto exportar: a vila não fica sem comida porque a
   indústria prefere o mundo.
4. **O dinheiro circula.** Margem do comércio e salários acima do necessário não podem parar em
   caixas e carteiras:
   - o comerciante leva para a família 10% por turno do caixa acima da **reserva de giro (600)**;
   - famílias com mais de **2 dias de custo de vida** guardados gastam um quarto do excedente por
     turno em bens de conforto, que voltam pelas lojas para a indústria.
   Sem isso, uma vila sem conexão sangrava ~25% da folha por dia (simulador: −539); com isso, perde só
   o imposto (−83/dia).
5. **O mundo de fora tem limites.** Saturação por produto com volume de absorção por camada e piso de
   sucata; frete por pilha (pesa em matéria-prima); contratos como comprador à parte.

## Parâmetros

| O quê | Valor | Onde | Por quê |
|---|---|---|---|
| Moeda | 1 esmeralda = 60 moedas | `COINS_PER_EMERALD` | o mesmo da despensa |
| Custo de vida | 40 / pessoa / dia | `LIVING_COST` | 30 de comida + 10 de bens a preços de referência |
| Salário | 4 / turno (80 / dia) | `WAGE_PER_TURN` | sustenta 2 pessoas |
| Atacado | 75% do preço local | `market::WHOLESALE` | margem do comerciante paga salário, imposto e giro |
| Reserva de giro da loja | 600 | `SHOP_RESERVE` | caixa para pagar atacado e importar |
| Poupança antes de consumir | 2 dias | `SAVINGS_DAYS` | o resto vira compra de bens |
| Imposto na fonte | 4 / 8 / 12 / 16% | `mayor::tax_rate` | cresce com a vila |
| Importação | mundial + 20% frete + 10% tarifa | `IMPORT_FREIGHT`, `IMPORT_TARIFF` | tarifa fica na prefeitura |
| Margem sobre importado | 10% | `market::IMPORT_MARGIN` | importar não dá prejuízo |
| Absorção por dia (preço cai à metade) | 256 / 512 / 512 / 256 / 128 por camada | `trade::absorb` | grandes volumes de base, nichos de alta tecnologia |
| Pressão no mercado | metade da média recente + vendido hoje | `trade::pressure` | o mercado se recupera em alguns dias |
| Piso de sucata | 25% do preço cheio | `trade::SCRAP` | inundar nunca zera a receita |
| Frete do porto | 4 + 6 por pilha; 9 pilhas a cada 2 min | `trade::schedule` | barato e lento |
| Frete da estação | 2 + 12 por pilha; 6 pilhas por minuto | `trade::schedule` | caro e frequente |
| Prêmio de contrato | 50% sobre o preço cheio do dia | `trade::PREMIUM` | sem saturação: comprador à parte |
| Reputação por camada | 1, 3, 6 | `trade::TIER_AT` | camada aberta nunca fecha |

## Resultados do simulador

```
Salário 80/dia · custo de vida 40/pessoa · pão 9.0 (importação 11.7, teto fase 1 13.4 a fase 4 15.3)
Frete por unidade: porto 0.101, estação 0.193
  produto       ref    256/dia → receita/dia   1024/dia → receita/dia
  raw_iron      2.0      263                     532
  wheat         1.0      131                     266
  bread         9.0     1554                    3207
  iron_plate    6.0     1036                    2138
  gear         14.0     2418                    4988
  circuit      42.0     5513                   11177
  speed_module 160.0   14268                   40960

cenário                         pão  comida  export/dia  importa  lucro/dia
Povoado sem conexão             7.1    100%           0      0.0        -83
Vila com porto                 12.4    100%        1222      0.0       1120
Cidade industrial              14.6    100%        3438      0.0       3062
Vila sem comida (importa)      14.0     86%         780     55.6        -15
```

Leitura:

- **Valor agregado manda.** 256 engrenagens por dia rendem 2.418; 256 minérios, 263. Quadruplicar o
  volume de um item só (1.024/dia) não quadruplica a receita (retorno decrescente), mas nunca zera.
- **Frete:** na estação, ~10% do valor do minério de ferro e ~1% da engrenagem. O frete é por pilha,
  então pesa onde o valor é baixo.
- **Sem conexão**, a companhia empata com a vila, menos o imposto: a vila consome, não paga uma
  indústria grande. **O comércio exterior é o que sustenta uma indústria maior.**
- **Com porto**, o pão local sobe até perto do teto (12,4 a 14,6): a indústria exporta o excedente e
  a vila continua 100% alimentada.
- **Sem comida local**, as lojas importam (55 pães por dia no cenário) e a vila chega a 86%
  alimentada; o lucro da companhia cai porque a importação leva moeda embora, que é o efeito
  desejado: produzir localmente sai mais barato.

## Regras de equilíbrio (testes)

1. Um salário paga comida e conforto de duas pessoas a preços de referência.
2. Em todas as fases e dias, o teto do preço cobre o custo de importar mais o imposto.
3. No teto, o atacado local rende tanto quanto exportar sem saturação.
4. O frete pesa pelo menos 5 vezes mais (em proporção) no minério que na engrenagem.
5. Processar comida compensa: o pão vale pelo menos 4 vezes os 2 trigos que usa.
6. Exportar o volume de absorção rende pelo menos metade do valor cheio; mais volume rende mais, com
   retorno decrescente; nunca abaixo da sucata.
7. Contrato paga mais que o mercado à vista e não derruba a cotação à vista.
8. Mercado aberto não fecha quando a reputação cai.

## Onde ajustar

- **A vila passa fome com porto:** subir `WHOLESALE` ou `IMPORT_MARGIN` (o atacado no teto precisa
  vencer a exportação).
- **Exportar rende demais/de menos:** `absorb` (volume) e `tier_factor` (valor agregado). Mudar a
  camada de um produto em `PRODUCTS` muda o preço dele dentro e fora da vila.
- **Família pobre demais:** `LIVING_COST` ajusta o salário junto (o salário é derivado dele).
- **Dinheiro parado:** `SHOP_RESERVE` e `SAVINGS_DAYS` controlam quanto fica guardado antes de
  circular.
