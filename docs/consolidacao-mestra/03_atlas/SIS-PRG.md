# SIS-PRG: Progressão

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. As duas camadas diferem muito: o pr47 acrescenta o Quadro da fábrica, licenças, pesquisa e grande projeto, e muda o que cada avanço libera.
- **Linhas citadas:** `native/api/src/` e `content/data/veio/advancement/` no pr47, salvo indicação.
- **Escopo:** avanços, Quadro da fábrica, pedido do dia, gargalos, central de contratos e grande projeto. O laboratório e a pesquisa da vila estão em [SIS-PES](SIS-PES.md). As escadas da vila (fases da prefeitura, prestígio) estão em [SIS-VIL](SIS-VIL.md) e [SIS-OBR](SIS-OBR.md).

## A. Identidade

Diz como o jogador avança: o que ele pode construir agora e qual é o próximo objetivo. **É o sistema mais fragmentado do jogo.** O pr47 tem seis mecanismos de avanço sobrepostos, criados por agentes diferentes em poucas horas e nunca desenhados em conjunto:

| # | Mecanismo | Quem criou | Libera ou dá |
|---|---|---|---|
| 1 | Avanços com receitas (`rewards.recipes`) | Codex/Claude local (0.4–0.9) | Receitas |
| 2 | Quadro da fábrica: 5 fases e 3 licenças | "Claude (MineFac)" ([DEC-016](../06_HISTORICO_E_DECISOES.md#dec-016)) | Itens e licenças que concedem avanços |
| 3 | Laboratório de pesquisa ([SIS-PES](SIS-PES.md)) | Claude do #40 ([DEC-018](../06_HISTORICO_E_DECISOES.md#dec-018)) | Avanços `veio:research/*` com as receitas centrais |
| 4 | Pedido do dia do Quadro | "Claude (MineFac)" | 1 a 6 esmeraldas |
| 5 | Central de contratos | Codex/Claude local (0.5) | Itens e esmeraldas |
| 6 | Grande projeto em 4 fases | Claude do #40 ([DEC-020](../06_HISTORICO_E_DECISOES.md#dec-020)) | Esmeraldas, módulos, kits, estrelas e faróis |

Fora daqui, a vila tem mais quatro escadas: fases da prefeitura, prestígio, reputação comercial e níveis de zona. **Nenhuma tela mostra todas juntas.**

- **Origem do problema:** o dono aprovou "conserte essas lacunas" em 16:32 ([DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017)). Uma das lacunas era "progressão por item": bastava ter 1 aço para liberar a eletrônica. Duas frentes atacaram o mesmo problema ao mesmo tempo: fases com pilha ([DEC-014](../06_HISTORICO_E_DECISOES.md#dec-014)/016) e laboratório ([DEC-018](../06_HISTORICO_E_DECISOES.md#dec-018)). As duas entraram no pr47.
- **Depende de:** [SIS-PRO](SIS-PRO.md) (o que se entrega), [SIS-PES](SIS-PES.md) e [SIS-ECO](SIS-ECO.md) (esmeraldas pagas).
- **Alimenta:** todos os sistemas cujas receitas são liberadas por avanço: [SIS-ENE](SIS-ENE.md), [SIS-EXT](SIS-EXT.md), [SIS-LOG](SIS-LOG.md), [SIS-TRA](SIS-TRA.md), [SIS-DEF](SIS-DEF.md), [SIS-MEG](SIS-MEG.md).

## B. Funcionamento

### B.1 Avanços que liberam receitas

Cada avanço tem uma lista `rewards.recipes`. Ao ser concedido, o Minecraft adiciona essas receitas ao livro de receitas do jogador.

**A trava pode não travar.** Nenhum mundo do Veio liga a regra `doLimitedCrafting`, em nenhum ramo (grep vazio, E2). Sem ela, no Minecraft vanilla, uma receita não liberada só some do livro, mas pode ser feita na bancada. O comportamento do Horizon é ND ([CNF-003](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [R-21](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)). Se for igual ao vanilla, toda a progressão por receita é só visual.

**O que cada avanço central libera:**

| Avanço | Critério main | Libera na main | Critério pr47 | Libera no pr47 |
|---|---|---|---|---|
| `engineering` | 1 engrenagem | chave, britador, forno de liga, kit de reparo, eólica | 32 engrenagens **ou** `permit_workshop` | o mesmo + `research_lab` |
| `steel` | 1 chapa de aço | bateria, geotérmica, eletrônica, lotes de esteira e cabo, poste de aço, vapor, silvicultura, rancho | 16 aço **ou** `permit_steel` | só eletrônica e elevador de carga |
| `circuits` | 1 circuito | solar, Mk.II, Tesla, quartel, módulos de velocidade e eficiência, lâmpada | 16 circuitos **ou** `permit_electronics` | quartel, copiadora, marco de obra |
| `upgrades` | os 3 módulos | broca Mk.III | os 3 módulos | nada |
| `research/metallurgy` | — | — | pesquisa concluída | bateria, geotérmica, lotes, poste de aço, vapor, silvicultura, rancho, monitor, túnel |
| `research/electronics` | — | — | idem | solar, Mk.II, Tesla, módulos, lâmpada, hidrelétrica, âncora, estações |
| `research/automation` | — | — | idem | fabricador, Mk.III |
| `research/scale` | — | — | idem | esteira expressa |

Fonte: `engineering.json`, `steel.json`, `circuits.json`, `upgrades.json` e `research/*.json` nos dois ramos (E2). Os critérios do pr47 usam `requirements: [["stock","permit"]]`, ou seja, pilha **ou** licença. Os de pesquisa usam o gatilho `impossible`: só o código concede (`research.rs:166`).

### B.2 Quadro da fábrica

O quadro é um painel com três funções (`milestones/mod.rs:1-13`):
1. **Fases**, no estilo Satisfactory: entregar a pilha da fase no quadro, à mão ou por esteira, devolve a recompensa.
2. **Pedido do dia**, no estilo Punch Club (B.3).
3. **Gargalos**, no estilo Factorio (B.4).

Na fase 1, o painel vira uma lista de montagem (energia, extratora, prensa, montador) que se marca sozinha quando as máquinas aparecem perto.

| Fase | Pilha | Recompensa | O painel diz que libera | De onde vem de fato no pr47 |
|---|---|---|---|---|
| 1 Primeira linha | 16 chapas de cobre | 32 esteiras | (mostra a recompensa) | — |
| 2 Oficina | 32 engrenagens | `permit_workshop` | chave, britador, forno de liga | `engineering` ✔ |
| 3 Aço | 16 chapas de aço | `permit_steel` | vapor, bateria, geotérmica | **`research/metallurgy`** ✘ |
| 4 Eletrônica | 16 circuitos | `permit_electronics` | Mk.II, solar, Tesla | **`research/electronics`** ✘ |
| 5 Profundidade | 32 circuitos | 2 diamantes | Mk.III | **`research/automation`** ✘ |

Fonte: `PHASES`, `milestones/mod.rs:37-52`; texto "Libera", `:246-247`.

**Defeito ([CNF-002](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [R-20](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)):** nas fases 3 a 5, o quadro promete máquinas que a licença não libera. No pr47 elas vêm da pesquisa. Quem entrega 16 aço recebe a licença, mas só ganha eletrônica e elevador, e vê "vapor, bateria, geotérmica" na tela.

### B.3 Pedido do dia do Quadro

- A cada amanhecer, um pedido que vale até o próximo amanhecer. É escolhido pelo dia e pela fase do quadro (`ORDERS`, `milestones/mod.rs:56-69`).
- 12 pedidos, de 64 chapas por 1 esmeralda até 64 circuitos por 6.
- Paga **esmeraldas criadas do nada** no slot 1 do quadro (`milestones/mod.rs:147-150`). Não passa pelos livros ([SIS-ECO](SIS-ECO.md) B.5).
- Uma vez por dia por dimensão (`census::claim_order`). O controle `paid_day` fica em memória (`census.rs:114`): **reabrir o mundo permite receber de novo no mesmo dia**.
- Coexiste com os pedidos da vila ([SIS-OBR](SIS-OBR.md) FUN-OBR-04), que são pagos pelo caixa público ([DEC-035](../06_HISTORICO_E_DECISOES.md#dec-035)). Ver [CNF-004](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) e [Q-02](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-02-dois-pedidos-diários).

### B.4 Alertas de gargalo (censo)

- Cada máquina que consome ou gera energia se registra quando muda de estado e, de resto, a cada 100 ticks (`census.rs:1-12`). Nada aqui lê o mundo.
- O quadro lê o censo a cada segundo. O tick global avisa os jogadores a até 32 blocos a cada 100 ticks: máquinas paradas por energia (depois de 200 ticks), combustível, saída cheia (depois de 1.200 ticks) ou dano. Também anuncia amanhecer e anoitecer.
- O quadro alcança 96 blocos (`BOARD_REACH`).
- O centro de controle tem um censo próprio, separado ([SIS-CTL](SIS-CTL.md)).

### B.5 Central de contratos

A central (`contract_hub`) aceita uma sequência de contratos (`CONTRACTS`, `contracts.rs`):

| # | Contrato | Entrega | Recompensa |
|---|---|---|---|
| 1 | Oficina | 24 chapas + 8 engrenagens | 64 esteiras + 32 cabos |
| 2 | Ferrovia | 24 aço + 12 módulos (main: 16 + 8) | 64 trilhos + 16 trilhos energizados |
| 3 | Povoado | 24 circuitos + 32 estruturas (main: 12 + 24) | 8 funis + **8 esmeraldas** |
| 4 | Farol (opcional, com estrela no slot) | 1 estrela do Nether + 48 circuitos (main: 32) | 1 farol + 2 brocas Mk.II |
| — | Exportação contínua (depois dos contratos) | 4 circuitos + 4 aço | **4 esmeraldas** + 2 frascos de experiência, sem limite |

As 8 esmeraldas do Povoado e as 4 da exportação são criadas fora dos livros ([CNF-001](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

### B.6 Grande projeto

A torre de transmissão, em 4 fases, como o elevador espacial do Satisfactory (`contracts.rs` `PHASES`). Cada fase recolhe até 4 peças por tick (`PHASE_INTAKE`) e paga ao fechar:

| Fase | Entrega | Recompensa |
|---|---|---|
| 1 | 200 aço + 100 módulos | 32 esmeraldas + 4 módulos de velocidade |
| 2 | 300 circuitos + 100 motores | 48 esmeraldas + 4 módulos de produtividade |
| 3 | 150 processadores + 50 núcleos de automação | 64 esmeraldas + 8 kits Mk.II |
| 4 | 200 núcleos + 50 kits Mk.II | 2 estrelas do Nether + 2 faróis |

A fase 4 concede o avanço `veio:grand_project` a `@a` (`lib.rs:1101`). Não há nada depois disso. Fim de jogo: [Q-10](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-10-fim-de-jogo).

### B.7 Caminho de um jogador no pr47 (síntese)

1. Cobre → 16 chapas no quadro (fase 1) → esteiras.
2. 32 engrenagens → `engineering` → britador, forno de liga e **laboratório**.
3. 16 aço → `steel` → eletrônica.
4. Laboratório: metalurgia (128 aço + 128 engrenagens) → vapor, bateria, geotérmica, poste de aço, monitor.
5. 16 circuitos → `circuits` → quartel e copiadora. Laboratório: eletrônica (128 circuitos + 128 estruturas) → solar, Mk.II, Tesla, módulos, hidrelétrica, âncora, estações.
6. Motores e processadores → automação → fabricador e Mk.III → núcleos → escala → esteira expressa.
7. Grande projeto, fases 1 a 4.

Em paralelo: a vila sobe de fase (prefeitura), de prestígio (obras) e de reputação (comércio).

## C. Experiência do jogador

- **Esperada:** metas claras na primeira hora (lista de montagem do quadro), depois marcos de produção crescentes e um projeto final.
- **Real no código:**
  - o jogador vê ao menos quatro barras de nível sem relação entre si;
  - o quadro anuncia o que não entrega;
  - se o Horizon seguir o vanilla, ele pode fabricar tudo desde o início na bancada.
- Nunca jogado.

## D. Implementação

**Arquivos**
- `milestones/mod.rs` (quadro, fases, pedidos), `milestones/census.rs` (censo), `milestones/tests.rs` (9 testes).
- `contracts.rs` (central e grande projeto).
- `content/data/veio/advancement/*.json` (84 arquivos no pr47, incluindo `research/`, `tutorial/` e `village/`; 66 na main).
- `lib.rs:1101` (concessão do `grand_project`).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-PRG-01 Avanços liberando receitas | Ver B.1; sem `doLimitedCrafting` | `advancement/*.json` (critério 1 item) · `advancement/*.json` (pilha ou licença; `research/*`) · — | E2; trava real ND ([CNF-003](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)) |
| FUN-PRG-02 Quadro, fases e licenças ([DEC-016](../06_HISTORICO_E_DECISOES.md#dec-016)) | 5 fases; 3 licenças; texto "Libera" | — · `milestones/mod.rs:37-52,246-247` · import-factory-board (contido) | E1 (9 testes) + E2; **texto divergente** |
| FUN-PRG-03 Pedido do dia do Quadro | 12 pedidos; 1–6 esmeraldas criadas; `paid_day` em memória | — · `milestones/mod.rs:56-69,147-150`; `census.rs:114` · idem | E2 |
| FUN-PRG-04 Alertas de gargalo | Censo a cada 100 t; alerta a 32 blocos | — · `milestones/census.rs:1-20` · idem | E2 |
| FUN-PRG-06 Central de contratos | 4 contratos + exportação contínua | `contracts.rs` (valores menores) · `contracts.rs:7-28,57-61` · — | E2 |
| FUN-PRG-07 Grande projeto ([DEC-020](../06_HISTORICO_E_DECISOES.md#dec-020)) | 4 fases; 4 peças/t; avanço final a `@a` | — · `contracts.rs:29-55`; `lib.rs:1101` · #40 (contido) | E2 |
| FUN-PRG-08 Progressão por fases do Codex (30/min) | Tempos longos e unidade de 30/min | — · **Substituído** · Substituído (`0ce91a4`) | E2 ([DEC-045](../06_HISTORICO_E_DECISOES.md#dec-045)) |
| FUN-PRG-09 Meta sustentada | 20 circuitos/min por 3 min ([DEC-023](../06_HISTORICO_E_DECISOES.md#dec-023)) | — · — · Concebido (README) | E3 |

FUN-PRG-05 (laboratório) passou a FUN-PES-01. Estados detalhados em [05](../05_ESTADO_REAL_DA_BUILD.md).

## E. Histórico

| Quando | Commit / fonte | Mudança |
|---|---|---|
| ≤ 0.5 | README | Central de contratos; avanços com 1 item |
| 07/11 ~23h | Grok/Jefter #20 | Missões rebalanceadas ([DEC-013](../06_HISTORICO_E_DECISOES.md#dec-013)) |
| 08/11 05:35 | `claude/progressao-fases` | Fases com pilha, unidade de 30/min, quadro ([DEC-014](../06_HISTORICO_E_DECISOES.md#dec-014), [DEC-016](../06_HISTORICO_E_DECISOES.md#dec-016)) |
| 08/11 madrugada | Codex local | "Fases substituem os tempos" ([DEC-015](../06_HISTORICO_E_DECISOES.md#dec-015)) |
| 08/11 16:32 | Dono | "Conserte essas lacunas" ([DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017)) |
| 08/11 ~16:40–17:06 | #40 | Laboratório, grande projeto; avanços `steel`/`circuits` perdem receitas para `research/*` ([DEC-018](../06_HISTORICO_E_DECISOES.md#dec-018), [DEC-020](../06_HISTORICO_E_DECISOES.md#dec-020)) |
| 08/11 19:16 | Dono | Tempos da 0.25 ([DEC-045](../06_HISTORICO_E_DECISOES.md#dec-045)); fases do Codex substituídas |

## F. Pendências

1. Decidir a espinha da progressão: laboratório, fases do quadro, ou fases como portas de níveis de pesquisa ([Q-03](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão)).
2. Corrigir o texto "Libera" das fases 3–5, ou mudar o que as licenças concedem.
3. Testar no Horizon se uma receita não liberada pode ser fabricada. Se puder, decidir se o Veio liga `doLimitedCrafting` ([CNF-003](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
4. Decidir o pedido do dia do Quadro: fundir com os pedidos da vila, pagar pelo livro ou manter ([Q-02](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-02-dois-pedidos-diários)). Persistir `paid_day` se ficar.
5. Decidir as esmeraldas criadas pela central e pelo grande projeto ([Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros)).
6. Definir o fim de jogo e a meta sustentada ([Q-10](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-10-fim-de-jogo)).
7. Confirmar o que acontece com quem entra no mundo depois de uma concessão a `@a` ([Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan)).
8. Atualizar o README §Progressão e `frentes/PROGRESSAO-FASES.md`, que descrevem tempos e unidades substituídos.
9. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-005](../08_BACKLOG_E_PRIORIDADES.md#tar-005): Verificar se o avanço tranca a receita (`doLimitedCrafting`)
- [TAR-016](../08_BACKLOG_E_PRIORIDADES.md#tar-016): Alinhar o Quadro e a pesquisa ("Libera")
- [TAR-028](../08_BACKLOG_E_PRIORIDADES.md#tar-028): Pedido do dia persistente ou fundido
- [TAR-043](../08_BACKLOG_E_PRIORIDADES.md#tar-043): Fim de jogo e meta sustentada

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (PRG-01, 06) · pr47 **Integrado** (01–04, 06, 07), **Substituído** (08) · disperso: Concebido (09) |
| Confiança | Baixa para o efeito em jogo (trava ND); alta para o conteúdo dos JSON (E2) |
| Prioridade (sugestão) | **Muito alta**: é a segunda pergunta da ordem sugerida ([Q-03](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão)) |
| Criticidade (sugestão) | Alta: se a trava não funciona, o jogo de fábrica perde a estrutura |
| Responsável conhecido | Codex e Claude local (avanços, central); "Claude (MineFac)" (quadro); Claude do #40 (pesquisa, grande projeto) |
| DEC | [DEC-013](../06_HISTORICO_E_DECISOES.md#dec-013), [DEC-014](../06_HISTORICO_E_DECISOES.md#dec-014), [DEC-015](../06_HISTORICO_E_DECISOES.md#dec-015), [DEC-016](../06_HISTORICO_E_DECISOES.md#dec-016), [DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017), [DEC-018](../06_HISTORICO_E_DECISOES.md#dec-018), [DEC-020](../06_HISTORICO_E_DECISOES.md#dec-020), [DEC-022](../06_HISTORICO_E_DECISOES.md#dec-022), [DEC-023](../06_HISTORICO_E_DECISOES.md#dec-023), [DEC-045](../06_HISTORICO_E_DECISOES.md#dec-045) |
| Q | [Q-03](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão), [Q-02](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-02-dois-pedidos-diários), [Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros), [Q-10](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-10-fim-de-jogo), [Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan) |
| CNF | [CNF-002](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (duas espinhas), [CNF-003](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (`doLimitedCrafting`), [CNF-004](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (dois pedidos), [CNF-001](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (emissões), [CNF-011](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (grande indústria fora da bolsa) |
| Referências | `frentes/PROGRESSAO-FASES.md`, `frentes/EXPERIENCIA-JOGADOR.md`, `frentes/GRANDE-INDUSTRIA.md`, `frentes/BALANCEAMENTO-MISSOES.md`; A2 PRG; A7 (licenças × pesquisa) |
