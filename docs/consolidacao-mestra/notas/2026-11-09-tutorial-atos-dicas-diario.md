# Começo do jogo: 4 atos, dicas do prefeito, setas e o Diário de Otávio

- **Data / frente / autor:** 2026-11-09 · tutorial · Claude, thread "Tornar o início de jogo claro e envolvente"
- **Onde:** branch `claude/project-thread-hckdtj` (Veio 0.35.0), PR #61, que também junta o capítulo 3 com 10 etapas (0.34.0 da outra sessão, eb14d54)
- **Sistemas:** tutorial (fábrica e capítulo 2 da vila) · **FUN:** sem ID novo
- **Tipo:** código · conteúdo
- **Continua** `2026-11-09-tutorial-16-etapas.md`. O capítulo 3 (cidade) não mudou: outra sessão está mexendo nele.

## Por quê
Luiz (2026-11-09): aprimorar o começo, as 16 etapas e o capítulo 2, para ficar mais intuitivo, mais fácil, mais divertido e com um world building legal.

## O que mudou
- **4 atos** nas 16 etapas (4 cada): A primeira chama, A linha que anda sozinha, O plano da vila, A vila conta com você. O começo de cada ato abre com uma linha no chat (`veio.onboarding.act.<n>.line`). O fim de cada ato tem fogos em volta do jogador e o título do ato seguinte no lugar de "Etapa concluída!". O fim das 16 tem "A fábrica está de pé!". Os fogos também saem no fim do capítulo 2.
- **Dica do prefeito:** depois de 3 min na mesma etapa (`HINT_AFTER`, transitório, não vai para o save), o prefeito dá uma vez a dica do erro mais comum daquela etapa (`veio.onboarding.hint.<etapa>`, e `veio.village.hint.<etapa>` no capítulo 2). Clicar nele também repete a dica.
- **Para onde ir:** longe do lugar da etapa (mais de 24 blocos), a barra soma a seta e a distância. O alvo é a casa (`home::dashboard`) nas etapas do Lar e do centro de controle e o terreno da oficina nas outras.
- **World building:**
  - O **Diário de Otávio**, livro dado junto com o gerador (`loot veio:tutorial/diary`, gerado em `build_tutorial.py`), conta em 5 páginas o veio de cobre e ferro que não acaba, a mina fechada do avô, o trato com a vila, o caminho dos atos e capítulos, e conselhos.
  - A fala do prefeito sobre os morros agora fala do veio.
- `check_project.py` confere as chaves novas.

## Evidência
- E1: `python tools/check_rust.py` → 372 testes passando (com os do capítulo 3). O teste novo `acts_end_with_fireworks_and_a_stuck_player_gets_one_hint_from_the_mayor` cobre os títulos dos atos, os 3 fogos, a dica só depois do tempo e uma vez, a contagem que recomeça em outra etapa e o alvo da seta. Os 3 ignorados passam com `--ignored --test-threads=1`. `python tools/check_project.py` ok.
- J: não testado em jogo. A confirmar no PC:
  - o SNBT dos fogos (`components` do 1.21);
  - o livro do diário com páginas `translate`;
  - a seta na barra junto com o objetivo.

## Toca no compilado
- Pacote `release/Veio-0.35.0.zip` no lugar do 0.34.0.
