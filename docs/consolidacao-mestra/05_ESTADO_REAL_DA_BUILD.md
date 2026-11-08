# 05: Estado real da build

> **Referência:** `main@53267af` · `pr47@80980db` · `#51@a0b0d72` · `#52@0b827f6`. Data: 2026-11-08.
> **Autor:** OPUS A, a partir de A3, A5 e A7, com as correções e a lista canônica do B7.

> ## ⚠ Nada foi testado em jogo nem compilado contra o Horizon real
> **Nenhuma** das 158 funcionalidades tem evidência J (vista em jogo). Nenhuma camada foi compilada nesta auditoria com `horizon-launcher native-build`.
> - O que se executou (E1) roda contra um **stub** que copia as assinaturas do Horizon 66f3981.
> - A única compilação nativa alegada é a da 0.16.1 do Codex (`4ffb605`, E3). Ela não vale para a main 0.25.0 nem para o pr47.
> - O README do pr47 (l. 31) admite: "as entregas da 0.11.0 à 0.25.0 aguardam teste no jogo".
>
> Por isso, **"Integrado" é o teto honesto** de qualquer linha, e **nenhuma linha é "Validado"**.

## 0. Como ler

### Camadas

| Camada | Ref. | O que é |
|---|---|---|
| **Consolidada** | `main@53267af` | Veio 0.25.0, lançada (`release/Veio-0.25.0.zip`, blob `e01ef740`) |
| **Candidata** | `pr47@80980db` | PR #47: 79 commits sobre a main (obras, Lar, sons, formas, transporte, grande indústria, água, rios, consolidação Codex, PRs do Grok, mapa finito desligado). **Sem pacote próprio** |
| **Dispersa** | PRs e branches fora da main | **Dentro do pr47** (ancestrais, PR ainda aberto): #7, #19, #20, #21, #23, #25, #28, #30 (já na main), #31, #40, #49, #50 e as importações do Codex. **Fora do pr47**: #51 (0.26.0, realismo e clima, 53 arquivos) e #52 (desempenho, 6 arquivos), abertos às 20:34–20:35Z sobre o pr47; zips 0.15.1/0.15.2 (substituídos). Genealogia em [10 §4](10_EVIDENCIAS_E_RASTREABILIDADE.md#4-branches-prs-e-genealogia) |

### Estados, evidência e IDs

- **Estados:** Concebido · Aprovado · Em desenvolvimento · Implementado isoladamente · Integrado · Validado · Bloqueado · Suspenso · Substituído · Descartado · Indeterminado. "—" quer dizer que a funcionalidade não existe na camada.
- **Critério de "Integrado":** código ligado aos ganchos de `lib.rs` e coberto pelas verificações offline.
- **Coluna "disperso":** "(contido)" quer dizer que a branch de origem é ancestral do pr47 (`merge-base --is-ancestor`, E2).
- **Evidência:** E1 executado · E2 código (arquivo:linha, pr47 salvo `@main`) · E3 documento ou commit · E4 chat/memória · ND não determinado · **J** visto em jogo, coluna **vazia em todas as linhas**.
- **IDs:** lista canônica do B7 §1. São 158 FUN = 150 do A3 − 2 fundidas + 10 novas. Os aliases estão no §1.26.
- **Fichas:** as fichas de cada sistema estão em `03_atlas/SIS-*.md` (link no título de cada grupo).

---

## 1. Matriz FUN × camada (158 linhas)

### 1.1 [SIS-PLT](03_atlas/SIS-PLT.md): plataforma, build e ferramentas (11)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência doc × código / nota |
|---|---|---|---|---|---|---|---|
| FUN-PLT-01 | Validação de conteúdo `check_project` | Integrado | Integrado | — | — | E1 X1 | pr47: aviso de 29 páginas do Guia da Vila acima de 14 linhas (contadas por idioma: 17 em pt_br e 12 em en_us) |
| FUN-PLT-02 | Testes Rust offline contra o stub | Integrado | Integrado | — | — | E1 X2: 176/0/5 × 325/0/9 | O stub é a API do Horizon 66f3981, não o Horizon |
| FUN-PLT-03 | Cenários `check_performance` | Integrado | Integrado | — | — | E1 X3 | Não cobrem rios, água, estações, som nem obras |
| FUN-PLT-04 | Compilação contra o Horizon real | Indeterminado | Indeterminado | Indeterminado (0.16.1 Codex) | — | E3 `4ffb605` | `read_columns` e `chunk_load` nunca foram compilados. "After full Horizon native compilation" vale só para a 0.16.1 |
| FUN-PLT-05 | `check_native_api.py` (assinaturas contra o `.rmeta`) | Implementado isoladamente | Implementado isoladamente | — | — | E2 | Nunca rodou: falta o `.rmeta` real |
| FUN-PLT-06 | Pacote `release/Veio-<v>.zip` | Integrado | — | Substituído (0.15.1/0.15.2); #51 traz `Veio-0.26.0.zip` | — | E1: blob `e01ef740` igual nas duas camadas | O pr47 carrega o zip **da main**. O README do pr47 (l. 86) manda usar `Veio-0.16.1.zip`, que não existe. `Veio-Realismo-1.1.zip` contém a 0.16.1. B2 PLT F.3 diz que os zips são diferentes (errado) |
| FUN-PLT-07 | Launcher instala e compila (`native-build`) | Integrado | Integrado | — | — | E1 X4: 2 × 6 testes | Cargo 0.10.0 × 0.16.1. O stamp difere (`launcher/src/main.rs:52@main` × `:56-57`). `check_project` não confere a versão do launcher |
| FUN-PLT-08 | Geradores Python reprodutíveis | Integrado | Integrado | — | — | E1 X12 | Só PNG (zlib) e `.ogg` mudam em bytes |
| FUN-PLT-09 | `sim_economy.py` (7 invariantes) | Integrado | Integrado | — | — | E1 via X1 | Não modela as fontes fora do livro nem as bordas sem esmeralda |
| FUN-PLT-10 | CI automática | — | — | — | — | E2 | `.github/` não existe |
| FUN-PLT-11 | Horizon fixado (`rev` + lock) | — | — | — | — | E2 `native/api/Cargo.toml` | `git` sem `rev`, sem `Cargo.lock` |

### 1.2 [SIS-MUN](03_atlas/SIS-MUN.md): mundo (6)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-MUN-01 | Pureza da jazida pelo quadrado 3×3 | Integrado | Substituído (por MUN-02 em minérios; as jazidas infinitas seguem no 3×3) | — | — | E2 `production/extraction.rs:3-4,23-28@main` | O README da main está certo. A DEC-010 data o 3D na 0.9 (errado); o resumo SIS-MUN do B2 ainda diz "main 3D" |
| FUN-MUN-02 | Veio conectado em 3D até 16 blocos + pureza industrial | — | Integrado | Implementado isoladamente (#23 protótipo; import-progressao; contidos) | — | E2 `extraction.rs:13-15,34-57,95` | Industrial: ×4 com o dobro de potência. Diamante, esmeralda e detritos param na pura |
| FUN-MUN-03 | Rios (nascente, afluentes, quedas, foz) | — | Integrado | — | — | E1 (9 testes) + E2 `rivers.rs:26-50,758,769` | Só em chunks novos. Dependem de APIs nunca compiladas |
| FUN-MUN-04 | Mapa finito (mixins, 15 000²) | — | Em desenvolvimento | Em desenvolvimento (`c8e94dc` = head por patch-id; thread "Definir limite do mapa e otimizar") | — | E2 + E3 `MAPA.md`, `CONTINUAR.md` | Desligado, fora do crate. "Decisão de produto" do `MAPA.md` é decisão de agente (CNF-019) |
| FUN-MUN-05 | Portais e Nether no mapa finito | — | Em desenvolvimento | Em desenvolvimento | — | E3 `CONTINUAR.md` | A auditoria do Nether falhou |
| FUN-MUN-06 | Jazidas maiores e espaçadas em terreno novo | — | — | Concebido (`JAZIDAS-CONECTADAS.md`, parte 2; DEC-011) | — | E3 | — |

### 1.3 [SIS-EXT](03_atlas/SIS-EXT.md): extração e processamento (4)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-EXT-01 | Brocas Mk.I/II/III | Integrado | Integrado | — | — | E2 `extraction.rs:21-23` | 40/20/10 t; 6/12/30 FE |
| FUN-EXT-02 | Britador, forno de liga, fundição | Integrado | Integrado | — | — | E2 `data.rs` | — |
| FUN-EXT-03 | "Veio normal pede duas prensas" | — | **Substituído** (DEC-045) | Implementado isoladamente (#19, contido) | — | E3 `96458cf`, `3eb4b3d` | Com a prensa a 30t (40/min) e a Mk.I a 30/min, uma prensa basta. O README do pr47 (l. 24) ainda diz "duas prensas" |
| FUN-EXT-04 | Tempos de receita da 0.25 (DEC-045) | Integrado | Integrado | Substituído (tempos por fase do Codex, `0ce91a4`/`50995e6`) | — | E1 (`data.rs` regenera igual) + E2 `data.rs:5-10` | Prensa 30t/8 FE, montador 40t/12 FE. `PROGRESSAO-FASES.md` e `GROK-BALANCE.md` estão desatualizados |

### 1.4 [SIS-PRO](03_atlas/SIS-PRO.md): produção (7)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-PRO-01 | Máquinas por tabela (`data.rs`) | Integrado | Integrado | — | — | E1 + E2 | 53 receitas × 57 (+ motor, processador, núcleo, kit Mk.II) |
| FUN-PRO-03 | Módulos de velocidade, eficiência e produtividade | Integrado | Integrado | — | — | E2 `upgrades.rs:31@main` × `:35` | Produtividade: 1 extra a cada 10 ciclos na main, a cada 5 no pr47. Mudou sem decisão do dono (c5db4f6; CNF-016, Q-14) |
| FUN-PRO-04 | Kit Mk.II | — | Integrado | — | — | E2 `upgrades.rs:52` | Tempo ×0,5, energia ×2 |
| FUN-PRO-05 | Cadeia funda (motor, processador, núcleo, fabricador) | — | Integrado | #40 (contido) | — | E2 | Fora da bolsa (CNF-011) |
| FUN-PRO-06 | Kit de reparo e integridade | Integrado | Integrado | — | — | E2 `REPAIR=50` | — |
| FUN-PRO-07 | Monitor de itens/min | — | Integrado | #40 (contido) | — | E2 `industry.rs` | Até 4 095/min |
| FUN-PRO-09 | Painel de produção (itens/min, % ativo, motivo de espera) | Integrado | Integrado | — | — | E2 `lib.rs:818-827@main`, `:1201-1210` (`MeterActive`) | Pedido do dono, 16:38 (E4) |

### 1.5 [SIS-ENE](03_atlas/SIS-ENE.md): energia (5)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-ENE-01 | Postes de madeira e aço, rede | Integrado | Integrado | — | — | E1 (256_poles 2,29 → 1,53 ms) | — |
| FUN-ENE-02 | Gerador a carvão e biomassa, e combustíveis (absorve ex-PRO-02) | Integrado | Integrado | — | — | E2 `generation.rs:14-20` | O combustível vem em ticks de fornalha. Gerador: 10 FE por tick de queima (carvão = 16 000 FE), saída de 40 FE/t. Máquina a vapor: 20 FE por tick (32 000 FE por carvão), 120 FE/t. O A2 tratou o número como FE (errado) |
| FUN-ENE-03 | Roda d'água, eólica, solar, geotérmica | Integrado | Integrado | — | — | E2 `generation.rs` | — |
| FUN-ENE-04 | Máquina a vapor e bateria | Integrado | Integrado | — | — | E2 | No pr47 são liberadas pela pesquisa (metallurgy); o vapor consome 5 mB de água |
| FUN-ENE-05 | Turbina hidrelétrica | — | Integrado | #40 (contido) | — | E2 `water.rs:328` | `HYDRO_MAX 320`; queda ≥ 3, superfície ≥ 12, queda contada até 32 |

### 1.6 [SIS-AGU](03_atlas/SIS-AGU.md): água encanada (4)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-AGU-01 | Bomba | — | Integrado | #40 (contido) | — | E2 `water.rs:25-28` | 25 mB × 4 fontes; 6 FE/t; tanque 4 000 |
| FUN-AGU-02 | Cano com vazão por face | — | Integrado | #40 (contido) | — | E2 `water.rs:23` | 200 mB/face; NBT "Water" |
| FUN-AGU-03 | Aspersor e fertilidade do solo | — | Integrado | #40 (contido) | — | E2 `water.rs:235-238` | Fertilidade até 400 |
| FUN-AGU-04 | Lava nos canos | — | Indeterminado | #40 (contido) | — | E2 `water.rs:41-42` | Só constantes: 5 mB/fonte, balde de 1 000. Comportamento ND. Lacuna aprovada às 16:32 (E4) |

### 1.7 [SIS-AGR](03_atlas/SIS-AGR.md): agro e extrativismo (7)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-AGR-01 | Lavoura automática | Integrado | Integrado | — | — | E2 `farm` | Raio 4, 3 camadas |
| FUN-AGR-02 | Plantadeira, colheitadeira, lâmpada de cultivo | Integrado | Integrado | — | — | E2 `crops.rs` | — |
| FUN-AGR-03 | Rancho e posto de pastor | Integrado | Integrado | — | — | E2 | Até 10 animais |
| FUN-AGR-04 | Silvicultura | Integrado | Integrado | — | — | E2 `forestry.rs` | — |
| FUN-AGR-05 | Moinho, cozinha, despensa | Integrado | Integrado | — | — | E2 `pantry.rs:1-21` | A despensa paga esmeraldas fora do livro (PRICE 60; +50% com 3 aldeões; CNF-001) |
| FUN-AGR-06 | Grok lote 2 (lavrador mais barato, trigo) | — | Substituído | Substituído (#7 aberto) | — | E2 + E3 `008a980` | Registrado como substituído, sem reaplicar o código |
| FUN-AGR-07 | Lavoura irrigada | — | Integrado | #40 (contido) | — | E2 | A interação com o lavrador só tem teste unitário |

### 1.8 [SIS-LOG](03_atlas/SIS-LOG.md): logística local (7)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-LOG-01 | Esteira e esteira rápida | Integrado | Integrado | — | — | E1 (16/17 testes) | — |
| FUN-LOG-02 | Esteira expressa | — | Integrado | #40 (contido) | — | E2 | — |
| FUN-LOG-03 | Túnel de esteira | — | Integrado | #40 (contido) | — | E2 | Alcance de 6 |
| FUN-LOG-04 | Divisor, unificador, filtros, prioridade | Integrado | Integrado | — | — | E1 | — |
| FUN-LOG-05 | Braços normal, rápido e longo | Integrado | Integrado | — | — | E1 (9/9) | 40/64/60 |
| FUN-LOG-06 | Recusa de entrega (`container_accepts`) | Integrado | Integrado | — | — | E1 (8/8) | — |
| FUN-LOG-07 | Prancheta | — | Integrado | **#40** (contido), não import-megafactories | — | E2 lang `:1402,:1447`; `recipe/clipboard.json`; `logistics_controls.rs` | Ex-FUN-MEG-03 (§1.26). É o item `veio:clipboard`: copia filtro e prioridade de braço, divisor e unificador (`GRANDE-INDUSTRIA.md:18,110`) |

### 1.9 [SIS-TRA](03_atlas/SIS-TRA.md): transporte entre zonas (7)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-TRA-01 | Terminal ferroviário `veio:rail_terminal` | — | Integrado | #28 como `freight_station` (contido) | — | E2 `a79f326` | Em mundos 0.16.1 com o ID antigo, o terminal vira estação de exportação (CNF-014) |
| FUN-TRA-02 | Drones e porto | — | Integrado | #28 (contido) | — | E1 + E2 | 64 blocos, 16 itens. PORTS fica em memória |
| FUN-TRA-03 | Caravanas | — | Integrado | #28 (contido) | — | E1 + E2 | 512 blocos |
| FUN-TRA-04 | Estações de carga e descarga | — | Integrado | #40 (contido) | — | E1 + E2 `lib.rs:550-551` | 4 slots; 16 canais × 4 096 |
| FUN-TRA-05 | Produção virtual com chunk descarregado | — | Integrado | #40 (contido) | — | E1 X9 (194,4 µs/1 000 estações) + E2 `stations.rs:117-140` | Cria itens sem insumo nem energia, segue com o posto desmontado, canais sem dimensão. **Diverge da intenção** (DEC-041/043; CNF-009; Q-05). O B2 ainda cita "≈ 80 µs", valor não reproduzido |
| FUN-TRA-06 | Âncora de chunk com custo de energia | — | Integrado | #40 (contido) | — | E2 `industry.rs:21,285-291` | `forceload` por comando; 32 FE/t; `ancoras.txt` |
| FUN-TRA-07 | Elevador de carga | — | Integrado | import-megafactories (contido) | — | E1 (2 testes) | — |

### 1.10 [SIS-TRB](03_atlas/SIS-TRB.md): aldeões trabalhadores (14)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-TRB-01 | Posto de carregador | Integrado | Integrado | — | — | E1 | Raio 6, lote 16 (+8 com a pesquisa CARGO) |
| FUN-TRB-02 | Mesa do capataz e fila | Integrado | Integrado | — | — | E1 | Fila de 8 |
| FUN-TRB-03 | Planejamento incremental do capataz | Integrado | Integrado | — | — | E2 `13758bd` | — |
| FUN-TRB-04 | Busca incremental do carregador | Integrado | Integrado | — | — | E2 `604369d` | — |
| FUN-TRB-05 | Lavrador e pastor | Integrado | Integrado | — | — | E1 | — |
| FUN-TRB-06 | Zonas de trabalho em 3 níveis | Integrado | Integrado | — | — | E1 + E2 `zones.rs:249@main`, `:256-268` | Melhoria: na main, só esmeraldas físicas. No pr47, primeiro o caixa (moeda destruída) e depois esmeraldas; Mercado dá −25% |
| FUN-TRB-07 | Crescimento 30 → 150 | Integrado | Integrado | — | — | E1 + E2 `village.rs:121-129` | No pr47 soma `projects::extra_cap` (CNF-008) |
| FUN-TRB-08 | Nascimento exige cama livre | — | Integrado | #21 (contido) | — | E2 `village.rs:50-51,104-105,153-154`; `society/mod.rs:328-341` | Ativo (a cabeceira conta) |
| FUN-TRB-09 | Spawn no centro da vila | Integrado | Integrado | — | — | E2 | `SEARCH 96` |
| FUN-TRB-10 | Cérebro, navegação e descanso noturno | Integrado | Integrado | — | — | E3 (0.16.0) | Nunca visto em jogo |
| FUN-TRB-11 | Carga visível nas mãos | Integrado | Integrado | — | — | E3 (0.16.1) | — |
| FUN-TRB-12 | Vila grande medida (150 aldeões + 300 animais) | Integrado | **Bloqueado** | Bloqueado (#51, #52) | — | E1 X7, R2 | Pânico `world_dir` no harness |
| FUN-TRB-13 | Contratação por esmeralda | Integrado | Integrado | — | — | E2 `workforce/mod.rs:851@main`, `:955` | A esmeralda some fisicamente, sem passar pelo livro |
| FUN-TRB-14 | Comida por tarefa (2 pontos; o pão paga 3) | Integrado | Integrado | — | — | E3 (Grok lote 3) + E2 `projects.rs:416` | NUTRITION −1 no pr47 |

### 1.11 [SIS-VIL](03_atlas/SIS-VIL.md): sociedade (6)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-VIL-01 | Casas, famílias, carteira | Integrado | Integrado | — | — | E2 | 4 moradores por casa |
| FUN-VIL-02 | Lojas e comerciantes | Integrado | Integrado | — | — | E2 | — |
| FUN-VIL-03 | Prefeitura, prefeito, fases | Integrado | Integrado | — | — | E2 `mayor.rs:14,17-18,189,198` | Só promove com prefeito; suspende se IDH + 5 < exigido |
| FUN-VIL-04 | Poço, saneamento e energia na casa | Integrado | Integrado | — | — | E2 | Raios 24/12 |
| FUN-VIL-05 | IDH, Gini, bairros, PIB | Integrado | Integrado | — | — | E2 `society/mod.rs:613`; `mayor.rs:188` | **PIB = varejo + exportação**: o guia, pág. 33, diz "tarefas e vendas" (CNF-007). **O IDH não inclui as obras**, embora `SOCIEDADE.md` preveja |
| FUN-VIL-06 | 12 bens de conforto | Integrado | Integrado | — | — | E2 `GOODS` | — |

### 1.12 [SIS-ECO](03_atlas/SIS-ECO.md): economia e contabilidade (9)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-ECO-01 | Livros Company/Public/Shop e carteira | Integrado | Integrado | — | — | E2 `society/ledger.rs` | — |
| FUN-ECO-02 | Imposto na fonte por fase | Integrado | Integrado | — | — | E2 `mayor.rs:14` | 4/8/12/16%, cobrado no varejo **e** no atacado (CNF-015, Q-13) |
| FUN-ECO-03 | Salário por empregador | Integrado | Integrado | — | — | E2 | 4 por turno |
| FUN-ECO-04 | Assistência pública | Integrado | Integrado | — | — | E2 | 1 por pessoa por turno |
| FUN-ECO-05 | Holding: lucro por divisão (11 abas) | Integrado | Integrado | — | — | E2 | Só em memória |
| FUN-ECO-06 | Bordas de moeda (`BORDERS`) | Integrado | Integrado | — | — | E2 `ledger.rs:159-167,199,243`; `trade.rs:336,365,598`; `market.rs:464`; `center.rs:592-601` | Entrada: esmeralda, compra do jogador, exportação. Saída: saque, importação, frete, obras e zona (destruição). Só em memória. "Só por esmeraldas" (README/DEC-030) diverge |
| FUN-ECO-07 | Recompensas pelo caixa público (DEC-035) | — | Integrado | zip das obras = `f1452f5` (contido) | — | E2 `center.rs:609-620` | Public → Company |
| FUN-ECO-08 | Rendimento da obra Prefeitura | — | Integrado | — | — | E2 `projects.rs:475-476`; `center.rs:307-308` | (caixa/16).min(8) por dia, pago pelo Public |
| FUN-ECO-09 | Fontes externas de moeda (fora do livro) | Integrado | Integrado | — | — | E2 `contracts.rs:23,35-49,63-66`; `pantry.rs`; `milestones/mod.rs:147-150`; `production.json:90` | Defeito de desenho, não funcionalidade desejada (CNF-001, Q-01) |

### 1.13 [SIS-MER](03_atlas/SIS-MER.md): mercado e comércio exterior (8)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-MER-01 | Preço local dinâmico | Integrado | Integrado | — | — | E2 `market.rs:30-32` | 50–200% |
| FUN-MER-02 | Atacado a 75% e liquidação | Integrado | Integrado | — | — | E2 `market.rs:26` | Só com cofre da companhia |
| FUN-MER-03 | Importação de comida | Integrado | Integrado | — | — | E2 | Até 16 unidades; tarifa fica na prefeitura |
| FUN-MER-04 | Porto `export_dock` | Integrado | Integrado | — | — | E2 | — |
| FUN-MER-05 | Estação `veio:freight_station` (comércio) | Integrado | Integrado | — | — | E2 `trade.rs:27` | 9 slots de entrada. O mesmo ID era o terminal ferroviário no Codex 0.16.1 (CNF-014) |
| FUN-MER-06 | Bolsa mundial de 34 produtos | Integrado | Integrado | — | — | E2 | STAPLE = pão (`trade.rs:84`); pressão = recente/2 + hoje (`:132-134`); grande indústria fora (CNF-011) |
| FUN-MER-07 | Contratos com prazo e reputação | Integrado | Integrado | — | — | E2 | +50%, até 3 |
| FUN-MER-08 | Venda ao jogador em lotes por esmeralda | Integrado | Integrado | — | — | E2 | Até 16 por esmeralda |

### 1.14 [SIS-OBR](03_atlas/SIS-OBR.md): obras e plantas (10)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-OBR-01 | Canteiro de obras | — | **Em desenvolvimento** (telas) | zip das obras (contido) | — | E2 `menu/site.json` `slots: 1` × `workforce/mod.rs:150` (3) | O 2º insumo e a saída não aparecem na tela (R-30) |
| FUN-OBR-02 | 10 obras com efeitos | — | Integrado | idem | — | E2 `projects.rs:42-51,440-466` | O efeito só vale com o prédio funcionando |
| FUN-OBR-03 | Prestígio da vila | — | Integrado | idem | — | E2 | 0/60/200/500 |
| FUN-OBR-04 | Pedidos diários da vila | — | Integrado | idem | — | E2 `requests.rs:110` | 3 por dia (4 com Correio); pagos pelo Public |
| FUN-OBR-05 | Decretos (CALL/RATION/PLENTY) | — | Integrado | idem | — | E2 `projects.rs:53-55` | Slot 7 do centro |
| FUN-OBR-06 | 9 plantas prontas | — | Integrado | idem | — | E1 (7 542 blocos) | — |
| FUN-OBR-08 | Boletim da manhã | — | Integrado | idem | — | E2 `center.rs:15` | — |
| FUN-OBR-09 | Ruínas | — | Concebido | Concebido (`HANDOFF_OBRAS.md`) | — | E3 | `workforce/ruins.rs` não existe |
| FUN-OBR-10 | Proteção de obras | — | Concebido | Concebido | — | E3 | Faltam 4 ganchos (04 §2.1) |
| FUN-OBR-11 | Funcionários das obras (`staff.rs`) | — | Integrado | zip das obras (contido) | — | E2 `staff.rs` | Faltam as traduções `veio.site.next.<papel>` |

### 1.15 [SIS-LAR](03_atlas/SIS-LAR.md): casa do fundador e Lar (4)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-LAR-01 | Casa do fundador (estrutura, mural) | — | Integrado | `d1e2f30` (contido) | — | E1 + E2 (251 comandos) | — |
| FUN-LAR-02 | Lar `veio:home` (lareira, despensa, conforto, refeição, banquete) | — | Integrado | zip das obras (contido) | — | E2 `workforce/home.rs:555-693`; `build_house.py:177-178` | A refeição dá efeitos ao jogador (saturação, regeneração, pressa, resistência…); o banquete dá `hero_of_the_village`. Nível máximo 4 |
| FUN-LAR-03 | Ritmo das máquinas pela comida (DEC-046; absorve ex-PRO-08) | — | Integrado | — | — | E2 `home/food.rs:22-29,59-69`; `home/mod.rs:99-103`; `upgrades.rs:44`; teste `home/tests.rs:163` | **Só comer fora de casa** dá ×0,95 (6 000 t) ou ×0,91 (9 600 t). O efeito é **global**. Em casa não vale, e **o Lar não mexe no ritmo**. B2 e B3 §3.1 descrevem o inverso. O README do pr47 (l. 21, "+15%") e o comentário de `food.rs:9` estão obsoletos |
| FUN-LAR-04 | Cama restrita à casa (`bed_allowed`) | — | Integrado | — | — | E2 `home/mod.rs:306` | — |

### 1.16 [SIS-CTL](03_atlas/SIS-CTL.md): centro de controle (4)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-CTL-01 | Centro com metas, cofre e saque | Integrado | Integrado | — | — | E2 `center.rs:395,419` | O "caixa da vila" é o livro Company. Slots em [04 §10.3](04_ARQUITETURA_E_DEPENDENCIAS.md#103-slots-do-centro-de-controle-veiocontrol_center) |
| FUN-CTL-02 | Abas da holding | Integrado | Integrado | — | — | E2 | — |
| FUN-CTL-03 | Páginas Resumo/Atenção/Obras | — | Integrado | zip das obras (contido) | — | E2 `PAGES 4` | — |
| FUN-CTL-04 | Foco e bússola | — | Integrado | idem | — | E2 `focus.rs` | Os códigos 40/41 não colidem hoje (CNF-013) |

### 1.17 [SIS-PRG](03_atlas/SIS-PRG.md): progressão (8)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-PRG-01 | Avanços que liberam receitas | Indeterminado | Indeterminado | — | — | E2 (receitas concedidas pelos JSON) + ND (bloqueio) | Não há `doLimitedCrafting` (CNF-003). main: 1 item; pr47: 32 engrenagens, 16 aço ou 16 circuitos **ou** a licença |
| FUN-PRG-02 | Quadro da fábrica, fases e licenças | — | Integrado | import-factory-board (contido) | — | E1 + E2 `milestones/mod.rs:37-52,246` | O texto "Libera" promete itens que vêm da pesquisa (CNF-002) |
| FUN-PRG-03 | Pedido do dia do quadro | — | Integrado | idem | — | E2 `milestones/mod.rs:147-150`; `census.rs:114` | 1–6 esmeraldas fora do livro. `paid_day` fica em memória: repete ao reabrir |
| FUN-PRG-04 | Alertas de gargalo (censo) | — | Integrado | idem | — | E2 `census.rs` | — |
| FUN-PRG-06 | Central de contratos | Integrado | Integrado | — | — | E2 `contracts.rs` | Valores maiores no pr47 |
| FUN-PRG-07 | Grande projeto em 4 fases | — | Integrado | #40 (contido) | — | E2 `contracts.rs:35-49`; `lib.rs:1101` | 32/48/64 esmeraldas; concede a `@a` |
| FUN-PRG-08 | Progressão por fases do Codex (30/min) | — | Substituído | Substituído (`0ce91a4`) | — | E2 | DEC-045 |
| FUN-PRG-09 | Meta sustentada e meta final | — | — | Concebido | — | E3/E4 | Q-10 |

### 1.18 [SIS-PES](03_atlas/SIS-PES.md): pesquisa (2)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-PES-01 | Laboratório da fábrica, 4 tópicos (ex-PRG-05) | — | Integrado | #40 (contido) | — | E1 + E2 `research.rs:22-26,45,166` | Estado global (`pesquisa.txt`), concede a `@a` |
| FUN-PES-02 | Pesquisa da vila, 5 tópicos (ex-OBR-07) | — | **Em desenvolvimento** | zip das obras (contido) | — | E2 `staff.rs:44-71,232,271,390-392`; `projects.rs:63,418,436,466` | Os efeitos estão ativos. Faltam 10 traduções (o jogador vê a chave crua), e `veio.research.done` colide (CNF-018). A chave é `cargo`, não "Logística" |

### 1.19 [SIS-TUT](03_atlas/SIS-TUT.md): tutorial e guias (6)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-TUT-01 | Dicas contextuais | Integrado | Integrado | — | — | E2 | — |
| FUN-TUT-02 | Guia principal | Integrado | Integrado | — | — | E1 | 93 → 100 páginas, que é o teto |
| FUN-TUT-03 | Guia da Vila Operária | Integrado | Integrado | — | — | E1 | 29 páginas acima de 14 linhas (contadas por idioma: 17 em pt_br e 12 em en_us); duas páginas "8."; pág. 33 com o PIB errado |
| FUN-TUT-04 | Guia de Transportes | — | Integrado | — | — | E1 | — |
| FUN-TUT-05 | Guia da Grande Indústria | — | Integrado | #40 (contido) | — | E1 | Fora do `check_page_fit` |
| FUN-TUT-06 | Tutorial interativo por jogador | — | Integrado | Substituído (zip 0.15.1) | — | E1 (19 + 3 testes, os 3 seriais) | Termina na central. A água só aparece no guia. `onboarding::session` usa `world_dir` |

### 1.20 [SIS-DEF](03_atlas/SIS-DEF.md): ameaças e defesa (6)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-DEF-01 | Poluição e ataques | Integrado | Integrado | — | — | E2 | Só em memória (DEC-059) |
| FUN-DEF-02 | Ondas noturnas com arqueiros e brutos | Integrado | Integrado | — | — | E2 `pollution.rs:28-47,220,236-238`; `raider_brute.json:4` | Arco a 26 blocos. O README do pr47 (l. 307) diz "64/18 blocos" (errado). No pr47, uma onda sem mobs não conta |
| FUN-DEF-03 | Quartel (4 classes) | Integrado | Integrado | Integrado (#30 aberto, já na main) | — | E2 | — |
| FUN-DEF-04 | Bobina de Tesla | Integrado | Integrado | — | — | E2 `defense.rs:12-15` | 2 400 FE por tiro |
| FUN-DEF-05 | Defensores patrulham à noite | — | Integrado | — | — | E3 `fa96ceb` | — |
| FUN-DEF-06 | Monstros caçam aldeões à noite | Integrado | Integrado | — | — | E2 `pollution.rs:427@main`, `:437` (`villager_hunter`) + teste `tests.rs:373@main`/`:397` | — |

### 1.21 [SIS-MEG](03_atlas/SIS-MEG.md): megafábricas (3)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-MEG-01 | Copiadora `veio:blueprint_designer` | — | Integrado | import-megafactories (contido) | — | E1 (9 testes) | Área de 5/9/15/21 × 24, até 6 000 blocos |
| FUN-MEG-02 | Marco de obra | — | Integrado | idem | — | E2 | — |
| FUN-MEG-04 | Escadas entre andares | — | Integrado | import-megafactories (contido) | — | E2 | Até 6 andares |

### 1.22 [SIS-APR](03_atlas/SIS-APR.md): apresentação (9)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-APR-01 | Painéis e telas | Integrado | Integrado | — | — | E2 | 36 × 44 status ([04 §10.1](04_ARQUITETURA_E_DEPENDENCIAS.md#101-status_keys-índice-de-status-exibido-nos-painéis)). `TRABALHADORES.md` diz 26 ou 30 |
| FUN-APR-02 | Texturas 16×16 | Integrado | Integrado | — | — | E1 | 65 → 89 |
| FUN-APR-03 | Silhuetas e colisão por máquina | — | Integrado | #31 (contido) | — | E1 X11 | 34 formas, 27 silhuetas |
| FUN-APR-04 | Animação de peças | — | Integrado | #50 (contido) | — | E1 | — |
| FUN-APR-05 | Som posicional | — | Integrado | #49 (contido) | — | E1 X10 | 30 `.ogg` CC0 |
| FUN-APR-06 | Ambiente sonoro limitado | — | Integrado | #50 (contido); #52: pula o som sem ouvinte (Em desenvolvimento) | — | E1 + E2 (#52 `soundscape.rs` +22) | — |
| FUN-APR-07 | Shader "realismo leve" | — | Integrado | Implementado isoladamente (zip Realismo 1.1 = 0.16.1) | — | E2 `horizon-shaders.toml` | O manifesto 0.1 é aceito pelo Horizon 5f7cadf; o WGSL não foi validado |
| FUN-APR-08 | Quadro desenhado sem alocação (0.17.1) | Integrado | — (regrediu) | — (#51 e #52 também: 329) | — | E1 R1 | A garantia da 0.17.1 não vale no pr47 |
| FUN-APR-09 | Realismo 2.1, clima e "estações do ano" | — | — | Em desenvolvimento (#51) | — | E2 (diff: `ambience.rs`, `climate.rs`, 9 shaders) | Não auditado; efeito de jogo ND (Q-07) |

### 1.23 [SIS-DES](03_atlas/SIS-DES.md): desempenho (5)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-DES-01 | Orçamento global por faixa | Integrado | Integrado | — | — | E1 (3 testes) | `budget.rs:6-10` |
| FUN-DES-02 | Topologia e busca incremental | Integrado | Integrado | — | — | E1 | — |
| FUN-DES-03 | Bench da fábrica de 480 blocos | Integrado | Integrado | — | — | E1 R1 | Mediana de 268,7 → 364,8 µs/tick (+36%) |
| FUN-DES-04 | Teto global de ticks e limite do mapa | — | Em desenvolvimento | Em desenvolvimento (#52; thread "Definir limite do mapa e otimizar") | — | E2 (#52) + E4 | — |
| FUN-DES-05 | Critério de desempenho para PC de 8 GB | — | — | Aprovado (sem medida) | — | E4 (dono, 20:09; DEC-056) | Não há número. #52: 16 080 blocos = 34,7% de um tick, 27 MB (E3) |

### 1.24 [SIS-MP](03_atlas/SIS-MP.md): multijogador LAN (2)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-MP-01 | Convidado LAN vai à vila do anfitrião | — | Integrado | #25 (contido) | — | E2 `spawn.rs:151` | — |
| FUN-MP-02 | Estado por jogador | — | Integrado | — | — | E2 | Só o tutorial é por UUID. Bônus de comida, pesquisa, canais e cofre são globais (CNF-017, Q-15) |

### 1.25 [SIS-PER](03_atlas/SIS-PER.md): persistência (4)

| ID | Funcionalidade | main | pr47 | disperso | J | Evidência | Divergência / nota |
|---|---|---|---|---|---|---|---|
| FUN-PER-01 | NBT de block entities | Integrado | Integrado | — | — | E2 | Só o comércio tem versão (`trade.rs:389`) |
| FUN-PER-02 | Arquivos em `<mundo>/veio/` | — | Integrado | — | — | E2 | 5 arquivos sem versão ([04 §10.7](04_ARQUITETURA_E_DEPENDENCIAS.md#107-arquivos-em-mundoveio-pr47)) |
| FUN-PER-03 | Reset de estado ao trocar de mundo | — | Integrado | — | — | E2 `lib.rs:89` | Parcial: home, census e blueprint, e o resto por `world_dir` |
| FUN-PER-04 | Gravação atômica | — | Integrado | — | — | E2 | Só rios e tutorial |

### 1.26 Aliases e IDs movidos

| ID antigo | Destino | Motivo |
|---|---|---|
| FUN-PRO-02 (combustíveis) | FUN-ENE-02 | Fusão (B7) |
| FUN-PRO-08 (bônus de refeição) | FUN-LAR-03 | Fusão (B7) |
| FUN-PRG-05 (laboratório) | FUN-PES-01 | Movido para SIS-PES (B7) |
| FUN-OBR-07 (pesquisa da vila) | FUN-PES-02 | Movido para SIS-PES (B7) |
| FUN-MEG-03 (Prancheta) | FUN-LOG-07 | Aplicado pelo supervisor na consolidação: a Prancheta é `veio:clipboard`, ferramenta de logística (A7 §0) |

**Contagem por camada** (calculada sobre as 158 linhas da matriz; Validado = 0 nas duas):

| Camada | Integrado | Em desenvolvimento | Substituído | Indeterminado | Implementado isoladamente | Bloqueado | Concebido | Ausente ("—") |
|---|---|---|---|---|---|---|---|---|
| main | 80 | 0 | 0 | 2 (PLT-04, PRG-01) | 1 (PLT-05) | 0 | 0 | 75 |
| pr47 | 134 | 5 (MUN-04/05, OBR-01, PES-02, DES-04) | 4 (MUN-01, EXT-03, AGR-06, PRG-08) | 3 (PLT-04, PRG-01, AGU-04) | 1 | 1 (TRB-12) | 2 (OBR-09/10) | 8 |

---

## 2. Visão transversal

### 2.1 Desconectadas: existem, mas não chegam ao jogador

| Item | Evidência |
|---|---|
| Menu do canteiro com 1 slot × 3 no código: o 2º insumo e a saída não aparecem | E2 `menu/site.json` × `workforce/mod.rs:150` |
| 10 traduções da pesquisa da vila e cerca de 25 das obras faltando: aparece a chave crua | E2 `staff.rs:390-392`; [04 §10.8](04_ARQUITETURA_E_DEPENDENCIAS.md#108-prefixos-de-tradução) |
| O texto "Libera" do Quadro aponta para receitas que vêm do laboratório | E2 `milestones/mod.rs:246` |
| Serviços das obras fora do IDH | E2 `society/mod.rs:613` |
| `Veio-Realismo-1.1.zip` entrega a 0.16.1; o zip 0.25.0 do pr47 é o da main | E1/E2 |
| Funções nunca chamadas: `sites_near`, `research_percent`, `distance` | E1 (avisos) |
| `check_native_api.py` e `native_chunk_tickets` existem e não são usados | E2 |

### 2.2 Dependentes de sistema inexistente ou não verificado

| Item | Depende de |
|---|---|
| Rios (MUN-03) | `read_columns`, `chunk_load(new)` e `edit_blocks` durante a geração, nunca compilados |
| Bloqueio de receitas (PRG-01) | `doLimitedCrafting`, que ninguém liga, ou um comportamento do Horizon (ND) |
| Proteção de obras e ruínas (OBR-09/10) | 4 ganchos ausentes e `ruins.rs` inexistente |
| Mapa finito (MUN-04/05) | Mixins de motor (22/24 injeções), fora da API de mods |
| `bench_large_village` (TRB-12) | `world_dir` no harness |
| Âncora, grande projeto e pesquisa | `run_command` (`forceload`, `advancement grant @a`) no servidor integrado (ND) |

### 2.3 Implementações divergentes ou duplicadas

| Par | Situação (E2) |
|---|---|
| Ritmo pela comida × Lar | O código cumpre a DEC-046: o bônus de ritmo vem só de comer fora; o Lar dá efeitos ao jogador. Os documentos dizem o contrário (LAR-03) |
| Três sistemas que pagam por entrega (contratos, pedidos da vila, pedido do dia) + grande projeto | Só os pedidos da vila respeitam o livro (CNF-001/004) |
| Quadro × laboratório × pesquisa da vila | Duas progressões e dois "research" com a mesma chave de texto (CNF-002/018) |
| Centro de controle × quadro da fábrica | Metas e alertas duplicados, censos separados, raio igual (64) |
| Âncora × estações | Duas respostas à DEC-041: carregar de verdade (`forceload`) ou simular sem custo |
| `veio:freight_station` | Sentido trocado entre a 0.16.1 do Codex e o pr47, sem migração (CNF-014) |
| Status 40/41 | Pendência do centro × status de frete; sem efeito hoje (CNF-013) |
| Dois detectores de servidor | `host.window()` e `industry::on_server` |

### 2.4 Apresentadas como concluídas sem validação

| Declaração | Realidade |
|---|---|
| "After full Horizon native compilation" (`4ffb605`, CONSOLIDACAO) | Vale para a 0.16.1 do Codex, não para o pr47 |
| 0.17.1: "nenhuma alocação ao desenhar um quadro" | Verdade na main, falsa no pr47, no #51 e no #52 (E1) |
| Rios e estações "prontos" (thread do #40) | APIs nunca compiladas |
| README do pr47: casa "+15%", zip 0.16.1, ondas "64/18", "duas prensas" | Contradiz o código (CNF-006) |
| `CONSOLIDACAO.md`: 558 JSON, 68 texturas, 248 testes | Hoje são 742/89/325 |
| 0.11.0 → 0.25.0 inteiras | Nenhum teste em partida (README do pr47, l. 31) |

### 2.5 Essenciais não abordadas

| Lacuna | TAR |
|---|---|
| Compilação reprodutível contra um Horizon fixado; CI | [TAR-001](08_BACKLOG_E_PRIORIDADES.md#tar-001), [TAR-002](08_BACKLOG_E_PRIORIDADES.md#tar-002), [TAR-019](08_BACKLOG_E_PRIORIDADES.md#tar-019) |
| Teste em partida de qualquer versão ≥ 0.11 | [TAR-003](08_BACKLOG_E_PRIORIDADES.md#tar-003) |
| Versão de formato e migração de saves (0.25 → pr47; `freight_station` da 0.16.1) | [TAR-032](08_BACKLOG_E_PRIORIDADES.md#tar-032), [TAR-018](08_BACKLOG_E_PRIORIDADES.md#tar-018), [TAR-007](08_BACKLOG_E_PRIORIDADES.md#tar-007) |
| Persistência de poluição, holding, `BORDERS` e pedido do dia | [TAR-029](08_BACKLOG_E_PRIORIDADES.md#tar-029) |
| Benchmarks de rios, água, estações em escala, som e obras; critério de 8 GB | [TAR-037](08_BACKLOG_E_PRIORIDADES.md#tar-037), [TAR-039](08_BACKLOG_E_PRIORIDADES.md#tar-039) |
| Contabilizar a moeda criada fora do livro | [TAR-035](08_BACKLOG_E_PRIORIDADES.md#tar-035) |

### 2.6 Trabalho em branch fora da main

**Tudo o que está fora da main e é conhecido até `80980db` está no pr47:** 79 commits, e os 12 PRs abertos #7–#50 são ancestrais dele. Fora do pr47 há:
- os zips 0.15.1/0.15.2, já substituídos;
- `c8e94dc`, com o mesmo patch-id do head;
- **#51** e **#52**, os únicos códigos novos.

Detalhes em [10 §4](10_EVIDENCIAS_E_RASTREABILIDADE.md#4-branches-prs-e-genealogia).

| PR | Toca | Estado | Resolve as regressões? |
|---|---|---|---|
| #52 `0b827f6` (+1 sobre o pr47) | `soundscape.rs`, `home/mural.rs`, `perf.rs` (contador `LIVE`), `perf/factory.rs` (`bench_factory_scale`), `home/tests.rs`, `frentes/DESEMPENHO.md` (+71: seção "Depois da consolidação") | Em desenvolvimento | **Não.** Declara −7,7% em instruções no callgrind (E3), abaixo do ruído do relógio. Não mexe nas 329 alocações nem no `world_dir` |
| #51 `a0b0d72` (+3) | 53 arquivos: `ambience.rs`, `climate.rs`, ganchos `visuals`/`shader_params`, stub +48, 9 shaders, `check_shaders.py`, `Veio-0.26.0.zip`, versão 0.26.0 | Em desenvolvimento | **Não.** Fábrica +29% (dentro do ruído do pr47), 329 alocações, pânico |

---

## 3. Resultados E1

Todos foram executados em 2026-11-08, entre 20:15 e 21:10 UTC, num Linux x86_64 de nuvem com 4 núcleos. O relógio desta máquina oscila ±20%. Os logs estão versionados em [`evidencias/logs/`](evidencias/logs/).

### 3.1 Verificações

| # | Comando (cwd = raiz da árvore) | main@53267af | pr47@80980db |
|---|---|---|---|
| X1 | `python3 tools/check_project.py` | exit 0: 529 JSON; 93 páginas; 65 texturas | exit 0: 742 JSON; 100 páginas; 89 texturas; aviso de 29 páginas (contadas por idioma: 17 em pt_br e 12 em en_us) > 14 linhas |
| X2 | `python3 tools/check_rust.py` | 176 ok / 0 falhas / 5 ignorados; 4 linhas de aviso | 325 / 0 / 9; 3 avisos |
| X3 | `python3 tools/check_performance.py --output <log>` | 7 cenários ok; checks `power_energy=129500 callbacks=33792` | idem, checks idênticos. Medianas (ns): publish_2000_cables 134 926 → 103 717; 256_poles 2 290 182 → 1 530 921; foreman_95 746 710 → 625 125 |
| X4 | `cargo test` em `launcher/` | 2 ok | 6 ok |
| X8 | `cargo test -- --ignored onboarding --test-threads=1` | — | 3/3 ok (falham em paralelo) |
| X9 | `cargo test --release -- --ignored stations_cost` | — | 194,373 µs por passo de 20 ticks com 1 000 estações |
| X10/X11 | `check_machine_audio.py` / `check_machine_forms.py` | — | 30 sons; 34 formas, 27 silhuetas |
| X12 | Todos os `tools/build_*.py` em cópia + diff | Só 2 PNG (zlib) | Só os 30 `.ogg` |

### 3.2 Benchmarks (`native/offline-check`, `--release`)

**R1:** `cargo test --release --offline -p veio-check -- --ignored --nocapture --test-threads=1 --exact veio::perf::factory::bench_factory veio::perf::factory::bench_frame`, em 5 rodadas intercaladas (main → pr47 → #52 → #51), log `bench_interleaved.log`.

| Árvore | `bench_factory` µs/tick, mediana (mín–máx) | Δ main | `bench_frame` µs, mediana | Alocações/quadro |
|---|---|---|---|---|
| main@53267af | 268,7 (243,4–288,5) | — | 45,2 | **0** |
| pr47@80980db | 364,8 (325,1–420,5) | **+36%** | 71,4 | **329** |
| #52@0b827f6 | 363,5 (336,3–372,9) | **+35%** | 66,0 | **329** |
| #51@a0b0d72 | 347,1 (342,5–357,0) | **+29%** | 68,7 | **329** |

**R2:** `… -- --ignored --nocapture --test-threads=1 bench_`, 2 rodadas por árvore, log `pr51_52_bench_release.log`.

| Árvore | `bench_large_village` | `bench_factory_scale` (só no #52) |
|---|---|---|
| main | 3,4 µs/tick (150 aldeões + 300 animais; rodada anterior, X7) | — |
| pr47 | **pânico** `not implemented: world_dir` (`horizon-client/src/lib.rs:475`) | — |
| #52 | **pânico** (idem, `:475`) | 480 blocos: 359–417 µs/tick; 0,7 MB; 1 517 B/bloco |
| #51 | **pânico** (idem, `:478`) | — |

**Rodadas anteriores (X5/X6):** 4 execuções, `bench_repeat.log`. main 253–352 µs/tick; pr47 349–392 µs/tick; 936 × 943 alocações por tick. Coerentes com o R1.

**Medições declaradas, não reproduzidas (E3):** callgrind do #52, 12,58 bi (perf/fluid) → 17,88 bi (pr47) → 16,51 bi (#52).
