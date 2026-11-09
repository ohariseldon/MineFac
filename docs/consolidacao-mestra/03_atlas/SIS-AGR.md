# SIS-AGR: Agro e extrativismo

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. Linhas em `native/api/src/` no pr47.
- **Escopo:** máquinas agrícolas, rancho, silvicultura, moinho, cozinha e armazém do povoado. Os postos de lavrador e pastor estão em [SIS-TRB](SIS-TRB.md). A irrigação está em [SIS-AGU](SIS-AGU.md).

## A. Identidade

Produz comida para os aldeões e as famílias e matéria-prima para a indústria: biomassa, couro, lã, madeira. A agricultura automática absorve poluição. O dono pediu "agricultura real" com irrigação e necessidades (15:55, [DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039)).

- **Depende de:** [SIS-ENE](SIS-ENE.md), [SIS-AGU](SIS-AGU.md).
- **Alimenta:** [SIS-TRB](SIS-TRB.md) (comida por tarefa), [SIS-VIL](SIS-VIL.md) (despensa das casas), [SIS-ENE](SIS-ENE.md) (biomassa), [SIS-MER](SIS-MER.md) (comida e bens), [SIS-DEF](SIS-DEF.md) (absorção).

## B. Funcionamento

### B.1 Máquinas de campo

| Máquina | Área | Regra | Energia | Fonte |
|---|---|---|---|---|
| Colheitadeira | 9×9 em 3 camadas (raio 4) | Colhe e replanta; farinha de osso adianta 2 estágios; cada colheita absorve 3 de poluição | 120 FE por colheita; 40 por crescimento | `farm.rs:6-17` |
| Semeadora e plantas altas | — | Planta e colhe cana, cacto, bambu, abóbora, melancia. As edições saem em fila no NBT, no máximo uma vez por segundo, porque `edit_blocks` reenvia o chunk inteiro | 20 FE por plantio; 50 por crescimento; fila de 16 | `production/crops.rs:1-18` |
| Lâmpada de cultivo | — | 1 estágio a cada 4 ticks (README, E3) | 50 FE | `crops.rs` |
| Silvicultora | Grade de 8 pontos com espaço de 3 | Planta mudas e derruba árvores com tronco ≥ 4, de uma vez com as folhas; replanta a mesma espécie (6 espécies) | 200 FE por corte; 20 por plantio | `production/forestry.rs:1-21` |
| Rancho | Raio 6 | Reproduz com ração, tosquia, ordenha, recolhe ovos, penas e couro; mantém até 10 por espécie (8 espécies) e abate o excedente. Animais com nome nunca são abatidos (README) | — | `production/ranch.rs:1-23` |

O rancho age a cada 100 ticks e lê o NBT de até 24 animais por vez. O registro de animais e aldeões, por dimensão, é feito em `entity_tick_pre` e também serve ao pastor, ao spawn e à sociedade.

### B.2 Processamento de comida

| Máquina | Receitas | Fonte |
|---|---|---|
| Moinho | trigo → farinha; cana → 2 açúcares; 4 plantas → biomassa; tronco → 4 biomassas | `tools/production.json` |
| Cozinha | 2 farinhas → pão; biscoitos, bolo, torta de abóbora, cenoura e maçã douradas, sopa de beterraba, ensopado | idem |
| Fornalha elétrica | Carnes e peixes assados, batata assada | idem ([SIS-EXT](SIS-EXT.md)) |

São 28 alimentos com valor no pr47 (`food`). O pão vale 6 pontos e paga três tarefas de um trabalhador ([SIS-TRB](SIS-TRB.md) B.1).

### B.3 Armazém do povoado

- Compra comida da fábrica e **paga em esmeraldas**: 1 esmeralda por 60 pontos de comida (`PRICE`). Comida processada vale mais por ingrediente.
- Com uma vila de pelo menos 3 aldeões a até 48 blocos, paga 50% a mais (`production/pantry.rs:1-11`).
- Recolhe até 4 itens a cada 20 ticks.
- Essas esmeraldas são criadas fora dos livros ([SIS-ECO](SIS-ECO.md) B.5, [CNF-001](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

### B.4 Lavoura irrigada (pr47)

Com aspersor, cada cultura gasta água e adubo por estágio e ganha estágios extras ([SIS-AGU](SIS-AGU.md) B.2). Sem aspersor, a lavoura segue o vanilla.

## C. Experiência do jogador

1. No começo, a colheitadeira cuida de um 9×9 de trigo e o moinho faz farinha.
2. A cozinha faz pão para os trabalhadores.
3. O rancho dá couro e lã, que as famílias compram como conforto.
4. O excedente vira esmeralda no armazém do povoado.
5. No pr47, a irrigação tira a lavoura do acaso.

## D. Implementação

**Arquivos:** `farm.rs`; `production/{crops,forestry,ranch,pantry}.rs`; receitas em `tools/production.json`. Marcadores `field_marker` e `pen_marker` em [SIS-TRB](SIS-TRB.md).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-AGR-01 Lavoura automática | 9×9×3; 120 FE por colheita; absorve 3 | `farm.rs` · `farm.rs:6-17` · — | E2 |
| FUN-AGR-02 Semeadora, colheita de plantas altas, lâmpada | Fila de 16; 20/50 FE | `crops.rs` · `crops.rs:1-18` · — | E2 |
| FUN-AGR-03 Rancho e posto de pastor | Raio 6; 10 por espécie; a cada 100 t | `ranch.rs` · `ranch.rs:12-23` · — | E2 |
| FUN-AGR-04 Silvicultura | 8 pontos; tronco ≥ 4; 200 FE | `forestry.rs` · `forestry.rs:9-21` · — | E2 |
| FUN-AGR-05 Moinho, cozinha, armazém | Preço 60; +50% com vila de 3 | `production.json`; `pantry.rs` · idem · — | E2 |
| FUN-AGR-06 Grok lote 2 (lavrador mais barato, trigo) | — | — · **Substituído** · Substituído: `2d5a202` registrado sem reaplicar em `008a980`; PR #7 aberto | E2 + E3 |
| FUN-AGR-07 Lavoura irrigada | Via aspersor | — · `water.rs:225-238` · #40 (contido) | E2 |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 19:10 (0.7) | `819ba92` | Colheitadeira |
| 07/11 20:04 (0.9) | `5944d37` | Lavoura, rancho, comida, armazém |
| 07/11 noite | Grok lote 2 (PR #7) | Lavrador mais barato; não reaplicado |
| 08/11 15:55 | Dono | Agricultura real ([DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039)) |
| 08/11 ~17h | #40 | Necessidades e irrigação |

## F. Pendências

1. Decidir se o armazém do povoado continua criando esmeraldas ([Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros)).
2. Fechar o PR #7 do Grok, já substituído, ou registrar o que dele ainda vale.
3. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-031](../08_BACKLOG_E_PRIORIDADES.md#tar-031): Testar irrigação × lavrador × colheitadeira
- [TAR-063](../08_BACKLOG_E_PRIORIDADES.md#tar-063): Clima e estações do ano com efeito de jogo

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01–05) · pr47 **Integrado** (01–05, 07), **Substituído** (06) |
| Confiança | Média (E2) |
| Prioridade (sugestão) | Baixa |
| Criticidade (sugestão) | Média: sem comida, os trabalhadores param |
| Responsável conhecido | Claude local (0.7–0.9); Grok (lotes); Claude do #40 (irrigação) |
| DEC | [DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039), [DEC-022](../06_HISTORICO_E_DECISOES.md#dec-022), [DEC-025](../06_HISTORICO_E_DECISOES.md#dec-025) |
| Q | [Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros) |
| CNF | [CNF-001](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (armazém do povoado) |
| Referências | README §Agricultura; `frentes/AGUA-E-RIOS.md`; A2 AGR |
