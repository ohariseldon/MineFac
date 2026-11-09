# Tutorial: capítulos 2 (vila) e 3 (cidade), sobre o começo 0.31.0

Branch `claude/trusting-ritchie-sc9ty8`, versões 0.32.0 e 0.33.0. Feita sobre a `project-thread-hckdtj`
(0.31.0, o começo como história). Para quem integrar ou estender o capítulo 1 (as etapas 9 a 16):

## Onde encaixa

- `onboarding/village.rs` (capítulo 2) e `onboarding/city.rs` (capítulo 3) são módulos à parte. Não mexem
  nas etapas da fábrica (`step`, `CRAFT`, `DO_LINES`, `ready`): o capítulo 1 pode crescer à vontade.
- Começam **depois** do fechamento da fábrica: em `intro::talk`, o braço `_ if !p.outro` (fechamento do
  capítulo 1) diz `veio.village.offer`; os cliques seguintes no prefeito vão para `village::talk` e, depois
  da vila, para `city::talk`; só então voltam as dicas do Quadro (`board_phase`). Se o capítulo 1 passar a ter
  16 etapas, basta que `p.step == DONE` e `p.outro` continuem marcando o fim dele.
- Também começam por comando: `/veiotutorial vila` e `/veiotutorial cidade` (pulam o prólogo, se preciso).
- No `tick` de `onboarding.rs`, a cidade e a vila em andamento vêm antes da fábrica e fazem `continue`
  (ocupam a barra sozinhas).

## Save

- v4 (a do começo novo) continua sendo lida. v5 acrescentou a vila (3 números + 6 blocos antes da linha do
  prefeito); v6 acrescentou a cidade (2 números). Ordem das linhas: 5 blocos, terreno, 6 blocos da vila,
  prefeito. Se o capítulo 1 precisar de campos novos, use a v7 e mantenha a leitura da v6.

## Textos

- `tools/build_onboarding.py`: `add_village_labels` e `add_city_labels` (chaves `veio.village.*` e
  `veio.city.*`). As fichas de receita saem dos JSON de receita (`recipe_line`).
- `check_project.py` confere as chaves dos dois capítulos e os nomes de todos os materiais das listas.

## Testes

- `the_village_chapter_checks_each_step_in_the_world`, `the_mayor_opens_the_village_chapter_and_celebrates_each_step`,
  `the_city_chapter_closes_each_step_from_what_the_village_registers`, `after_the_village_the_mayor_opens_the_city_chapter`.
