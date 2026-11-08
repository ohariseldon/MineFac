# SIS-CTL: Centro de controle executivo

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. `center.rs` passa de 324 linhas (main) a 1.052 (pr47) com páginas, foco, obras, pedidos e caixa. Linhas em `native/api/src/workforce/` no pr47.

## A. Identidade

O painel executivo da vila e a sede da holding. Responde "o que está parado, onde e o que fazer". Atende à lacuna "falta de visão global" do documento de lacunas ([DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017)) com listas, não com mapa.

- **Depende de:** registros em memória publicados por [SIS-TRB](SIS-TRB.md) (postos, zonas, armazéns), [SIS-ECO](SIS-ECO.md) (livro Company, holding) e [SIS-OBR](SIS-OBR.md).
- **Alimenta:** [SIS-OBR](SIS-OBR.md) (slots Obra e Decreto), [SIS-TRB](SIS-TRB.md) (+20 no teto da vila com administrador; spawn), [SIS-LAR](SIS-LAR.md) (escritório).

## B. Funcionamento

### B.1 Slots (DEC-053)

A main tem os slots 0 a 5. O pr47 acrescenta 6 e 7.

| Slot | Função | Fonte |
|---|---|---|
| 0 | Comida do administrador | `mod.rs` |
| 1–3 | Metas de estoque: um item no slot pede `quantidade × 16` daquele item nos armazéns da área (4 pães = 64 pães) | `GOAL_SLOTS`, `META_UNIT`, `center.rs:26-30` |
| 4 | Cofre: esmeraldas depositadas viram moedas do livro Company | `center.rs:256-264,395` |
| 5 | Saque: devolve até 16 esmeraldas por vez | `cash_out`, `center.rs:419` |
| 6 | Obra: materiais das obras e dos pedidos; esmeraldas não pedidas vão para o caixa | `projects.rs:68`; `center.rs:256-264` |
| 7 | Decreto | `projects.rs:70` |

### B.2 Páginas

Um administrador contratado junta o que postos, zonas, armazéns e aldeões a até 64 blocos publicam em memória. A chave troca a página (`center.rs:1-22`):

1. **Resumo:** aldeões e teto, trabalhadores e esperas, comida guardada (em tarefas), zonas que podem crescer, armazéns, fila e metas.
2. **Atenção:** até 10 pendências em ordem de urgência, com distância e direção (`MAX_ISSUES`).
3. **Obras** ([SIS-OBR](SIS-OBR.md)).
4. **Pedidos:** prestígio, caixa, decreto e pedidos do dia.
5. **Abas da holding:** uma por empresa, mais comércio exterior ([SIS-ECO](SIS-ECO.md) FUN-ECO-05).

- **Livro:** clicar com um livro manda ao chat um relatório: postos com problema (posição e motivo), zonas e o que falta para o próximo nível, armazéns, metas.
- **Boletim da manhã:** tarefas de ontem, população, pedidos atendidos, pedidos de hoje, pendências.
- **Ritmo:** atualiza a cada 10 ciclos; registros com mais de 200 ticks são descartados.

### B.3 Pendências e foco

- Uma pendência entra em Atenção se o status do posto for de espera. `waiting()` aceita só {3, sem trabalhador (18), ausente (19), sem comida (21), sem caminho (22), sem esmeralda (25), canteiro bloqueado (37)} (`center.rs:111-113,193-205`).
- O centro também usa os códigos próprios 40 (zona pronta para crescer) e 41 (armazém cheio). São os mesmos números dos status de frete "sem rota" e "sem animal" ([SIS-TRA](SIS-TRA.md) B.4). Hoje não há efeito, porque `waiting()` filtra ([CNF-013](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
- **Foco** (`focus.rs:1-14`): com uma bússola, clique no centro, ou no Lar com escritório. O jogador recebe a pendência mais urgente:
  - a barra de ação mostra o que é, a distância e a direção;
  - partículas verdes marcam o bloco;
  - perto dele, a barra explica o que fazer;
  - resolvido, toca um sinal e vem a próxima.
  - Agachar limpa o foco. Um foco vence em 10 minutos (`EXPIRE 12000`). Lê só os registros em memória, sem varrer blocos.

### B.4 Centro e Quadro

O Quadro da fábrica ([SIS-PRG](SIS-PRG.md) B.2) também mostra metas, alertas e pedidos, com censo próprio. O centro vê a vila; o Quadro vê a fábrica. Os dois têm raio parecido (64 e 96) e nenhuma tela junta os dois.

## C. Experiência do jogador

1. A pessoa contrata um administrador e vê, numa página, quantos aldeões trabalham e quanta comida resta.
2. Na página Atenção, vê "lavrador sem comida, 23 blocos a NE". Pega a bússola, clica no centro e é guiada até lá.
3. Deposita esmeraldas no cofre para pagar obras e zonas.

## D. Implementação

**Arquivos:** `center.rs`, `focus.rs`, `presentation.rs` (painéis), `holding.rs` (abas). Acoplamento `workforce` ↔ `presentation`: 39 e 23 referências (A2).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-CTL-01 Centro com metas, cofre e saque | 8 slots; metas × 16; saque até 16 | `center.rs:16-33` (6 slots: 0–5) · `center.rs:26-45,256-264,395-419` · — | E2 |
| FUN-CTL-02 Abas da holding | Uma por empresa | `center.rs`; `holding.rs` · idem · — | E2 |
| FUN-CTL-03 Páginas Resumo, Atenção, Obras, Pedidos | 4 páginas; 10 pendências | — · `center.rs:1-22,30-41` · zip das obras (contido) | E2 |
| FUN-CTL-04 Foco e bússola | Vence em 12.000 t | — · `focus.rs:1-14,110-118` · idem | E2 |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 22:10 (0.13.0) | `36e43b6` | Centro de controle com metas |
| 08/11 16:31 (0.20.0) | `95a1f7c` | Holding |
| 08/11 | Obras | 8 slots, páginas, foco, boletim, caixa ([DEC-053](../06_HISTORICO_E_DECISOES.md#dec-053)) |

## F. Pendências

1. Criar um registro central de status, ou renumerar os do frete, para tirar a ambiguidade 40/41 ([CNF-013](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
2. Criar a tradução `veio.center.project.idle` (`center.rs:690`).
3. A "visão global" pedida na lacuna ainda é uma lista. Mapa ou fluxo continuam Concebidos.
4. Decidir se centro e Quadro se juntam numa só tela.
5. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-034](../08_BACKLOG_E_PRIORIDADES.md#tar-034): Registro central de status, slots e reservas
- [TAR-061](../08_BACKLOG_E_PRIORIDADES.md#tar-061): Visão global da fábrica no centro

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01, 02) · pr47 **Integrado** (01–04) |
| Confiança | Média (E2) |
| Prioridade (sugestão) | Baixa |
| Criticidade (sugestão) | Média: é onde o jogador entende a vila |
| Responsável conhecido | Claude nuvem ("Cloud"); agente das obras (páginas, foco) |
| DEC | [DEC-053](../06_HISTORICO_E_DECISOES.md#dec-053), [DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017) |
| Q | [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem) (nomes das escadas exibidas) |
| CNF | [CNF-013](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (status 40/41) |
| Referências | `HANDOFF_OBRAS.md`; [04 §10.2–10.3](../04_ARQUITETURA_E_DEPENDENCIAS.md); A2 CTL; A7 (40/41) |
