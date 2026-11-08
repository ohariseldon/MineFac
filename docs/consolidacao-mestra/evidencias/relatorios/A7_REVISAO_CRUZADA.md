# A7: Revisão cruzada do trabalho do OPUS B pelo OPUS A

- **Autor:** OPUS A. **Data:** 2026-11-08, ~21h30 UTC.
- **Bases:** main `53267af` (0.25.0) e pr47 `80980db`. Os PRs #51 (`a0b0d72`) e #52 (`0b827f6`) só aparecem no §3.
- **Insumos:** `SP/b/B1`–`B7`, na versão atual. B2 e B3 já trazem as marcas "[B7]", que absorvem parte do A1–A6. Esta revisão confere o texto como está agora.
- **Linhas citadas:** salvo indicação, `arquivo:linha` é do pr47 `80980db`, com caminhos relativos a `native/api/src/`.
- **Logs:** novos neste A7, em `SP/a/logs/`:
  - `pr51_52_bench_release.log`
  - `bench_interleaved.log`

---

## 0. Autocorreções do A (erros meus que o B herdou ou pode herdar)

| Onde | O que eu escrevi | Correto | Evidência |
|---|---|---|---|
| A1:89, A2:158 SIS-MEG, A3 FUN-MEG-03 | Tratei a "prancheta" como `blueprint_designer` e pus a FUN-MEG-03 em MEG, vinda de `import-megafactories` | `blueprint_designer` é a **Copiadora**. A **Prancheta** é o item `veio:clipboard`, que copia filtro e prioridade de braço, divisor e unificador. Veio do #40. **Não existe na main** | E2: lang pt_br `:1402` ("Prancheta"), `:1447`; `content/data/veio/recipe/clipboard.json`; `frentes/GRANDE-INDUSTRIA.md:18,110`; ausente em `git ls-tree 53267af` |
| A2:49 | Combustíveis do gerador: carvão 1600 etc., lidos como energia | Os valores estão em **ticks de fornalha**. Para ter FE: gerador ×10 (carvão = 16 000 FE); caldeira ×20 | E2: `production/generation.rs:14-20` (`burner` 40/10; caldeira 120/20) |

**Correção de ID:** a FUN-MEG-03 vai para **FUN-LOG-07 "Prancheta (copiar configuração)"**, com alias MEG-03 e a MEG-04 do B. Estado: main ausente, pr47 Integrado (E2). No B7 §1.4, a linha "MEG-04 Prancheta = MEG-03" deve apontar para LOG-07.

---

## 1. As 15 confirmações pedidas

| # | Pergunta | Resposta | Evidência | Efeito no B |
|---|---|---|---|---|
| 1 | Ondas no pr47 = 0.17? Raio 26 × 64 | **Sim. Arco a 26 blocos.** Constantes iguais às da main: NOTICE 400, RAID 1200, MAX_WAVE 12, saqueadores 4 a 12, arqueiros a partir da 3ª noite, brutos a partir da 5ª, uma onda por noite. O pr47 traz uma diferença: se a onda não gerar nenhum mob, ela não conta. O "64 blocos / 18 blocos" está **só no README do pr47** | E2: `defense/pollution.rs:28-47` (`ARRIVAL 26.0` em `:47`), `:220`, `:236-238`; `raider_brute.json:4` (40 de vida); README pr47 `:307` × README main `:213` | FUN-DEF-03 está correta. Falta registrar a regressão do README do pr47 |
| 2 | `data.rs` regenerado com prensa 30t e montador 40t? | **Sim**, e sem divergência em relação ao JSON | E2: `production/data.rs:5-6` (prensa 30t, 8 FE), `:10` (montador 40t, 12 FE). E1: `build_production` regenera o arquivo igual | DEC-045 "ok no `data.rs`" |
| 3 | Fontes de esmeralda fora do livro | **Todas confirmadas.** EXPORT da central dá 4 esmeraldas, sem limite, também na main. Contrato Povoado: 8. Grande projeto: 32/48/64. Armazém: PRICE 60, +50% com 3 aldeões. Pedido do dia do Quadro. Mk.II em esmeralda: 400 t = 20 s na pureza normal, 10 s na pura, 4 800 FE. A esmeralda só entra no livro quando depositada no cofre | E2: `contracts.rs:23`, `:35-49`, `:63-66`; `production/pantry.rs:1-21`; `milestones/mod.rs:147-150` (`paid_day` em memória, `census.rs:114`); `tools/production.json:90` (slow 20) + `extraction.rs:22` (Mk.II 20t, 12 FE); `ledger.rs:192-200 take_emeralds` | B3 §3.5 correto. Precisão: até 1 esmeralda a cada 10 s em jazida pura |
| 4 | "Caixa da vila" é o livro Public? | **Não. É o livro Company** (NBT "Company" do centro). Esmeraldas no slot SITE que nenhuma obra pediu vão para ele. As obras pagam com ele (`add_coins` negativo, REDEEM): **moedas destruídas**, não transferidas. Juros = (caixa/16).min(8) com Prefeitura, pagos pelo **Public** via `reward` (Public → Company) | E2: `workforce/center.rs:256-264`, `:395 take_treasury`, `:419 cash_out` (até 16), `:592-601 add_coins`, `:609-620 reward`; `projects.rs:475-476` | B3 3.10n: caixa = Company. Trocar "caixa público" por "caixa da companhia" onde for o caso |
| 5 | Códigos 40/41: ambiguidade com efeito? | **Sem efeito hoje.** `waiting()` só aceita {3, NO_WORKER, ABSENT, NO_FOOD, PATH, NO_WAGE, SITE_BLOCKED}. Os status 40–43 de frete nunca viram pendência, então `problem_text`, `advice` e `focus::resolved` só recebem 40/41 de zona ou armazém. **Risco latente**: se um status de frete entrar na lista, aparece com o texto de zona | E2: `center.rs:39-40`, `:111-113`, `:193-205`; `freight` `:15-16` (NO_ROUTE/NO_ANIMAL); `focus.rs:110-118` | B3 3.10b: passar de "possível ambiguidade" para "latente, sem efeito" |
| 6 | "Cama livre" ativa? | **Sim.** Teto = min(desenvolvimento, camas); a cabeceira conta. Desenvolvimento = 30 + 3/posto com trabalhador + 5/nível de zona + 2/armazém + 20/centro + `extra_cap` das obras, até 150. O nascimento exige ainda a fase, a Prefeitura não suspensa e vaga em casa com alvará | E2: `workforce/village.rs:50-51`, `:104-105`, `:121-129`, `:153-154`; `society/mod.rs:328-341 birth_allowed` | FUN-TRB-09: acrescentar `extra_cap` (obras) e a festa na chance de nascimento (`village.rs:136-146`) |
| 7 | Licenças × pesquisa: bloqueio mútuo? | **Não.** Os avanços engineering, steel e circuits pedem `[["stock","permit"]]`, ou seja, a pilha OU a licença. `research/*.json` usam o gatilho `impossible`, concedido pelo código. `research.rs` não lê licenças, e o Quadro não lê a pesquisa. O texto "Libera" do Quadro promete itens que vêm da pesquisa. Não há `doLimitedCrafting` | E2: `content/data/veio/advancement/{engineering,steel,circuits}.json`; `research.rs:166`; `milestones/mod.rs:37-52`, `:246` | B2 FUN-PRG-01/02 corretos (conferi as receitas da main e do pr47) |
| 8 | Staff e `research_bit` mudam o jogo? Serviços das obras no IDH? | **Mudam.** NUTRITION: −1 em `food_cost`. CARGO: +8 em `haul_batch`. AGRONOMY: +1 em `zone_credit`. PEDAGOGY dobra o crescimento do staff. ENGINEERING atua no cientista. **O IDH não inclui as obras**: serviços = (saneamento + energia + comércio) × 100/3, e `society/` não referencia `projects` | E2: `projects.rs:63`, `:230`, `:418`, `:436`, `:466`; `staff.rs:232`, `:271`; `society/mod.rs:613` | B2 SIS-VIL F.2 e SIS-OBR F.5: **não integrado** (Concebido) |
| 9 | Produção virtual consome insumos e energia? | **Não.** `simulate` soma ritmo × tempo. Os canais não levam a dimensão | E2: `production/stations.rs:117-140`; `lib.rs:550-551` (4 slots) | Já absorvido no B7 |
| 10 | Fatores 0,95 e 0,91 são do Lar? | **Não, é o contrário.** 0,95 (6 000 t) e 0,91 (9 600 t) só valem **comendo fora de casa**: dentro da casa, `eaten` devolve `None`. O Lar dá **efeitos ao jogador** (saturação, regeneração, pressa 180 s × poder, resistência, visão noturna, absorção; o banquete dá `hero_of_the_village`) e **não mexe** no ritmo das máquinas. O comentário de `food.rs:9` ("comer dentro de casa dobra a duração") está obsoleto | E2: `home/food.rs:22-29`, `:59-69`; `home/mod.rs:99-103`; `upgrades.rs:44`; `lib.rs:316-323`; `workforce/home.rs:555`, `:601`, `:642`, `:693`; teste `home/tests.rs:163,169-174` | **Corrigir FUN-LAR-03, DEC-046 e §3.1** (ver §2) |
| 11 | Mundo salvo com a `freight_station` antiga do Codex | **Sem migração.** No Codex (`50995e6`), `veio:freight_station` era o terminal ferroviário: 6 slots, saídas 0..3. O `a79f326` o renomeou para `veio:rail_terminal`, e o ID `veio:freight_station` passou a ser a **estação de exportação** (9 slots de entrada, só produtos). Num mundo da 0.16.1 do Codex, inclusive os feitos com `Veio-Realismo-1.1.zip`, o terminal vira estação de exportação, e a carga que for produto **é exportada** | E2: `freight.rs:7@50995e6`; `lib.rs:112@50995e6`; `society/trade.rs:27`; `society/mod.rs:104,117,415`; `git log -S` → `a79f326` | B2 TRA F.3 "migração ND" → **determinado** |
| 12 | `chunk_load`, `read_columns`, `world_dir`, forceload | `chunk_load` (gancho) existe nos `native-hooks.json` 66f3981 e 5f7cadf. `read_columns` e `world_dir` existem como `native_*`. `read_columns` e `chunk_load` entraram em `bcfb9d5`, depois de `4ffb605`: **nunca compilados**. `world_dir` (`c1b3ba5`) é anterior e provavelmente compilou na 0.16.1 (E3). **forceload não é API**: é `run_command`. A alternativa é `native_chunk_tickets.rs` | E2: `SP/a/horizon_ref`; `industry.rs:285-291` | — |
| 13 | Stamp do launcher difere? | **Sim.** main: `built.trim() != version`. pr47: `"{version}\nprofile={NATIVE_PROFILE}"`. Trocar de camada força recompilação. Cargo do launcher: 0.10.0 na main, 0.16.1 no pr47 | E2: `launcher/src/main.rs:52@main`, `:56-57@pr47` | FUN-PLT-01 correta |
| 14 | PIB em `mayor.rs` | **PIB = varejo do dia (DSold) + exportações (DExport).** Só promove com prefeito. Suspende quando IDH + 5 < exigido. O relatório do jogo diz "consumo + exportação"; o guia diz "tarefas feitas e vendas" (errado) | E2: `society/mayor.rs:187-189`, `:198`; lang `:1202` × `:1720` | **Corrigir B2 SIS-VIL e B3 3.10f** |
| 15 | Melhoria de zona pelo livro? | **pr47: em parte.** Paga primeiro com o caixa (Company → `add_coins` negativo: moedas destruídas). O resto sai de esmeraldas físicas do inventário. Se falhar, `return_treasury` devolve. Mercado dá −25%. **main: só esmeraldas físicas** | E2: `zones.rs:256-268`; `projects.rs:460`; `zones.rs:249@main` | B2 SIS-AGR F.1 respondida |

---

## 2. Afirmações de B2/B3 contraditas pelo código ou sem sustentação nele

Lista apenas o que **continua errado** no texto atual. O que o B7 já corrigiu ficou de fora: MUN-01 (exceto o resumo), PRO-03, TRA-05, ECO-09, PRG-02 e a parte global do LAR-03.

| ID (SIS/FUN/DEC) | Afirmação do B | O que o código faz | Evidência | Correção proposta |
|---|---|---|---|---|
| SIS-PLT F.3 (B2:99); B3 §1.4.5 item 5 (B3:99) | "Os dois `Veio-0.25.0.zip` são diferentes entre as camadas (E2: blobs diferentes)" | **São idênticos.** O pr47 carrega o zip da main: não há pacote do conteúdo do pr47 com esse nome | E1: `git rev-parse 53267af:release/Veio-0.25.0.zip` = `80980db:…` = `e01ef740…`; sha256 `7dc57f01…` | Trocar por "o zip 0.25.0 do pr47 **é o da main**; o pr47 não tem pacote próprio (os Realismo-1.1 são da 0.16.1)". Mantém o risco do jogador instalar a build errada |
| FUN-LAR-03 (B2:542); DEC-046 (B3:174); B3 §3.1 | "Ritmo só por refeição e banquete do Lar (`time_factor = food::time_factor`)"; "bônus de ritmo só pelo Lar" | **Invertido.** `food::time_factor` é o bônus de **comer fora de casa**. Dentro da casa ele não vale. O Lar dá efeitos ao jogador e não toca no ritmo | E2: `home/food.rs:59-69`; `home/mod.rs:99-103`; `workforce/home.rs:555-693`; teste `home/tests.rs:163` ("estar em casa não acelera as máquinas") | "Ritmo das máquinas: só comendo fora de casa (×0,95 por 6 000 t; ×0,91 por 9 600 t; global). O Lar dá efeitos ao jogador (pressa, regeneração etc.), não ritmo." A DEC-046 fica **cumprida**, se "bônus de casa" quer dizer os efeitos do Lar. Anotar o comentário obsoleto de `food.rs:9` |
| DEC-030 (B3:153); B2 SIS-ECO A | "Moeda só entra ou sai por esmeraldas físicas" | O código tem **4 bordas sem esmeralda**: exportação cria moeda (+); compra do jogador (+); importação e frete destroem (−). Zona e obras também destroem (`add_coins` negativo). O "princípio" só vale como texto do README | E2: `society/ledger.rs:159-167` (o comentário lista as bordas), `:199`, `:243`; `society/trade.rs:336-337`, `:365`, `:598`; `society/market.rs:464`; `center.rs:592-601` | Reescrever: "dinheiro circula entre 4 livros; entra e sai por bordas contadas em `BORDERS` (esmeralda, exportação, importação, frete, compras, pagamentos destruídos)". B2 SIS-ECO B já lista parte das bordas (linha 20): alinhar com a DEC-030 |
| SIS-VIL "PIB" (B2:386); B3 3.10f (B3:223) | "PIB = consumo final no varejo do último dia" | `gdp = DSold + DExport`: **varejo mais exportações** | E2: `society/mayor.rs:187-188`; lang pt_br `:1202` | Canônico = varejo + exportação. Corrigir só o guia (`veio.village.page.33`, lang `:1720`) e o README/SOCIEDADE |
| FUN-TRA-01 (B2:290) | "1.000 estações ≈ 80 µs a cada 20 ticks" (mantido ao lado do 194 µs) | Medido: **194,4 µs** por passo de 20 ticks. O valor de 80 µs não se reproduz | E1: `SP/a/logs/pr47_stations_cost.log:26` (`--release`) | Apagar o "≈ 80 µs" (E3 sem reprodução) e deixar só os 194 µs (E1) |
| FUN-MEG-04 (B2:686); FUN-MEG-01 (B2:683); B7 §1.4 | "Prancheta (#40): ferramenta de planejamento"; "Copiadora: Prancheta de 5/9/15/21…" | A Prancheta é `veio:clipboard` (copia filtro e prioridade, SIS-LOG). A Copiadora não é uma prancheta | Ver §0 | Mover para **FUN-LOG-07**. Em MEG-01, trocar "Prancheta de" por "área de" |
| Resumo SIS-MUN (B2:34) | Coluna main: "Integrado (só pureza 3D)" | A main mede o **3×3** abaixo da extratora. O 3D conectado só existe no pr47 | E2: `production/extraction.rs:3-4,23-28@main` × `:13-15,34-39,95@pr47` | "Integrado (pureza 3×3)". Hoje contradiz a FUN-MUN-01 já corrigida |
| B3 3.10b (B3:219) | "Possível ambiguidade se um terminal entrar na lista… OPUS A confirmar" | Sem efeito hoje: `waiting()` filtra os códigos | Ver §1 #5 | "Latente, sem efeito (E2)". Correção técnica sugerida: renumerar os status de frete (decisão de agente, não confirmada pelo dono) |
| B3 3.10n (B3:231); B2 SIS-ECO F.2, FUN-OBR-05 (B2:508) | "Caixa da vila × livros: Public, quinto saldo ou esmeraldas?" | É o **Company**. Pagar uma obra destrói moedas | Ver §1 #4 | OBR-05: "caixa = livro Company; obras e zona pagam destruindo moeda (borda −); juros vêm do Public" |
| B2 SIS-VIL F.2 (B2:400); SIS-OBR F.5 (B2:527) | "Integrar os serviços das obras no IDH (está feito?)" | **Não está feito** | E2: `society/mod.rs:613`; `grep projects society/` vazio | Estado: **Concebido** (pendência da HANDOFF/SOCIEDADE) |
| SIS-TRA F.3 (B2:306) | "`freight_station` antiga do Codex: migração ND" | Determinado: sem migração. O terminal vira estação de exportação e exporta a carga | Ver §1 #11 | Estado determinado. Novo risco: mundos 0.16.1 e Realismo 1.1 (ligado ao R-23) |
| FUN-PES-03 (B2:616) | "Agronomia, **Logística**, Nutrição, Pedagogia, Engenharia" | A chave é `cargo`. O jogo mostra `veio.research.{key}.name` e `.effect`, que **não existem** no lang: o jogador vê a chave crua. `veio.research.done` tem 1 argumento no laboratório e recebe 2 na vila | E2: `staff.rs:44-48`, `:62-71`, `:390-392`; lang pt_br `:1406-1416` (só as 4 do laboratório) | Nome provisório "Carga (`cargo`)". Registrar 10 traduções faltando (5 × name/effect) além das ~25 do R-30 |
| FUN-DEF-03 (B2:662) | Correto, mas cita só o README da main | O README do pr47 diz "até 64 blocos… ou 18" | README pr47 `:307` | Registrar como regressão de documentação no pr47 |
| B2 SIS-AGR F.1 (B2:255) | "Melhoria de zona: confirmar se passa pelo livro" | Ver §1 #15 | E2: `zones.rs:256-268` | Fechar a pendência: "pr47: caixa (destrói moeda) e depois esmeraldas; main: só esmeraldas" |

**Conferidas e corretas** (amostra, E2): estas afirmações não precisam mudar.
- ENE: 16 000 FE por carvão; hidrelétrica com queda ≥ 3, superfície ≥ 12 e queda limitada a 32.
- AGU: tanques de 4000/1000/4000/8000.
- LOG: braços 40/64/60.
- PLT: stamp do launcher.
- TUT: tutorial salvo a cada 100 t (`onboarding.rs:403`).
- ECO: holding "Gestão e segurança".
- APR: perfis de realismo (`visuals.rs:122-144`).
- MER:
  - STAPLE = pão (`trade.rs:84`);
  - pressão = recent/2 + hoje (`trade.rs:132-134`).
- OBR:
  - pedidos 3, ou 4 com Correio (`requests.rs:110`);
  - Ferraria 24 (16 + 8);
  - escola, estalagem e prestígio (`projects.rs:440-458`).
- PRG-01 e PRG-02: receitas da main e do pr47 conferem com os JSON.
- TRB-09: números conferem (só falta o `extra_cap`).
- PLT: `native_chunk_generate` ausente no 66f3981.

---

## 3. PRs #51 e #52 (inventário, sem auditoria a fundo)

`git fetch` feito no clone `/home/user/MineFac`. Os dois descendem do pr47 `80980db`: o #51 tem +3 commits, o #52 tem +1.

| PR | Head | Arquivos e sistemas tocados |
|---|---|---|
| #52 "Definir limite do mapa e otimizar" | `0b827f6` | 6 arquivos, +164/−6:<br>• `soundscape.rs` (+22, `near()`: não toca som sem ouvinte; SIS-APR)<br>• `home/mural.rs` (`IDLE_SCAN_EVERY 20` sem espectador; SIS-LAR)<br>• `home/tests.rs`<br>• `perf.rs` (contador `LIVE` de bytes vivos)<br>• `perf/factory.rs` (+34, novo `bench_factory_scale`)<br>• `frentes/DESEMPENHO.md` (+71)<br>Não mexe no stub (`world_dir`) nem na cadeia `tick_factory` e quadro |
| #51 "Aumentar o realismo sem pesar o PC" | `a0b0d72` | 53 arquivos, +2683/−189:<br>• novos `ambience.rs` (238) e `climate.rs` (160)<br>• `lib.rs` +16 (ganchos `visuals` e `shader_params`; `ambience::maintain`/`observe`)<br>• também mexe em `generation.rs`, `production/mod.rs`, `water.rs` e testes, apresentação e `perf.rs`<br>• stub +48 (`fn player()`, `NativePlayer`, `NativeBlockRim`)<br>• versão **0.26.0**, `release/Veio-0.26.0.zip`, 9 shaders e `tools/check_shaders.py`<br>Os ganchos `visuals`/`shader_params` existem nos `native-hooks.json` 66f3981 e 5f7cadf; o Horizon HEAD tem `native_visuals.rs`, `native_shaders.rs` e `native_player_*.rs` (E2) |

**Resolvem as regressões?** E1, `--release`, 5 rodadas intercaladas na mesma sessão (`SP/a/logs/bench_interleaved.log`):

| Árvore | `bench_factory` µs/tick (mediana; mín–máx) | Δ main | `bench_frame` µs (mediana) | Alocações/quadro | `bench_large_village` |
|---|---|---|---|---|---|
| main `53267af` | 268,7 (243–289) | — | 45,2 | **0** | ok (3,4 µs, A5) |
| pr47 `80980db` | 364,8 (325–421) | **+36 %** | 71,4 | **329** | pânico `world_dir` |
| #52 `0b827f6` | 363,5 (336–373) | **+35 %** | 66,0 | **329** | pânico `world_dir` (stub `:475`) |
| #51 `a0b0d72` | 347,1 (343–357) | **+29 %** (dentro do ruído do pr47) | 68,7 | **329** | pânico `world_dir` (stub `:478`) |

**Conclusão: nenhum dos dois resolve as três regressões.**
- **#52.** Acrescenta ao `frentes/DESEMPENHO.md` (que já existia) a seção "Depois da consolidação" (E3):
  - callgrind `bench_factory`: 17,88 → 16,51 bilhões de instruções (**−7,7 %**);
  - "a consolidação subiu o custo de 12,58 bi para 17,88 bi (+42 %)", porque sons, censo do quadro, holding, indústria, medidor e poluição passaram a rodar em todo bloco, todo tick;
  - `bench_factory_scale`: 16 080 blocos = 34,7 % de um tick de 50 ms, 27 MB.
  - Um ganho de 7,7 % fica abaixo do ruído de relógio desta máquina (±20 %); por isso a medição E1 não o confirma nem o refuta.
  - O #52 **não toca** nas 329 alocações por quadro nem no pânico do `world_dir`. Ele mesmo lista como próximo passo, pendente de decisão, os postes e as máquinas paradas.
- **Commits perf/fluid.** Os 9 commits (base 0.16.1 `a5f0eb5`, "quadro: 0 alocações") estão na main e no pr47 (E1: `merge-base --is-ancestor`). A regressão, portanto, **não é um commit perdido**: vem de código adicionado depois, o que confirma o diagnóstico do próprio #52.
- **#51.** Não toca nos três pontos. Ele próprio altera o stub, então compilar offline não prova compatibilidade com o Horizon real.
- **Versão.** O #51 repete o padrão de versão sem dono (0.26.0, com zip embutido), como o B7 §4 já anotou.
