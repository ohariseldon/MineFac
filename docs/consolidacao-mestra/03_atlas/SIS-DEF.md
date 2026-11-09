# SIS-DEF: Ameaças e defesa

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. Linhas em `native/api/src/` no pr47. O conteúdo do PR #30 (quartel) já está na main.

## A. Identidade

O custo de crescer. A poluição atrai ataques à noite, no espírito do Factorio, "mas sem quebrar o Minecraft" (`pollution.rs:1`). É o quarto gargalo, depois de extração, energia e fio. A resposta: muros, Tesla, quartel, lavoura que absorve poluição e módulos de eficiência.

- **Depende de:** [SIS-PRO](SIS-PRO.md) (emissão por máquina e módulos), [SIS-AGR](SIS-AGR.md) (absorção), [SIS-ENE](SIS-ENE.md) (Tesla), [SIS-TRB](SIS-TRB.md) (soldados).
- **Alimenta:** [SIS-PRO](SIS-PRO.md) (dano às máquinas), [SIS-VIL](SIS-VIL.md) (poluição local acima de 400 piora a moradia).

## B. Funcionamento

### B.1 Poluição

Fonte: `pollution.rs:1-18`, constantes `:28-47`.

1. Máquinas trabalhando emitem no chunk em que estão. Gerador 6/s; forno de ligas e Mk.II 4; extratora, triturador e geotérmico 2; demais 1 (README, E3). O módulo de eficiência corta pela metade; velocidade e produtividade aumentam ([SIS-PRO](SIS-PRO.md) B.2).
2. A cada 64 ticks, a poluição se espalha aos chunks vizinhos (4%, só acima de 20) e 2% se dissipam. Cada colheita automática absorve 3 ([SIS-AGR](SIS-AGR.md)).
3. **Aviso:** chunk acima de 400, pelo chat.
4. **Ataque:** só um dia inteiro depois do aviso, só à noite, com chunk acima de 1.200 e alerta ao anoitecer.
5. **Estado em memória:** recomeça ao reabrir o mundo; "a poluição volta em poucos minutos" ([DEC-059](../06_HISTORICO_E_DECISOES.md#dec-059)).

### B.2 Ondas

| Regra | Valor |
|---|---|
| Chegada | Pela borda da fábrica, junta, de um lado; a direção muda a cada noite; num arco a 26 blocos da máquina mais poluída (`ARRIVAL`) |
| Tamanho | 4 a 12 saqueadores (`veio:raider`, zumbi com faro de 40 blocos) |
| Arqueiros | Esqueletos a partir da 3ª noite de ataque |
| Brutos | `veio:raider_brute`, zumbi grande e lento, 40 de vida (`raider_brute.json:4`), a partir da 5ª |
| Frequência | No máximo uma onda por noite. No pr47, uma onda que não gera nenhum mob não conta (`:220,236-238`) |
| Sol | Todos queimam |

- **Alvos:** saqueadores e brutos atacam jogadores, aldeões e golens. Esqueletos e aranhas da noite também caçam aldeões, com dano real (FUN-DEF-06).
- **Máquinas:** sem ninguém por perto, saqueadores golpeiam a máquina poluidora mais próxima, 25 de dano por golpe (`BLOW`); o bruto vai primeiro às máquinas e dá 50. Com 100 de dano a máquina para, até um kit de reparo (`pollution.rs:344-349`).
- **Regressão de documento:** o README do pr47 (`:307`) diz que as ondas chegam a 64 e 18 blocos. A main (`:213`) e o código dizem 26 ([DEC-060](../06_HISTORICO_E_DECISOES.md#dec-060), [CNF-006](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

### B.3 Torre Tesla

Abate o hostil mais próximo no alcance, com energia da fábrica (`defense.rs:1-15`):
- alcance 10 blocos;
- 2.400 FE por disparo;
- recarga de 40 ticks (2 s);
- guarda 20.000 FE.

Os hostis são contados em `entity_tick_pre` e guardados por dimensão (até 4.096). O relógio global do mod (`CLOCK`, `defense.rs:21-45`) também é usado por outros sistemas.

### B.4 Quartel

Um prédio, um soldado (`workforce/barracks.rs:1-26`):

| Classe | Dano | Detalhe |
|---|---|---|
| Milícia | 7 | Chega rápido |
| Guarda | 5 | Cura 3 a cada golpe: o tanque |
| Arqueiro | 5 | Atira a 16 blocos |
| Mago | 10 | Mais a metade num segundo monstro a até 5 blocos |

- Agachar com uma esmeralda troca a classe. Com alguém contratado, a troca gasta uma esmeralda do slot de salário.
- Cada golpe custa uma tarefa de comida. Sem comida, ninguém sai.
- Patrulha a 18 blocos. O soldado fica de plantão à noite e não foge (FUN-DEF-05, pr47 `fa96ceb`).
- Os golpes tiram vida de verdade (`set_mob_health`).

## C. Experiência do jogador

1. A fábrica cresce e o chat avisa: poluição alta.
2. Um dia depois, à noite, a corneta toca e saqueadores chegam de um lado.
3. A pessoa põe Teslas, muros e um quartel com guarda e arqueiro.
4. Troca módulos de velocidade por eficiência e planta mais para absorver.

## D. Implementação

**Arquivos:** `pollution.rs`, `defense.rs`, `workforce/barracks.rs`; entidades em `content/`.

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-DEF-01 Poluição e ataques | 400 aviso; 1.200 ataque; espalha a cada 64 t; em memória | `pollution.rs` · `pollution.rs:1-47` · — | E2 |
| FUN-DEF-02 Ondas | 4–12; arqueiros na 3ª; brutos na 5ª; 26 blocos | `pollution.rs` · idem + onda vazia não conta · — | E2 |
| FUN-DEF-03 Quartel (4 classes) | 7/5+3/5 a 16/10+metade | `barracks.rs` · `barracks.rs:1-26` · #30 aberto, conteúdo na main | E2 |
| FUN-DEF-04 Bobina de Tesla | 10 blocos; 2.400 FE; 40 t; 20.000 | `defense.rs` · `defense.rs:12-15` · — | E2 |
| FUN-DEF-05 Patrulha noturna dos defensores | Plantão à noite, sem fuga | — · `barracks.rs:7` (`duty`) · `fa96ceb` | E3 + E2 |
| FUN-DEF-06 Monstros caçam aldeões | Esqueletos e aranhas com `NearestTarget` | Integrado · `pollution.rs:11-13` · — | E2 |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 19:10 (0.7) | `819ba92` | Torre Tesla |
| 07/11 19:37 (0.8) | `7644aab` | Poluição e saqueadores ([DEC-059](../06_HISTORICO_E_DECISOES.md#dec-059)) |
| 07/11 noite | Grok, PR #30 | "Noite" e quartel |
| 08/11 05:53 (0.17.0) | `0deee22` | Ondas por um lado, arqueiros, brutos, quartel funcional ([DEC-060](../06_HISTORICO_E_DECISOES.md#dec-060)) |
| pr47 | `fa96ceb` | Patrulha noturna; onda vazia não conta |

## F. Pendências

1. Corrigir o README do pr47 `:307` (26 blocos, não 64/18).
2. Persistir poluição e relógio, ou registrar como limitação aceita ([DEC-059](../06_HISTORICO_E_DECISOES.md#dec-059), [SIS-PER](SIS-PER.md)).
3. Fechar o PR #30, cujo conteúdo já está na main.
4. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-029](../08_BACKLOG_E_PRIORIDADES.md#tar-029): Persistir o que some ao reiniciar

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01–04, 06) · pr47 **Integrado** (01–06) |
| Confiança | Média (E2) |
| Prioridade (sugestão) | Baixa |
| Criticidade (sugestão) | Média: o equilíbrio de risco nunca foi jogado |
| Responsável conhecido | Claude local (0.7–0.8); Grok (#30); Claude nuvem (0.17) |
| DEC | [DEC-059](../06_HISTORICO_E_DECISOES.md#dec-059), [DEC-060](../06_HISTORICO_E_DECISOES.md#dec-060) |
| Q | — |
| CNF | [CNF-006](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (README do pr47) |
| Referências | README main §Ameaças; A2 DEF; A7 (constantes de poluição) |
