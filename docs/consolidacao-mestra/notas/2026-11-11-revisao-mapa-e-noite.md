# Revisão: moradores dormem, Saqueadores do acampamento, sem patrulhas, chão limpo; mapa Costa oeste fechado

- **Data / frente / autor:** 2026-11-11 · tutorial / vila / mapa · Claude, thread "trusting-ritchie"
- **Onde:** branch `claude/trusting-ritchie-sc9ty8` (Veio 0.39.0)
- **Tipo:** código · conteúdo · documentação

## Código (vale para quem juntar)
- **`onboarding/rest.rs` (novo):** ao anoitecer, o prefeito (`veio:mayor`) e os vizinhos com nome
  (`veio:smith`, `veio:farmer`, `veio:potter`, `veio:grocer`, `veio:cowherd`, `veio:collier`) vão para a
  cama mais perto de onde trabalham (busca única por mundo, 28 blocos, uma cama por pessoa) e voltam ao
  amanhecer. Usa `workforce::night`; chamado no `tick` do tutorial a cada 5 s.
  - Para isso, `intro.rs` ganhou `plaza_spot` e `mayor_id`, e `favors.rs` e `errands.rs` ganharam
    `residents`.
- **`pollution.rs`:** a onda nasce do lado do Acampamento dos Saqueadores (`camp_angle`, fixo por vila,
  ±0,6 rad por noite). O aviso `veio.raid.warning` diz isso.
- **`lib.rs`:** `gamerule doPatrolSpawning false` no login. As patrulhas de pillagers do Minecraft
  (também "Saqueadores" em português) confundiam com as ondas da história.
- **`workforce/spawn.rs`:** `clear_trees` também tira `leaf_litter`, arbustos e trepadeiras
  (`FOREST_FLOOR`; numa versão sem esses blocos, o comando só falha). Provável causa da "serrapilheira"
  espalhada vista no último teste.
- **Testes:** 384. Novo: `named_villagers_find_their_own_bed_near_where_they_work`.
- **Falta testar no jogo:**
  - o `tp` dos moradores para a cama, se eles ficam em pé em cima da cama;
  - o `gamerule`;
  - o `fill` da serrapilheira na versão do Horizon.

## Mapa (planta, ainda não no gerador)
Ver `docs/mapa/costa-oeste/README.md`:
- terra até a borda ao norte, a leste e ao sul, e o Pantanal do Leste;
- a revisão física dos rios;
- 59 estruturas com o bioma conferido, em `plano.json`;
- o plano de crescimento da vila em 6 estágios, com a checagem de conflitos.
