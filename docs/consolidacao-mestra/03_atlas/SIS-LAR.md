# SIS-LAR: Casa do fundador e Lar do jogador

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `pr47@80980db`. O sistema não existe na main. Linhas em `native/api/src/` no pr47.
- **Duas peças, uma decisão:** a casa do fundador (`home/`, do Claude local via Codex) e o Lar `veio:home` (`workforce/home.rs`, do agente das obras). Em 19:16 o dono decidiu: a casa do fundador com o Lar dentro; refeição e bônus de casa só pelo Lar; o bônus de comer fora continua ([DEC-046](../06_HISTORICO_E_DECISOES.md#dec-046)).

## A. Identidade

É a casa do jogador no centro da vila: um lugar para voltar todo dia, comer, dormir e melhorar com os produtos da própria fábrica.

- **Depende de:** [SIS-TRB](SIS-TRB.md) (centro da vila), [SIS-CTL](SIS-CTL.md) (painel que vem na casa), [SIS-AGR](SIS-AGR.md) (comida).
- **Alimenta:** [SIS-PRO](SIS-PRO.md) (ritmo das máquinas, pelo bônus de comer fora) e [SIS-OBR](SIS-OBR.md) (prestígio).

## B. Funcionamento

### B.1 Casa do fundador

Fonte: `home/mod.rs:1-17`.

1. **Construção:** uma vez por mundo, quando o centro da vila é conhecido. O Veio procura terreno plano ao norte do centro e roda a função `veio:home/build`, gerada por `tools/build_house.py` (251 comandos; pegada 13 × 11). A marca `veio:home built` no armazenamento de comandos impede uma segunda casa.
2. **Conteúdo:** centro de controle, sofá, cama ao lado do painel, mural de mapas (4 mapas, id 1970000) e, no lugar da lareira, o bloco `veio:home` (`build_house.py:177-178`).
3. **Em casa:** perto do painel e debaixo do telhado (alcance 9). Quem entra recebe um aviso. **A casa em si não acelera as máquinas.**
4. **Camas** (`bed_allowed`, `home/mod.rs:306`): a primeira cama de cada jogador é uma da casa. Depois ele pode marcar só mais uma fora; para trocar, quebra a antiga.
5. **Persistência:** `Home = 1` e `HomeBeds` no NBT do painel.

### B.2 Lar (`veio:home`)

Fonte: `workforce/home.rs:1-61`.

| Parte | Slot | Regra |
|---|---|---|
| Lareira | 0 | Carvão, carvão vegetal, biomassa, troncos ou tábuas. Acesa, aquece e solta fumaça; do nível 2 em diante, cozinha o cru da despensa |
| Despensa | 1–3 | Comida. Com uma tigela na mão, clique no Lar: **refeição em casa**, uma a cada 2 minutos (`MEAL_COOLDOWN 2400`) |
| Melhoria | 4 | Materiais da fábrica sobem o nível (até 4) |
| Convidados | 6 | Banquete de 1.200 ticks |

- **Dono:** quem estiver perto quando o Lar começa a funcionar. Renasce ali.
- **Efeitos da refeição em casa** (no jogador, não nas máquinas): saturação, regeneração, pressa por 180 s × poder, resistência, visão noturna, absorção. Crescem com o valor da comida, o cardápio variado (pratos diferentes nas últimas 5 refeições) e o conforto. O banquete dá `hero_of_the_village` (`workforce/home.rs:555,601,642,693`).
- **Conforto** (0–100): o Lar lê a casa num 9 × 9, de um bloco abaixo a três acima, 27 blocos por ciclo. Soma cama (15), porta (10), janela, luz, baú, fogão, bancada, decoração e acabamento (15).
- **Descanso:** quem passa a noite em casa com a lareira acesa acorda descansado.
- **Níveis:**
  1. inicial;
  2. casa de tijolos (24 tijolos, 12 vidros, 8 chapas de cobre): a lareira cozinha; efeitos +50%;
  3. casa elétrica (8 fios, 2 circuitos, 8 chapas de ferro): escritório; o painel mostra a vila e a bússola dá o foco das pendências;
  4. casa de engenheiro (4 chapas de aço, 4 engrenagens, 1 módulo): pressa II.
- **Coisas a fazer:** o painel lista o dia (comer em casa, lareira acesa, despensa abastecida). Um dia em ordem rende experiência, conta a sequência e dá +2 de prestígio. O banquete dá +5 ([SIS-OBR](SIS-OBR.md) B.3).

### B.3 Bônus de comer fora de casa (o único efeito nas máquinas)

Fonte: `home/food.rs:1-70`; `home/mod.rs:99-103`; `upgrades.rs:44`.

| Comida | Exemplos | Máquinas | Duração |
|---|---|---|---|
| Lanche | cru, fruta, biscoito | — | — |
| Refeição | carnes e peixes assados, pão, batata assada, torta | ciclo × 0,95 | 6.000 ticks (5 min) |
| Banquete | ensopados, sopas, cenoura dourada | ciclo × 0,91 | 9.600 ticks (8 min) |

- **Vale só fora de casa.** Dentro da casa, `food::eaten` não dá o bônus; a refeição em casa é a do Lar.
- **Global:** o estado fica em dois atômicos (`MEAL_UNTIL`, `FEAST_UNTIL`). Vale para **todas as máquinas do mundo**. Os bônus de vários jogadores não se somam; vale o maior ([CNF-017](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan)).
- O teste `home/tests.rs:163` confirma: "estar em casa não acelera as máquinas".
- O comentário `food.rs:9` ("comer dentro de casa dobra a duração") está obsoleto.

### B.4 Leitura da DEC-046

A decisão diz "refeição e bônus de casa só pelo Lar; o bônus de comer fora permanece". O código cumpre, se "bônus de casa" quer dizer os **efeitos no jogador** dados pelo Lar. O "+15% em casa" da casa do fundador ([DEC-036](../06_HISTORICO_E_DECISOES.md#dec-036), `HOME_TIME 0,87`) foi removido. O README do pr47 l.21 ainda o promete ([CNF-006](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

## C. Experiência do jogador

1. O jogador nasce numa casa pronta, com painel, cama e lareira.
2. Acende a lareira, enche a despensa e come em casa: ganha pressa e regeneração.
3. Vai trabalhar. Se comer um pão no campo, as máquinas do mundo inteiro andam 5% mais rápido por 5 minutos.
4. Melhora a casa até o escritório e passa a ver as pendências da vila dali.

Nunca jogado.

## D. Implementação

**Arquivos:** `home/mod.rs` (424 linhas), `home/food.rs`, `home/mural.rs`, `home/tests.rs` (5 testes); `workforce/home.rs` (871 linhas); `tools/build_house.py`.

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-LAR-01 Casa do fundador (estrutura, mural) | Uma por mundo; 251 comandos; 13 × 11 | — · `home/mod.rs:1-17`; `build_house.py` · `d1e2f30` (contido) | E1 + E2 |
| FUN-LAR-02 Lar `veio:home` | Lareira, despensa, conforto, níveis 1–4, banquete | — · `workforce/home.rs:30-61,555-693` · zip das obras (contido) | E2 |
| FUN-LAR-03 Ritmo: bônus de comer fora (inclui a ex-PRO-08) | ×0,95 por 6.000 t; ×0,91 por 9.600 t; só fora de casa; global | — · `home/food.rs:22-29,59-69`; `home/mod.rs:99-103`; `upgrades.rs:44` · — | E2; teste `home/tests.rs:163` (E1) |
| FUN-LAR-04 Cama restrita à casa | Primeira cama na casa; uma fora | — · `home/mod.rs:306` · — | E2 |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 08/11 madrugada | `claude/ajustes-vila-mobs`, "sexta entrega" | Casa do fundador com +15% em casa, refeição e banquete ([DEC-036](../06_HISTORICO_E_DECISOES.md#dec-036)) |
| 08/11 madrugada | Codex local 0.15.3 | Integra a casa (zip `E:/veio-casa-do-fundador.zip`) |
| 08/11 | Obras | Lar `veio:home` ([DEC-037](../06_HISTORICO_E_DECISOES.md#dec-037)) |
| 08/11 19:16 | Dono | Casa do fundador com o Lar dentro ([DEC-046](../06_HISTORICO_E_DECISOES.md#dec-046)) |

## F. Pendências

1. Corrigir o README do pr47 l.21 e a "sexta entrega" de `TRABALHADORES.md`, que ainda falam em +15%.
2. Corrigir o comentário `home/food.rs:9`.
3. Decidir se o bônus de comer fora deve ser por jogador ou por área ([Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan), [CNF-017](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
4. Persistir o bônus de refeição, hoje perdido ao reiniciar ([SIS-PER](SIS-PER.md)).
5. Teste em jogo: nenhum.

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main — · pr47 **Integrado** (4 de 4) |
| Confiança | Média: E2 completo e teste do ponto central; J nenhum |
| Prioridade (sugestão) | Baixa |
| Criticidade (sugestão) | Baixa para o save; média para a LAN (bônus global) |
| Responsável conhecido | Claude local (casa); Codex local (integração); agente das obras (Lar); integrador do #47 (fusão) |
| DEC | [DEC-036](../06_HISTORICO_E_DECISOES.md#dec-036), [DEC-037](../06_HISTORICO_E_DECISOES.md#dec-037), [DEC-046](../06_HISTORICO_E_DECISOES.md#dec-046) |
| Q | [Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan) |
| CNF | [CNF-006](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (README do pr47), [CNF-017](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (bônus global) |
| Referências | `frentes/CASA-FUNDADOR-INTEGRACAO.md`, `TRABALHADORES.md`; A2 LAR; A7 (ritmo invertido) |
