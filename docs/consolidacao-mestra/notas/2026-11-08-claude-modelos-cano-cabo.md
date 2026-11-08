# Cano e cabo deixam de ser "cruz 3D" e passam a ligar nos vizinhos

- **Data / frente / autor:** 2026-11-08 · claude · thread "Corrigir modelos 3D em formato de cruz" (pedido do dono, 23:47)
- **Onde:** branch `claude/project-thread-18ys28`, PR sobre `claude/project-thread-l94frq` (#47)
- **Sistemas:** água encanada, rede elétrica, modelos · **Tipo:** visual · correção

## O que mudou
- O dono viu o encanamento como "várias cruzes, uma cruz 3D". Causa: `pipe` tinha um modelo fixo com braços nas seis faces (e `cable` um "+" no chão), desenhado igual estivesse ligado ou não. Máquinas não tinham esse problema (formas de `tools/machine_forms.py`).
- `tools/build_content.py`: cano e cabo agora são `multipart`, como cerca e redstone do vanilla. Uma propriedade booleana por face (`down/up/north/south/west/east` no cano; `up/north/south/west/east` no cabo), um braço por face ligada, uma junta de cobre só em curva, tê e ponta; trecho reto vira tubo contínuo, com um anel de flange na divisa entre blocos. A caixa de seleção e colisão segue a forma (64 e 32 estados). O item é o trecho reto.
- `native/api/src/joints.rs` (novo): relê os vizinhos no primeiro tick e depois a cada 20 ticks (fase pela posição) e grava só as faces que mudaram. Cano liga em tudo que guarda fluido (`water::holds`); cabo liga em cabo, bateria, postes, geradores e consumidores. Chamado de `water::pipe` e de `tick_power` (só para o cabo).
- Teste novo `pipes_and_cables_draw_only_the_arms_that_touch_a_neighbor`; `idle_cables_skip_neighbor_lookups` ganhou `block()` no mundo de teste (ler blocos não move energia).

## Evidência
`check_rust` (326 testes ok), `check_project` (760 JSON ok, só os avisos antigos do guia), `check_performance` (checagens idênticas). O formato (`state_properties` booleanas, `multipart` com `AND`/`OR`, `shapes` por propriedades) foi conferido no código do Horizon 66f3981. Prévia antes/depois em `/mnt/project-files/visual/modelos/cano-cabo-antes-depois.png`. Falta ver no jogo.

## Toca no compilado
- Atlas de água/energia: cano e cabo têm estado de ligação desenhado; nenhum efeito na vazão.
- Mundos antigos: o cano/cabo já colocado aparece só com a junta até o primeiro tick, depois se liga.
