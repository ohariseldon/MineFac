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

## 0.34.0

O capítulo 3 passou a ter 10 etapas (impostos e conta pública; uma festa da Agenda) e `DONE_CITY` = 11.
`Progress` ganhou dois campos que não vão para o save (`city_since`, `city_hinted`: a dica do prefeito
quando o jogador fica 3 min parado). O formato do save continua a v6.

## 0.35.0

- Capítulo 3: o prefeito só dá dica quando clicado (a barra avisa depois de 20 min parado); lista ✔/✘ na
  barra (`city::checks`), "plim" quando o número da etapa sobe (`city::metric`), 3 s entre etapas.
- **Correção que vale para o capítulo 1 também:** `playsound` pelo console toca no spawn do mundo, não no
  jogador. Os sons do tutorial (inclusive `intro::sound`) agora usam `execute as <nome> at @s run playsound ... @s`.

## 0.36.0: a noite e a nova ordem (ver `frentes/HISTORIA.md`)

- Ordem depois da fábrica: **noite** (`onboarding/night.rs`, capítulo 2) → **vila operária** (`village.rs`,
  agora capítulo 3) → **a vila cresce** (`city.rs`, agora capítulo 4). Em `intro::talk`, os braços seguem essa ordem.
- **A primeira noite** (`night::overlay`) roda antes de tudo no `tick`, inclusive no prólogo: no primeiro
  anoitecer, o resto do tutorial espera o amanhecer. Para o capítulo 1 (etapas 9 a 16), nada muda além disso.
- `player_bed_enter` avisa o tutorial (`onboarding::slept`); `place_block` conta tochas, muros e portões.
- Save **v7** (6 números da noite); a v6 e anteriores continuam abrindo (quem já começou a fábrica não revive a
  primeira noite).
- **Um prefeito só:** o aldeão contratado é o "secretário de Otávio" nos textos (`build_tutorial.py`, wiki).

## 0.37.0: capítulos 5 e 6, e a noite com 6 etapas

- Capítulo 2 (noite) agora tem 6 etapas: luz, muralha, noite em paz, o preço da fábrica (medidor de
  poluição), muralha da fábrica e o conselho de defesa.
- **Motor comum** `onboarding/chapter.rs` (`Spec`): anunciar, ficha da receita, materiais, barra/lista ✔/✘,
  comemoração, abertura e fim. Capítulos novos só descrevem o `Spec` e quando cada etapa fecha.
- Capítulo 5, **a grande indústria** (`forge.rs`, 8 etapas, `/veiotutorial industria`): fases do Quadro
  (Aço, Eletrônica), Mk.II colocada, aviso de poluição (`pollution::alarm`), quartel com soldado e comida,
  Tesla com 1 tiro, acumulador carregado, kit de reparo. Uma onda no meio vira fala do prefeito.
- Capítulo 6, **a cidade** (`metro.rs`, 6 etapas, `/veiotutorial cidade`): lê a vila por `city::survey`
  (agora com emprego, serviços, PIB e obra cívica) até a fase Cidade.
- Capítulo 4 abre com `/veiotutorial crescer` (antes `cidade`).
- Save **v9** (29 números + blocos da forja); v1 a v8 continuam abrindo.
- Para o capítulo 1 (etapas 9 a 16): nada muda. As etapas 1 e 2 do capítulo 5 usam as fases do Quadro;
  se o capítulo 1 passar a ensinar essas fases, o capítulo 5 simplesmente fecha essas etapas na hora.
