# SIS-AGU: Água encanada

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `pr47@80980db`. O sistema não existe na main. Linhas em `native/api/src/production/water.rs`, salvo indicação.

## A. Identidade

Leva água real de rios, lagos e mar até a lavoura, a caldeira e, com lava, até a geotérmica. O dono pediu "água e energia (bombas, canos, usinas reais)" e "explicar no tutorial", "sem remodelar do zero" (15:55, [DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039)).

- **Escolha técnica:** a água fica no NBT `Water` de cada bloco, em mB (1.000 por balde), como a energia em `energy`. Não usa os tanques de fluido do Horizon ([DEC-050](../06_HISTORICO_E_DECISOES.md#dec-050); `water.rs:3-4`).
- **Depende de:** [SIS-MUN](SIS-MUN.md) (rios), [SIS-ENE](SIS-ENE.md) (bomba a 6 FE/t).
- **Alimenta:** [SIS-AGR](SIS-AGR.md) (irrigação) e [SIS-ENE](SIS-ENE.md) (caldeira, geotérmica).

## B. Funcionamento

### B.1 Bomba e cano

| Bloco | Regra | Tanque | Fonte |
|---|---|---|---|
| Bomba (`veio:pump`) | 25 mB/t por bloco de água parada encostado, até 4 (100 mB/t); gasta 6 FE/t enquanto bombeia | 4.000 mB | `:25-28`, `:57` |
| Cano (`veio:pipe`) | Iguala o nível com os canos vizinhos, até 200 mB/t por face, e entrega tudo o que cabe a quem usa água | 1.000 mB | `:22-23` |
| Aspersor (`veio:sprinkler`) | Ver B.2 | 4.000 mB | `:59` |
| Caldeira | 5 mB por tick de queima | 8.000 mB | `:30`, `:60` |

Cada cano leva um fluido por vez. `Fluid = 1` é lava. Vazio, aceita qualquer um (`:37-43`).

### B.2 Irrigação

1. O aspersor mantém úmida a terra arada num 9×9, sem a poça de água no meio.
2. A cada passada (4 s), uma cultura avança um estágio com chance **1 em 10** se tiver água e adubo, ou **1 em 30** só com água (`FED_ONE_IN`, `WATERED_ONE_IN`, `:233-234`).
3. **Fertilidade:** farinha de osso dá 40 e biomassa dá 20, até 400 (`:225-235`).
4. **Consumo por estágio** (`frentes/AGUA-E-RIOS.md`, E3; proporções da demanda real):

| Cultura | Água (mB) | Adubo |
|---|---|---|
| Cenoura | 45 | 4 |
| Trigo | 60 | 6 |
| Flor-tocha | 60 | 5 |
| Batata | 70 | 6 |
| Abóbora | 80 | 7 |
| Melancia | 90 | 7 |
| Beterraba | 150 | 14 |

5. **Evaporação:** 2 mB por terra arada visitada; metade quando chove (`SOIL_WATER`, `:237`).

### B.3 Lava encanada

A mesma bomba, encostada em lava parada, puxa 5 mB/t por fonte. Os canos levam a lava até o gerador geotérmico; 1.000 mB valem um balde (`LAVA_PER_SOURCE`, `LAVA_BUCKET`, `:37-42`). O comportamento da lava nos canos em jogo é ND.

## C. Experiência do jogador

1. A pessoa põe uma bomba na margem do rio, encana até o aspersor e vê a lavoura ganhar estágios extras.
2. Com farinha de osso no aspersor, a chance de avanço triplica (1 em 10 contra 1 em 30).
3. Depois encana água para a caldeira e lava para a geotérmica.

**Tutorial:** as páginas 15 ("Água e irrigação") e 16 ("Rios e represas") do guia explicam a água. O tutorial na tela não explica ([SIS-TUT](SIS-TUT.md)). O pedido do dono foi atendido só em parte.

## D. Implementação

**Arquivos:** `production/water.rs`; `production/water_tests.rs`. NBT `Water` e `Fluid` em cada bloco ([SIS-PER](SIS-PER.md)).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-AGU-01 Bomba | 25 mB/t × 4 fontes; 6 FE/t; 4.000 mB | — · `water.rs:25-28,57` · #40 (contido) | E2 + testes de água (E1) |
| FUN-AGU-02 Cano | 200 mB/face; 1.000 mB; um fluido por vez | — · `water.rs:22-23,37-43` · #40 (contido) | E2 |
| FUN-AGU-03 Aspersor e fertilidade | 9×9; 1/10 e 1/30; fertilidade até 400 | — · `water.rs:225-238` · #40 (contido) | E2 |
| FUN-AGU-04 Lava nos canos | 5 mB/t por fonte; balde = 1.000 | — · `water.rs:37-42` · #40 (contido) | E2; comportamento ND |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 08/11 15:55 | Dono | Pedido de água, irrigação e explicação no tutorial ([DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039)) |
| 08/11 ~16:00–17:06 | #40 (renumerado de 0.18 a 0.25) | Bomba, cano, aspersor, lava |
| 08/11 19:06 | `eedfd47` | Entra no pr47 |

## F. Pendências

1. Explicar água e irrigação no tutorial na tela, como o dono pediu.
2. Ver em jogo a lava nos canos e a mistura de fluidos.
3. Pôr bomba, cano e aspersor em `check_performance`.
4. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-026](../08_BACKLOG_E_PRIORIDADES.md#tar-026): Lava nos canos
- [TAR-031](../08_BACKLOG_E_PRIORIDADES.md#tar-031): Testar irrigação × lavrador × colheitadeira

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main — · pr47 **Integrado** (4 de 4) |
| Confiança | Média: E2 e testes; nunca visto em jogo |
| Prioridade (sugestão) | Média |
| Criticidade (sugestão) | Média: afeta a lavoura e a energia, não o save |
| Responsável conhecido | Claude da thread "agricultura" (#40) |
| DEC | [DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039), [DEC-050](../06_HISTORICO_E_DECISOES.md#dec-050) |
| Q | — |
| CNF | — |
| Referências | `frentes/AGUA-E-RIOS.md`; guia págs. 15–16; A2 AGU |
