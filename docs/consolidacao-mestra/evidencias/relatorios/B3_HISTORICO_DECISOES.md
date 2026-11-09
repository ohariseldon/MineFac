# B3 — Histórico e registro de decisões do Veio

Autor: OPUS B (arquiteto de produto, sistemas e governança). Data: 2026-11-08, ~21h UTC.
Base: `main@53267af` (0.25.0) e `pr47@80980db` (branch `claude/project-thread-l94frq`), mais as branches remotas, o chat do projeto e a memória.

**Convenções**

- Os horários estão em **UTC**. Os commits do Codex local e as máquinas do dono assinam em -0300; os do Claude nuvem e do Grok, em +0000. Converti tudo para UTC, que é o fuso dos horários do chat.
- Nível de evidência de cada linha:
  - **E1** = executei e vi passar.
  - **E2** = li no código (`arquivo:linha@ramo`).
  - **E3** = está escrito em doc, commit ou PR.
  - **E4** = foi dito no chat ou está na memória.
  - **ND** = não determinado.
- **Tipos de decisão:**
  - ideia;
  - proposta;
  - decisão aprovada (só o Luiz aprova);
  - decisão de agente (não confirmada pelo dono);
  - implementada;
  - substituída;
  - revogada;
  - abandonada.
- **"Código acompanha?"** registra o que eu vi no código (E2) ou deixa "a confirmar pelo OPUS A".
- **"audiões"**, no pedido do usuário, quase certamente quer dizer **aldeões**. Nenhum doc, código ou chat usa "audião", e o tema de aldeões, vila e trabalhadores domina o projeto. Registrado como inferência.
- **Fora de escopo e inacessíveis:** Projeto-Moba e Wasému/pesquisa-macro. Não há nenhuma referência a eles neste repositório.

---

## 1. Cronologia das versões

### 1.1 Antes do git
O histórico começa na 0.6 (`5eaea2c`). As versões 0.1 a 0.5 não estão no git e só aparecem em textos (E3):

- o README menciona a "progressão da 0.4";
- a central de contratos é da 0.5;
- existe o mundo de demonstração `Veio03-Laboratorio`.

### 1.2 Linha da main
A main vai de 0.6 a 0.25 em **cerca de 23 horas**: de 07/11 às 18:24 até 08/11 às 17:29 UTC.

| Versão | Data/hora UTC | Autor (commit) | Conteúdo | Fonte |
|---|---|---|---|---|
| 0.6 | 07/11 18:24 | Codex local | Consolida a jogabilidade de fábrica: extratora, prensa, montador, gerador, cabos e o Guia do Veio | `5eaea2c` |
| 0.7 | 07/11 19:10 | Claude (local, -0300) | Visual "no estilo Minecraft", colheitadeira, torre Tesla e integração vanilla | `819ba92` |
| 0.8 | 07/11 19:37 | Claude local | Postes com fios aéreos, poluição e Saqueadores, módulos de melhoria | `7644aab` |
| 0.8.1 | 07/11 20:00 | Codex local | Unificador de duas entradas | `e8d3d62` |
| 0.9 | 07/11 20:04 | Claude local | Jazidas com pureza, receitas por tabela, renováveis, lavoura, rancho e comida | `5944d37` |
| 0.9.1 | 07/11 20:23–20:37 | Codex local | Esteiras com curvas/rampas, `LOGISTICA.md`, **COORDENACAO** (frentes Cloud/Codex/Grok) e interface de postos | `26599ec`, `9e8b601`, `20f9dba` |
| PR #1 | 07/11 20:40 | merge "bizi" | `codex/expansion` entra na main | `ec3db34` |
| 0.10.0 | 07/11 21:04 | Claude nuvem ("Cloud") | Consolida as três frentes (lavrador, interface de postos, Grok lote 1) e põe o launcher.exe no repositório | `9421883`, PR #3 |
| 0.11.0 | 07/11 21:34 | Cloud | Carregador e capataz | `903c4ec` |
| 0.12.0 | 07/11 21:43 | Cloud | Fila do capataz, pastor e Grok lote 3 | `6ac0c62`, `6fd1b44` |
| 0.13.0 | 07/11 22:10 | Cloud | Zonas, celeiro, silo, centro de controle; vila até 30 | `36e43b6` |
| 0.14.0 | 07/11 22:33 | Cloud | Vilas até 150, spawn no centro da vila, Guia da Vila | `a691802` |
| 0.15.0 | 07/11 22:39 | Cloud | "Build consolidada" Cloud + Codex + Grok (lotes 1, 3–8) | `8abe435`, PR #17 |
| #22, #24, #27, #29 | 07/11 23:09–23:46 | Codex (merge "bizi") | Otimização, escala, orçamento global e topologia incremental | `65cab50` … `cf37004` |
| #32 | 08/11 05:08 | Codex | Capatazes incrementais | `5d76390` |
| 0.16.0 (main) | 08/11 05:26 | Cloud | Aldeões andam pelo cérebro vanilla, descanso noturno, vilas grandes mais baratas | `939c552` |
| 0.16.1 | 08/11 05:36 | Cloud | O trabalhador mostra na mão o que carrega | `7208324` |
| 0.17.0 | 08/11 05:53 | Cloud | Ondas por um lado, arqueiros, Bruto, quartel funcional | `0deee22` |
| 0.17.1 / 0.17.2 | 08/11 07:17 / 13:13 | Cloud | Desempenho (−36% de trabalho por tick, offline); esteira "andando" | `0472750`, `494d3e7` |
| #38 | 08/11 13:21 | Codex | Busca incremental dos carregadores | `5217b62` (a `main` local parou aqui) |
| 0.18.0 | 08/11 16:06 | Cloud | Sociedade da vila: casas, lojas, prefeito, fases | `0aa3fbb` |
| 0.19.0 | 08/11 16:25 | Cloud | Contas companhia/pública, impostos, preços locais | `a48e14c` |
| 0.20.0 | 08/11 16:31 | Cloud | Holding: uma aba por empresa | `95a1f7c` (base das obras: `542b0a0`) |
| 0.21.0 | 08/11 16:48 | Cloud | Economia fechada, imposto na fonte, lucro por empresa | `9331dee` |
| 0.22.0 | 08/11 17:04 | Cloud | Comércio exterior pelo cais | `cbc53a9` |
| 0.23.0 | 08/11 17:13 | Cloud | Conexões externas no estilo Cities: Skylines | `e8ac991` |
| 0.24.0 | 08/11 17:20 | Cloud | Acabamento do comércio exterior | `b25b9d0` |
| **0.25.0** | 08/11 17:29 | Cloud | Balanço econômico (simulador `tools/sim_economy.py`) | `87ee998`, merge PR #48 `53267af` |

### 1.3 Fora da main
Linhas paralelas, que hoje estão no pr47 ou dispersas (E3/E2, git log):

| Linha | Data/hora UTC | Agente | Conteúdo | Destino |
|---|---|---|---|---|
| `cursor/grok-*` / balanço | 07/11 noite | Grok (Cursor) e commits "Jefter Pereira" | Duas prensas (#19), missões (#20), economia da 0.15 (BALANCEAMENTO-ECONOMIA), vila só com cama (#21), LAN (#25), lote 2 (#7) | pr47 (via consolidação Codex); PRs continuam abertos |
| `codex/interzone-logistics` | 07/11 23:33 → 08/11 00:00 | Codex nuvem (autor "Jefter Pereira") | Terminais, drones e caravanas | pr47 (primeiro pai); PR #28 aberto |
| `codex/machine-identity` | 08/11 00:25 | Codex nuvem | 34 formas e 27 silhuetas | pr47; PR #31 |
| Codex local 0.15.1–0.15.3 | 08/11 madrugada | Codex local | Tutorial na tela e integração da casa do fundador (zip `E:/veio-casa-do-fundador.zip`) | pr47 via `fd2a446` |
| `claude/progressao-fases` | 08/11 05:35 | "Claude (MineFac)" | Fases com pilha, unidade de 30/min, quadro da fábrica | pr47 via consolidação Codex |
| `claude/ajustes-vila-mobs` 0.16.0 | 08/11 madrugada | Claude local | **Segunda 0.16.0**: casa + quartel + Grok + interzone sobre `cf37004` | zip `E:/minefac-0.16.0.zip` → Codex → pr47 |
| Megafábricas | 08/11 13:16 | Codex local | Copiadora, marco de obra, elevador, escadas | pr47 |
| `codex/consolidated-016` → `checkpoint-consolidacao` | 08/11 18:11 | Codex local | Consolidação 0.16.1/0.17.2 (248 testes). Integração **interrompida a pedido do usuário** | pr47 via `5c57405` |
| Sons | 08/11 16:42 / 17:11 | Codex nuvem | Áudio posicional, ambiente e movimento | pr47; PRs #49 e #50 abertos |
| Obras (`cloud/obras-planta 4a096e9`) | 08/11, horário ND | Cloud (ficou sem tokens) | Plantas, canteiro, prestígio, pedidos, decretos, staff, pesquisas da vila | Nunca enviada; zip enviado pelo dono → `f1452f5` no pr47 |
| PR #40 | 08/11 15:55 → 17:06 | Claude (thread "agricultura") | Água, rios, grande indústria, estações, âncora, laboratório, monitor | Fundido no pr47 (`eedfd47`, 19:06) |
| #47 | 08/11 17:41 → 19:29 | Claude (thread "integração") | `rail_terminal`, merges de 0.25/sons/obras/#40/Codex | PR #47 aberto |
| Mapa finito | 08/11 18:16 | Codex local (cherry-pick `c8e94dc`) | Rascunho desligado em `work-in-progress/finite-world` | pr47 `80980db` |

### 1.4 Colisões de versão
Todas E3/git:

1. **Duas 0.16.0 diferentes:** a da main (`939c552`) e a de `claude/ajustes-vila-mobs` (`2a74e44`/`3cd563d`).
2. O **PR #40** foi renumerado de 0.18 para 0.19, 0.20, 0.22, 0.23 e por fim 0.25. A main usou 0.18–0.25 para sociedade e economia ao mesmo tempo.
3. O **Codex local** numerou 0.15.1 e 0.15.3, e o "Realismo 1.1" tem numeração própria.
4. O **pr47** tem conteúdo 0.25.0 e launcher 0.16.1 (`launcher/Cargo.toml`). O stamp do launcher no pr47 é `"{version}\nprofile=performance"` (`launcher/src/main.rs:56-57`); na main é só a versão (`main.rs:52`). Isso força recompilação ao trocar de camada mesmo com a mesma versão 0.25.0 (a confirmar pelo OPUS A).
5. **`release/Veio-0.25.0.zip` existe nas duas camadas com conteúdo diferente** (E2: blobs diferentes). Um jogador não consegue distinguir uma build da outra pelo nome.

---

## 2. Registro de decisões (DEC-NNN)

Regra de leitura:
- **Dono** = texto do Luiz no chat de 08/11 (E4, fonte primária).
- **"Pedido do usuário" (E3)** = um agente escreveu num doc que o usuário pediu, sem registro direto. Vale como indício, não como aprovação verificada.
- Na coluna **Código**: `ok` = verifiquei que o código acompanha (E2); `parcial`; `não`; `OPUS A` = a confirmar.

### 2.1 Fundamentos (identidade e plataforma)

| ID | Data | Quem | Tipo | Decisão | Fonte | Código | Canônico |
|---|---|---|---|---|---|---|---|
| DEC-001 | ≤07/11; reafirmada em 08/11 16:27 | Dono | Decisão aprovada | O Veio é um **jogo de fábrica tipo Factorio/Satisfactory dentro do Minecraft** | Chat 16:27 (E4); README 0.6 "Automação industrial no mundo procedural do Minecraft" (E3) | ok (base inteira) | Sim |
| DEC-002 | ≤07/11 | ND (projeto anterior ao git) | Decisão de fato | Plataforma **Horizon** (Minecraft em Rust, 26.3); mod nativo em Rust (`native/api`) mais conteúdo em `content/` | README; CONTRIBUTING (E3) | ok | Sim |
| DEC-003 | 07/11 (0.6) | Codex local | Decisão de agente (não confirmada pelo dono) | O conteúdo é **gerado por Python** (`tools/build_*`); `production/data.rs` é gerado a partir de `tools/production.json` | CONTRIBUTING (E3) | ok | Sim |
| DEC-004 | 07/11 | Usuário, segundo agente | Pedido do usuário (E3) | `launcher.exe` e o pacote `release/` ficam versionados no Git | CONTRIBUTING: "por pedido do usuário" (E3) | ok | Sim |
| DEC-005 | 07/11 | Usuário, segundo agente | Pedido do usuário (E3) | **A validação no jogo fica com o jogador**; evitar testes no jogo a cada iteração | README pr47 "Instalação e dados"; CODEX.md: "Deixar o teste dentro do jogo com o usuário" (E3) | n/a | Vigente. Consequência: **nada foi testado em partida desde a 0.11** (README: "As entregas da 0.11.0 à 0.25.0 aguardam teste no jogo") |
| DEC-006 | 07/11 | Dono, segundo MAPA.md | Decisão aprovada (só E3) | **Mundo finito de 15.000 × 15.000**, centro (0,0), muralha de bedrock, todos os biomas e recursos garantidos por geração adaptada, Nether de 1.875 de lado (8:1) e portais seguros. Substituiu a estimativa inicial de "15 minutos correndo" | MAPA.md: "decisão de produto"; "autorizada pelo usuário" (E3). No chat de 08/11 20:09 o dono pede "limite do mapa e otimizar" (E4) | **não**: rascunho desligado (`work-in-progress/finite-world`); `native_chunk_generate` não existe no Horizon `66f3981`; auditoria do Nether falha (CONTINUAR.md) | Contrato vigente; implementação Em desenvolvimento |
| DEC-007 | 07/11 20:30 | Codex local; "frente atribuída pelo usuário" (CLOUD.md, GROK.md) | Decisão de agente com indício de pedido do usuário | **Três frentes:** Cloud = trabalhadores; Grok = desempenho/balanço; Codex local = expansão e integração | COORDENACAO.md (E3) | n/a | Substituída de fato: hoje o integrador é o Claude do #47 (DEC-041) |
| DEC-008 | 07/11 | Codex local | Decisão de agente | "Não subir versão nas frentes; o Codex integra" (COORDENACAO) **×** "subir a versão a cada entrega que mude `native/`" (CONTRIBUTING) | E3 | n/a | **Conflito não resolvido** → B6-Q8 |
| DEC-009 | 07/11 | Codex local / Grok | Decisão de agente | Restrições do Horizon: worldgen não vem de datapack; rios são traçados em tempo de execução sobre chunks novos | Memória (E4); AGUA-E-RIOS (E3) | OPUS A (APIs `chunk_load`, `read_columns` e `world_dir` não verificadas no Horizon real) | Sim |

### 2.2 Produção, progressão e balanço

| ID | Data | Quem | Tipo | Decisão | Fonte | Código | Canônico |
|---|---|---|---|---|---|---|---|
| DEC-010 | 07/11 20:04 (0.9) | Claude local | Decisão de agente / implementada | **Pureza da jazida por conexão 3D:** 1–2 blocos impura (×0,5), 3–6 normal, 7–15 pura (×2) | README "Produção (0.9)" (E3) | **[B7] parcial:** a main conta o 3×3 sem conexão; a medida 3D conectada só existe no pr47 (`extraction.rs:34-57`, E2 do A) | Sim (pr47) |
| DEC-011 | 07/11 noite | "Pedido" (Shapez) por agente; PR #23 | Proposta | **Jazidas conectadas e espaçadas** inspiradas em Shapez: (1) leitura 3D com teto; (2) depósitos maiores em terreno novo | JAZIDAS-CONECTADAS.md: "é uma proposta" (E3) | parcial: classe **industrial ≥16 blocos (×4)** está no README do pr47; o item (2) não foi feito | (1) canônico no pr47; (2) Concebido |
| DEC-012 | 07/11 ~23h | Grok (#19) | Decisão de agente | **Prensa em 80 ticks:** "a primeira linha pede duas prensas por veio" | GROK-BALANCE.md (E3) | **não**: o pr47 usa 30t (`tools/production.json`) | **Substituída** por DEC-045 |
| DEC-013 | 07/11 ~23h | Grok/Jefter (#20) | Decisão de agente | Missões rebalanceadas; projetos industriais; **módulo de produtividade +20% a cada 5 ciclos** | BALANCEAMENTO-MISSOES (E3); README (E3) | **[B7]** produtividade: main 10 % (a cada 10 ciclos), pr47 20 % (`upgrades.rs:35`, c5db4f6 "Jefter Pereira") | Decisão de agente **não confirmada** → B6-Q14 |
| DEC-014 | 08/11 05:35 | "Claude (MineFac)" | Decisão de agente | **Progressão por fases:** a passagem de fase pede uma pilha (32 engrenagens, 16 aço, 16 circuitos); unidade de 30/min; tempos mais longos (prensa 40t etc.) | PROGRESSAO-FASES.md (E3) | Os gates de contagem estão no pr47 (avanços engineering/steel/circuits; E2 resumido em B2 SIS-PRG). **Os tempos não** | Gates: vigentes. Tempos: **revogados** por DEC-045 |
| DEC-015 | 08/11 (consolidação) | Codex local | Decisão de agente | "A progressão por fases **substitui os tempos anteriores**" | CONSOLIDACAO.md (E3) | não (pr47 com tempos 0.25) | **Revogada pelo dono** (DEC-045) |
| DEC-016 | 08/11 madrugada | Claude (MineFac) / Codex | Decisão de agente | **Quadro da fábrica:** 5 fases (16 chapas → 32 engrenagens → 16 aço → 16 circuitos → 32 circuitos), licenças `permit_*` e pedido diário que paga esmeraldas | EXPERIENCIA-JOGADOR (E3); `milestones/mod.rs:37-51`, `:146-151` @pr47 (E2) | ok | Integrado no pr47; emissão de moeda em conflito com DEC-030 (ver §3.5) |
| DEC-017 | 08/11 16:32 | **Dono** | Decisão aprovada | "**Conserte essas lacunas**": aprovou o documento de lacunas (6 lacunas maiores: progressão por item, cadeia rasa, fábrica longe para, falta de planejamento, falta de visão global, falta de meta final) | Chat 16:32 (E4); artefato `UhUqLwEVPnRrKFzKbK6NMD` (E3) | ok/parcial (#40 → pr47) | Sim |
| DEC-018 | 08/11 ~16:40–17:06 | Claude (#40) | Decisão de agente que implementa DEC-017 | **Laboratório de pesquisa:** 4 pesquisas (metallurgy, electronics, automation, scale) de 128–256 peças com consumo contínuo; desloca as liberações dos avanços steel/circuits para a pesquisa | GRANDE-INDUSTRIA (E3); avanços com gatilho `impossible` no pr47 (E2) | ok no pr47 | Vigente no pr47. **Coexiste com DEC-014/016 sem coordenação** → B6-Q3 |
| DEC-019 | 08/11 ~17h | Claude (#40) | Decisão de agente | **Cadeia mais funda:** motor (100t), processador (160t), núcleo de automação (300t), kit Mk.II (240t); Mk.II = ciclo ×0,5, energia ×2, poluição ×1,25 | GRANDE-INDUSTRIA (E3); `production.json` @pr47 (E2) | ok | Vigente |
| DEC-020 | 08/11 ~17h | Claude (#40) | Decisão de agente | **Grande projeto em 4 fases** com recompensas em esmeraldas, velocidades, produtividade, kits, estrelas e faróis | GRANDE-INDUSTRIA (E3) | ok (OPUS A: origem das esmeraldas) | Vigente; emissão de moeda em conflito com a economia fechada |
| DEC-021 | 08/11 16:38 | **Dono** | Decisão aprovada | **"Razões visíveis":** usar os mecanismos de Satisfactory/Factorio (itens por minuto no guia e no painel) | Chat 16:38 (E4) | parcial: painel por minuto existe desde a 0.5; o guia de indústria mostra razões (E3) | Sim |
| DEC-022 | ≤0.5 / 0.7 | Codex/Claude local | Decisão de agente | **Exportação contínua da central:** 4 circuitos + 4 aço → **4 esmeraldas** + 2 frascos, sem fim; armazém do povoado paga 1 esmeralda por 60 pontos de comida (+50% com 3 aldeões a 48 blocos) | README (E3) | OPUS A | **Conflita** com a economia fechada (DEC-030) → B6-Q1 |
| DEC-023 | README "Direção de jogo" | Agente (ND) | Ideia | **Meta de produção sustentada:** 20 circuitos/min durante 3 min; meta energética de atravessar uma noite | README (E3); doc de lacunas (proposta) | não | Concebido |

### 2.3 Aldeões, vila, sociedade e economia

| ID | Data | Quem | Tipo | Decisão | Fonte | Código | Canônico |
|---|---|---|---|---|---|---|---|
| DEC-024 | 07/11 (0.10–0.12) | Cloud; "frente atribuída pelo usuário" | Decisão de agente com indício de pedido | **Aldeões contratados** (lavrador, carregador, capataz, pastor) por 1 esmeralda, com comida de 2 pontos por tarefa e fila de tarefas | TRABALHADORES.md (E3); CLOUD.md (E3) | OPUS A | Sim |
| DEC-025 | 07/11 (Grok lote 3) | Grok | Decisão de agente | **A comida não fecha o ciclo:** pão paga 3 tarefas; trigo cru não alimenta | GROK-LOTE3 (E3) | OPUS A | Sim |
| DEC-026 | 07/11 22:10 → 22:33 | Cloud | Decisão de agente | **Teto da vila:** 30 (0.13) → 150 (0.14) com teto dinâmico (30 + 3/posto + 5/nível de zona + 2/armazém + 20/centro, máx. 150) | TRABALHADORES (E3); `village.rs:121` @pr47 (E2) | ok | Ver reconciliação §3.8 |
| DEC-027 | 07/11 (#21) | Grok; "regra de moradia" | Decisão de agente | **A vila só cresce com cama livre** (conta a cabeceira; raio 32) | VILA-CIDADE (E3); README pr47 (E3) | pr47: OPUS A; main: não | Vigente no pr47 |
| DEC-028 | 08/11 05:26 | Cloud | Decisão de agente | **Os aldeões andam pelo cérebro vanilla**; descanso noturno dos civis | 0.16.0 (E3); `workforce/mod.rs:83` `RESTING = 28` @pr47 (E2) | ok | Sim |
| DEC-029 | 08/11 16:06 | Cloud | Decisão de agente | **Sociedade:** casas, famílias, lojas, prefeito, fases Povoado/Vila/Cidade/Metrópole, IDH/Gini/PIB | SOCIEDADE.md (E3); `mayor.rs` @pr47 (E2) | ok | Sim |
| DEC-030 | 08/11 16:48 (0.21) | Cloud | Decisão de agente | **Economia fechada:** esmeralda = 60 moedas; o dinheiro só circula entre 4 livros (Companhia, Lojas, Prefeitura, Famílias); imposto retido na fonte; moeda só entra ou sai por esmeraldas físicas | README 0.21 (E3); `ledger.rs` @pr47 | OPUS A (identidade contábil) | **Princípio canônico**, mas com exceções não decididas → B6-Q1 |
| DEC-031 | 08/11 17:13 (0.23) | Cloud | Decisão de agente | **Comércio exterior** no estilo das conexões externas de Cities: Skylines: porto, estação, bolsa mundial, contratos, reputação, importação | README 0.23 (E3); `trade.rs` @pr47 (E2) | ok | Sim |
| DEC-032 | 08/11 17:29 (0.25) | Cloud | Decisão de agente | **Balanço:** preço único dentro e fora; salário = 2 × custo de vida ÷ 20 turnos; atacado a 75%; teto pela importação; piso de sucata de 25% | ECONOMIA.md (E3); `society/mod.rs`, `market.rs` @pr47 (E2); `sim_economy.py --check` = "Invariantes de equilíbrio: ok" (**E1**, modelo Python) | ok | Sim |
| DEC-033 | 08/11 (obras, 7ª–9ª entregas) | Cloud (agente das obras) | Decisão de agente | **Obras da vila** com prédio desenhado; **o efeito só vale com o prédio de pé**; prestígio com níveis 0/60/200/500; pedidos diários; caixa; decretos | HANDOFF_OBRAS (E3); `projects.rs:42-140`, `:75` @pr47 (E2) | ok | Vigente no pr47 |
| DEC-034 | 08/11 17:13 | **Dono** | Decisão aprovada | **Integrar a build parcial de obras**; "se tiver conflito pergunte para mim" | Chat 17:13 (E4) | ok (`f1452f5`) | Sim |
| DEC-035 | 08/11 17:18 | **Dono**, contra a recomendação do agente | Decisão aprovada | **Recompensas dos pedidos da vila e rendimento da obra Prefeitura pagos pelo caixa público.** Sem prefeitura ou sem saldo, só prestígio. Opção recomendada pelo agente e recusada: "entrada externa" | Chat 17:18 (E4); `workforce/center.rs:609` `reward()` tira do livro Public e credita a Company @pr47 (E2) | ok | Sim, **restrita a esses dois fluxos** |
| DEC-036 | 08/11 madrugada | Claude local (`ajustes-vila-mobs`) | Decisão de agente | **Casa do fundador:** casa mobiliada gerada uma vez por mundo, centro de controle, lareira, sofá, cama, mural; **+15% em casa** (HOME_TIME 0,87); refeição 0,95 por 5 min; banquete 0,91 por 8 min; duas camas | TRABALHADORES "sexta entrega" (E3); CASA-FUNDADOR-INTEGRACAO (E3) | Bônus de +15% removido no pr47 | **Substituída em parte** por DEC-046 |
| DEC-037 | 08/11 (obras) | Cloud | Decisão de agente | **Lar `veio:home`:** lareira, conforto 0–100, melhorias nível 2–4, dono, renascimento | TRABALHADORES (E3) | OPUS A | Vigente no pr47 dentro da casa do fundador |
| DEC-038 | 07/11 23:03 | Pedido do usuário relatado pelo Grok/Cursor | Proposta | **Vila que vira cidade:** casas variadas com profissão visível, praça-mercado, vida entre cama, sino e banca | VILA-CIDADE.md (E3) | não | Concebido (parte superada pela sociedade 0.18) |

### 2.4 Distância, logística, transporte e mundo

| ID | Data | Quem | Tipo | Decisão | Fonte | Código | Canônico |
|---|---|---|---|---|---|---|---|
| DEC-039 | 08/11 15:55 | **Dono** | Decisão aprovada | **Agricultura real** (irrigação, necessidades), **água e energia** (bombas, canos, usinas reais, hidrelétrica) e **rios** com nascente nas montanhas, afluentes, quedas e foz no mar; explicar no tutorial; **não remodelar do zero** o sistema do Minecraft | Chat 15:55 (E4); AGUA-E-RIOS: "Pedido do Luiz" (E3) | ok no pr47 (#40); OPUS A (APIs) | Sim |
| DEC-040 | 08/11 16:38 | **Dono** (escolheu a opção recomendada 0 entre âncora / produção estimada offline / deixar) | Decisão aprovada | **Âncora de chunk com custo de energia** (32 FE/t, forceload) | Chat 16:38 (E4); `industry.rs:21` `ANCHOR_POWER = 32` @pr47 (E2) | ok | Vigente como **opcional** (ver DEC-042) |
| DEC-041 | 08/11 17:01 | **Dono** | Decisão aprovada | **Fábrica distante continua produzindo e entregando "sem carregar tudo"**, como no Satisfactory, com leveza | Chat 17:01 e 17:03 (E4) | ok: `stations.rs` @pr47 | Sim. **Canônico sobre a âncora** |
| DEC-042 | 08/11 17:02 → 17:08 | Claude (#40) | Proposta abandonada | **Recuperação por máquina ao voltar** ("catch-up"). Abandonada porque "contaria duas vezes" com as estações | Chat 17:08 (E4); commit `e36ee2e` (E3) | não | **Abandonada** |
| DEC-043 | 08/11 ~17:06 | Claude (#40) | Decisão de agente que implementa DEC-041 | **Estações de carga/descarga** com produção virtual depois de 100 ticks sem rodar; 8 itens a cada 8 ticks; 10 FE/item; canal por corante de 4.096 itens; `estacoes.txt` | GRANDE-INDUSTRIA (E3); `stations.rs` (E2 resumido) | **[B7] parcial:** `simulate` (`stations.rs:117-140`) cria itens **sem insumo nem energia**, continua com o posto desmontado, e os canais são globais entre dimensões (E2 do A). Custo: 194 µs por 1.000 estações (E1 do A) | Mecanismo sim; **regra de custo aberta** → B6-Q5 |
| DEC-044 | 08/11 17:41 | **Dono** (opção 0) | Decisão aprovada | O terminal ferroviário do Codex passa a ser **`veio:rail_terminal`**; `veio:freight_station` fica com a estação de carga do comércio exterior | Chat 17:41 (E4); `a79f326` (E3) | OPUS A (varredura de IDs) | Sim |
| DEC-045 | 08/11 19:16 | **Dono** (opção 0) | Decisão aprovada | **Tempos de receita da 0.25**, não os das fases do Codex. Motivo do agente: "a cadeia da grande indústria depende desses tempos" | Chat 19:16 (E4); `tools/production.json` @pr47: prensa 30t, montador 40t (E2) | ok no JSON; OPUS A no `data.rs` regenerado | Sim. **Os docs PROGRESSAO-FASES e o README §Progressão ficaram desatualizados** |
| DEC-046 | 08/11 19:16 | **Dono** (opção 0) | Decisão aprovada | **Casa do fundador do Codex com o Lar (`veio:home`) dentro**; refeição e bônus de casa só pelo Lar; o bônus de comer fora do Codex permanece | Chat 19:16 (E4); `tools/build_house.py:177-178` (lareira = `veio:home`); `home/mod.rs:101` (`time_factor` = só `food::time_factor`) @pr47 (E2) | ok | Sim. **README do pr47 (linha 21) ainda diz "+15%"** |
| DEC-047 | 08/11 17:51 | **Dono** (opção 0) | Decisão aprovada | **Sons, caravanas e formatos de máquina entram no PR #47** com as obras (um PR só) | Chat 17:51 (E4) | ok (merges `a245f28`, `e5c837a`) | Sim |
| DEC-048 | 08/11 19:04 / 19:14 | **Dono** | Decisão aprovada | **"Junte tudo":** #40, PRs do Grok e consolidação Codex `50995e6` no #47 | Chat 19:04, 19:14 (E4) | ok (`eedfd47`, `5c57405`) | Sim |
| DEC-049 | 08/11 (antes de 18:11) | Usuário, segundo o Codex | Pedido do usuário (E3) | **Interromper a consolidação do Codex local** ("não apresentar os ZIPs anteriores como a build final") | CHECKPOINT-INTEGRACAO.md (E3) | n/a | Vigente. O integrador passou a ser o Claude do #47 |
| DEC-050 | 08/11 | Claude (#40) | Decisão de agente | **Água é NBT própria em mB**, não tanques do Horizon | Memória (E4); AGUA-E-RIOS (E3) | OPUS A | Sim |
| DEC-051 | 07/11 | Codex nuvem | Decisão de agente | **Caravanas** com rotas de até 512 blocos que **dependem de áreas simuladas** | TRANSPORTES.md; README pr47 linha 49 (E3) | OPUS A | Em conflito com DEC-041 (não usam produção virtual) → B6-Q5 |
| DEC-052 | 08/11 | Integrador #47 | Decisão de agente | **Status no painel:** descanso 28, casa 29–31, conexões externas 32–35, canteiro 36–39, frete/caravana 40–43 (tabela global `STATUS_KEYS`, 44 entradas) | `presentation.rs:19-34` @pr47 (E2) | ok | Sim (ver §3.9) |
| DEC-053 | 08/11 (obras) | Agente das obras | Decisão de agente | **Centro de controle com 8 slots:** 0 comida; 1–3 metas; 4 cofre; 5 saque; 6 obra; 7 decreto | HANDOFF_OBRAS (E3); `projects.rs:68-70` (`SITE = 6`, `DECREE = 7`), `center.rs:29` (`GOAL_SLOTS = 1..4`) @pr47 (E2) | ok | Sim |

### 2.5 Apresentação, desempenho e plataforma

| ID | Data | Quem | Tipo | Decisão | Fonte | Código | Canônico |
|---|---|---|---|---|---|---|---|
| DEC-054 | 07/11 19:10 (0.7) | Claude local | Decisão de agente | **Visual no estilo Minecraft:** painéis como baú/fornalha, texturas desenhadas em texto (`build_textures.py`) | 0.7 (E3); README "Visual" | ok | Base canônica |
| DEC-055 | 08/11 madrugada | Codex local | Decisão de agente | **Realismo leve 1.1 opcional:** shader WGSL MIT em `visuals/veio-industrial`, perfis Realismo (8 chunks, simulação 5, 85%) e Leve (6 chunks, 75%), escolhidos antes de jogar | CREDITS.md; README pr47 (E3) | OPUS A | Vigente como opcional |
| DEC-056 | 08/11 20:09 | **Dono** | Decisão aprovada (prioridade) | **"Realismo maior sem pesar o PC"**, com referência de PC de 8 GB ("é Rust, dá para pôr muito mais"), e **limite do mapa + otimização** | Chat 20:09 (E4) | Em desenvolvimento (2 threads "working") | Sim. Relação com DEC-054/055 → B6-Q7 |
| DEC-057 | 07/11–08/11 | Codex local | Decisão de agente | **Orçamento global de trabalho de fundo** por categoria (elétrica 512, agricultura 384, fios 4.096, coordenação 2.048, alvos 512, carregadores 1.024), com fila FIFO | ORCAMENTO-GLOBAL (E3) | OPUS A | Sim |
| DEC-058 | 08/11 | Codex/integrador | Decisão de agente | **Guia principal limitado a 100 páginas** (check_project); terminais só no livro Transportes; página com no máximo 14 linhas | Memória (E4); CHECKPOINT (E3) | ok (verificador) | Vigente; 29 páginas do Guia da Vila acima de 14 linhas (aviso aberto) |
| DEC-059 | 07/11 (0.8) | Claude local | Decisão de agente | **Poluição em memória**, recomeça ao reabrir o mundo | README (E3) | OPUS A | Vigente (limitação conhecida) |
| DEC-060 | 08/11 05:53 (0.17) | Cloud | Decisão de agente | **Ondas:** arco a 26 blocos, arqueiros a partir da 3ª noite, Bruto a partir da 5ª | README main (E3); `pollution.rs:44,47` (`ARCHERS_FROM = 3`, `ARRIVAL = 26.0`) @pr47 (E2) | ok | Sim. **O README do pr47 voltou ao texto antigo ("a 64 blocos")**: regressão só de documentação |
| DEC-061 | 07/11 (Grok #25) | Grok | Decisão de agente | **LAN:** convidados vão para a vila do anfitrião | README pr47 (E3) | OPUS A | Vigente no pr47 |
| DEC-062 | 08/11 20:34 | Thread "Aumentar o realismo sem pesar o PC" | Decisão de agente (não confirmada pelo dono) | PR #51: **Veio 0.26.0**, Realismo 2.1, **clima e estações do ano** (`ambience.rs`, `climate.rs`, 9 shaders, `release/Veio-0.26.0.zip`), base pr47 | A4 (E3); coordenador (E4) | Não auditado | Pendente. Conflito de versão com a recomendação de B6-Q8; o efeito do clima no jogo é ND (B6-Q7); "estações" colide com estações de carga (B7 §4) |
| DEC-063 | 08/11 20:35 | Thread "Definir limite do mapa e otimizar" | Decisão de agente (não confirmada pelo dono) | PR #52: sons só perto do jogador, mural ocioso, medição de escala, base pr47 | A4 (E3) | Não auditado | Pendente. Relevante para a regressão de desempenho do pr47 (B7 §3) |

---

## 3. Reconciliação dos conflitos pedidos

| # | Conflito | O que dizem as fontes | Reconciliação proposta (canônico) | Situação |
|---|---|---|---|---|
| 3.1 | **Casa do fundador × Lar** | DEC-036: +15% em casa (HOME_TIME 0,87), refeição/banquete na casa. DEC-037: o Lar `veio:home` com conforto e melhorias. O dono decidiu em 19:16 | **Canônico (dono, DEC-046):** a casa do fundador é o edifício gerado no spawn; a lareira é o bloco `veio:home` (`build_house.py:177-178`); bônus de ritmo só pelo Lar (refeição/banquete via `food::time_factor`, `home/mod.rs:101`). **O +15% de "estar em casa" deixou de existir.** Comer fora do Codex continua valendo | **Reconciliado.** Faltam ajustes de doc: README pr47 linha 21 e TRABALHADORES "sexta entrega" |
| 3.2 | **Tempos 0.25 × fases do Codex** | PROGRESSAO-FASES: prensa 40t e linhas de 30/min. GROK-BALANCE: prensa 80t. CONSOLIDACAO: "fases substituem os tempos". Dono em 19:16: tempos da 0.25 | **Canônico:** tempos da 0.25 (prensa 30t, montador 40t em `production.json` @pr47). Os **gates de fase** (32/16/16) continuam como portas de avanço, sem os tempos do Codex. A "unidade de 30/min" e "duas prensas por veio" **não valem mais** | **Reconciliado no código.** Não reconciliado nos docs: README §Progressão, PROGRESSAO-FASES, GROK-BALANCE e README linha 24 ("a primeira linha pede duas prensas") |
| 3.3 | **`freight_station` × `rail_terminal`** | Dois blocos com o mesmo ID (Codex: trem entre zonas; Cloud: estação de comércio exterior) | **Canônico (dono, DEC-044):** `veio:rail_terminal` para o trem do Codex; `veio:freight_station` para o comércio exterior | **Reconciliado** (`a79f326`). Mundos com o terminal antigo: migração ND (OPUS A) |
| 3.4 | **Âncora × estações** | 16:38 o dono escolhe a âncora. 17:01 esclarece: fábrica longe continua "sem carregar tudo" | **Canônico:** a **estação de carga/descarga com produção virtual** é o mecanismo principal de fábrica distante (DEC-041/043). A âncora (32 FE/t, forceload) fica **opcional**, para quem quer simulação real de uma área. A recuperação por máquina foi abandonada (DEC-042) | **Reconciliado** como interpretação; a escolha de 16:38 não foi revogada, foi complementada. **Pendente:** caravanas e drones não usam produção virtual (B6-Q5) |
| 3.5 | **Recompensas pela prefeitura × moeda criada** | DEC-035 (dono): pedidos da vila e juros da Prefeitura saem do caixa público. Ao mesmo tempo, emitem esmeraldas fora do livro: armazém do povoado (DEC-022), exportação contínua da central (4 esm), contrato Povoado (8 esm), **pedido diário do quadro** (`milestones/mod.rs:146-151` adiciona esmeraldas reais ao slot 1, E2), grande projeto (32/48/64 esm), Mk.II minerando esmeralda (1 a cada 20 s, 4.800 FE) e trocas vanilla | **Canônico parcial:** a regra do dono cobre só os dois fluxos que ele decidiu. **As demais fontes nunca passaram pelo dono e contradizem o princípio da economia fechada (DEC-030).** Proponho classificá-las como "fontes externas de moeda" explícitas, ou removê-las, por decisão do dono | **Não resolvido** → B6-Q1 (decisão de maior impacto) |
| 3.6 | **Fábrica parada × produção virtual** | Antes do #40, fora dos chunks carregados a fábrica parava (lacuna 3). O dono pediu continuidade | **Canônico:** a produção virtual vale **só para o que passa por estações**: depois de 100 ticks sem rodar, a estação estima o fluxo. Máquinas soltas sem estação continuam paradas fora da simulação. Cumpre a regra da ordem Cloud ("progresso fora de simulação só com regra explícita e aprovada", CLOUD.md) | **[B7] Reaberto.** O OPUS A confirmou que a produção virtual **não** consome entradas nem energia: a fábrica distante vira uma fonte grátis. Isso diverge da intenção "continua produzindo" e da economia fechada → B6-Q5 |
| 3.7 | **Realismo leve × visual vanilla** | DEC-054: estilo Minecraft. DEC-055: shader opcional. Dono 20:09: "realismo maior sem pesar o PC" | **Canônico:** a base vanilla é obrigatória e jogável sem shader; o realismo é **camada opcional** escolhida no launcher (`Escolher-Visual.cmd`, `Jogar-Leve.cmd`) e não pode quebrar a leitura dos estados (luzes, placas). O teto de custo é o PC de 8 GB | **Parcial:** a thread de realismo está em andamento; "quanto realismo" e se vira padrão → B6-Q7 |
| 3.8 | **Limites de vila 30 → 150** | 0.13: 30. 0.14: teto dinâmico até 150. 0.18: limites por fase (30/60/100/150). Grok: só com cama livre. Obras: escola +15, estalagem +20, prestígio +5 por nível (somados em `projects::extra_cap`) | **Canônico (código pr47):** `população máx. = min(teto dinâmico + extra das obras, 150)` (`village.rs:121`), e nascimento só com o limite da fase e a vaga em casa (`society::birth_allowed`). Os bônus das obras **não passam de 150 e não furam o limite da fase** | **Reconciliado no código.** Questão de design: os bônus das obras podem não ter efeito numa vila madura → B6-Q6 |
| 3.9 | **Tamanho do guia** | check_project limita o guia principal a 100 páginas. Páginas acima de 14 linhas são cortadas. 29 páginas do Guia da Vila estão acima de 14. A consolidação corrigiu 30 páginas cortadas | **Canônico:** guia principal ≤ 100 páginas; página ≤ 14 linhas; Transportes num livro próprio; Grande Indústria num livro próprio | **Não resolvido:** o aviso de 29 páginas segue aberto. Há ainda duas páginas "8." no Guia da Vila e textos de guia contraditórios (PIB; "Comércio" ainda pela central) |

### 3.10 Outros conflitos encontrados

| # | Conflito | Evidência | Proposta |
|---|---|---|---|
| a | **Descanso 28 × 29** | COORDENACAO: "freight_route 28, resting 29". Merge `5c57405`: "rest 28, freight/caravan 40-43". Código pr47: `RESTING = 28` (`workforce/mod.rs:83`), `HOUSED = 29` (`society/mod.rs:391`), frete 40–43 (`freight.rs:15-18`) | **Canônico = código.** COORDENACAO está **histórico** (B5) |
| b | **Códigos 40/41 com dois significados** | `center.rs:39-40`: `ZONE_READY = 40`, `STORE_FULL = 41` (códigos de pendência do centro); `freight.rs:15-16`: `NO_ROUTE = 40`, `NO_ANIMAL = 41` (status). `center.rs:195-196` e `focus.rs:110` tratam 40/41 como zona/armazém | Possível ambiguidade se um terminal entrar na lista de pendências. **OPUS A confirmar**; registrar no REGISTRO de IDs (B5) |
| c | **Canteiro 32–35 → 33–36 → 36–39** | HANDOFF_OBRAS (E3); código `site.rs:42-45` = 36–39 (E2) | Canônico 36–39 |
| d | **Slots do centro 5 → 6 → 8** | HANDOFF (E3); código (E2) | Canônico 8 slots (DEC-053) |
| e | **Numeração das entregas** no TRABALHADORES: duas "Sexta" e duas "Sétima" | E3 | Renomear para IDs FUN (B5); não renumerar o histórico |
| f | **PIB:** o guia (pág. 33 da vila) diz "tarefas feitas e vendas"; SOCIEDADE e README dizem "consumo final no varejo" | E3 | Canônico = código (`mayor.rs`, OPUS A); corrigir o guia |
| g | **Dois "pedidos diários"** | Quadro da fábrica (Codex, paga esmeraldas criadas, em memória) × pedidos da vila (obras, 3 por dia, 4 com correio, pagos pelo caixa público) | B6-Q2 |
| h | **Três escadas de níveis com nomes parecidos** | Prefeitura (Povoado/Vila/Cidade/Metrópole), prestígio (Aldeia/Vila/Vila próspera/Cidade operária), quadro (fases 1–5), mais o nível de reputação comercial e os níveis de zona | B6-Q4 |
| i | **"Prefeitura" homônima** | Obra Prefeitura (`HALL`, prestígio +50%, juros) × bloco do prefeito (sociedade) | Renomear a obra (ex.: "Paço") ou fundir; B6-Q4 |
| j | **Itens do #40 fora da bolsa** | `trade.rs` PRODUCTS = 34 sem motor/processador/núcleo/kit Mk.II (E2) | Decidir se a grande indústria exporta (B6-Q1/Q3) |
| k | **README do pr47 desatualizado** em 4 pontos: fases/aço; +15% em casa; ondas a 64 blocos; "duas prensas" | E3 × E2 | Corrigir na publicação do #47 |
| l | **Ordem Cloud × produção virtual** | CLOUD.md: "Não produza progresso ilimitado fora de simulação sem regra explícita e aprovada" | Atendida por DEC-041 (dono) |
| m | **MAPA.md sem registro no chat** | O documento diz "decisão de produto" e "autorizada pelo usuário" (E3); no chat de 08/11 só existe "limite do mapa" (20:09) | Confirmar o tamanho de 15.000 com o dono (B6-Q9) |
| n | **"Caixa da vila" × livros** | Obras: caixa com saque de até 16 e juros de 1/16 por dia até 8 (TRABALHADORES, E3) × livros Company/Public (0.21) | OPUS A: o caixa é o livro Public, um quinto saldo ou esmeraldas físicas? |
| o | **Guia "Comércio"** ainda descreve a exportação pela central (4 esmeraldas) e não o porto/estação | lang pt_br @pr47 (E2) | Corrigir o guia |

---

## 4. Linha do tempo das decisões do dono em 08/11 (UTC)

| Hora | Mensagem (resumo) | Decisão |
|---|---|---|
| 15:55 | Pede agricultura real, água, energia e rios | DEC-039 |
| 16:27 | "Não tem build consolidada ainda"; ideia central de fábrica tipo Factorio/Satisfactory no Minecraft | DEC-001 |
| 16:32 | "Conserte essas lacunas" | DEC-017 |
| 16:38 | Âncora (opção 0); "razões visíveis" com mecanismos de Satisfactory/Factorio | DEC-040, DEC-021 |
| 17:01 / 17:03 | Comportamento Satisfactory para fábricas longe; leveza | DEC-041 |
| 17:13 | Integrar a build parcial de obras; perguntar nos conflitos | DEC-034 |
| 17:18 | Pagar pela prefeitura (opção 1, contra a recomendação) | DEC-035 |
| 17:36 / 17:38 | Sons e outras branches; aponta `7024938` | DEC-047 |
| 17:41 | `rail_terminal` | DEC-044 |
| 17:51 | Tudo no PR #47 | DEC-047 |
| 19:04 / 19:14 | Indica `50995e6`; "junte tudo, chegaram mais" | DEC-048 |
| 19:16 | Casa do fundador com o Lar; tempos da 0.25 | DEC-046, DEC-045 |
| 20:07 | "Não perdemos trabalho?" A auditoria do #47 diz que nada ficou de fora no GitHub; commits locais não enviados são invisíveis | (registro) |
| 20:09 | Realismo sem pesar o PC (8 GB); limite do mapa e otimização | DEC-056 |
| 20:12 | Missão Fable (esta consolidação) | (registro) |

Nenhuma decisão do dono revogou outra decisão do dono. As revogações foram de **decisões de agente** (DEC-012, DEC-015, parte da DEC-036).
