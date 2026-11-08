# SIS-MUN: Mundo (jazidas, rios, mapa finito)

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. Linhas em `native/api/src/` no pr47, salvo indicação.

## A. Identidade

O mundo vanilla é o tabuleiro da fábrica. O Veio não remodela o Minecraft do zero (dono, 15:55, [DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039)). Ele lê e altera o mundo em três pontos:
1. **jazidas:** a pureza do minério sob a extratora define o ritmo;
2. **rios:** traçados por cima do terreno da seed, dão água e quedas para a energia;
3. **limite do mapa:** um mundo finito, com todos os recursos ao alcance (rascunho desligado).

- **Restrição de plataforma:** o Horizon não lê worldgen de data pack, e o gancho de geração substitui o terreno inteiro ([DEC-009](../06_HISTORICO_E_DECISOES.md#dec-009); `rivers.rs:4-5`).
- **Alimenta:** [SIS-EXT](SIS-EXT.md) (pureza), [SIS-AGU](SIS-AGU.md) e [SIS-ENE](SIS-ENE.md) (água e quedas), [SIS-DES](SIS-DES.md) (limite).

## B. Funcionamento

### B.1 Pureza da jazida

| Camada | Como mede | Classes |
|---|---|---|
| main | O quadrado 3×3 abaixo da extratora (`extraction.rs:3-4,23-28@main`) | Até 9 blocos: impura, normal, pura |
| pr47 | O **veio conectado em 3D** a partir do bloco abaixo, por faces, até 16 blocos (`VEIN_LIMIT`). No máximo 97 leituras, a cada 200 ticks, dentro da cota de terreno (`VEIN_COST 24`) (`extraction.rs:1-16`) | 0–2 impura (metade da velocidade); 3–6 normal; 7–15 pura (dobro); ≥ 16 **industrial** (quádruplo, com o dobro de energia via `rich_draw`, `:34-57`) |

- Minério comum e de ardósia contam como a mesma jazida.
- Pedra, areia e as demais jazidas infinitas continuam no 3×3. Por isso não chegam à classe industrial.
- Diamante, esmeralda e detritos param na classe pura (README, E3).
- As jazidas não se esgotam.

### B.2 Rios com nascente

Fonte: `rivers.rs:1-18` e constantes `:26-50`.

1. **Quando:** só em chunks recém-gerados. `chunk_load` com `new` marca o chunk (`lib.rs:288` → `rivers.rs:758`) e a janela é de 6.000 ticks (`FRESH_WINDOW`). Construções e mundos antigos ficam intactos. Terreno com blocos feitos pelo homem, como vilas, é contornado.
2. **Nascente:** acima de y 100 (`SPRING_MIN`), a pelo menos 40 blocos de outra (`SPRING_SPACING`).
3. **Traçado:** segue a descida mais íngreme. Num buraco sem saída, procura a passagem com menor corte, até 8 blocos (`MAX_CUT`). Sem passagem, vira lagoa.
4. **Quedas:** o nível só desce, em degraus de pelo menos 3 blocos. Daí as cachoeiras, onde dá para represar ([SIS-ENE](SIS-ENE.md) B.4).
5. **Vazão:** nascentes somadas mais um tanto pela distância percorrida (`RUN_PER_FLOW 48`). Largura e profundidade crescem com a vazão. Termina no mar (y 63) ou num lago. Percurso máximo de 6.000 blocos.
6. **Custo por tick:** 48 passos, 2 varreduras e 2 chunks editados por tick; cache de 1.536 colunas.
7. **Espera:** a frente para quando o próximo chunk não existe e continua quando o jogador explora.
8. **Estado:** `<mundo>/veio/rios.txt`, gravado a cada 1.200 ticks por arquivo temporário (`rivers.rs:769`).

**Risco:** `read_columns` e `chunk_load` entraram em `bcfb9d5`, depois da última compilação real ([R-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)). Editar blocos dentro de `chunk_load` e ler colunas de um chunk recém-gerado nunca foram vistos no Horizon.

### B.3 Mapa finito

- **Contrato (MAPA.md):** Overworld de 15.000 × 15.000 (de −7.500 a 7.500), muralha de bedrock, todos os biomas e estruturas garantidos, Nether de 1.875 de lado (8:1), portais seguros. Cerca de 45 min a pé de borda a borda.
- **Estado:** rascunho **desligado** em `work-in-progress/finite-world/` (10 arquivos `.rs`, cerca de 1.245 linhas), fora do crate. Usa mixins de motor: 22 de 24 injeções. `native_chunk_generate` não existe no Horizon `66f3981`. A auditoria do Nether falha e não há testes de portal (`CONTINUAR.md`, E3).
- **Origem:** `MAPA.md` diz "decisão de produto autorizada pelo usuário", mas não há registro no chat. Em 20:09 o dono pediu "limite do mapa e otimizar" ([DEC-006](../06_HISTORICO_E_DECISOES.md#dec-006), [DEC-056](../06_HISTORICO_E_DECISOES.md#dec-056), [CNF-019](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
- **Thread ativa:** "Definir limite do mapa e otimizar". O PR #52 dessa thread não toca no mapa ([07](../07_AGENTES_E_WORKFLOW.md)).

## C. Experiência do jogador

1. Achar um veio de cobre com 16 blocos conectados vale montar ali uma operação maior (pr47). Na main, só o 3×3 importa.
2. Explorando montanhas novas, a pessoa encontra córregos que viram rios e caem em degraus. Ali põe uma represa e uma hidrelétrica.
3. O mapa finito daria um mundo com borda e todos os recursos. Hoje não existe no jogo.

## D. Implementação

**Arquivos:** `production/extraction.rs`; `rivers.rs` (849 linhas) e `rivers/`; `work-in-progress/finite-world/` (fora do crate). Testes: rivers 9 (E1).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-MUN-01 Pureza 3×3 | Até 9 blocos | Integrado (`extraction.rs:23-28@main`) · **Substituído** por MUN-02 · — | E2 |
| FUN-MUN-02 Veio conectado e classe industrial | 3D por faces; até 16; ×4 com o dobro de energia | — · `extraction.rs:11-15,34-57` · #23 protótipo e import-progressao (contidos) | E2 |
| FUN-MUN-03 Rios com nascente ([DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039)) | Ver B.2 | — · `rivers.rs:26-50,758,769`; `lib.rs:288` · #40 (contido) | E1 (9 testes) + E2; APIs ND no Horizon |
| FUN-MUN-04 Mapa finito ([DEC-006](../06_HISTORICO_E_DECISOES.md#dec-006)) | 15.000²; muralha; 8:1 | — · Em desenvolvimento (desligado) · `c8e94dc` = `80980db` | E3 |
| FUN-MUN-05 Portais e Nether seguros | Portais dentro do limite | — · Em desenvolvimento · idem | E3 (auditoria falha) |
| FUN-MUN-06 Jazidas maiores e espaçadas ([DEC-011](../06_HISTORICO_E_DECISOES.md#dec-011), item 2) | Depósitos maiores em terreno novo, no estilo Shapez | — · — · Concebido (`frentes/JAZIDAS-CONECTADAS.md`) | E3 |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 20:04 (0.9) | `5944d37` | Jazidas com pureza ([DEC-010](../06_HISTORICO_E_DECISOES.md#dec-010)) |
| 07/11 noite | PR #23, `frentes/JAZIDAS-CONECTADAS.md` | Proposta Shapez ([DEC-011](../06_HISTORICO_E_DECISOES.md#dec-011)) |
| 07/11 | `MAPA.md` | Contrato do mapa finito ([DEC-006](../06_HISTORICO_E_DECISOES.md#dec-006)) |
| 08/11 15:55 | Dono | Rios com nascente, afluentes, quedas e foz ([DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039)) |
| 08/11 ~17h | #40 | Rios |
| 08/11 18:16 | `c8e94dc` (cherry-pick em `80980db`) | Rascunho do mapa finito |
| 08/11 20:09 | Dono | "Limite do mapa e otimizar" ([DEC-056](../06_HISTORICO_E_DECISOES.md#dec-056)) |

## F. Pendências

1. Confirmar com o dono o tamanho de 15.000 e se ele quer geração garantida ou só limite ([Q-09](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-09-mapa-finito), [CNF-019](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
2. Compilar e ver os rios no Horizon real: `read_columns`, `chunk_load(new)`, `edit_blocks` durante a geração ([R-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
3. Pôr os rios em `check_performance`.
4. Dar versão de formato a `rios.txt` ([SIS-PER](SIS-PER.md)).
5. Implementar a FUN-MUN-06, se aprovada.

### Tarefas no backlog

- [TAR-024](../08_BACKLOG_E_PRIORIDADES.md#tar-024): Rios em jogo
- [TAR-025](../08_BACKLOG_E_PRIORIDADES.md#tar-025): Mapa finito e limite do mapa
- [TAR-058](../08_BACKLOG_E_PRIORIDADES.md#tar-058): Jazidas maiores e espaçadas em terreno novo

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main: MUN-01 **Integrado** · pr47: 02–03 **Integrado**, 01 **Substituído**, 04–05 **Em desenvolvimento** · disperso: 06 **Concebido** |
| Confiança | Baixa para os rios no jogo (APIs nunca compiladas); alta para a pureza (E2) |
| Prioridade (sugestão) | Alta para compilar os rios; média para o mapa, que aguarda a [Q-09](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-09-mapa-finito) |
| Criticidade (sugestão) | Alta: rios dependem de APIs não verificadas; um erro aqui afeta a geração de mundo |
| Responsável conhecido | Claude local (pureza); Codex/fases (industrial); Claude do #40 (rios); Codex local e thread do mapa (mapa finito) |
| DEC | [DEC-006](../06_HISTORICO_E_DECISOES.md#dec-006), [DEC-009](../06_HISTORICO_E_DECISOES.md#dec-009), [DEC-010](../06_HISTORICO_E_DECISOES.md#dec-010), [DEC-011](../06_HISTORICO_E_DECISOES.md#dec-011), [DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039), [DEC-056](../06_HISTORICO_E_DECISOES.md#dec-056) |
| Q | [Q-09](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-09-mapa-finito) |
| CNF | [CNF-019](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (`MAPA.md` sem registro no chat) |
| Referências | `MAPA.md`, `CONTINUAR.md`, `frentes/AGUA-E-RIOS.md`, `frentes/JAZIDAS-CONECTADAS.md`; A2 MUN; A7 (APIs de mundo) |
