# 06 — Histórico e decisões

> Referência: `main@53267af` (0.25.0), `pr47@80980db` (candidata), `#51@a0b0d72`, `#52@0b827f6`. Data: 2026-11-08, cerca de 21h UTC. O que aconteceu entre 20:38 e 21:41 (PRs #53 e #54, primeira compilação real) entrou como registro preliminar: últimas linhas do §2.3, §2.4 item 7, §2.5 e §3.6.
> Este arquivo guarda a **cronologia**, o **registro de decisões** ([DEC-001](#dec-001) a [DEC-066](#dec-066)), as **reconciliações** e as **funcionalidades substituídas ou abandonadas**. Conflitos ainda abertos ficam em [09 §2](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf); perguntas ao dono, em [09 §5](09_CONFLITOS_LACUNAS_E_RISCOS.md#5-decisões-pendentes-do-dono).

## 1. Como ler

- **Horários em UTC.** Commits do Codex local e das máquinas do dono assinam em −0300; os do Claude nuvem e do Grok, em +0000. Tudo foi convertido para UTC, o fuso do chat.
- **Evidência:** E1 executado · E2 lido no código · E3 documento ou commit · E4 chat ou memória · ND não determinado · J teste em jogo (nenhum até hoje).
- **Quem decide.** Só o dono (Luiz) aprova. Cada DEC traz um destes tipos:

| Tipo | Significado |
|---|---|
| **Dono** | Texto do dono no chat de 08/11 (E4, fonte primária) |
| **Dono, só em documento** | Um agente escreveu que o usuário pediu, sem registro no chat. Vale como indício |
| **Decisão de agente (não confirmada pelo dono)** | Escolha de um agente. Vale enquanto o dono não disser o contrário |
| **Proposta** | Escrita como proposta; não aprovada |
| **Abandonada / Substituída / Revogada** | Fora de vigor |

- **Campos de cada DEC:** quando, quem, tipo, texto, fonte, FUN afetadas, estado do código, relação (substitui ou substituída por), canônico.
- **Nova DEC:** só se acrescenta; o número nunca é reaproveitado. O procedimento está em [11 §4](11_PLANO_DE_CONTINUIDADE.md#4-quando-atualizar-o-quê) e o modelo em [07 §7](07_AGENTES_E_WORKFLOW.md#7-registro-de-decisões).

## 2. Cronologia

### 2.1 Antes do git

O histórico começa na 0.6 (`5eaea2c`). As versões 0.1 a 0.5 aparecem só em textos (E3): a "progressão da 0.4" no README, a central de contratos da 0.5 e o mundo de demonstração `Veio03-Laboratorio`.

### 2.2 Linha da main

A main vai de 0.6 a 0.25 em **cerca de 23 horas**, de 07/11 18:24 a 08/11 17:29.

| Versão | UTC | Autor | Conteúdo | Commit |
|---|---|---|---|---|
| 0.6 | 07/11 18:24 | Codex local | Extratora, prensa, montador, gerador, cabos, Guia do Veio | `5eaea2c` |
| 0.7 | 07/11 19:10 | Claude local | Visual no estilo Minecraft, colheitadeira, Tesla, integração vanilla | `819ba92` |
| 0.8 | 07/11 19:37 | Claude local | Postes com fios aéreos, poluição e saqueadores, módulos | `7644aab` |
| 0.8.1 | 07/11 20:00 | Codex local | Unificador de duas entradas | `e8d3d62` |
| 0.9 | 07/11 20:04 | Claude local | Jazidas com pureza, receitas por tabela, renováveis, lavoura, rancho, comida | `5944d37` |
| 0.9.1 | 07/11 20:23–20:37 | Codex local | Esteiras com curvas e rampas, `LOGISTICA.md`, `COORDENACAO.md` (três frentes), interface de postos | `26599ec`, `9e8b601`, `20f9dba` |
| PR #1 | 07/11 20:40 | merge "bizi" | `codex/expansion` na main | `ec3db34` |
| 0.10.0 | 07/11 21:04 | Claude nuvem ("Cloud") | Consolida as três frentes; launcher no repositório | `9421883`, PR #3 |
| 0.11.0 | 07/11 21:34 | Cloud | Carregador e capataz | `903c4ec` |
| 0.12.0 | 07/11 21:43 | Cloud | Fila do capataz, pastor, Grok lote 3 | `6ac0c62`, `6fd1b44` |
| 0.13.0 | 07/11 22:10 | Cloud | Zonas, celeiro, silo, centro de controle; vila até 30 | `36e43b6` |
| 0.14.0 | 07/11 22:33 | Cloud | Vila até 150, spawn no centro da vila, Guia da Vila | `a691802` |
| 0.15.0 | 07/11 22:39 | Cloud | Build consolidada Cloud + Codex + Grok (lotes 1, 3–8) | `8abe435`, PR #17 |
| #22, #24, #27, #29 | 07/11 23:09–23:46 | Codex | Otimização, escala, orçamento global, topologia incremental | `65cab50` … `cf37004` |
| #32 | 08/11 05:08 | Codex | Capatazes incrementais | `5d76390` |
| 0.16.0 | 08/11 05:26 | Cloud | Cérebro vanilla, descanso noturno, vilas grandes mais baratas | `939c552` |
| 0.16.1 | 08/11 05:36 | Cloud | Carga visível nas mãos | `7208324` |
| 0.17.0 | 08/11 05:53 | Cloud | Ondas por um lado, arqueiros, bruto, quartel funcional | `0deee22` |
| 0.17.1 / 0.17.2 | 08/11 07:17 / 13:13 | Cloud | −36% de trabalho por tick, quadro sem alocação; esteira "andando" | `0472750`, `494d3e7` |
| #38 | 08/11 13:21 | Codex | Busca incremental dos carregadores | `5217b62` |
| 0.18.0 | 08/11 16:06 | Cloud | Sociedade: casas, lojas, prefeito, fases | `0aa3fbb` |
| 0.19.0 | 08/11 16:25 | Cloud | Contas, impostos, preços locais | `a48e14c` |
| 0.20.0 | 08/11 16:31 | Cloud | Holding | `95a1f7c` |
| 0.21.0 | 08/11 16:48 | Cloud | "Economia fechada", imposto na fonte | `9331dee` |
| 0.22.0 | 08/11 17:04 | Cloud | Comércio exterior pelo cais | `cbc53a9` |
| 0.23.0 | 08/11 17:13 | Cloud | Conexões externas no estilo Cities: Skylines | `e8ac991` |
| 0.24.0 | 08/11 17:20 | Cloud | Acabamento do comércio exterior | `b25b9d0` |
| **0.25.0** | 08/11 17:29 | Cloud | Balanço econômico; `tools/sim_economy.py` | `87ee998`; merge PR #48 `53267af` |

### 2.3 Linhas paralelas

| Linha | UTC | Agente | Conteúdo | Destino |
|---|---|---|---|---|
| `cursor/grok-*` e balanço | 07/11 noite | Grok (Cursor), commits "Jefter Pereira" | Duas prensas (#19), missões (#20), vila só com cama (#21), LAN (#25), lote 2 (#7) | pr47 via consolidação do Codex; PRs abertos |
| `codex/interzone-logistics` | 07/11 23:33 → 08/11 00:00 | Codex nuvem | Terminais, drones, caravanas | pr47; PR #28 aberto |
| `codex/machine-identity` | 08/11 00:25 | Codex nuvem | 34 formas, 27 silhuetas | pr47; PR #31 |
| Codex local 0.15.1–0.15.3 | 08/11 madrugada | Codex local | Tutorial na tela; integração da casa do fundador | pr47 via `fd2a446` |
| `claude/progressao-fases` | 08/11 05:35 | "Claude (MineFac)" | Fases com pilha, unidade de 30/min, quadro da fábrica | pr47 via Codex |
| `claude/ajustes-vila-mobs` 0.16.0 | 08/11 madrugada | Claude local | **Segunda 0.16.0**: casa, quartel, Grok, interzone sobre `cf37004` | zip → Codex → pr47 |
| Megafábricas | 08/11 13:16 | Codex local | Copiadora, marco, elevador, escadas | pr47 |
| Sons | 08/11 16:42 / 17:11 | Codex nuvem | Áudio posicional, ambiente, animação | pr47; PRs #49 e #50 abertos |
| PR #40 | 08/11 15:55 → 17:06 | Claude (thread "agricultura") | Água, rios, grande indústria, estações, âncora, laboratório, monitor | pr47 (`eedfd47`, 19:06) |
| Obras `cloud/obras-planta 4a096e9` | 08/11, hora ND | Cloud (sem tokens) | Plantas, canteiro, prestígio, pedidos, decretos, staff, pesquisa da vila | Nunca enviada; zip do dono → `f1452f5` no pr47 |
| `codex/consolidated-016` → `checkpoint-consolidacao` | 08/11 18:11 | Codex local | Consolidação 0.16.1/0.17.2 (248 testes); **interrompida a pedido do usuário** | pr47 via `5c57405` |
| PR #47 | 08/11 17:41 → 19:29 | Claude (thread "integração") | `rail_terminal` (`a79f326` 17:41); merges `a245f28` 17:46, `e5c837a` 17:47, `eedfd47` 19:06, `5c57405` 19:29 | Aberto |
| Mapa finito | 08/11 18:16 | Codex local | Rascunho desligado em `work-in-progress/finite-world` (`c8e94dc`, cherry-pick em `80980db`) | pr47 |
| PR #51 | 08/11 20:34 | Thread "Aumentar o realismo sem pesar o PC" | 0.26.0, Realismo 2.1, clima e estações do ano; base `claude/project-thread-mrtok9@a0b0d72` sobre o #47 | Aberto ([DEC-062](#dec-062)) |
| PR #52 | 08/11 20:35 | Thread "Definir limite do mapa e otimizar" | Sons perto do jogador, mural ocioso, bench de escala; base `claude/project-thread-e9lkha@0b827f6` sobre o #47 | Aberto ([DEC-063](#dec-063)) |
| PR #53 | 08/11 20:38 → 20:56 | Thread "Concluir pendências das obras da vila" (Opus 5.5, esforço médio) | Traduções de funcionários e pesquisas, menu do canteiro com 3 slots, guia da Vila, ruínas, proteção do território, rótulos de obra e decreto do centro; `claude/project-thread-6fbc15@5c9ffdc`, 20 arquivos, +1.611/−151, sobre o #47 | Aberto ([DEC-064](#dec-064)); não auditado |
| #51 depois do corte | 08/11 20:53 / 21:10 | Thread "Aumentar o realismo sem pesar o PC" | `b48a51f` (cores das estações do ano, neblina nativa, neve e gelo) e `c57bc66` ("Realismo 3.0": nuvens volumétricas, parallax, sombras suaves, água espelhada) | Aberto; não auditado |
| #52 depois do corte | 08/11 20:55 | Thread "Definir limite do mapa e otimizar" | `639a16f`: esteiras ociosas pulam ticks; energia dos postes em lotes de 4 ticks | Aberto; não medido |
| Pacote de teste combinado | 08/11 21:01 | Thread do #47, renomeada "Testar a build combinada no PC" | `Veio-0.26.0-teste-combinado.zip` = #47 + #51 + #52 + #53, só como zip; a thread relata 351 testes passando (E4) | Usado na primeira compilação real ([DEC-065](#dec-065)) |
| Primeira compilação real do #47 | 08/11 21:18 → 21:41 | Mesma thread, por controle remoto no PC do dono | Horizon `66f3981` do cache `E:\GPTmodMine\.build`; parou em 4 erros no código do Veio; correção `3f7cb28` no #47 (21:40) | Resultado da recompilação ND no corte |
| PR #54 | 08/11 21:35 / 21:36 | Claude "Cloud" (sessão `01APXd4y…`, conta `jefterpereira`) | Veio 0.27.0, agenda da vila: calendário, feriados, domingos, visitantes `veio:visitor`, obras cívicas, ânimo e lazer no IDH; `claude/trusting-ritchie-sc9ty8@2c49d41`, 56 arquivos, +2.632/−187, **sobre o #51** | Aberto ([DEC-066](#dec-066)); não auditado |

Detalhe de branches e PRs: [10](10_EVIDENCIAS_E_RASTREABILIDADE.md).

### 2.4 Colisões de versão

1. **Duas 0.16.0:** a da main (`939c552`) e a de `claude/ajustes-vila-mobs` (`2a74e44`/`3cd563d`).
2. **O PR #40 foi renumerado cinco vezes:** 0.18 → 0.19 → 0.20 → 0.22 → 0.23 → 0.25. Ao mesmo tempo, a main usou 0.18–0.25 para sociedade e economia.
3. **Codex local:** 0.15.1 e 0.15.3; "Realismo 1.1" com numeração própria.
4. **pr47:** conteúdo 0.25.0 (`mod-veio`), launcher 0.16.1. O carimbo do launcher é `"{version}\nprofile=performance"` no pr47 e só a versão na main, o que força recompilação ao trocar de camada.
5. **`release/Veio-0.25.0.zip` é o mesmo arquivo nas duas camadas** (blob `e01ef740`, sha256 `7dc57f01…`, E1). O pr47 não tem pacote próprio. `release/Veio-Realismo-1.1.zip` contém o Veio 0.16.1. O README do pr47 aponta para um `Veio-0.16.1.zip` que não existe.
6. **#51 já se declara 0.26.0** e traz `release/Veio-0.26.0.zip`.
7. **#54 se declara 0.27.0** e empilha sobre o #51. O pacote de teste combinado também se chama 0.26.0. Nada disso está na main, e o #47 continua com o conteúdo em 0.25.0.

Encaminhamento: [CNF-005](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-08](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera).

### 2.5 Decisões do dono em 08/11

| UTC | Mensagem (resumo) | DEC |
|---|---|---|
| 15:55 | Agricultura real, água, energia e rios; explicar no tutorial | [DEC-039](#dec-039) |
| 16:27 | "Não tem build consolidada ainda"; fábrica tipo Factorio/Satisfactory no Minecraft | [DEC-001](#dec-001) |
| 16:32 | "Conserte essas lacunas" | [DEC-017](#dec-017) |
| 16:38 | Âncora (opção 0); "razões visíveis" | [DEC-040](#dec-040), [DEC-021](#dec-021) |
| 17:01 / 17:03 | Fábrica distante como no Satisfactory, com leveza | [DEC-041](#dec-041) |
| 17:13 | Integrar a build parcial de obras; perguntar nos conflitos | [DEC-034](#dec-034) |
| 17:18 | Pagar pela prefeitura (opção 1, contra a recomendação) | [DEC-035](#dec-035) |
| 17:36 / 17:38 | Sons e outras branches; aponta `7024938` | [DEC-047](#dec-047) |
| 17:41 | `rail_terminal` | [DEC-044](#dec-044) |
| 17:51 | Tudo no PR #47 | [DEC-047](#dec-047) |
| 19:04 / 19:14 | Indica `50995e6`; "junte tudo, chegaram mais" | [DEC-048](#dec-048) |
| 19:16 | Casa do fundador com o Lar; tempos da 0.25 | [DEC-046](#dec-046), [DEC-045](#dec-045) |
| 20:07 | "Não perdemos trabalho?" A auditoria do #47 diz que nada ficou fora do GitHub; commits locais não enviados são invisíveis | (registro) |
| 20:09 | Realismo sem pesar o PC (8 GB); limite do mapa e otimização | [DEC-056](#dec-056) |
| 20:12 | Missão de consolidação documental | (registro) |
| 20:37 / 20:38 | Pergunta se as obras já estão no projeto; "crie um agente opus 5.5 no médio para prosseguir" | [DEC-064](#dec-064) |
| 20:59 | "Quero testar o jogo aqui localmente … na build consolidada e atual" | [DEC-065](#dec-065) |
| 21:10 / 21:23 | Pede que o agente use o PC dele; "pode rodar o launcher completo, que eu quero abrir o jogo" | (registro, dentro da [DEC-065](#dec-065)) |

**Nenhuma decisão do dono revogou outra decisão do dono.** As revogações foram de decisões de agente: [DEC-012](#dec-012), [DEC-015](#dec-015) e parte da [DEC-036](#dec-036).

**Quase tudo o que caracteriza o Veio hoje é decisão de agente.** Isso inclui sociedade, economia fechada, comércio exterior, obras, quadro da fábrica e laboratório ([DEC-016](#dec-016), [DEC-018](#dec-018), [DEC-029](#dec-029) a [DEC-033](#dec-033)). O dono confirmou a direção geral ([DEC-001](#dec-001), [DEC-017](#dec-017)) e decidiu pontos de integração.

## 3. Registro de decisões

### 3.1 Fundamentos e plataforma

#### DEC-001
**Jogo de fábrica tipo Factorio/Satisfactory dentro do Minecraft.** ≤ 07/11; reafirmada em 08/11 16:27 · **Dono**
- **Fonte:** chat 16:27 (E4); README 0.6, "Automação industrial no mundo procedural do Minecraft" (E3).
- **FUN:** todas. **Código:** acompanha. **Relação:** —. **Canônico:** sim.

#### DEC-002
**Plataforma Horizon** (Minecraft 26.3 em Rust); mod nativo em `native/api` e conteúdo em `content/`. ≤ 07/11 · Decisão de fato, autor ND
- **Fonte:** README, CONTRIBUTING (E3).
- **FUN:** SIS-PLT. **Código:** acompanha. **Canônico:** sim.

#### DEC-003
**Conteúdo gerado por Python;** `production/data.rs` sai de `tools/production.json`. 07/11 (0.6) · Codex local · Decisão de agente (não confirmada pelo dono)
- **Fonte:** CONTRIBUTING (E3).
- **FUN:** FUN-PLT-08, FUN-PRO-01. **Código:** acompanha; `data.rs` do pr47 regenerado com os tempos da 0.25 (E2). **Canônico:** sim.

#### DEC-004
**`launcher.exe` e `release/` versionados no Git.** 07/11 · Dono, só em documento
- **Fonte:** CONTRIBUTING, "por pedido do usuário" (E3).
- **FUN:** FUN-PLT-06, FUN-PLT-07. **Código:** acompanha. **Canônico:** sim.

#### DEC-005
**A validação em jogo fica com o jogador;** evitar testes no jogo a cada iteração. 07/11 · Dono, só em documento
- **Fonte:** README do pr47, "Instalação e dados"; `CODEX.md`, "Deixar o teste dentro do jogo com o usuário" (E3).
- **FUN:** todas (coluna J). **Código:** n/a. **Canônico:** vigente. **Consequência:** nada foi testado em partida desde a 0.11 ([R-41](09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).

#### DEC-006
**Mundo finito de 15.000 × 15.000,** centro (0,0), muralha de bedrock, biomas e recursos garantidos, Nether de 1.875 (8:1), portais seguros. 07/11 · **Atribuída ao dono só pelo próprio documento (E3), sem registro do dono no chat; não confirmada até a Q-09**
- **Fonte:** `MAPA.md`, "decisão de produto autorizada pelo usuário" (E3). No chat de 08/11 só há "limite do mapa e otimizar", em 20:09 (E4).
- **FUN:** FUN-MUN-04, FUN-MUN-05, FUN-DES-04. **Código:** rascunho desligado; `native_chunk_generate` não existe no Horizon `66f3981`; auditoria do Nether falha. **Canônico:** contrato em vigor até a [Q-09](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-09-mapa-finito); Em desenvolvimento. Ver [CNF-019](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf).

#### DEC-007
**Três frentes:** Cloud = trabalhadores; Grok = desempenho e balanço; Codex local = expansão e integração. 07/11 20:30 · Codex local, "frente atribuída pelo usuário" · Decisão de agente com indício de pedido
- **Fonte:** `COORDENACAO.md`, `CLOUD.md`, `GROK.md` (E3).
- **Código:** n/a. **Relação:** substituída de fato pelo integrador do #47 ([07](07_AGENTES_E_WORKFLOW.md)). **Canônico:** não.

#### DEC-008
**Quem sobe a versão:** "não subir versão nas frentes; o Codex integra" (COORDENACAO) × "subir a versão a cada entrega que mude `native/`" (CONTRIBUTING). 07/11 · Codex local · Decisão de agente (não confirmada pelo dono)
- **FUN:** FUN-PLT-06, FUN-PLT-07. **Canônico:** **conflito aberto** → [Q-08](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera), [CNF-005](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf).

#### DEC-009
**Restrições do Horizon:** worldgen não vem de data pack; rios são traçados em tempo de execução sobre chunks novos. 07/11 · Codex local e Grok · Decisão de agente
- **Fonte:** memória (E4); `frentes/AGUA-E-RIOS.md` (E3); `rivers.rs:4-8` (E2).
- **FUN:** FUN-MUN-03. **Código:** `chunk_load` existe como gancho no Horizon `66f3981` e `5f7cadf`; `read_columns` e `chunk_load` entraram em `bcfb9d5`, depois da última compilação real (`4ffb605`), e nunca foram compilados ([R-01](09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)). **Canônico:** sim.

### 3.2 Produção, progressão e balanço

#### DEC-010
**Pureza da jazida:** impura, normal, pura. 07/11 20:04 (0.9) · Claude local · Decisão de agente
- **Fonte:** README §Produção (E3).
- **FUN:** FUN-MUN-01, FUN-MUN-02. **Código:** a main conta o 3×3 abaixo da extratora; a medida 3D conectada, com a classe industrial ≥ 16, só existe no pr47 (`extraction.rs:34-57`). **Canônico:** a forma do pr47.

#### DEC-011
**Jazidas conectadas e espaçadas,** inspiradas em Shapez: (1) leitura 3D com teto; (2) depósitos maiores em terreno novo. 07/11 noite · agente, PR #23 · Proposta
- **Fonte:** `frentes/JAZIDAS-CONECTADAS.md`, "é uma proposta" (E3).
- **FUN:** FUN-MUN-02 (item 1), FUN-MUN-06 (item 2). **Código:** item 1 no pr47; item 2 não feito. **Canônico:** (1) sim no pr47; (2) Concebido.

#### DEC-012
**Prensa em 80 ticks:** "a primeira linha pede duas prensas por veio". 07/11 ~23h · Grok (#19) · Decisão de agente
- **Fonte:** `frentes/GROK-BALANCE.md` (E3).
- **FUN:** FUN-EXT-03. **Código:** não; o pr47 usa 30 t. **Relação:** substituída por [DEC-045](#dec-045). **Canônico:** não.

#### DEC-013
**Missões rebalanceadas; projetos industriais; módulo de produtividade +20%** (um lote extra a cada 5 ciclos). 07/11 ~23h · Grok e "Jefter Pereira" (#20) · Decisão de agente (não confirmada pelo dono)
- **Fonte:** `frentes/BALANCEAMENTO-MISSOES.md`, README (E3); `c5db4f6`.
- **FUN:** FUN-PRO-03. **Código:** main 10% (`upgrades.rs:31`); pr47 20% (`upgrades.rs:35`). **Canônico:** aberto → [Q-14](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-14-produtividade-10-ou-20), [CNF-016](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf).

#### DEC-014
**Progressão por fases:** passar de fase pede uma pilha (32 engrenagens, 16 aço, 16 circuitos); unidade de 30/min; tempos mais longos. 08/11 05:35 · "Claude (MineFac)" · Decisão de agente
- **Fonte:** `frentes/PROGRESSAO-FASES.md` (E3).
- **FUN:** FUN-PRG-01, FUN-PRG-02, FUN-PRG-08. **Código:** os critérios de pilha estão no pr47, como "pilha **ou** licença" (`engineering.json`, `steel.json`, `circuits.json`); os tempos não. **Relação:** tempos revogados por [DEC-045](#dec-045). **Canônico:** pilhas sim; tempos não.

#### DEC-015
**"A progressão por fases substitui os tempos anteriores".** 08/11 · Codex local · Decisão de agente
- **Fonte:** `CONSOLIDACAO.md` (E3).
- **FUN:** FUN-EXT-04, FUN-PRG-08. **Código:** não. **Relação:** revogada pelo dono em [DEC-045](#dec-045). **Canônico:** não.

#### DEC-016
**Quadro da fábrica:** 5 fases (16 chapas → 32 engrenagens → 16 aço → 16 circuitos → 32 circuitos), licenças `permit_*` e pedido do dia que paga esmeraldas. 08/11 madrugada · "Claude (MineFac)" e Codex · Decisão de agente (não confirmada pelo dono)
- **Fonte:** `frentes/EXPERIENCIA-JOGADOR.md` (E3); `milestones/mod.rs:37-69,147-150` (E2).
- **FUN:** FUN-PRG-02, FUN-PRG-03, FUN-PRG-04. **Código:** acompanha, mas o texto "Libera" das fases 3–5 promete o que no pr47 vem da pesquisa (`:246-247`), e o pedido do dia cria moeda. **Canônico:** Integrado no pr47; em conflito → [CNF-002](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [CNF-001](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [CNF-004](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-03](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão), [Q-02](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-02-dois-pedidos-diários).

#### DEC-017
**"Conserte essas lacunas":** aprovou o documento de lacunas (progressão por item, cadeia rasa, fábrica longe para, falta de planejamento, falta de visão global, falta de meta final). 08/11 16:32 · **Dono**
- **Fonte:** chat 16:32 (E4); artefato `UhUqLwEVPnRrKFzKbK6NMD` (E3).
- **FUN:** FUN-PES-01, FUN-PRO-05, FUN-TRA-04, FUN-TRA-05, FUN-PRO-07, FUN-CTL-03, FUN-PRG-07. **Código:** parcial (#40 no pr47); meta final não existe ([Q-10](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-10-fim-de-jogo)). **Canônico:** sim.

#### DEC-018
**Laboratório de pesquisa:** 4 pesquisas (metalurgia, eletrônica, automação, escala) de 32 a 256 peças com consumo contínuo; desloca as liberações dos avanços `steel` e `circuits` para a pesquisa. 08/11 ~16:40–17:06 · Claude (#40) · Decisão de agente que implementa [DEC-017](#dec-017)
- **Fonte:** `frentes/GRANDE-INDUSTRIA.md` (E3); `research.rs:13-26`, `research/*.json` com gatilho `impossible` (E2).
- **FUN:** FUN-PES-01, FUN-PRG-01. **Código:** acompanha no pr47. **Canônico:** vigente no pr47; coexiste com [DEC-014](#dec-014) e [DEC-016](#dec-016) sem coordenação → [Q-03](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão), [CNF-002](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf).

#### DEC-019
**Cadeia mais funda:** motor (100 t), processador (160 t), núcleo de automação (300 t), kit Mk.II (240 t); Mk.II = ciclo ×0,5, energia ×2, poluição ×1,25. 08/11 ~17h · Claude (#40) · Decisão de agente
- **Fonte:** `frentes/GRANDE-INDUSTRIA.md` (E3); `tools/production.json`, `upgrades.rs:50-52` (E2).
- **FUN:** FUN-PRO-04, FUN-PRO-05. **Código:** acompanha. **Canônico:** sim. Itens fora da bolsa: [CNF-011](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf).

#### DEC-020
**Grande projeto em 4 fases,** pago com esmeraldas, módulos, kits, estrelas e faróis. 08/11 ~17h · Claude (#40) · Decisão de agente
- **Fonte:** `frentes/GRANDE-INDUSTRIA.md` (E3); `contracts.rs:29-55` (E2).
- **FUN:** FUN-PRG-07. **Código:** acompanha; 32, 48 e 64 esmeraldas criadas fora dos livros. **Canônico:** sim, com conflito → [CNF-001](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-01](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros).

#### DEC-021
**"Razões visíveis":** usar os mecanismos de Satisfactory e Factorio (itens por minuto no guia e no painel). 08/11 16:38 · **Dono**
- **Fonte:** chat 16:38 (E4).
- **FUN:** FUN-PRO-09, FUN-PRO-07, FUN-TUT-05. **Código:** painel por minuto desde a 0.5; monitor e guia de indústria no pr47. **Canônico:** sim.

#### DEC-022
**Exportação contínua da central** (4 circuitos + 4 aço → 4 esmeraldas + 2 frascos, sem fim) e **armazém do povoado** (1 esmeralda por 60 pontos de comida; +50% com 3 aldeões a 48 blocos). ≤ 0.5 / 0.7 · Codex e Claude local · Decisão de agente
- **Fonte:** README (E3); `contracts.rs:57-61`, `production/pantry.rs:1-21` (E2).
- **FUN:** FUN-PRG-06, FUN-AGR-05, FUN-ECO-09. **Código:** acompanha nas duas camadas. **Canônico:** em conflito com a economia fechada → [CNF-001](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-01](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros).

#### DEC-023
**Meta de produção sustentada:** 20 circuitos/min por 3 min; meta energética de atravessar uma noite. Data ND · agente · Ideia
- **Fonte:** README "Direção de jogo"; documento de lacunas (E3).
- **FUN:** FUN-PRG-09. **Código:** não. **Canônico:** Concebido → [Q-10](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-10-fim-de-jogo).

### 3.3 Aldeões, vila, sociedade e economia

#### DEC-024
**Aldeões contratados** (lavrador, carregador, capataz, pastor) por 1 esmeralda, com comida de 2 pontos por tarefa e fila de tarefas. 07/11 (0.10–0.12) · Cloud, "frente atribuída pelo usuário" · Decisão de agente com indício de pedido
- **Fonte:** `TRABALHADORES.md`, `CLOUD.md` (E3); `workforce/mod.rs:65-66,96` (E2).
- **FUN:** FUN-TRB-01, 02, 05, 13, 14. **Código:** acompanha. **Canônico:** sim.

#### DEC-025
**A comida não fecha o ciclo:** pão paga 3 tarefas; trigo cru não paga uma. 07/11 (Grok lote 3) · Grok · Decisão de agente
- **Fonte:** `GROK-LOTE3` (E3); `workforce/mod.rs:94-96` (E2).
- **FUN:** FUN-TRB-14. **Código:** acompanha. **Canônico:** sim.

#### DEC-026
**Teto da vila:** 30 (0.13) → 150 (0.14), com teto dinâmico: 30 + 3/posto + 5/nível de zona + 2/armazém + 20/centro, até 150. 07/11 22:10 → 22:33 · Cloud · Decisão de agente
- **Fonte:** `TRABALHADORES.md` (E3); `village.rs:21-28,121-129` (E2).
- **FUN:** FUN-TRB-07. **Código:** acompanha; no pr47 soma o extra das obras. **Canônico:** sim (ver §4.8).

#### DEC-027
**A vila só cresce com cama livre;** a cabeceira conta; raio 32. 07/11 (#21) · Grok, "regra de moradia" · Decisão de agente
- **Fonte:** `frentes/VILA-CIDADE.md`, README do pr47 (E3); `village.rs:30-33,96-108,153-154` (E2).
- **FUN:** FUN-TRB-08. **Código:** ativo no pr47; ausente na main. **Canônico:** sim no pr47.

#### DEC-028
**Os aldeões andam pelo cérebro vanilla;** descanso noturno dos civis. 08/11 05:26 (0.16.0) · Cloud · Decisão de agente
- **Fonte:** 0.16.0 (E3); `workforce/mod.rs:83-90` (`RESTING = 28`) (E2).
- **FUN:** FUN-TRB-10. **Código:** acompanha. **Canônico:** sim.

#### DEC-029
**Sociedade:** casas, famílias, lojas, prefeito, fases Povoado/Vila/Cidade/Metrópole, IDH, Gini, PIB. 08/11 16:06 (0.18) · Cloud · Decisão de agente (não confirmada pelo dono)
- **Fonte:** `frentes/SOCIEDADE.md` (E3); `society/mod.rs`, `mayor.rs` (E2).
- **FUN:** FUN-VIL-01 a 06. **Código:** acompanha. **Canônico:** sim; peso no jogo em aberto → [Q-12](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-12-peso-da-sociedade-frente-à-fábrica).

#### DEC-030
**Economia fechada:** esmeralda = 60 moedas; o dinheiro circula entre 4 livros (Companhia, Lojas, Prefeitura, Famílias); imposto retido na fonte; moeda só entra ou sai por esmeraldas físicas. 08/11 16:48 (0.21) · Cloud · Decisão de agente (não confirmada pelo dono)
- **Fonte:** README 0.21 (E3); `ledger.rs` (E2).
- **FUN:** FUN-ECO-01, 02, 06, 09. **Código:** **não sustenta o princípio.** Além do depósito e do saque, a moeda entra pela exportação e pelas compras do jogador e sai pela importação, pelo frete e pelas obras e zonas, que destroem moeda com `add_coins` negativo (`ledger.rs:159-167,199,243`; `trade.rs:336-337,365,598`; `market.rs:464`; `center.rs:592-601`). Fora dos livros, contratos, pedido do dia, grande projeto, armazém e esmeralda minerada criam esmeraldas ([SIS-ECO](03_atlas/SIS-ECO.md) B.4–B.5). **Canônico:** princípio declarado; o código descreve uma economia com bordas → [CNF-001](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-01](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros).

#### DEC-031
**Comércio exterior** no estilo das conexões externas de Cities: Skylines: porto, estação, bolsa mundial, contratos, reputação, importação. 08/11 17:13 (0.23) · Cloud · Decisão de agente
- **Fonte:** README 0.23 (E3); `trade.rs` (E2).
- **FUN:** FUN-MER-03 a 08. **Código:** acompanha. **Canônico:** sim.

#### DEC-032
**Balanço:** preço único dentro e fora; salário = 2 × custo de vida ÷ 20 turnos; atacado a 75%; teto pela importação; piso de sucata de 25%. 08/11 17:29 (0.25) · Cloud · Decisão de agente
- **Fonte:** `frentes/ECONOMIA.md` (E3); `society/mod.rs`, `market.rs` (E2); `sim_economy.py --check` = ok (E1, modelo Python, sem as emissões).
- **FUN:** FUN-MER-01, 02; FUN-ECO-03. **Código:** acompanha. **Canônico:** sim. Imposto cobrado duas vezes: [CNF-015](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-13](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-13-imposto-uma-ou-duas-vezes).

#### DEC-033
**Obras da vila** com prédio desenhado; o efeito só vale com o prédio de pé; prestígio 0/60/200/500; pedidos diários; caixa; decretos. 08/11 (obras) · agente das obras · Decisão de agente
- **Fonte:** `HANDOFF_OBRAS.md` (E3); `projects.rs:1-140` (E2).
- **FUN:** FUN-OBR-01 a 06, 08, 11; FUN-PES-02. **Código:** acompanha; menu do canteiro com 1 slot ([R-30](09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)). **Canônico:** sim no pr47.

#### DEC-034
**Integrar a build parcial de obras;** "se tiver conflito pergunte para mim". 08/11 17:13 · **Dono**
- **Fonte:** chat 17:13 (E4); `f1452f5`.
- **FUN:** SIS-OBR inteiro. **Código:** acompanha. **Canônico:** sim.

#### DEC-035
**Recompensas dos pedidos da vila e rendimento da obra Prefeitura pagos pelo caixa público;** sem prefeitura ou sem saldo, só prestígio. A opção recomendada pelo agente, "entrada externa", foi recusada. 08/11 17:18 · **Dono**
- **Fonte:** chat 17:18 (E4); `center.rs:609-620` (`reward`: Public → Company), `projects.rs:475-476` (E2).
- **FUN:** FUN-ECO-07, FUN-ECO-08, FUN-OBR-04. **Código:** acompanha. **Canônico:** sim, **só para esses dois fluxos**.

#### DEC-036
**Casa do fundador:** casa mobiliada gerada uma vez por mundo, com centro de controle, lareira, sofá, cama e mural; **+15% em casa** (`HOME_TIME 0,87`); refeição 0,95 por 5 min; banquete 0,91 por 8 min; duas camas. 08/11 madrugada · Claude local (`ajustes-vila-mobs`) · Decisão de agente
- **Fonte:** `TRABALHADORES.md` "sexta entrega", `frentes/CASA-FUNDADOR-INTEGRACAO.md` (E3).
- **FUN:** FUN-LAR-01, 03, 04. **Código:** o +15% foi removido no pr47; refeição e banquete passaram a valer só **fora** de casa. **Relação:** substituída em parte por [DEC-046](#dec-046). **Canônico:** só a estrutura e as camas.

#### DEC-037
**Lar `veio:home`:** lareira, despensa, conforto 0–100, melhorias até o nível 4, dono, renascimento. 08/11 (obras) · agente das obras · Decisão de agente
- **Fonte:** `TRABALHADORES.md` (E3); `workforce/home.rs:1-61` (E2).
- **FUN:** FUN-LAR-02. **Código:** acompanha. **Canônico:** sim, dentro da casa do fundador.

#### DEC-038
**Vila que vira cidade:** casas variadas com profissão visível, praça-mercado, vida entre cama, sino e banca. 07/11 23:03 · pedido do usuário relatado pelo Grok · Proposta
- **Fonte:** `frentes/VILA-CIDADE.md` (E3).
- **FUN:** nenhuma FUN própria. **Código:** não. **Canônico:** Concebido; parte superada pela sociedade da 0.18.

### 3.4 Distância, logística, transporte e mundo

#### DEC-039
**Agricultura real** (irrigação, necessidades), **água e energia** (bombas, canos, usinas reais, hidrelétrica) e **rios** com nascente, afluentes, quedas e foz; explicar no tutorial; **não remodelar do zero** o Minecraft. 08/11 15:55 · **Dono**
- **Fonte:** chat 15:55 (E4); `frentes/AGUA-E-RIOS.md`, "Pedido do Luiz" (E3).
- **FUN:** FUN-AGU-01 a 04, FUN-AGR-07, FUN-ENE-05, FUN-MUN-03. **Código:** acompanha no pr47; APIs de mundo nunca compiladas; o tutorial na tela não explica a água (só o guia, págs. 15–16). **Canônico:** sim.

#### DEC-040
**Âncora de chunk com custo de energia** (32 FE/t, `forceload`). O dono escolheu a opção recomendada 0, entre âncora, produção estimada e deixar como está. 08/11 16:38 · **Dono**
- **Fonte:** chat 16:38 (E4); `industry.rs:19-21,285-291` (E2).
- **FUN:** FUN-TRA-06. **Código:** acompanha; `forceload` é comando de console, não API. **Canônico:** vigente como **opcional** (§4.4).

#### DEC-041
**A fábrica distante continua produzindo e entregando "sem carregar tudo",** como no Satisfactory, com leveza. 08/11 17:01 e 17:03 · **Dono**
- **Fonte:** chat 17:01, 17:03 (E4).
- **FUN:** FUN-TRA-04, FUN-TRA-05. **Código:** `stations.rs` no pr47, mas **sem custo** ([DEC-043](#dec-043)). Trem, drones e caravanas param fora da área carregada. **Canônico:** sim; prevalece sobre a âncora. Ver [CNF-009](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-05](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo).

#### DEC-042
**Recuperação por máquina ao voltar** ("catch-up"). 08/11 17:02 → 17:08 · Claude (#40) · Abandonada
- **Fonte:** chat 17:08 (E4); `e36ee2e` (E3). Motivo: "contaria duas vezes" com as estações.
- **FUN:** nenhuma. **Código:** não. **Canônico:** não.

#### DEC-043
**Estações de carga e descarga** com produção virtual depois de 100 ticks sem rodar; 8 itens a cada 8 ticks; 10 FE por item; canal por corante de 4.096 itens; `estacoes.txt`. 08/11 ~17:06 · Claude (#40) · Decisão de agente que implementa [DEC-041](#dec-041)
- **Fonte:** `frentes/GRANDE-INDUSTRIA.md` (E3); `stations.rs:17-31,117-140` (E2).
- **FUN:** FUN-TRA-04, FUN-TRA-05. **Código:** parcial. `simulate` cria itens **sem insumo nem energia**, continua com o posto desmontado, e os canais são globais entre dimensões. Custo: 194,4 µs por 1.000 estações a cada 20 ticks (E1). **Canônico:** mecanismo sim; regra de custo aberta → [Q-05](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo), [CNF-009](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf).

#### DEC-044
**O terminal ferroviário do Codex passa a ser `veio:rail_terminal`;** `veio:freight_station` fica com a estação do comércio exterior. 08/11 17:41 · **Dono** (opção 0)
- **Fonte:** chat 17:41 (E4); `a79f326` (E3); `freight.rs:7`, `society/trade.rs:27` (E2).
- **FUN:** FUN-TRA-01, FUN-MER-05. **Código:** acompanha. **Sem migração:** em mundos 0.16.1 e Realismo 1.1, o terminal antigo vira estação de exportação e exporta a carga ([CNF-014](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)). **Canônico:** sim.

#### DEC-045
**Tempos de receita da 0.25,** não os das fases do Codex. Motivo dado pelo agente: "a cadeia da grande indústria depende desses tempos". 08/11 19:16 · **Dono** (opção 0)
- **Fonte:** chat 19:16 (E4); `tools/production.json` e `production/data.rs:5-6,10` no pr47: prensa 30 t / 8 FE, montador 40 t / 12 FE (E2, regeneração conferida).
- **FUN:** FUN-EXT-04, FUN-EXT-03, FUN-PRG-08. **Código:** acompanha. **Relação:** substitui [DEC-012](#dec-012), [DEC-015](#dec-015) e os tempos da [DEC-014](#dec-014). **Canônico:** sim. Documentos desatualizados: `frentes/PROGRESSAO-FASES.md`, `frentes/GROK-BALANCE.md`, README do pr47 §Progressão e l.24.

#### DEC-046
**Casa do fundador do Codex com o Lar (`veio:home`) dentro;** refeição e bônus de casa só pelo Lar; o bônus de comer fora do Codex permanece. 08/11 19:16 · **Dono** (opção 0)
- **Fonte:** chat 19:16 (E4); `tools/build_house.py:177-178` (lareira = `veio:home`); `home/mod.rs:99-103`; `home/food.rs:22-29,59-69`; `workforce/home.rs:555-693`; teste `home/tests.rs:163` (E2).
- **FUN:** FUN-LAR-01 a 04. **Código:** acompanha, com esta leitura:
  - a casa não acelera as máquinas;
  - o Lar dá efeitos **ao jogador** (saturação, regeneração, pressa, resistência, visão noturna, absorção);
  - o único efeito nas máquinas é o de **comer fora de casa** (×0,95 por 6.000 t; ×0,91 por 9.600 t), e ele é **global** ([CNF-017](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
- **Relação:** substitui em parte [DEC-036](#dec-036). **Canônico:** sim. O README do pr47 l.21 ainda promete +15% ([CNF-006](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

#### DEC-047
**Sons, caravanas e formatos de máquina entram no PR #47 com as obras** (um PR só). 08/11 17:36–17:51 · **Dono** (opção 0)
- **Fonte:** chat 17:36, 17:38, 17:51 (E4); merges `a245f28`, `e5c837a`.
- **FUN:** FUN-APR-03 a 06, FUN-TRA-03. **Código:** acompanha. **Canônico:** sim.

#### DEC-048
**"Junte tudo":** #40, PRs do Grok e consolidação do Codex `50995e6` no #47. 08/11 19:04 e 19:14 · **Dono**
- **Fonte:** chat 19:04, 19:14 (E4); merges `eedfd47`, `5c57405`.
- **FUN:** todas as do pr47. **Código:** acompanha. **Canônico:** sim.

#### DEC-049
**Interromper a consolidação do Codex local;** "não apresentar os ZIPs anteriores como a build final". 08/11, antes de 18:11 · Dono, só em documento
- **Fonte:** `CHECKPOINT-INTEGRACAO.md` (E3).
- **FUN:** FUN-PLT-06. **Código:** n/a. **Canônico:** vigente; o integrador passou a ser o Claude do #47.

#### DEC-050
**A água é NBT própria em mB,** não tanques do Horizon. 08/11 · Claude (#40) · Decisão de agente
- **Fonte:** memória (E4); `frentes/AGUA-E-RIOS.md` (E3); `production/water.rs:3-4` (E2).
- **FUN:** FUN-AGU-01 a 04. **Código:** acompanha. **Canônico:** sim.

#### DEC-051
**Caravanas** com rotas de até 512 blocos que **dependem de áreas simuladas.** 07/11 · Codex nuvem · Decisão de agente
- **Fonte:** `frentes/TRANSPORTES.md`, README do pr47 l.49 (E3); `freight/caravan.rs:1-8` (E2).
- **FUN:** FUN-TRA-03. **Código:** acompanha; a caravana para fora da área carregada. **Canônico:** sim, em conflito com [DEC-041](#dec-041) → [Q-05](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo).

#### DEC-052
**Status no painel:** descanso 28, casa 29–31, conexões externas 32–35, canteiro 36–39, frete e caravana 40–43; tabela global `STATUS_KEYS` com 44 entradas. 08/11 · integrador do #47 · Decisão de agente
- **Fonte:** `presentation.rs:19-34` (E2).
- **FUN:** FUN-APR-01. **Código:** acompanha. Os códigos de pendência 40/41 do centro reutilizam os números; hoje sem efeito ([CNF-013](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)). **Canônico:** sim.

#### DEC-053
**Centro de controle com 8 slots:** 0 comida; 1–3 metas; 4 cofre; 5 saque; 6 obra; 7 decreto. 08/11 (obras) · agente das obras · Decisão de agente
- **Fonte:** `HANDOFF_OBRAS.md` (E3); `projects.rs:68-70`, `center.rs:26-30` (E2).
- **FUN:** FUN-CTL-01, FUN-OBR-05. **Código:** acompanha (main: 6 slots). **Canônico:** sim.

### 3.5 Apresentação, desempenho e plataforma

#### DEC-054
**Visual no estilo Minecraft:** painéis como baú e fornalha; texturas desenhadas em texto (`build_textures.py`). 07/11 19:10 (0.7) · Claude local · Decisão de agente
- **Fonte:** 0.7, README "Visual" (E3).
- **FUN:** FUN-APR-01, FUN-APR-02. **Código:** acompanha. **Canônico:** base obrigatória.

#### DEC-055
**Realismo leve 1.1 opcional:** shader WGSL (MIT) em `visuals/veio-industrial`; perfis Realismo (8 chunks, simulação 5, 85%) e Leve (6 chunks, 75%), escolhidos antes de jogar. 08/11 madrugada · Codex local · Decisão de agente
- **Fonte:** `CREDITS.md`, README do pr47 (E3); `launcher/src/visuals.rs:118-146` (E2).
- **FUN:** FUN-APR-07. **Código:** acompanha; WGSL nunca validado no Horizon. **Canônico:** vigente como opcional.

#### DEC-056
**"Realismo maior sem pesar o PC",** com referência de PC de 8 GB ("é Rust, dá para pôr muito mais"), e **limite do mapa e otimização.** 08/11 20:09 · **Dono** (prioridade)
- **Fonte:** chat 20:09 (E4).
- **FUN:** FUN-APR-09, FUN-DES-04, FUN-DES-05, FUN-MUN-04. **Código:** Em desenvolvimento em duas threads (#51, #52). **A candidata contraria a meta:** +36% por tick e 329 alocações por quadro (E1). **Canônico:** sim → [Q-07](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-07-realismo-clima-e-estações-do-ano), [Q-09](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-09-mapa-finito).

#### DEC-057
**Orçamento global de trabalho de fundo** por categoria, com fila FIFO. 07/11–08/11 · Codex local · Decisão de agente
- **Fonte:** `frentes/ORCAMENTO-GLOBAL.md` (E3); `budget.rs:8` (E2).
- **FUN:** FUN-DES-01, FUN-DES-02. **Código:** acompanha. **Canônico:** sim.

#### DEC-058
**Guia principal limitado a 100 páginas;** terminais só no livro Transportes; página com no máximo 14 linhas. 08/11 · Codex e integrador · Decisão de agente
- **Fonte:** memória (E4); `CHECKPOINT-INTEGRACAO.md` (E3); `check_project` (E1).
- **FUN:** FUN-TUT-02 a 05. **Código:** verificador acompanha; 29 páginas do Guia da Vila acima de 14 linhas (contadas por idioma: 17 em pt_br e 12 em en_us) (aviso). **Canônico:** sim → [CNF-012](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf).

#### DEC-059
**Poluição em memória;** recomeça ao reabrir o mundo. 07/11 (0.8) · Claude local · Decisão de agente
- **Fonte:** README (E3); `pollution.rs:18` (E2).
- **FUN:** FUN-DEF-01, FUN-PER-02. **Código:** acompanha. **Canônico:** vigente como limitação conhecida ([R-09](09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).

#### DEC-060
**Ondas:** arco a 26 blocos, arqueiros a partir da 3ª noite, bruto a partir da 5ª. 08/11 05:53 (0.17) · Cloud · Decisão de agente
- **Fonte:** README da main `:213` (E3); `pollution.rs:44-47` (E2).
- **FUN:** FUN-DEF-02. **Código:** acompanha. **Canônico:** sim. O README do pr47 `:307` voltou ao texto antigo, "64/18 blocos" ([CNF-006](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

#### DEC-061
**LAN:** convidados vão para a vila do anfitrião. 07/11 (#25) · Grok · Decisão de agente
- **Fonte:** README do pr47 (E3); `workforce/spawn.rs:151-152` (E2).
- **FUN:** FUN-MP-01. **Código:** acompanha. **Canônico:** sim no pr47; o resto do estado em LAN está em aberto → [Q-15](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan).

#### DEC-062
**PR #51: Veio 0.26.0, Realismo 2.1, clima e estações do ano** (`ambience.rs`, `climate.rs`, 9 shaders, `release/Veio-0.26.0.zip`), sobre o pr47. 08/11 20:34 · thread "Aumentar o realismo sem pesar o PC" · Decisão de agente (não confirmada pelo dono)
- **Fonte:** PR #51 (E3); `bench_interleaved.log` (E1).
- **FUN:** FUN-APR-09. **Código:** não auditado linha a linha; mede +29% por tick e 329 alocações por quadro. **Canônico:** pendente. Versão em conflito → [Q-08](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera); efeito do clima → [Q-07](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-07-realismo-clima-e-estações-do-ano); "estações" colide com estações de carga → [CNF-010](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf).

#### DEC-063
**PR #52: sons só perto do jogador, mural ocioso, medição de escala,** sobre o pr47. 08/11 20:35 · thread "Definir limite do mapa e otimizar" · Decisão de agente (não confirmada pelo dono)
- **Fonte:** PR #52 (E3); `bench_interleaved.log` (E1).
- **FUN:** FUN-APR-05, FUN-DES-03, FUN-DES-04. **Código:** 6 arquivos; não resolve a regressão (+35%, 329 alocações). `frentes/DESEMPENHO.md` afirma "0 alocações" ([CNF-020](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)). Não toca no mapa. **Canônico:** pendente.

### 3.6 Depois dos commits de referência (registro preliminar, 20:38–21:41)

Estas três entradas usam só o chat (E4) e os metadados do GitHub: título, descrição, branch, commit e contagem de arquivos (E3). O código não foi lido nem medido.

#### DEC-064
**Concluir as pendências das obras num PR separado sobre o #47:** traduções, menu do canteiro com 3 slots, ruínas e proteção do território. 08/11 20:38 · **Dono** ("perfeito, crie um agente opus 5.5 no médio para prosseguir")
- **Fonte:** chat do projeto 20:37–20:38 (E4); thread "Concluir pendências das obras da vila", 20:56 (E4); PR #53 `claude/project-thread-6fbc15@5c9ffdc` (E3).
- **FUN:** FUN-OBR-01, FUN-OBR-09, FUN-OBR-10, FUN-OBR-11, FUN-PES-02 (textos), FUN-CTL-01 (rótulos dos slots 6 e 7). TAR: [TAR-006](08_BACKLOG_E_PRIORIDADES.md#tar-006), [TAR-020](08_BACKLOG_E_PRIORIDADES.md#tar-020), [TAR-021](08_BACKLOG_E_PRIORIDADES.md#tar-021), [TAR-022](08_BACKLOG_E_PRIORIDADES.md#tar-022).
- **Código:** não auditado. A thread relata 331 testes e as checagens passando (E4). Duas ressalvas da própria thread:
  - os ganchos de proteção seguem os formatos do `HANDOFF_OBRAS.md`, e só a compilação confirma se batem com o motor;
  - um bloco posto encostado na borda, pelo lado de fora, ainda pode cair dentro do território.
- **Decisão de agente dentro do PR (não confirmada pelo dono):** ruínas a 50–62 blocos do centro, e não até 80 como dizia o handoff, porque o centro de controle alcança 64.
- **Canônico:** pendente (aberto sobre o #47).

#### DEC-065
**Testar localmente a "build consolidada e atual".** 08/11 20:59 · **Dono**
- **Fonte:** chat do projeto 20:59 (E4); thread do #47, 21:01–21:41 (E4); commit `3f7cb28` (E3).
- **O que aconteceu:**
  1. A thread do #47 montou o `Veio-0.26.0-teste-combinado.zip` (#47 + #51 + #52 + #53). Ele só existe como zip, e a thread relata 351 testes passando (E4).
  2. Às 21:18, por controle remoto no PC do dono, a thread compilou contra o Horizon `66f3981` que já estava no cache do launcher (`E:\GPTmodMine\.build`).
  3. Às 21:39, a compilação parou em 4 erros no código do Veio.
  4. Às 21:40, o commit `3f7cb28` no #47 corrigiu dois pontos. `with_nbt_key`, `FastHasher` e `FastHash` passaram de `pub(super)` para `pub(crate)`. `Mob::on_climbable` é método no Horizon, e o substituto offline ganhou o método.
- **FUN:** FUN-PLT-04, FUN-PLT-07, FUN-PLT-11. TAR: [TAR-001](08_BACKLOG_E_PRIORIDADES.md#tar-001), [TAR-002](08_BACKLOG_E_PRIORIDADES.md#tar-002), [TAR-003](08_BACKLOG_E_PRIORIDADES.md#tar-003).
- **Relação:** é a primeira compilação real do #47. Confirma o [R-01](09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43). Mostra também que o substituto offline divergia do Horizon em pelo menos um método, e por isso o E1 não garante a compilação. A revisão do cache do dono é `66f3981`, a mesma do substituto, o que informa a escolha da [TAR-002](08_BACKLOG_E_PRIORIDADES.md#tar-002).
- **Canônico:** em andamento. A recompilação e o teste em jogo estão ND no corte.

#### DEC-066
**PR #54: Veio 0.27.0, agenda da vila.** 08/11 21:36 · Claude "Cloud" (sessão `01APXd4y…`, conta `jefterpereira`) · Decisão de agente (não confirmada pelo dono). Conteúdo, segundo a descrição do PR:
- calendário de 4 × 6 dias sobre o `climate.rs` do #51, com 6 feriados por ano e um evento de domingo;
- visitantes `veio:visitor` com pedido de entrega, que a conta pública reembolsa pelo valor de referência;
- obras cívicas a cada 4 dias, pagas pela prefeitura (240, 420 ou 720);
- ânimo de 0 a 100, que multiplica o IDH de 0,9× a 1,1×;
- lazer como quarto serviço do IDH.

Ficha da decisão:
- **Fonte:** descrição do PR #54 e `frentes/AGENDA.md` (E3, não lido); `claude/trusting-ritchie-sc9ty8@2c49d41`, 56 arquivos, +2.632/−187.
- **FUN:** novas, ainda sem ID. Afetam FUN-VIL-05 (IDH), as missões de [SIS-MER](03_atlas/SIS-MER.md) e FUN-OBR-11, porque as obras já têm visitantes.
- **Relação:**
  - Empilha sobre o #51, que empilha sobre o #47 ([R-42](09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
  - Abre uma terceira numeração ([Q-08](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera)).
  - As estações do ano voltam a colidir com as estações de carga ([CNF-010](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
  - Mostra que a frente Cloud, dada como histórica, continua ativa.
  - A descrição afirma que nenhuma moeda é criada.
- **Canônico:** pendente; não auditado.

## 4. Reconciliações

Conflitos já resolvidos por decisão do dono, com o código conferido, ou fechados pela leitura do código. Os abertos estão em [09 §2](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf).

| # | Conflito | Fontes em choque | Canônico | Situação |
|---|---|---|---|---|
| 4.1 | **Casa do fundador × Lar** | [DEC-036](#dec-036) (+15% em casa; refeição na casa) × [DEC-037](#dec-037) (Lar) | [DEC-046](#dec-046): a casa é o edifício; a lareira é o `veio:home`; a casa não acelera máquinas; o Lar dá efeitos ao jogador; comer **fora** de casa dá ×0,95/×0,91 global | **Reconciliado.** Faltam o README do pr47 l.21, a "sexta entrega" de `TRABALHADORES.md` e o comentário `home/food.rs:9` |
| 4.2 | **Tempos 0.25 × fases do Codex** | PROGRESSAO-FASES (prensa 40 t, 30/min), GROK-BALANCE (80 t), CONSOLIDACAO ("fases substituem") × dono 19:16 | [DEC-045](#dec-045): tempos da 0.25. As pilhas de fase (32/16/16) seguem como critério de avanço, sem os tempos do Codex | **Reconciliado no código.** Documentos desatualizados |
| 4.3 | **`freight_station` × `rail_terminal`** | Mesmo ID para o trem do Codex e para a estação do comércio | [DEC-044](#dec-044) | **Reconciliado** (`a79f326`). Migração de saves antigos: [CNF-014](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) |
| 4.4 | **Âncora × estações** | 16:38 âncora; 17:01 "sem carregar tudo" | Estações com produção virtual são o mecanismo principal ([DEC-041](#dec-041)/043); âncora opcional ([DEC-040](#dec-040)); recuperação por máquina abandonada ([DEC-042](#dec-042)) | **Reconciliado** como interpretação. O custo da produção virtual reabriu o tema: [CNF-009](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-05](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo) |
| 4.5 | **Recompensas pela prefeitura × moeda criada** | [DEC-035](#dec-035) × contratos, pedido do dia, grande projeto, armazém, esmeralda minerada | [DEC-035](#dec-035) vale só para pedidos da vila e juros da Prefeitura | **Reconciliado para esses dois fluxos.** As demais fontes: [CNF-001](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-01](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros) |
| 4.6 | **Sons, caravanas e formas junto com as obras** | PRs separados × um PR | [DEC-047](#dec-047) | **Reconciliado** |
| 4.7 | **Realismo leve × visual vanilla** | [DEC-054](#dec-054) × [DEC-055](#dec-055) × dono 20:09 | Base vanilla obrigatória, jogável sem shader; realismo como camada opcional no launcher; teto de custo do PC de 8 GB | **Parcial:** quanto realismo e se vira padrão → [Q-07](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-07-realismo-clima-e-estações-do-ano) |
| 4.8 | **Limites da vila: 30 → 150** | 0.13: 30; 0.14: teto dinâmico; 0.18: limites por fase 30/60/100/150; Grok: cama livre; obras: +15, +20, +5/nível | `min(desenvolvimento + extra das obras, 150)` e mínimo com as camas; nascimento também exige fase, prefeitura não suspensa e vaga com alvará (`village.rs:103-154`; `society/mod.rs:328-341`) | **Reconciliado no código.** Design: [CNF-008](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-06](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-06-teto-de-população-e-bônus-das-obras) |
| 4.9 | **Descanso 28 × 29** | COORDENACAO: "freight_route 28, resting 29" × código | Código: `RESTING = 28`, `HOUSED = 29`, frete 40–43. `COORDENACAO.md` é histórico | **Reconciliado** |
| 4.10 | **Status do canteiro 32–35 → 33–36 → 36–39** | `HANDOFF_OBRAS.md` × código | 36–39 (`site.rs:42-45`) | **Reconciliado** |
| 4.11 | **Slots do centro 6 → 8** | Main 0–5 × obras | 8 slots ([DEC-053](#dec-053)) | **Reconciliado** |
| 4.12 | **"Caixa da vila" × livros** | Obras falam em "caixa" × livros Company/Public | O caixa é o livro **Company**. Esmeraldas não pedidas no slot Obra entram nele; obras e zonas pagam destruindo moeda; os juros saem do Public via `reward` (`center.rs:256-264,592-620`; `projects.rs:475-476`) | **Reconciliado pela leitura do código** |
| 4.13 | **Licenças × pesquisa: há trava mútua?** | Quadro × laboratório | Não há trava mútua. Os avanços `engineering`, `steel` e `circuits` aceitam pilha **ou** licença; os de pesquisa só o código concede. O que diverge é o texto "Libera" | **Fato reconciliado.** O desenho segue aberto: [CNF-002](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-03](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão) |
| 4.14 | **PIB** | Guia: "tarefas feitas e vendas" × SOCIEDADE e README: "consumo final no varejo" | Código: varejo do dia + exportações (`mayor.rs:187-189`) | **Fato reconciliado.** Corrigir o guia: [CNF-007](09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) |
| 4.15 | **Numeração das entregas** em `TRABALHADORES.md` (duas "sexta", duas "sétima") | E3 | Usar IDs FUN; não renumerar o histórico | **Reconciliado** |

## 5. Funcionalidades substituídas e abandonadas

| Funcionalidade | Estado | Substituída por | Quando | DEC | FUN |
|---|---|---|---|---|---|
| Prensa de 80 t, "duas prensas por veio" | Substituído | Tempos da 0.25 (prensa 30 t) | 19:16 | [DEC-012](#dec-012) → [DEC-045](#dec-045) | FUN-EXT-03 |
| Fases do Codex com tempos longos e unidade de 30/min | Substituído | Tempos da 0.25; ficaram as pilhas | 19:16 | [DEC-014](#dec-014) (tempos), [DEC-015](#dec-015) → [DEC-045](#dec-045) | FUN-PRG-08 |
| Pureza pelo 3×3 | Substituído no pr47 | Veio conectado em 3D com classe industrial | pr47 | [DEC-010](#dec-010), [DEC-011](#dec-011) | FUN-MUN-01 → FUN-MUN-02 |
| Grok lote 2 (lavrador mais barato, trigo) | Substituído | Registrado sem reaplicar (`008a980`); PR #7 aberto | 08/11 | — | FUN-AGR-06 |
| Recuperação por máquina ao voltar | Abandonada | Estações com produção virtual | 17:08 | [DEC-042](#dec-042) | — |
| +15% por estar na casa do fundador (`HOME_TIME 0,87`) | Removido | Efeitos do Lar no jogador; bônus de comer fora | 19:16 | [DEC-036](#dec-036) → [DEC-046](#dec-046) | FUN-LAR-03 |
| `veio:freight_station` como terminal ferroviário | Substituído | `veio:rail_terminal` | 17:41 | [DEC-044](#dec-044) | FUN-TRA-01 |
| Avanços `steel` e `circuits` liberando vapor, bateria, geotérmica, solar, Mk.II, Tesla | Substituído no pr47 de fato, sem decisão do dono | `research/*.json` | ~17h | [DEC-018](#dec-018) | FUN-PRG-01, FUN-PES-01 |
| Tutorial do zip 0.15.1 (`TRACE_TTL 200`) | Substituído | `onboarding.rs` (`TRACE_TTL 6000`) | 08/11 | — | FUN-TUT-06 |
| Zips 0.15.1 e 0.15.2 | Substituído | `release/Veio-0.25.0.zip` (da main) | 08/11 | [DEC-049](#dec-049) | FUN-PLT-06 |
| Consolidação do Codex local (`codex/consolidated-016`) | Interrompida | Integração pelo Claude do #47 | antes de 18:11 | [DEC-049](#dec-049), [DEC-048](#dec-048) | — |
| Três frentes Cloud/Grok/Codex | Substituído de fato | Integrador do #47 e threads por tema | 08/11 | [DEC-007](#dec-007) | — |
| Segunda 0.16.0 (`claude/ajustes-vila-mobs`) | Absorvida | Conteúdo entrou no pr47 pelo Codex | 08/11 | — | FUN-LAR-01, FUN-DEF-03 |
| "Vila que vira cidade" | Concebido, superado em parte | Sociedade da 0.18 | 16:06 | [DEC-038](#dec-038) → [DEC-029](#dec-029) | — |
| Prancheta como FUN de megafábrica | Renumerada (não é mudança de jogo) | FUN-LOG-07 | consolidação | — | FUN-MEG-03 → FUN-LOG-07 |
| Laboratório e pesquisa da vila como FUN de PRG e OBR | Renumeradas | FUN-PES-01, FUN-PES-02 | consolidação | — | FUN-PRG-05, FUN-OBR-07 |
