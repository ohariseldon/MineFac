# SIS-LOG: Logística local

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. Linhas em `native/api/src/` no pr47.
- **Escopo:** movimento de itens dentro de uma área carregada. O transporte entre zonas está em [SIS-TRA](SIS-TRA.md).

## A. Identidade

A rede do Factorio dentro do Minecraft: esteiras, túneis, braços, divisor, unificador, filtros, prioridade e redstone. A produção declara os slots de cada máquina; a logística esvazia as saídas (`logistics.rs:1-2`). O contrato comum entre logística e trabalhadores é `operations.rs`, só de leitura, sem estado global (`:1-2`).

- **Depende de:** [SIS-ENE](SIS-ENE.md) (braços), [SIS-PRO](SIS-PRO.md) (slots), [SIS-DES](SIS-DES.md) (cotas).
- **Alimenta:** [SIS-PRO](SIS-PRO.md), [SIS-TRA](SIS-TRA.md) (estações), [SIS-TUT](SIS-TUT.md) (prova de automação).

## B. Funcionamento

### B.1 Esteiras

| Esteira | Itens por passo | Velocidade (README) | Camada |
|---|---|---|---|
| Comum | 1 | 2,5 itens/s | main e pr47 |
| Rápida | 2 | 5 itens/s | main e pr47 |
| Expressa | 4 | 10 itens/s | pr47 (pesquisa Escala) |

- Curvas e rampas desde a 0.9.1.
- Um item que chega a uma esteira vazia espera a vez de andar por até 8 ticks (`BLOCKED_AFTER`). Isso conta como "andando", e a luz não pisca laranja a cada item (`logistics.rs:51-54`, 0.17.2).
- **Túnel** (pr47): a saída fica a até 6 blocos da entrada, na mesma direção. Os túneis alternam entrada e saída ao longo da linha, como no Factorio (`TUNNEL_REACH`, `logistics.rs:22-37`).
- Lotes: a receita inicial faz 16 esteiras; com aço, 64 (README).

### B.2 Braços

| Braço | Ritmo | Energia | Alcance |
|---|---|---|---|
| Normal (`veio:inserter`) | 1 item/s | 40 FE por item | 1 bloco |
| Rápido | 2,5 itens/s | 64 FE | 1 bloco |
| Longo | 1 item/s | 60 FE | 2 blocos |

A transferência usa a operação atômica e lateral de inventário do Horizon (`inserters.rs:1-7`). Filtro por tipo de item.

### B.3 Divisor, unificador, filtros e prioridade

- Divisor e unificador trabalham equilibrados ou com prioridade (`RoutingPriority`) e têm modo filtrado. A amostra do filtro não é consumida (`logistics_controls.rs:1`).
- Pausa por redstone, reavaliada em até 8 ticks (`frentes/CODEX-LOGISTICA.md`, E3).
- **Recusa de entrega** (`operations::accepts_delivery`, `container_accepts`): uma máquina só recebe o que a receita, o combustível ou o pedido atual aceita. O mesmo contrato serve aos carregadores ([SIS-TRB](SIS-TRB.md)).

### B.4 Prancheta (`veio:clipboard`, pr47)

- Copia filtro e prioridade de um braço, divisor ou unificador e cola em outros, como o copiar e colar configurações do Factorio. Agachado copia; em pé cola (`logistics_controls.rs:62-98`).
- Cola o que couber: o unificador não tem filtro; os braços não têm prioridade.
- **A cópia é uma por mundo aberto, compartilhada** (`COPIED`, estático). Em LAN, dois jogadores usam a mesma prancheta ([Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan)).
- Não confundir com a copiadora de plantas (`blueprint_designer`, [SIS-MEG](SIS-MEG.md)). O nome colide ([CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

## C. Experiência do jogador

1. A pessoa liga extratora, prensa e montador por esteiras e põe braços entre a esteira e a máquina.
2. Usa divisor com prioridade para mandar sobra a um baú.
3. Com a prancheta, copia o filtro de um braço para outros dez.
4. O tutorial aceita esteiras, braços, funis e carregadores como prova de automação.

## D. Implementação

**Arquivos:** `logistics.rs` e `logistics/`; `logistics_controls.rs`; `inserters.rs` e `inserters/`; `operations.rs` e `operations/`. Testes (main/pr47, E1): logistics 16/17; inserters 9/9; logistics_controls 2/3; operations 8/8. Desempenho (E1): `configured_arm` 585 → 373 µs; `publish_2000_cables` 135 → 104 µs.

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-LOG-01 Esteira comum e rápida | 1 e 2 itens por passo; espera de 8 t | `logistics.rs` · `logistics.rs:40-54` · — | E1 |
| FUN-LOG-02 Esteira expressa | 4 itens por passo | — · `logistics.rs:42` · #40 (contido) | E2 |
| FUN-LOG-03 Túnel de esteira | Até 6 blocos; alterna entrada e saída | — · `logistics.rs:22-37` · #40 (contido) | E2 |
| FUN-LOG-04 Divisor, unificador, filtros, prioridade | Equilibrado ou prioridade; filtro sem consumo | `logistics_controls.rs` · idem · — | E1 |
| FUN-LOG-05 Braços normal, rápido e longo | 40/64/60 FE; 1/2,5/1 itens/s | `inserters.rs` · `inserters.rs:4-7` · — | E1 |
| FUN-LOG-06 Recusa de entrega | `container_accepts` | `operations.rs` · idem · — | E1 |
| FUN-LOG-07 Prancheta (ex-MEG-03) | Copia filtro e prioridade; cópia global | — · `logistics_controls.rs:62-98` · #40 (contido) | E1 (`clipboard_copies_filter_and_priority_to_compatible_blocks`) |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 20:00 (0.8.1) | `e8d3d62` | Unificador |
| 07/11 20:23–20:37 (0.9.1) | `26599ec`, `LOGISTICA.md` | Curvas e rampas; interface de postos (`operations`) |
| 07/11 (PR #1/#2) | Codex | Braços |
| 08/11 13:13 (0.17.2) | `494d3e7` | Esteira "andando" enquanto espera |
| 08/11 ~17h | #40 | Esteira expressa, túnel, prancheta |

## F. Pendências

1. Decidir se a cópia da prancheta deve ser por jogador ([Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan)).
2. Resolver o nome "prancheta" × copiadora ([Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem), [CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
3. "Logística compacta" é lacuna menor do documento de lacunas: Concebido.
4. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-062](../08_BACKLOG_E_PRIORIDADES.md#tar-062): Logística compacta

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01, 04–06) · pr47 **Integrado** (01–07) |
| Confiança | Alta (E1 em testes e desempenho) |
| Prioridade (sugestão) | Baixa |
| Criticidade (sugestão) | Alta: o jogo de fábrica depende dela |
| Responsável conhecido | Codex local (base, braços, controles); Claude do #40 (expressa, túnel, prancheta) |
| DEC | [DEC-057](../06_HISTORICO_E_DECISOES.md#dec-057) (cotas que a logística usa) |
| Q | [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem), [Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan) |
| CNF | [CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (prancheta × copiadora), [CNF-017](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (estado global em LAN) |
| Referências | `LOGISTICA.md`, `frentes/CODEX-LOGISTICA.md`; A2 LOG; A7 (prancheta) |
