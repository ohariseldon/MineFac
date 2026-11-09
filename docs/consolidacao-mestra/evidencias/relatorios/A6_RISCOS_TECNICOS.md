# A6 — Riscos técnicos

Autor: OPUS A. Data: 2026-11-08. Severidade: **Crítica** (impede jogar/lançar ou corrompe save) · **Alta** (quebra sistema visível ou invalida a candidata) · **Média** (comportamento errado ou dívida que cresce) · **Baixa** (clareza, cosmético).
Mitigações são sugestões de agente (não decisões do dono).

## 1. APIs do Horizon e compilação

| ID | Risco | Evidência | Sev. | Mitigação sugerida |
|---|---|---|---|---|
| R-01 | Nenhuma compilação do pr47 contra o Horizon real; `read_columns` e `chunk_load` (rios) entraram depois da única compilação alegada (4ffb605, 0.16.1) | E2 git log -S (bcfb9d5); E3 CONSOLIDACAO; `check_native_api.py` sem rmeta | **Crítica** | `horizon-launcher native-build` numa máquina com Horizon antes de fundir o #47 |
| R-02 | Dependência `horizon-client` por `git` sem `rev` e sem Cargo.lock: cada build pega um Horizon diferente (66f3981 do stub × 5f7cadf atual) | E2 `native/api/Cargo.toml` | Alta | Fixar `rev` e registrar o hash testado no README |
| R-03 | Assinatura de `set_block_property` difere (real tem `&mut Ticks`, `flags: i32`) | E2 comparação stub × `native_*.rs` | Média (ND se o trait gerado adapta) | Conferir com `check_native_api.py` |
| R-04 | Semântica não verificada: `read_columns` em chunk recém-gerado, `edit_blocks` dentro de `chunk_load`, `run_command("forceload …")`, `advancement grant @a`, `place_block` da copiadora | E2 (uso) + ND (comportamento) | Alta | Teste em partida com roteiro curto |
| R-05 | Âncora usa comando `forceload` em vez da API de tickets do Horizon (`native_chunk_tickets.rs`) | E2 `industry.rs:285`; Horizon | Média | Avaliar migração para tickets (sem comando, sem permissão de operador) |
| R-06 | Mapa finito depende de mixins de motor (22/24 injeções), fora da API de mods; quebra a cada mudança do Horizon | E3 CONTINUAR.md | Alta (para a frente do mapa) | Manter desligado até haver API; preferir limite por borda do mundo |
| R-07 | Shader `api = "0.1"` aceito pelo manifesto do Horizon 5f7cadf, mas WGSL não validado; PR #51 já reescreve o pacote | E2 `manifest.rs:611-626` | Média | Rodar `horizon-shader check` (ou `tools/check_shaders.py` do #51) |

## 2. Save e compatibilidade

| ID | Risco | Evidência | Sev. | Mitigação |
|---|---|---|---|---|
| R-08 | Arquivos `<mundo>/veio/*.txt` sem número de versão; `estacoes.txt`, `ancoras.txt`, `pesquisa.txt` gravados sem arquivo temporário (queda no meio = arquivo truncado) | E2 (`stations.rs:207`, `industry.rs:260`, `research.rs:45`; rios e tutorial usam temporário) | Alta | Cabeçalho de versão + gravação atômica comum (SIS-PER) |
| R-09 | Estado em memória perdido ao reiniciar: poluição, contas da holding, `BORDERS`, CONNECTIONS, PORTS dos drones, bônus de refeição, relógio do mod (`defense::CLOCK` começa em 0 a cada processo) | E2 `defense.rs:21-45`, `ledger.rs:48`, `home/food.rs:28-29` | Média | Persistir o que é indicador visível (holding, poluição) |
| R-10 | Reset entre mundos parcial: `server_about_to_start` (pr47) limpa só home, census, blueprint; os demais dependem de detectar `world_dir`; na main não há reset | E2 `lib.rs:89` (pr47) | Média | Um reset central de todos os `static` |
| R-11 | Save da main 0.25 aberto no pr47: todos os 55 blocos e 16 itens da main existem no pr47 (E2, conjunto contido); NBT novo ("Water", "Home") nasce vazio; mudança de liberação de receitas por avanço (steel → research) altera progresso de mundos existentes | E2 | Média | Nota de migração; teste de abrir um mundo 0.25 |
| R-12 | Rios só em chunks novos: mundo antigo nunca recebe rios perto da base | E2 `rivers.rs:758` (`new`) | Baixa | Documentar |
| R-13 | Canais de estação globais entre dimensões; produção virtual cria itens sem insumo/energia e continua mesmo se o posto foi desmontado enquanto descarregado | E2 `stations.rs:117-140` | Média (balanço/exploração) | Chave (dimensão, canal); cobrar energia virtual; invalidar ao quebrar |

## 3. Desempenho e limites

| ID | Risco | Evidência | Sev. | Mitigação |
|---|---|---|---|---|
| R-14 | `bench_factory` ≈ +40 % por tick no pr47 (~272 → ~385 µs, 480 blocos) | E1 X5 (4 execuções cada) | Alta (prioridade 11 do dono: leveza em PC de 8 GB) | Perfilar `tick_factory` (cadeia de `if/else` de 13 sistemas por bloco) |
| R-15 | `bench_frame` 0 → 329 alocações por quadro, 41–49 → 59–82 µs | E1 X6; `frentes/DESEMPENHO.md` (0 alocações) | Alta | Bisseção entre main e pr47 (suspeitos: `appearance`, `motion`, `soundscape`) — o PR #52 mexe em som/mural |
| R-16 | `bench_large_village` quebrado no pr47 (panic `world_dir`): a escala de 150 aldeões deixou de ser medida | E1 X7 | Alta | Implementar `world_dir` no harness da vila |
| R-17 | Sistemas novos fora de `check_performance`: rios (48 passos/t, 2 edições de chunk/t), água, estações, som, obras, copiadora (6000 blocos) | E2 | Média | Novos cenários |
| R-18 | Guia principal no teto de 100 páginas (`check_project` falha na 101ª); `industry_guide` sem `check_page_fit`; 29 páginas do guia da vila acima de 14 linhas | E1 X1 | Média | Dividir livros; incluir todos no `check_page_fit` |
| R-19 | Cotas de `budget` fixas (`LIMITS`), sem teto global de tempo por tick; mapa sem limite (frente em andamento) | E2 `budget.rs:8`; E4 | Média | Orçamento por tempo medido |

## 4. Inconsistências código × documento/decisão

| ID | Inconsistência | Evidência | Sev. |
|---|---|---|---|
| R-20 | Quadro anuncia que licenças liberam steam_engine/battery/geothermal, drill_mk2/solar/tesla, drill_mk3; no pr47 isso vem de `research/*.json` | E2 `milestones/mod.rs:37-52,246`; advancements | Alta (jogador recebe licença e nada libera) |
| R-21 | Bloqueio de receitas depende de avanços, mas nenhum `doLimitedCrafting` é definido → no vanilla as receitas continuam fabricáveis | E2 (grep vazio nos dois ramos); comportamento do Horizon ND | Alta (progressão inteira) |
| R-22 | README pr47 l.21: casa do fundador "rendem 15% mais" — código não dá bônus de casa (decisão 8) | E2 `home/mod.rs:101` | Média |
| R-23 | README pr47 l.86 manda usar `release/Veio-0.16.1.zip` (inexistente); `release/Veio-Realismo-1.1.zip` contém Veio 0.16.1 | E2 | Alta (jogador instala código antigo) |
| R-24 | `launcher.exe` do pr47 é o binário 0.16.1 do Codex; `check_project` não confere versão do launcher | E2 | Média |
| R-25 | `TRABALHADORES.md:234` estados do canteiro 32–35 (código 36–39); `:589` "26 estados" e `:131` "30 entradas" (código 44) | E2 | Baixa |
| R-26 | `CONSOLIDACAO.md` números da 0.16.1 (558 JSON, 68 texturas, 248 testes) e `frentes/PROGRESSAO-FASES.md` (tempos por fase) apresentados como estado atual | E2 × E1 | Baixa |
| R-27 | Produtividade 10 % → 20 % no pr47 (Grok/Jefter c5db4f6) sem decisão explícita do dono | E2 `upgrades.rs:35` | Média (balanço) |
| R-28 | Economia "fechada" com emissões fora do ledger: EXPORT ilimitado (main e pr47), contrato village, grande projeto, pedido do dia; `sim_economy.py` não as modela | E2 `contracts.rs:63-66`, `milestones/mod.rs:147-150` | Média |
| R-29 | Duas pesquisas (laboratório × vila) com chave `veio.research.done` de aridade diferente | E2 `staff.rs:391`, `research.rs` | Média (texto errado na tela) |
| R-30 | Menu do canteiro `slots: 1` × código 3 slots; ~25 traduções faltando das obras | E2 | Alta (obra sem insumo visível) |

## 5. Dívidas técnicas

| ID | Dívida | Evidência | Sev. |
|---|---|---|---|
| R-31 | Sem CI: nenhuma das verificações roda automaticamente em PR | E2 | Alta |
| R-32 | `tick_factory` como cadeia de `if/else` por id de bloco (13 destinos no pr47) | E2 `lib.rs:1093` | Média |
| R-33 | Acoplamento cíclico `workforce ↔ presentation` (39/23) e `workforce ↔ holding` (20/25); `defense` é dono do relógio global | E2 contagem de referências | Média |
| R-34 | Códigos de status numéricos sem registro central (40/41 colidem entre `freight` e `center`) | E2 `freight.rs:15-16`, `center.rs:39-40` | Baixa |
| R-35 | Código morto (`sites_near`, `research_percent`, `distance`; import `Mutex` não usado na main) | E1 avisos | Baixa |
| R-36 | Testes de onboarding dependem de execução serial (`--test-threads=1`) | E1 X8 | Baixa |
| R-37 | Importações de patch triplicadas no histórico do pr47 (0ce91a4/0be045c/823d626 etc.) dificultam `bisect` e `blame` | E2 `git log` | Baixa |
| R-38 | `sim_economy.py` duplica constantes do Rust à mão | E2 | Média |
| R-39 | Versões colidem entre frentes (0.16.0, 0.16.1, 0.18–0.23, agora 0.26.0 no #51) | E2/REST | Média (rastreabilidade) |
| R-40 | `launcher.exe` versionado no git apesar de `package.py` dizer que é gerado | E2 | Baixa |

## 6. Processo e escopo

| ID | Risco | Evidência | Sev. |
|---|---|---|---|
| R-41 | Nada da 0.11.0 à 0.25.0 foi testado em partida (README pr47 l.31) | E3 | **Crítica** para "build consolidada" |
| R-42 | Trabalho novo empilhado sobre o pr47 antes da revisão (#51 com 53 arquivos e versão 0.26.0; #52) | REST | Alta (o alvo do #47 continua a mudar) |
| R-43 | Bônus/pesquisa/grande projeto globais (`@a`, atomics) em LAN: um jogador muda o estado de todos | E2 | Baixa (coop) |

## 7. Ranking (dez mais urgentes)
1. R-01 compilação real ausente (Crítica).
2. R-41 nenhum teste em partida (Crítica).
3. R-21 bloqueio de receitas possivelmente inócuo.
4. R-20 licenças do quadro que não liberam nada no pr47.
5. R-14/R-15/R-16 regressões de desempenho e benchmark quebrado.
6. R-23 pacote/README apontando para 0.16.1.
7. R-30 menu do canteiro e traduções das obras.
8. R-08 persistência sem versão nem atomicidade.
9. R-02 Horizon não fixado + R-31 sem CI.
10. R-42 PRs #51/#52 empilhados sobre a candidata.
