# Tutorial em 16 etapas: a fábrica e os sistemas da vila

- **Data / frente / autor:** 2026-11-09 · tutorial · Claude, thread "Tornar o início de jogo claro e envolvente"
- **Onde:** branch `claude/project-thread-hckdtj` (Veio 0.32.0), PR #59 (base #54)
- **Sistemas:** tutorial, Quadro da fábrica, postes, trabalhadores, Lar, centro de controle · **FUN:** sem ID novo
- **Tipo:** código · conteúdo
- **Continua** `2026-11-09-tutorial-historia-do-inicio.md` (prólogo da vila e do prefeito, sem mudança).

## Por quê
Luiz (2026-11-09) gostou do começo e das 8 etapas e pediu mais 8 que ensinem os sistemas básicos do jogo, para que em 16 passos o jogador entenda como o jogo funciona e fique cativado; os tutoriais de sistemas mais avançados virão depois, conforme forem desbloqueados. Também pediu para aprimorar as 8 primeiras.

## O que mudou
- **As 8 primeiras:** mesmos exercícios; "Etapa X de 16" e "X/16" na barra; uma fala do prefeito em cada etapa (`veio.onboarding.story.<etapa>`, no anúncio e quando se clica nele) dizendo por que aquilo importa para a vila.
- **As 8 novas** (`onboarding.rs`, `BOARD_STEP`..`CENTER_STEP`), cada uma conferida no mundo, nunca no inventário:
  9. Quadro da fábrica colocado (receita dada na etapa, ficha e lista de materiais).
  10. Primeira fase do Quadro: 16 chapas entregues (lê `Phase`/`Delivered` do Quadro colocado; longe, vale `census::board_phase`). A barra conta x/16.
  11. Dois postes colocados e o último com energia (`energy` > 0).
  12. Posto do carregador colocado (qualquer posto de trabalho vale). A ficha explica a chapa de ferro (prensa) e a esmeralda (comércio vanilla com os aldeões, nada criado pelo Veio).
  13. Aldeão contratado no posto (`workforce::employers`; um posto de trabalho com aldeão a até 48 blocos também vale; prefeitura e centro não contam).
  14. Comida no posto (espaço de comida ou ração ≥ `TASK_COST`).
  15. Refeição no Lar (gancho em `workforce/home.rs::meal`, só na etapa).
  16. Abrir o centro de controle da casa (clique, não colocar).
- **Fim:** a conversa final do prefeito foi reescrita (o que você aprendeu, gargalos, 2ª fase do Quadro com 32 engrenagens, pedido do dia em esmeraldas para contratar mais).
- **Save** `veio-onboarding-5`: + postes, eventos (Lar, centro) e as posições do Quadro, do posto e do último poste. Saves v1 a v4 abrem; quem tinha terminado as 8 antigas segue para a etapa 9 e volta ao prefeito no fim das 16.
- `crafted` passou a 32 bits (as etapas novas usam bits acima de 16).
- Correção: a lista `goals` de `build_onboarding.py` nunca era escrita no JSON (a barra mostrava o texto antigo "Coloque um gerador na oficina"); agora é. `check_project.py` confere as chaves das 16 etapas e os nomes do Quadro, poste e posto.

## Evidência
- E1: `python tools/check_rust.py` → 366 testes passando (novo: `the_village_systems_steps_follow_the_board_the_poles_the_worker_and_the_house`; saves antigos e v1 ajustados). Os 3 ignorados do tutorial passam com `--ignored --test-threads=1`. `python tools/check_project.py` ok.
- J: não testado em jogo. A confirmar no PC: a energia do poste (NBT `energy`) quando ele só repassa, o clique no centro de controle chegar ao `right_click_block` antes de abrir o painel, e a contratação aparecer em `employers` no tick.

## Toca no compilado
- O prefeito do tutorial continua sendo um personagem, distinto do prefeito contratado na prefeitura (decisão pendente com o Luiz).
- Pacote `release/Veio-0.32.0.zip` no lugar do 0.31.0.
