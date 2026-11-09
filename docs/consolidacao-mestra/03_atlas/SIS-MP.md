# SIS-MP: Multijogador em LAN

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `pr47@80980db`. Na main, a chegada dos convidados à vila não existe. Linhas em `native/api/src/` no pr47.

## A. Identidade

Jogar com amigos na mesma rede local, no mundo do anfitrião. Ninguém decidiu o que é de cada jogador e o que é do mundo. Cada sistema escolheu sozinho ([Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan)).

- **Depende de:** [SIS-TRB](SIS-TRB.md) (spawn), [SIS-TUT](SIS-TUT.md), [SIS-PES](SIS-PES.md), [SIS-LAR](SIS-LAR.md), [SIS-ECO](SIS-ECO.md).

## B. Funcionamento

### B.1 Chegada

O convidado novo vai para a vila do anfitrião. Quem já tem a tag `veio_village` não é movido de novo (`workforce/spawn.rs:151-152`; Grok #25, [DEC-061](../06_HISTORICO_E_DECISOES.md#dec-061)).

### B.2 O que é de quem (estado atual do código)

| Estado | Por jogador | Do mundo ou da dimensão | Fonte |
|---|---|---|---|
| Progresso do tutorial | ✔ `tutorial/<UUID>.txt` | | `onboarding.rs` |
| Casa do fundador: camas | ✔ por jogador | | `home/mod.rs:306` |
| Dono do Lar | ✔ quem estava perto ao ativar | | `workforce/home.rs:3-4` |
| Resultado da copiadora | ✔ quem clicou | | CHECKPOINT (E3) |
| Bônus de comer fora | | ✔ global (atômicos); vale o maior | `home/food.rs:22-29` |
| Pesquisa do laboratório | | ✔ global; concedida a `@a` (quem está online) | `research.rs:166` |
| Grande projeto | | ✔ concedido a `@a` | `lib.rs:1101` |
| Prancheta (cópia) | | ✔ uma por mundo aberto | `logistics_controls.rs:66-67` |
| Pedido do dia do Quadro | | ✔ por dimensão | `census::claim_order` |
| Cofre e caixa da companhia | | ✔ um livro Company por dimensão | `ledger.rs` |
| Bolsa, contratos, reputação | | ✔ por dimensão | `trade.rs` |

Consequência: em LAN, um jogador que come um pão acelera as máquinas do outro. Quem entra depois de uma pesquisa concluída não recebe o avanço ([CNF-017](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

## C. Experiência do jogador

1. O anfitrião abre o mundo em LAN. O amigo entra e aparece na vila.
2. Cada um tem o seu tutorial.
3. Os dois compartilham dinheiro, pesquisa e bônus sem regra explícita.

Sem teste com dois jogadores.

## D. Implementação

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-MP-01 Convidado LAN vai à vila do anfitrião | Tag `veio_village` | — · `workforce/spawn.rs:151-152` · #25 (contido) | E2 |
| FUN-MP-02 Estado por jogador | Ver B.2 | — · tutorial por UUID; demais globais · — | E2 |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 | Grok, PR #25 | Convidados vão à vila do anfitrião ([DEC-061](../06_HISTORICO_E_DECISOES.md#dec-061)) |
| 08/11 madrugada | Codex local | Tutorial por UUID |

## F. Pendências

1. Decidir o que é de cada jogador e o que é do mundo ([Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan)). Recomendação de agente: tecnologia e cofre compartilhados; bônus local.
2. Reconceder avanços de pesquisa no login, se a tecnologia for compartilhada ([SIS-PES](SIS-PES.md)).
3. Teste com dois jogadores: nenhum.

### Tarefas no backlog

- [TAR-041](../08_BACKLOG_E_PRIORIDADES.md#tar-041): O que é de cada jogador em LAN

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main — · pr47 **Integrado** (01, 02) |
| Confiança | Baixa: E2 sem nenhum teste com dois jogadores |
| Prioridade (sugestão) | Baixa até a [Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan) |
| Criticidade (sugestão) | Média: estado global pode surpreender em LAN |
| Responsável conhecido | Grok (#25); cada frente para o seu estado |
| DEC | [DEC-061](../06_HISTORICO_E_DECISOES.md#dec-061) |
| Q | [Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan) |
| CNF | [CNF-017](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (estado global em LAN) |
| Referências | README do pr47 §LAN; A2 MP |
