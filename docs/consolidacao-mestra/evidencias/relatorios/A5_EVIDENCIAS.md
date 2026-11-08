# A5 — Índice de evidências

Autor: OPUS A. Data: 2026-11-08. Confiabilidade: **Alta** = executado ou lido no código nesta sessão; **Média** = documento com data e autor, coerente com o código; **Baixa** = documento desatualizado ou contraditório com o código; **ND** = não verificável aqui.
Logs desta auditoria: `SP/a/logs/` (SP = `/tmp/claude-0/-home-user-MineFac/2438a94b-e52e-53f5-9ff0-c6cae7a87c22/scratchpad`). Cópias de trabalho para geradores: `SP/a/tmp/{main,pr47}`. Referência do Horizon (só leitura, nada executado): `SP/a/horizon_ref/`.

## 1. Execuções (E1)

| # | Comando (cwd) | main@53267af | pr47@80980db | Log |
|---|---|---|---|---|
| X1 | `python3 tools/check_project.py` | exit 0 — "Veio 0.25.0: 529 JSON válidos; 93 páginas e traduções conferidas; 65 texturas 16x16" | exit 0 — "742 JSON válidos; 100 páginas; 89 texturas" + aviso "29 páginas de village_guide passam de 14 linhas" | `main_check_project.log`, `pr47_check_project.log` |
| X2 | `python3 tools/check_rust.py` | 176 passed, 0 failed, 5 ignored (1,03 s); 4 linhas de aviso em `workforce/foreman.rs` (`:21` import `Mutex`, `:150` `fn distance`) | 325 passed, 0 failed, 9 ignored; 3 avisos (`foreman.rs:149`, `site.rs:142`, `staff.rs:124`) | `main_check_rust.log`, `pr47_check_rust.log` |
| X3 | `python3 tools/check_performance.py --output SP/a/logs/<r>_perf.json` | 7 cenários, 3 amostras, ok | 7 cenários, ok | `*_check_perf.log`, `*_perf.json` |
| X4 | `cargo test` do launcher | 2 ok (Cargo 0.10.0) | 6 ok (Cargo 0.16.1) | `*_launcher.log` |
| X5 | `cargo test -- --ignored bench_factory` (×4) | 275,5/351,6/268,3/253,3 µs/tick; 936 aloc./tick | 380,9/389,2/391,7/348,7 µs/tick; 943 aloc./tick | `bench_repeat.log`, `*_bench_factory.log` |
| X6 | `… bench_frame` | 41–49 µs, 0 alocações | 59–82 µs, 329 alocações | `*_bench_frame.log` |
| X7 | `… bench_large_village` | 3,4 µs/tick (150 aldeões + 300 animais) | **panic** "not implemented: world_dir" via `onboarding::session` | `*_bench_large_village.log` |
| X8 | `… --ignored onboarding --test-threads=1` | — | 3/3 ok | `pr47_ignored_onboarding.log` |
| X9 | `… --release stations_cost` | — | "1000 estações descarregadas: 194.373µs por passo de 20 ticks" | `pr47_stations_cost.log` |
| X10 | `python3 tools/check_machine_audio.py` | — | "30 licensed spatial sound files checked" | `pr47_check_audio.log` |
| X11 | `python3 tools/check_machine_forms.py` | — | "34 machine forms… 27 distinct silhouettes" | `pr47_check_forms.log` |
| X12 | Todos os `tools/build_*.py` em cópias (`SP/a/tmp`) + diff contra a árvore | só `belt_rail.png`, `fast_belt_rail.png` diferem (pixels iguais) | só os 30 `.ogg` diferem | (saída no transcript) |
| X13 | `cargo` com rede | Não precisou de rede: dependências do stub locais; nenhuma falha de rede registrada | idem | — |

`git status` limpo nas duas árvores após as execuções (`target/` é ignorado).

## 2. Código (E2) — arquivos-chave e o que provam

| Arquivo | Ramo | Linhas | Prova |
|---|---|---|---|
| `native/api/src/lib.rs` | main | 872 | 16 módulos, 23 ganchos, `tick` `:232`, `tick_factory` `:735` |
| `native/api/src/lib.rs` | pr47 | 1255 | 29 módulos, 33 ganchos, `server_about_to_start` `:89`, `chunk_load` `:288`, `tick` `:326`, grande projeto `:1101` |
| `native/offline-check/horizon-client/src/mod_api.rs` (stub) | main/pr47 | 955/1090 | 44/52 métodos de `Host` |
| `native/api/Cargo.toml` | ambos | — | Horizon por `git` sem `rev` |
| `workforce/society/{mod,ledger,market,trade,mayor}.rs` | ambos (idênticos) | — | Economia (A2 §Economia) |
| `workforce/center.rs` | pr47 | 1052 | `reward` `:609` (decisão 1), `take_treasury` `:395`, `cash_out` `:419` |
| `home/mod.rs`, `home/food.rs` | pr47 | 424/— | `time_factor` `:101` sem bônus de casa (decisão 8); atomics `food.rs:28-29` |
| `tools/build_house.py` | pr47 | — | `:177-178` Lar como lareira da casa (decisão 8) |
| `production.json` | ambos | — | 53 receitas da main idênticas no pr47 (decisão 9) |
| `trade.rs` `STATION`, bloco `rail_terminal` | pr47 | — | Decisão 6 |
| `stations.rs` | pr47 | 384 | Produção virtual `simulate` `:117-140` sem insumo/energia (decisão 2) |
| `industry.rs` | pr47 | 399 | Âncora `forceload` `:285`, `on_server` `:106`, monitor itens/min (decisão 3) |
| `rivers.rs` | pr47 | 849 | Constantes `:26-50`, `chunk_loaded` `:758`, `rios.txt` `:769` |
| `research.rs` | pr47 | — | 4 tópicos `:22-26`, `advancement grant @a` `:166`, `pesquisa.txt` `:45` |
| `milestones/mod.rs` | pr47 | — | Fases `:37-52`, texto "Libera" `:246`, pedidos `:56-69`, esmeraldas no slot 1 `:147-150` |
| `contracts.rs` | ambos | — | Contratos (main 16/8, 12/24, 1/32; pr47 24/12, 24/32, 1/48), EXPORT `:63-66`, fases `:35-49` |
| `upgrades.rs` | ambos | — | Produtividade 10 % (main `:31`) × 20 % (pr47 `:35`); Mk.II `:52` |
| `production/extraction.rs` | ambos | — | Tiers `:21-23`; pureza main `:26-28` × pr47 `:34-57` |
| `content/data/veio/advancement/**` | ambos | — | steam_engine etc.: main `steel.json`/`circuits.json`/`upgrades.json`; pr47 `research/*.json` |
| `content/data/veio/horizon/menu/site.json` | pr47 | — | `slots: 1` × código 3 (`workforce/mod.rs:150`) |
| `visuals/veio-industrial/horizon-shaders.toml` | pr47 | — | `api = "0.1"` |
| `launcher/src/main.rs`, `visuals.rs` | pr47 | — | Perfil `performance` `:26`, visual `:23-32`, `HORIZON_SHADER_PACK` `:95` |
| Horizon `native-hooks.json`, `game/native_*.rs`, `horizon-shader-api/src/manifest.rs` | 66f3981 / 5f7cadf | — | Ganchos e métodos existem; shader 0.1 aceito (`manifest.rs:611-626`) |

Confiabilidade: **Alta** para existência e números; **ND** para comportamento em jogo.

## 3. Documentos (E3) — data, escopo e confiabilidade

| Documento | Ramo | Linhas | Último commit (autor, data) | Escopo | Confiabilidade / divergências |
|---|---|---|---|---|---|
| `README.md` | main | 244 | 87ee998 Claude, 08/11 17:29Z | Visão 0.25.0 | Média |
| `README.md` | pr47 | 338 | 5c57405 Claude, 19:29Z | Candidata | **Baixa** em partes: l.9 "Candidata ainda em integração"; l.21 casa do fundador "rendem 15% mais" (contradiz decisão 8 e `home::time_factor`); l.86 `release/Veio-0.16.1.zip` (não existe); l.31 "entregas da 0.11.0 à 0.25.0 aguardam teste no jogo" (correto) |
| `TRABALHADORES.md` | main / pr47 | 413 / 684 | 87ee998 / 5c57405 | Diário técnico Cloud | Média; pr47 l.234 estados do canteiro 32–35 (código 36–39); l.131 "30 entradas", l.589 "26 estados" (código 44) — histórico não atualizado |
| `CONSOLIDACAO.md` | pr47 | 64 | 50995e6 Codex, 15:11 −03 | Consolidação Codex 0.16.1 | **Baixa**: 558 JSON, 68 texturas, 248 testes (hoje 742/89/325); "compilação nativa 0.16.1 sobre 66f3981 em 7 min 14 s"; "O jogo não foi aberto" |
| `HANDOFF_OBRAS.md` | pr47 | 43 | f12f980 Claude, 17:25Z | Pendências das obras | Alta (todas as pendências conferidas abertas, A2 SIS-OBR) |
| `MAPA.md` | pr47 | 161 | 80980db Codex, 19:29Z | Mapa finito 15 000×15 000, muralha de bedrock | Média como intenção; "Decisão de produto de 7 de novembro" = decisão de agente |
| `work-in-progress/finite-world/CONTINUAR.md` | pr47 | 115 | 80980db | Estado do mapa finito | Baixa em versão ("bundle ativo… 0.16.1"); útil para o checklist técnico (22/24 injeções; Nether falhou) |
| `COORDENACAO.md` | main / pr47 | 86 / 123 | 8e4d611 / 4ffb605 Codex | Coordenação entre frentes | Média/Baixa (pr47 parou na 0.16.1 do Codex) |
| `PERFORMANCE.md` | ambos | 121 | 13758bd Codex, 08/11 02:05 −03 | Otimizações | Média; não cobre sistemas do pr47 |
| `LOGISTICA.md` | ambos | 51 | 26599ec Codex, 07/11 | Esteiras/fusor | Média |
| `TRANSPORTES.md` | pr47 | 213 | 8b8e8d9 Jefter, 08/11 00:00Z | Ferrovia/drones/caravanas | Média; anterior ao rename `rail_terminal` (a79f326) |
| `VISUAL-MAQUINAS.md` | pr47 | 90 | d396f69 Jefter | Silhuetas | Média |
| `CONTRIBUTING.md`, `CREDITS.md` | pr47 | 86 / 23 | 5c57405 | Regras e créditos (Kenney CC0) | Média |
| `frentes/CHECKPOINT-INTEGRACAO.md` | pr47 | 44 | 50995e6 Codex | Estado da integração Codex | Média: "cache incoerente impediu verificação de API; candidata não compilada" |
| `frentes/branches-consolidadas.json` | pr47 | 175 | f99f66a Codex, 10:36 −03 | 29 referências | Baixa (anterior a #40, obras, sons) |
| `frentes/DESEMPENHO.md` | ambos | 80 | 494d3e7 Claude | 0.17.x: quadro 1223 → **0** alocações, tick 1151 → 936 | Alta para main (X6 confirma 0 e 936); falsa para pr47 (329) |
| `frentes/PROGRESSAO-FASES.md` | pr47 | 144 | fe30583 Claude (MineFac) | Progressão por fases do Codex (30/min) | **Baixa**: tempos substituídos pela decisão 9 |
| `frentes/AGUA-E-RIOS.md`, `GRANDE-INDUSTRIA.md` | pr47 | 83 / 121 | 46152c8 / e36ee2e Claude | #40 | Média; APIs novas declaradas não verificadas |
| `frentes/ECONOMIA.md`, `SOCIEDADE.md` | ambos | 122 / 168 | 87ee998 Claude | Economia 0.25 | Média-alta (código idêntico) |
| `frentes/AUDIO-MAQUINAS.md`, `AMBIENTE-E-MOVIMENTO.md` | pr47 | 101 / 109 | 7024938 Jefter | Som/animação | Média |
| `frentes/MEGAFABRICAS.md` | pr47 | 136 | 2720da4 Codex | Megafábricas | Média |
| `frentes/JAZIDAS-CONECTADAS.md` + `prototipos/jazidas_conectadas.py` | pr47 | 206 | 2edc6c0 Jefter | Pesquisa (#23) | Média como pesquisa |
| `frentes/GROK-LOTE*.md`, `GROK*.md`, `CODEX*.md`, `CLOUD.md`, `VILA-CIDADE.md`, `INTERFACE_POSTOS.md`, `ORCAMENTO-GLOBAL.md`, `REDES-INCREMENTAIS.md`, `CAPATAZES-*.md`, `CARREGADORES-*.md`, `ESCALA-E-ENERGIA.md`, `BALANCEAMENTO-*.md`, `EXPERIENCIA-JOGADOR.md`, `CASA-FUNDADOR-INTEGRACAO.md` | pr47 (21 na main) | 21–130 | 07/11–08/11 | Diários por frente | Média; históricos |
| `visuals/veio-industrial/README.md` | pr47 | 65 | 3106849 Codex | Shader 1.1 | Média |

## 4. Testes por módulo (E1)
Ver A1 §9. Totais: main 176/0/5; pr47 325/0/9. Ignorados main: preview, bench_factory, bench_frame, profile_scale, bench_large_village. pr47: os mesmos + 3 de onboarding + stations_cost.

## 5. Scripts

| Script | Ramo | Função | Executado? |
|---|---|---|---|
| `tools/check_project.py` | ambos | JSON, traduções, páginas, texturas, `sim_economy` | Sim (X1) |
| `tools/check_rust.py` | ambos | `cargo test` offline | Sim (X2) |
| `tools/check_performance.py` | ambos | 7 cenários `profile_scale` | Sim (X3, com `--output` em `SP/a/logs`) |
| `tools/check_native_api.py` | ambos | Confere assinaturas contra `.rmeta` real | Não (sem Horizon compilado) |
| `tools/check_machine_audio.py`, `check_machine_forms.py` | pr47 | Sons e silhuetas | Sim (X10, X11) |
| `tools/build_*.py` (6 na main, 12 no pr47) | ambos | Geradores | Sim, em cópia (X12) |
| `tools/package.py` | ambos | Monta zip | Não (escreveria em `dist/`) |
| `tools/sim_economy.py` | ambos (idêntico) | 7 invariantes | Sim via X1 |
| `tools/benchmark_factory.py`, `prepare_world_map_check.py`, `world_map_probe.rs`, `preview_*` | pr47 | Apoio | Não |

## 6. Commits e PRs de referência

| Ref. | O que prova | Confiabilidade |
|---|---|---|
| 87ee998 (#48) | 0.25.0 na main | Alta |
| 80980db | Head do pr47; patch-id = c8e94dc | Alta |
| 5c57405 / eedfd47 | Merge da consolidação Codex e do #40 no pr47 | Alta |
| a79f326 | Rename para `veio:rail_terminal` (decisão 6) | Alta |
| f1452f5 | Build parcial de obras = upload do usuário | Alta (67 arquivos idênticos) |
| a3303bb / 4ffb605 | "Veio 0.16.1" do Codex e alegação de compilação nativa | E3 (Média) |
| 008a980 | Lote 2 do Grok registrado como substituído sem reaplicar código | Alta |
| c8c43c5 | Range-diff das importações triplicadas | Média |
| bcfb9d5 | Entrada de `read_columns`/`chunk_load` no stub (depois da compilação do Codex) | Alta |
| c1b3ba5, fd2a446, d1e2f30, 90721ca | Origem dos demais métodos novos do stub | Alta |
| 9c94c55 | "Quadro e tick sem alocações" | Alta para main |
| PR #51, #52 | Trabalho novo sobre o pr47 (20:34–20:35Z) | Só metadados REST (E3) |
| PRs #1–#48 fundidos | Todos com base `main` | Alta |
