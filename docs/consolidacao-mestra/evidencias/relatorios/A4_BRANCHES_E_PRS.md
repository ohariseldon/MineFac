# A4 — Branches, PRs e genealogia

Autor: OPUS A. Data: 2026-11-08 (~20:45 UTC). Fonte: clone `/home/user/MineFac` (42 branches remotas buscadas antes desta auditoria), `gh api repos/ohariseldon/MineFac/pulls` (REST; GraphQL indisponível).
Método de contenção (E2): `git merge-base --is-ancestor <head> 53267af` (main) e `… 80980db` (pr47); `git rev-list --count 53267af..<head>`; para o que não é ancestral, `git cherry`/`patch-id` e comparação de blobs.
**Recomendações nesta página são de agente (não confirmadas pelo dono).**

## 1. As 42 branches remotas

Legenda: M = contida na main; P = contida no pr47; "+n" = commits à frente da main.

| Branch | Head | Autor do head / data | M | P | +n | PR | Conteúdo | Recomendação |
|---|---|---|---|---|---|---|---|---|
| `main` | 53267af | jefterpereira 08/11 14:29 −03 | ✓ | ✓ | 0 | — | Veio 0.25.0 | Base |
| `claude/trusting-ritchie-sc9ty8` | 53267af | jefterpereira | ✓ | ✓ | 0 | #3…#48 (fundidos) | Linha Cloud 0.10→0.25 | Pode ser apagada (idêntica à main) |
| `claude/project-thread-l94frq` | 80980db | Codex 08/11 19:29Z | — | ✓ | 79 | **#47 aberto** | Candidata consolidada | Ver §5 |
| `claude/project-thread-6tjsil` | 304b642 | Claude 17:07Z | — | ✓ | 7 | #40 aberto | Água, rios, grande indústria, estações (0.23.0 do #40) | Fechar #40 quando o #47 for decidido |
| `codex/checkpoint-consolidacao` | 50995e6 | Codex 15:11 −03 | — | ✓ | 60 | — | Consolidação Codex (fundida em 5c57405) | Arquivar/apagar após #47 |
| `codex/consolidated-016` | 50995e6 | Codex | — | ✓ | 60 | — | Mesmo head da anterior | Idem |
| `codex/consolidated-0161` | bc2d0ca | Codex 16:09 −03 | — | — | 43 (2 fora do pr47) | — | c8e94dc (patch-id = 80980db) + zips 0.15.1/0.15.2 | Manter só como arquivo histórico ou apagar; nenhum código único |
| `codex/arquivos-builds-anteriores` | bd626c7 | Codex 16:07 −03 | — | — | 43 (2 fora do pr47) | — | c8e94dc + "builds de tutorial não rastreados" | Idem |
| `codex/arquivos-historicos` | 4ddad22 | Codex 16:08 −03 | — | — | 1 | — | Só `release/Veio-0.15.1.zip` e `0.15.2.zip` (+4 arquivos, 11 linhas) | Idem |
| `codex/factory-ambience-motion` | 7024938 | Jefter Pereira 17:11Z | — | ✓ | 8 | #50 aberto (draft, base machine-audio) | Ambiente sonoro e animação | Fechar #50 após #47 |
| `codex/machine-audio` | cfde2b0 | Jefter Pereira 16:42Z | — | ✓ | 7 | #49 aberto (draft, base machine-identity) | Som posicional (30 `.ogg`) | Fechar #49 após #47 |
| `codex/machine-identity` | d396f69 | Jefter Pereira 00:25Z | — | ✓ | 6 | #31 aberto (base interzone-logistics) | Silhuetas e colisão | Fechar #31 após #47 |
| `codex/interzone-logistics` | 8b8e8d9 | Jefter Pereira 00:00Z | — | ✓ | 5 | #28 aberto | Ferrovia, drones, caravanas | Fechar #28 após #47 |
| `codex/import-factory-board` | fe30583 | Claude (MineFac) 02:43 −03 | — | ✓ | 4 | — | Quadro da fábrica | Apagar após #47 |
| `codex/import-megafactories` | e0ee148 | Claude (MineFac) 10:11 −03 | — | ✓ | 5 | — | Copiadora, marco, elevador, escadas | Apagar após #47 |
| `codex/import-progressao` | 2b9a3f9 | Claude (MineFac) 02:17 −03 | — | ✓ | 2 | — | Progressão por fases (tempos substituídos pela decisão 9) | Apagar após #47 |
| `codex/local-snapshot` | c1b3ba5 | Codex 02:04 −03 | — | ✓ | 2 | — | Tutorial local, casa do fundador, visual, checkpoint do mapa finito | Apagar após #47 |
| `codex/realismo-performance` | fd2a446 | Codex 02:07 −03 | — | ✓ | 2 | — | Casa do fundador + checkpoint mapa finito + `server_about_to_start` | Apagar após #47 |
| `codex/expansion` | 20f9dba | Codex 07/11 | ✓ | ✓ | 0 | #1 | Postos e entregas | Apagar |
| `codex/global-budget` | 690f554 | Codex | ✓ | ✓ | 0 | #27 | Orçamento global | Apagar |
| `codex/incremental-foremen` | 13758bd | Codex | ✓ | ✓ | 0 | #32 | Capataz incremental | Apagar |
| `codex/incremental-hauler-search` | 604369d | Codex | ✓ | ✓ | 0 | #38 | Carregador incremental | Apagar |
| `codex/incremental-topology` | cad77c3 | Codex | ✓ | ✓ | 0 | #29 | Topologia de energia incremental | Apagar |
| `codex/inserters` | d5a85fa | Codex | ✓ | ✓ | 0 | #8 | Braços e filtros | Apagar |
| `codex/optimization` | 65cab50 | bizi (dono) | ✓ | ✓ | 0 | #22 (head da época 4244a7e) | Menos alocações | Apagar |
| `codex/scaling` | 350f064 | Codex | ✓ | ✓ | 0 | #24 | Picos de descoberta de energia | Apagar |
| `logistics/merger` | 26599ec | Codex | ✓ | ✓ | 0 | — | Fusor | Apagar |
| `cursor/grok-performance-balance-c7d9` | 068a61b | Cursor Agent | ✓ | ✓ | 0 | #2 | Lote 1 | Apagar |
| `cursor/grok-lote3-c7d9` … `lote8-c7d9` (6 branches) | b6a6cdf, 0a435a3, 4c541ac, 8d64fe3, dda34fa, a8e4741 | Cursor Agent | ✓ | ✓ | 0 | #9, #11, #12, #14, #16, #18 | Lotes 3–8 | Apagar |
| `cursor/grok-noite-c7d9` | ce0e7d1 | Cursor Agent | ✓ | ✓ | 0 | #26 | Noite caça o jogador | Apagar |
| `cursor/grok-quartel-c7d9` | a6d2744 | Cursor Agent | ✓ | ✓ | 0 | **#30 aberto** (base grok-noite) | Quartel | Fechar #30: conteúdo já na main |
| `cursor/grok-lote2-c7d9` | 2d5a202 | Cursor Agent | — | ✓ | 1 | **#7 aberto** | Lote 2 — no pr47 só "registrado" (008a980), código não reaplicado | Fechar #7 como substituído pelo lote 3 |
| `cursor/grok-balance-c7d9` | 96458cf | Cursor Agent | — | ✓ | 1 | #19 aberto | Duas prensas por veio | Fechar após #47 |
| `cursor/grok-lan-c7d9` | cf2a63c | Cursor Agent | — | ✓ | 1 | #25 aberto | Convidado LAN | Fechar após #47 |
| `cursor/grok-vilas-c7d9` | 19a311e | Cursor Agent | — | ✓ | 1 | #21 aberto | Cama livre para crescer | Fechar após #47 |
| `grok/mission-balance` | c5db4f6 | Jefter Pereira | — | ✓ | 2 | #20 aberto | Projetos industriais; produtividade 10 → 20 % | Fechar após #47; registrar a mudança de produtividade como decisão pendente |
| `research/connected-veins` | 2edc6c0 | Jefter Pereira | — | ✓ | 1 | #23 aberto (draft) | Pesquisa/protótipo de veios conectados (docs) | Fechar após #47 |

Contagem: 42 branches = 20 contidas na main e no pr47 (incluindo `main` e `trusting-ritchie`) + 19 só no pr47 (incluindo o próprio head `l94frq`) + 3 fora das duas.

## 2. PRs

### 2.1 Abertos (REST, 20:45Z)

| PR | Título (resumido) | Head → base | Autor | Contido no pr47? | Recomendação de agente |
|---|---|---|---|---|---|
| **#52** | Desempenho depois da consolidação: sons só perto do jogador, mural ocioso, medição de escala | `claude/project-thread-e9lkha`@0b827f6 → `l94frq` | ohariseldon | **Não** (criado 20:35Z, depois do início desta auditoria; branch não buscada) | Auditar antes de fundir no #47; mexe em `soundscape.rs`, `home/mural.rs`, `perf/factory.rs` (+164/−6, 6 arquivos) |
| **#51** | Veio 0.26.0: visual Realismo 2.1, clima e estações | `claude/project-thread-mrtok9`@a0b0d72 → `l94frq` | ohariseldon | **Não** (criado 20:34Z) | Auditar: 53 arquivos, +2683/−189; novos `ambience.rs`, `climate.rs`, `tools/check_shaders.py`, 9 shaders novos, `release/Veio-0.26.0.zip`, +48 linhas no stub do Horizon |
| #50 | Suaviza ambiente sonoro e anima peças | `codex/factory-ambience-motion`@7024938 → `codex/machine-audio` | jefterpereira (draft) | Sim | Fechar após #47 |
| #49 | Feedback sonoro posicional (CC0) | `codex/machine-audio`@cfde2b0 → `codex/machine-identity` | jefterpereira (draft) | Sim | Fechar após #47 |
| **#47** | Integra obras, sons, grande indústria e consolidação Codex com a main 0.25.0 | `l94frq`@80980db → main | ohariseldon | — | Ver §5 |
| #40 | Veio 0.23.0: grande indústria, postos distantes, água, rios | `6tjsil`@304b642 → main | ohariseldon | Sim (eedfd47) | Fechar após #47 |
| #31 | Formatos e materiais das máquinas | `codex/machine-identity`@d396f69 → `codex/interzone-logistics` | jefterpereira | Sim | Fechar após #47 |
| #30 | Quartel | `cursor/grok-quartel-c7d9`@a6d2744 → `cursor/grok-noite-c7d9` | ohariseldon | Sim (e na main) | Fechar já |
| #28 | Caravanas, drones e ferrovias | `codex/interzone-logistics`@8b8e8d9 → main | jefterpereira | Sim | Fechar após #47 |
| #25 | LAN: convidado na vila do anfitrião | `cursor/grok-lan-c7d9`@cf2a63c → main | ohariseldon | Sim | Fechar após #47 |
| #23 | Pesquisa: jazidas conectadas | `research/connected-veins`@2edc6c0 → main | jefterpereira (draft) | Sim | Fechar após #47 |
| #21 | Vila só cresce com cama | `cursor/grok-vilas-c7d9`@19a311e → main | ohariseldon | Sim | Fechar após #47 |
| #20 | Progressão industrial, produtividade, recompensas | `grok/mission-balance`@c5db4f6 → main | jefterpereira | Sim | Fechar após #47 |
| #19 | Duas prensas; máquina mostra que trabalha | `cursor/grok-balance-c7d9`@96458cf → main | ohariseldon | Sim | Fechar após #47 |
| #7 | Grok lote 2 | `cursor/grok-lote2-c7d9`@2d5a202 → main | ohariseldon | Ancestral, mas código não reaplicado (008a980) | Fechar como substituído |

O contexto da missão listava 13 PRs abertos; há **15**: #51 e #52 surgiram durante a auditoria, ambos com base no pr47 (trabalho das threads "Aumentar o realismo sem pesar o PC" e "Definir limite do mapa e otimizar", E4).

### 2.2 Fundidos (todos com base `main`)
#1 (codex/expansion), #2 (Grok lote 1), #3 (0.10.0), #4–#5 (launcher), #6 (0.11.0), #8 (inserters), #9 (lote 3), #10 (0.12.0), #11–#12 (lotes 4–5), #13 (0.13.0), #14 (lote 6), #15 (0.14.0), #16 (lote 7), #17 (0.15.0), #18 (lote 8), #22 (optimization), #24 (scaling), #26 (grok-noite), #27 (global-budget), #29 (incremental-topology), #32 (incremental-foremen), #33–#37 (0.16.0–0.17.2), #38 (hauler-search), #39 (0.18.0), #41–#46 (0.19.0–0.24.0), #48 (0.25.0). Nenhum PR fechado sem merge.

## 3. Linha do tempo 0.9 → 0.25 (main) e versões paralelas

| Versão | Commit | Autor | PR / merged_at (UTC) | Conteúdo (título do commit) |
|---|---|---|---|---|
| 0.6 | 5eaea2c | Codex | — (07/11 15:24 −03) | Base inicial |
| 0.7 / 0.8 | 819ba92 / 7644aab | Claude | — | |
| 0.9 | 5944d37 | Claude | — (07/11 17:04 −03) | Jazidas com pureza, receitas por tabela, renováveis, lavoura, rancho, comida |
| 0.10.0 | 9421883 | Claude | #3 21:06Z (junto #1 Codex, #2 Grok lote 1) | Consolida as três frentes + launcher.exe |
| — | 12cd631 / 0eaeb34 | Claude | #4 21:13Z / #5 21:19Z | launcher.exe no repositório; cria instância |
| 0.11.0 | 903c4ec | Claude | #6 21:34Z | Carregador, capataz, interface de tarefas |
| 0.12.0 | 6ac0c62 (6fd1b44 no PR) | Claude | #10 21:45Z (+#9 lote 3) | Fila do capataz, pastor |
| 0.13.0 | 36e43b6 | Claude | #13 22:10Z (+#11, #12) | Zonas, celeiro/silo, centro de controle, crescimento |
| 0.14.0 | a691802 | Claude | #15 22:33Z (+#14) | Vilas até 150, spawn no centro, Guia da Vila |
| 0.15.0 | 8abe435 | Claude | #17 22:39Z (+#8 inserters, #16 lote 7) | Build consolidada Cloud+Codex+Grok |
| — | — | Codex/Grok | #18 23:18Z, #22 23:09Z, #24 23:18Z, #27 23:30Z, #29 23:46Z | Lote 8, otimizações, orçamento, topologia |
| 0.16.0 | 939c552 | Claude | #33 08/11 05:26Z (+#32 05:08Z) | Cérebro dos aldeões, descanso noturno |
| 0.16.1 | 7208324 | Claude | #34 05:36Z | Trabalhador segura o que carrega |
| 0.17.0 | 0deee22 | Claude | #35 05:53Z (+#26 grok-noite 05:53Z) | Inimigo melhor e quartel funcionando |
| 0.17.1 | 0472750 | Claude | #36 07:17Z | Ticks e quadros mais leves |
| 0.17.2 | 494d3e7 | Claude | #37 13:14Z (+#38 13:21Z) | Esteira aguardando vez aparece em movimento |
| 0.18.0 | 0aa3fbb | Claude | #39 16:06Z | Sociedade da vila, lojas, prefeito |
| 0.19.0 | a48e14c | Claude | #41 16:26Z | Contas separadas, impostos, preços locais |
| 0.20.0 | 95a1f7c | Claude | #42 16:31Z | Painel da holding |
| 0.21.0 | 9331dee | Claude | #43 16:48Z | Economia fechada, imposto na fonte, lucro por empresa |
| 0.22.0 | cbc53a9 | Claude | #44 17:05Z | Comércio exterior pelo porto |
| 0.23.0 | e8ac991 | Claude | #45 17:14Z | Comércio exterior por conexões |
| 0.24.0 | b25b9d0 | Claude | #46 17:20Z | Polimento do comércio exterior |
| 0.25.0 | 87ee998 | Claude | #48 17:29Z | Balanço da economia local e externa |

Colisões de número de versão (E2):
- **0.16.0** duas vezes: main 939c552 (05:26Z) × integração Claude 2a74e44 "Veio 0.16.0: rodada de integração e revisão" (07/11 21:50 −03, hoje dentro do pr47) com "Pacote 0.16.0 refeito" (3cd563d).
- **0.16.1** duas vezes: main 7208324 × consolidação Codex a3303bb "Release consolidated Veio 0.16.1" (08/11 02:25 −03 = 05:25Z) e 4ffb605 "after full Horizon native compilation". O `launcher.exe` do pr47 e `release/Veio-Realismo-1.1.zip` vêm dessa 0.16.1.
- **0.18–0.23** no #40: bcfb9d5 "0.18.0" → 55579bc "0.19.0" → 1a38b93 "0.20.0" → 46152c8 "0.22.0" → 304b642 "0.23.0", colidindo com 0.18–0.23 da main.
- **0.15.1/0.15.2**: só zips locais do Codex (branches de arquivo).
- **0.26.0**: PR #51 (aberto) já usa o próximo número.

## 4. Verificação independente de "nenhum código ficou fora do #47"

| Verificação | Resultado (E2) |
|---|---|
| Ancestralidade das 42 branches em relação a 80980db | 39 contidas; 3 não: `codex/arquivos-builds-anteriores` (bd626c7), `codex/arquivos-historicos` (4ddad22), `codex/consolidated-0161` (bc2d0ca) |
| Commits dessas 3 fora do pr47 | bd626c7 (preserva builds de tutorial), 4ddad22 (zips 0.15.1/0.15.2), bc2d0ca (arquiva 0.15.1/0.15.2), **c8e94dc** (mapa finito/portais) |
| c8e94dc × 80980db | `patch-id` idêntico (b6d1ef2e…); `git cherry` marca "−" — é o mesmo patch reaplicado como head do pr47 |
| Zip 0.15.1 (blobs normalizados CRLF→LF contra os 5586 objetos do pr47) | 12 arquivos sem equivalente, 6 nativos (`Cargo.toml`, `lib.rs`, `logistics.rs`, `onboarding.rs`, `onboarding/tests.rs`, `horizon-mixin.toml`): iteração anterior do tutorial (`TRACE_TTL 200` × 6000 atual), substituída |
| Zip 0.15.2 | 5 arquivos sem equivalente (`lib.rs`, `workforce/mod.rs`): é c1b3ba5 menos a casa do fundador — subconjunto |
| Upload do usuário (`/mnt/project-files/uploads/hearth/bfe368aa…`) | 67 arquivos de `consolidada/arquivos/` idênticos aos de f1452f5; BASELINE base 542b0a0, head 4a096e9, `cloud/obras-planta`; patches 0001–0005 presentes |
| Heads dos PRs abertos #7–#50 | Todos ancestrais do pr47 (#30 também da main) |
| PRs fundidos | Todos com base `main` → contidos na main e no pr47 |
| **Conclusão** | Confirmado: nenhum código único das branches/PRs/zips conhecidos ficou fora do pr47 **até 80980db**. Ressalvas: (a) commits locais nunca enviados são invisíveis; (b) **#51 e #52** (20:34–20:35Z) são trabalho novo fora do pr47, ainda não auditado em código. |

## 5. O pr47 por dentro (79 commits) e o que está desligado

Grupos de commits (ordem topológica, E2 `git log 53267af..80980db`):
1. PRs do Grok/Jefter (07/11 21:35–23:19Z): 2d5a202, 3b3008d, c5db4f6, 2edc6c0, cf2a63c, 96458cf, 19a311e.
2. Logística entre zonas Codex/Jefter: b616285 (realismo opcional), 90721ca, afbc389, 8b8e8d9, d396f69 (silhuetas).
3. Integração Claude "0.16.0" (07/11 20:55–21:51 −03): d1e2f30 (casa do fundador), 4145e3d (quartel), merges de connected-veins, grok-lan, grok-vilas, grok-balance, mission-balance, interzone-logistics; 7d2e0be (guias ≤ 14 linhas); 2a74e44; 848e1b8 (clippy); 3cd563d.
4. Consolidação Codex (08/11 02:04–02:49 −03): c1b3ba5, 9457afa, fd2a446, 07eaff2, 730e305, 355f24a, 008a980, 8d3659b, 6227e24, a3303bb (0.16.1), 4ffb605 (compilação nativa alegada), 3106849, 2bc62ec (atualização gráfica 1.1), bd988ea.
5. Importações de patches em três cópias (mesmo conteúdo, rebase): 0ce91a4/0be045c/823d626 (progressão por fases), 2b9a3f9/13074eb/32aa2a5, a19ebd7/a3e79d5 (quadro), fe30583/799aa13, e0ee148 (megafábricas); c8c43c5 registra a equivalência por range-diff.
6. Codex 10:11–10:36 −03: cb45a27, 2720da4, fa96ceb, 52f5155 (merge da 0.17.2 Cloud), f99f66a.
7. PR #40 (Claude 16:23–17:07Z): bcfb9d5, 55579bc, 1a38b93, 82ccaf6, 46152c8, e36ee2e, 304b642.
8. Sons e movimento (Jefter 16:42–17:11Z): cfde2b0, 7024938.
9. Obras e integração final (Claude 17:14–19:29Z): f1452f5 (build parcial de obras), 843a286, f12f980, e293d6b, a79f326 (rail_terminal), a245f28, e5c837a, 50995e6 (checkpoint Codex), eedfd47 (merge #40), 5c57405 (merge consolidação Codex).
10. Head 80980db (Codex, "Checkpoint finite world generation and portal safety work"), commit date 19:29Z.

Desligado/WIP no pr47:
- `work-in-progress/finite-world/` (10 `.rs`, `hooks/`, `horizon-mixin.toml.draft`, `CONTINUAR.md`): fora do crate, não compilado pelos testes; usa mixins de motor (22/24 injeções aplicadas, E3); auditoria do Nether falhou (E3).
- `tools/world_map_probe.rs`, `tools/prepare_world_map_check.py`: apoio ao mapa finito, fora das verificações.
- Funções mortas: `sites_near`, `research_percent`, `distance`.
- Pesquisas da vila (`staff.rs`) sem traduções → aparecem como chaves cruas (ND no jogo).

Recomendação de agente sobre o #47: antes de fundir, (1) decidir o que fazer com #51/#52 (base = pr47); (2) consertar o harness de `bench_large_village` e investigar +40 % no `bench_factory` e 329 alocações por quadro; (3) gerar pacote 0.25.x/0.26 próprio e remover/rotular `Veio-Realismo-1.1.zip` (0.16.1); (4) fechar as pendências do HANDOFF_OBRAS que quebram a tela (menu do canteiro, traduções); (5) uma compilação real com `horizon-launcher native-build` fixando a revisão do Horizon.
