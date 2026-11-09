# Tutorial da fábrica em 16 etapas, junto com o capítulo 2 (vila)

- **Data / frente / autor:** 2026-11-09 · tutorial · Claude, thread "Tornar o início de jogo claro e envolvente"
- **Onde:** branch `claude/project-thread-hckdtj` (Veio 0.33.0), sobre `claude/trusting-ritchie-sc9ty8` (940bbb5, 0.32.0 com o capítulo 2 da vila e o #59 já juntado)
- **Sistemas:** tutorial, Quadro da fábrica, divisor, postes, Lar, centro de controle · **FUN:** sem ID novo
- **Tipo:** código · conteúdo
- **Continua** `2026-11-09-tutorial-historia-do-inicio.md` (prólogo da vila e do prefeito, sem mudança) e o capítulo 2 da outra sessão (`onboarding/village.rs`, sem mudança de etapas).

## Por quê
Luiz (2026-11-09) gostou do começo e das 8 etapas e pediu mais 8 que ensinem os sistemas básicos, para que em 16 passos o jogador entenda o jogo e fique cativado; também pediu para aprimorar as 8 primeiras. Na mesma hora outra sessão fez o capítulo 2 (a vila operária: posto, contratar, zona, colheita, carregador, capataz, casa, prefeitura) e já vai fazer o capítulo 3. Luiz pediu para juntar sem sobreposição, preferindo esta versão onde houver escolha.

## O que mudou
- **Sem repetir o capítulo 2:** as 8 etapas novas ficam na fábrica e na casa; contratar, postos e comida dos trabalhadores ficam só no capítulo 2 (a primeira versão desta frente tinha posto, contratar e comida, e saiu).
- **As 8 primeiras:** mesmos exercícios; "Etapa X de 16" e "X/16"; uma fala do prefeito em cada etapa (`veio.onboarding.story.<etapa>`, no anúncio e ao clicar nele).
- **As 8 novas** (`onboarding.rs`, `BOARD_STEP`..`CENTER_STEP`), conferidas no mundo, nunca no inventário:
  9. Quadro da fábrica colocado. 10. Primeira fase do Quadro (16 chapas; lê `Phase`/`Delivered`, a barra conta x/16).
  11. Outra extratora colocada na etapa (o gargalo é o minério). 12. Um divisor colocado na etapa.
  13. Dois postes e o último com energia. 14. Pedido do dia pago (`OrderPaid` do Quadro; a barra conta o entregue).
  15. Refeição no Lar (gancho em `workforce/home.rs::meal`). 16. Abrir o centro de controle da casa.
- **Fim:** a conversa final do prefeito fala dos gargalos e das esmeraldas do pedido e emenda no convite do capítulo 2 (`veio.village.offer`, que já existia).
- **Save** `veio-onboarding-6` = v5 do capítulo 2 + extratoras novas, eventos (divisor, Lar, centro), Quadro e último poste. Até a v5 a fábrica acabava na etapa 8: quem já abriu o capítulo da vila continua nele (fábrica pronta); os outros seguem para a etapa 9.
- `crafted` passou a 32 bits. Correção: a lista `goals` de `build_onboarding.py` nunca era escrita no JSON (a barra mostrava "Coloque um gerador na oficina"); agora é. `check_project.py` confere as chaves das 16 etapas. Wiki "Primeiros passos" com as 16 etapas.

## Evidência
- E1: `python tools/check_rust.py` → 368 testes passando (novo: `the_systems_steps_follow_the_board_the_bottlenecks_the_poles_the_order_and_the_house`; saves v1 a v5 com e sem capítulo da vila). Os 3 ignorados do tutorial passam com `--ignored --test-threads=1`. `python tools/check_project.py` ok.
- J: não testado em jogo. A confirmar no PC: a energia do poste (NBT `energy`) quando ele só repassa, o clique no centro de controle chegar ao `right_click_block`, e o pedido do dia pagar com o Quadro do jogador.

## Toca no compilado
- O prefeito do tutorial continua sendo um personagem, distinto do prefeito contratado na prefeitura (decisão pendente com o Luiz).
- O capítulo 3 é da outra sessão: ele começa depois do capítulo 2 e não mexe na contagem 1..16 da fábrica.
- Pacote `release/Veio-0.33.0.zip` no lugar do 0.32.0.
