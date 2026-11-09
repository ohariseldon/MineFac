# SIS-DES: Desempenho

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af`, `pr47@80980db`, `#51@a0b0d72`, `#52@0b827f6`. Medições do OPUS A em `--release`, 5 rodadas intercaladas (E1, `evidencias/logs/bench_interleaved.log`). Nenhuma medição no Horizon real.

## A. Identidade

O jogo tem de rodar leve num PC de 8 GB: "é Rust, dá para pôr muito mais" (dono, 20:09, [DEC-056](../06_HISTORICO_E_DECISOES.md#dec-056)). Uma fábrica grande no Minecraft normalmente pesa. O Veio responde com cotas globais de trabalho por tick, buscas incrementais e produção virtual em vez de simular o que está longe.

- **Depende de:** todos os sistemas que trabalham por tick.
- **Alimenta:** a viabilidade de [SIS-TRB](SIS-TRB.md), [SIS-ENE](SIS-ENE.md), [SIS-LOG](SIS-LOG.md) e [SIS-TRA](SIS-TRA.md) em escala.

## B. Funcionamento

### B.1 Orçamento global

Cada categoria tem uma cota de trabalho por tick, com fila FIFO. O que não cabe espera o próximo tick (`budget.rs:8`; [DEC-057](../06_HISTORICO_E_DECISOES.md#dec-057)):

| Categoria | Cota | Validade (ticks) | Quem usa |
|---|---|---|---|
| Área (elétrica) | 512 | 2 | Postes |
| Terreno | 384 | 2 | Lavoura, silvicultura, pureza, rios |
| Topologia | 4.096 | 2 | Fios e redes |
| Planejamento | 2.048 | 2 | Fila do capataz |
| Trabalhadores | 512 | 20 | Postos, canteiro |
| Busca de carga | 1.024 | 2 | Carregadores |

### B.2 Buscas incrementais

O capataz, o carregador, as redes de fios e a conferência do canteiro avançam um pedaço por ciclo e retomam de onde pararam ([SIS-TRB](SIS-TRB.md) B.3; Codex `13758bd`, `604369d`). A fábrica distante não é simulada: as estações estimam o fluxo ([SIS-TRA](SIS-TRA.md) B.1), a 194,4 µs por 1.000 estações a cada 20 ticks.

### B.3 Medições

| Ramo | `bench_factory` (480 blocos) | vs. main | `bench_frame` | Alocações por quadro | `bench_large_village` (150 aldeões + 300 animais) |
|---|---|---|---|---|---|
| main | 268,7 µs/tick | — | 45,2 µs | 0 | 3,4 µs/tick |
| pr47 | 364,8 | **+36%** | 71,4 | **329** | **pânico** (`world_dir` no harness) |
| #52 | 363,5 | +35% | 66,0 | 329 | pânico |
| #51 | 347,1 | +29% | 68,7 | 329 | pânico |

- **A candidata regrediu.** `check_performance` passa assim mesmo, porque os limites são folgados.
- Os commits de otimização de desempenho e fluidos já estão na main e no pr47. A regressão vem de código acrescentado depois. Suspeitos: `appearance`, `motion`, `soundscape` e a cadeia `if/else` de `tick_factory`, com 13 destinos (`lib.rs:1093`) (09 [R-14](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)/15/16).
- **O #52 não resolve.** Muda 6 arquivos: `soundscape.rs` (`near()`), `home/mural.rs` (`IDLE_SCAN_EVERY 20`), `perf.rs` (contador `LIVE`), `perf/factory.rs` (`bench_factory_scale`), `home/tests.rs` e `frentes/DESEMPENHO.md` (+71 linhas). O documento afirma "0 alocações por quadro" num ramo que mede 329 ([CNF-020](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
- **O #51 também não resolve.**

Ganhos anteriores, medidos offline (README, E3): 0.16.0 deixou vilas grandes cerca de 4 vezes mais baratas; 0.17.1 cortou 36% do trabalho por tick.

### B.4 O que não é medido

Rios, água, estações em escala real, som, obras e copiadora com 6.000 blocos estão fora de `check_performance`. Não há medida de FPS nem de memória no Horizon real. Os lotes do Grok também não alegam FPS.

## C. Experiência do jogador

Esperada: uma fábrica de centenas de máquinas e uma vila de 150 rodando a 60 FPS num PC de 8 GB. Real: desconhecido. Nenhuma versão desde a 0.11 foi jogada.

## D. Implementação

**Arquivos:** `budget.rs` (3 testes), `perf.rs` e `perf/` (cenários e benches), `tools/check_performance.py` (7 cenários); `lib.rs:1093` (`tick_factory`).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-DES-01 Orçamento global por faixa | 6 cotas; FIFO | `budget.rs` · `budget.rs:8` · — | E1 (3 testes) |
| FUN-DES-02 Topologia e busca incremental | Retomáveis | Integrado · Integrado · — | E1 |
| FUN-DES-03 Bench fábrica 480 blocos | Sem regressão sobre a main | 268,7 µs · 364,8 µs (+36%) · — | E1 |
| FUN-DES-04 Teto global de ticks e limite do mapa | — | — · Em desenvolvimento · thread "Definir limite do mapa e otimizar"; #52 | E4 + E3 |
| FUN-DES-05 Critério de 8 GB ([DEC-056](../06_HISTORICO_E_DECISOES.md#dec-056)) | FPS e memória mínimos numa fábrica de referência | — · — · Concebido | — |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 23:09–23:46 | Codex, PRs #22, #24, #27, #29 | Otimização, escala, orçamento global, topologia incremental ([DEC-057](../06_HISTORICO_E_DECISOES.md#dec-057)) |
| 08/11 05:08 / 13:21 | #32, #38 | Capatazes e carregadores incrementais |
| 08/11 05:26 (0.16.0) | `939c552` | Vilas grandes mais baratas |
| 08/11 07:17 (0.17.1) | `0472750` | −36% por tick; quadro sem alocação |
| 08/11 20:09 | Dono | 8 GB; limite do mapa e otimizar ([DEC-056](../06_HISTORICO_E_DECISOES.md#dec-056)) |
| 08/11 20:35 | PR #52 | Sons perto do jogador; mural ocioso; bench de escala ([DEC-063](../06_HISTORICO_E_DECISOES.md#dec-063)) |

## F. Pendências

1. Bisseção main ↔ pr47 para achar a regressão de +36% e as 329 alocações ([R-14](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)/15).
2. Implementar `world_dir` no harness para medir a vila grande no pr47 ([R-16](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
3. Apertar os limites de `check_performance` para pegar regressões de 10% ou mais.
4. Criar cenários para rios, água, estações em escala, som, obras e copiadora.
5. Definir o critério de 8 GB e medir no Horizon real (FUN-DES-05).
6. Corrigir `frentes/DESEMPENHO.md` do #52 ([CNF-020](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

### Tarefas no backlog

- [TAR-008](../08_BACKLOG_E_PRIORIDADES.md#tar-008): Recuperar o custo por tick da fábrica
- [TAR-009](../08_BACKLOG_E_PRIORIDADES.md#tar-009): Quadro sem alocação de novo
- [TAR-013](../08_BACKLOG_E_PRIORIDADES.md#tar-013): Auditar e reintegrar o #52
- [TAR-025](../08_BACKLOG_E_PRIORIDADES.md#tar-025): Mapa finito e limite do mapa
- [TAR-037](../08_BACKLOG_E_PRIORIDADES.md#tar-037): Benchmarks dos sistemas novos
- [TAR-038](../08_BACKLOG_E_PRIORIDADES.md#tar-038): Teto global de tempo por tick
- [TAR-039](../08_BACKLOG_E_PRIORIDADES.md#tar-039): Critério numérico para 8 GB e medição no Horizon

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01–03) · pr47 **Integrado** (01, 02), **regressão** em 03, **Em desenvolvimento** (04) · disperso: 05 **Concebido** |
| Confiança | Alta para a medição offline (E1, 5 rodadas); nula para o jogo real |
| Prioridade (sugestão) | **Muito alta**: condição do merge ([Q-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-11-condições-para-fundir-o-47)) |
| Criticidade (sugestão) | Alta: contraria uma meta explícita do dono |
| Responsável conhecido | Codex (orçamento, incrementais); Claude nuvem (0.16–0.17); thread "Definir limite do mapa e otimizar" (#52) |
| DEC | [DEC-056](../06_HISTORICO_E_DECISOES.md#dec-056), [DEC-057](../06_HISTORICO_E_DECISOES.md#dec-057), [DEC-063](../06_HISTORICO_E_DECISOES.md#dec-063) |
| Q | [Q-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-11-condições-para-fundir-o-47), [Q-07](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-07-realismo-clima-e-estações-do-ano) |
| CNF | [CNF-020](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (#52 afirma 0 alocações) |
| Referências | `PERFORMANCE.md`, `frentes/ORCAMENTO-GLOBAL.md`, `frentes/DESEMPENHO.md` (#52); A2 DES; A7 §4 |
