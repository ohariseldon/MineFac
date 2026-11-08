# SIS-PRO: Produção, máquinas e grande indústria

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` (53 receitas) e `pr47@80980db` (57). Receitas em `tools/production.json`; código em `native/api/src/` no pr47.

## A. Identidade

O coração do jogo de fábrica: máquinas definidas por tabela, módulos que trocam um gargalo por outro, e uma cadeia profunda que leva à grande indústria. O documento de lacunas aprovado pelo dono ([DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017)) mediu a main em 9 intermediários e 53 receitas e chamou a cadeia de rasa. O #40 acrescentou motor, processador, núcleo de automação e kit Mk.II ([DEC-019](../06_HISTORICO_E_DECISOES.md#dec-019)).

- **Depende de:** [SIS-EXT](SIS-EXT.md) (brutos e chapas), [SIS-ENE](SIS-ENE.md), [SIS-LOG](SIS-LOG.md) e [SIS-TRB](SIS-TRB.md) (abastecimento).
- **Alimenta:** [SIS-MER](SIS-MER.md) (produtos), [SIS-PRG](SIS-PRG.md) e [SIS-PES](SIS-PES.md) (entregas), [SIS-DEF](SIS-DEF.md) (poluição).

## B. Funcionamento

### B.1 A cadeia (pr47)

| Nível | Máquina | Receita | Tempo / energia |
|---|---|---|---|
| 1 | Montador | 2 chapas de cobre → engrenagem | 40 t / 12 FE |
| 1 | Cortadora | chapa de cobre → fio; chapa de ferro → estrutura | 20 t / 6 FE; 30 t / 8 FE |
| 2 | Integrador | estrutura + engrenagem → módulo mecânico | 60 t / 16 FE |
| 2 | Eletrônica | 2 fios + silício → circuito; ou 4 fios + 2 redstone | 60 t / 12 FE; 120 t / 14 FE |
| 3 | Integrador | aço + 4 fios → **motor** | 100 t / 20 FE |
| 3 | Eletrônica | 2 circuitos + 2 silício → **processador** | 160 t / 24 FE |
| 4 | Fabricador | 2 motores + 2 processadores + 4 aço → **núcleo de automação** | 300 t / 40 FE |
| 4 | Fabricador | 2 motores + 1 processador + 8 engrenagens → **kit Mk.II** | 240 t / 32 FE |

Também: moinho (farinha, açúcar, biomassa) e cozinha (pão, biscoito, bolo, tortas, ensopados), em [SIS-AGR](SIS-AGR.md). Os níveis 3 e 4 só existem no pr47.

**Razão de referência** (guia de indústria do pr47): uma prensa faz 40 chapas/min; um montador consome 60 chapas/min para fazer 30 engrenagens; logo **3 prensas alimentam 2 montadores**. Os tempos são os da 0.25 ([DEC-045](../06_HISTORICO_E_DECISOES.md#dec-045)). O dono pediu "razões visíveis", com itens por minuto no guia e no painel ([DEC-021](../06_HISTORICO_E_DECISOES.md#dec-021)).

### B.2 Módulos

Um por máquina, posto com o botão direito. "Cada um resolve um gargalo e cria outro" (`upgrades.rs:1-2`).

| Módulo | Tempo do ciclo | Energia por tick | Poluição | Produto extra |
|---|---|---|---|---|
| Velocidade | ×0,6 | ×1,8 | ×1,5 | — |
| Eficiência | ×1,0 | ×0,6 | ×0,5 | — |
| Produtividade | ×1,15 | ×1,4 | ×1,2 | main: +1 a cada 10 ciclos (10%); **pr47: +1 a cada 5 (20%)** |

Fonte: `effect_of`, `upgrades.rs:28-34` (pr47); main `:31`. A mudança para 20% veio da frente de balanço (`c5db4f6`, "Jefter Pereira") sem decisão do dono ([DEC-013](../06_HISTORICO_E_DECISOES.md#dec-013), [CNF-016](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-14](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-14-produtividade-10-ou-20)). Com 10%, a produtividade dá menos produto por minuto que nenhum módulo (1,10 ÷ 1,15).

**Kit Mk.II** (pr47): troca o motor de uma máquina de receita ou extratora. Ciclo ×0,5, energia ×2, poluição ×1,25; soma-se ao módulo (`with_tier`, `upgrades.rs:50-52`). A mesma energia por produto: uma máquina rende o que duas rendiam.

**Ritmo de refeição:** por cima do módulo, o tempo é multiplicado por `home::time_factor`, o bônus de comer fora de casa ([SIS-LAR](SIS-LAR.md) B.3). Esse bônus é global.

### B.3 Integridade e reparo

- Ataques da poluição dão dano às máquinas, até 100 (`pollution.rs:344-349`). Com 100, a máquina para ([SIS-DEF](SIS-DEF.md)).
- O kit de reparo, usado com o botão direito, tira 50 de dano (`REPAIR`, `lib.rs:75-77,180-185`).

### B.4 Ler a fábrica

- **Painel da máquina:** itens por minuto e % ativo, com leitura fechada por minuto, e motivo de espera: material, energia, saída ou redstone (desde a 0.5; FUN-PRO-09).
- **Monitor da fábrica** (pr47, `industry.rs:1-7`), como as estatísticas do Factorio: cada máquina com receita ou broca registra o que produz e consome por minuto. O monitor soma tudo, mostra os 9 itens de maior fluxo (até 4.095/min) e avisa quando há máquinas paradas por saída cheia, energia ou desgaste. O registro fica em memória e esquece máquinas paradas há 200 ticks.

## C. Experiência do jogador

1. A pessoa monta linhas com razões inteiras: 3 prensas para 2 montadores.
2. Os gargalos aparecem em sequência: extração, energia, fio, poluição.
3. O painel mostra por que a máquina espera.
4. Com a grande indústria (pr47), ela passa a ter linhas de 4 níveis e um monitor para achar o gargalo da fábrica inteira.

## D. Implementação

**Arquivos:** `tools/production.json` → `production/data.rs` (prensa `:5-6`, montador `:10`); `production/mod.rs`; `upgrades.rs`; `industry.rs` (monitor). Testes de produção: 16 (main) e 39 (pr47); balance tests (E1).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-PRO-01 Máquinas por tabela | 53 receitas (main); 57 (pr47) | `data.rs` · `data.rs` · — | E1 + E2 |
| FUN-PRO-03 Módulos | Ver B.2; um por máquina | `upgrades.rs:31` (10%) · `upgrades.rs:28-35` (20%) · — | E2 |
| FUN-PRO-04 Kit Mk.II | ×0,5 / ×2 / ×1,25; `MK2_FLAG 16` | — · `upgrades.rs:8-11,50-52` · — | E2 |
| FUN-PRO-05 Cadeia funda ([DEC-019](../06_HISTORICO_E_DECISOES.md#dec-019)) | Motor, processador, núcleo, fabricador, kit | — · `production.json` (4 receitas novas) · #40 (contido) | E2 |
| FUN-PRO-06 Kit de reparo e integridade | −50 de dano; quebra com 100 | `lib.rs` · `lib.rs:75-77,180-185`; `pollution.rs:344-349` · — | E2 |
| FUN-PRO-07 Monitor de itens/min | 9 linhas; até 4.095/min; esquece em 200 t | — · `industry.rs:1-7` · #40 (contido) | E2 |
| FUN-PRO-09 Painel de itens/min e motivo de espera | Leitura por minuto; % ativo | Integrado · Integrado · — | E3 (README §Projetos); E2 em `presentation.rs` |

FUN-PRO-02 (combustíveis) foi fundida em FUN-ENE-02. FUN-PRO-08 (bônus de refeição) foi fundida em FUN-LAR-03.

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 18:24 (0.6) | `5eaea2c` | Extratora, prensa, montador |
| 07/11 19:37 (0.8) | `7644aab` | Módulos de melhoria |
| 07/11 20:04 (0.9) | `5944d37` | Receitas por tabela |
| 07/11 ~23h | Grok/Jefter #20; `c5db4f6` | Produtividade a 20% |
| 08/11 16:38 | Dono | "Razões visíveis" ([DEC-021](../06_HISTORICO_E_DECISOES.md#dec-021)) |
| 08/11 ~17h | #40 | Cadeia funda, Mk.II, monitor ([DEC-019](../06_HISTORICO_E_DECISOES.md#dec-019)) |
| 08/11 19:16 | Dono | Tempos da 0.25 ([DEC-045](../06_HISTORICO_E_DECISOES.md#dec-045)) |

## F. Pendências

1. Decidir a produtividade: 10% ou 20% ([Q-14](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-14-produtividade-10-ou-20)).
2. Decidir se os itens da grande indústria entram na bolsa ([CNF-011](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)): hoje não são exportáveis.
3. Persistir o monitor ou registrar que ele recomeça vazio a cada sessão.
4. Subproduto e fluido industrial: sugeridos no documento de lacunas, não feitos.
5. Corrigir `frentes/PROGRESSAO-FASES.md`, `frentes/GROK-BALANCE.md` e o README do pr47 l.24.
6. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-045](../08_BACKLOG_E_PRIORIDADES.md#tar-045): Produtividade 10% ou 20%
- [TAR-056](../08_BACKLOG_E_PRIORIDADES.md#tar-056): Grande indústria na bolsa
- [TAR-059](../08_BACKLOG_E_PRIORIDADES.md#tar-059): Subproduto (escória) e fluido industrial

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01, 03, 06, 09) · pr47 **Integrado** (todas) |
| Confiança | Alta para receitas e módulos (E2, E1); média para o monitor (E2) |
| Prioridade (sugestão) | Média |
| Criticidade (sugestão) | Alta: é o núcleo do jogo |
| Responsável conhecido | Codex e Claude local (base); Claude do #40 (grande indústria); frente de balanço do Grok (produtividade) |
| DEC | [DEC-013](../06_HISTORICO_E_DECISOES.md#dec-013), [DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017), [DEC-019](../06_HISTORICO_E_DECISOES.md#dec-019), [DEC-021](../06_HISTORICO_E_DECISOES.md#dec-021), [DEC-045](../06_HISTORICO_E_DECISOES.md#dec-045) |
| Q | [Q-14](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-14-produtividade-10-ou-20), [Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros) |
| CNF | [CNF-016](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (produtividade), [CNF-011](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (grande indústria fora da bolsa) |
| Referências | README §Produção e §Projetos; `frentes/GRANDE-INDUSTRIA.md`; A2 PRO; A7 (`data.rs`) |
