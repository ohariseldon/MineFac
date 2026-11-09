# História dos capítulos 1, 5 e 6: o prefeito conta, os moradores comentam

- **Data / frente / autor:** 2026-11-09 · tutorial · Claude, thread "Tornar o início de jogo claro e envolvente"
- **Onde:** branch `claude/project-thread-hckdtj` (Veio 0.38.0), PR #61
- **Sistemas:** tutorial (capítulos 1, 5 e 6), história (`frentes/HISTORIA.md`) · **FUN:** sem ID novo
- **Tipo:** conteúdo · código pequeno
- **Continua** `2026-11-09-tutorial-favor-da-prensa.md`.

## Por quê
Luiz (2026-11-09): por enquanto, só a história, em especial a do capítulo 1 e a dos capítulos 5 e 6 novos, mais refinada e polida. A meta é ter todos os sistemas funcionais até o capítulo 6.

## O que mudou
- **Junta a base 1fc9cc2 e c5b54a6** (outra sessão): a noite em 6 etapas e os capítulos 5 (`forge.rs`) e 6 (`metro.rs`), sobre o motor `chapter.rs`.
  - **Save v11:** a v10 desta linha mais os 4 números dos capítulos 5 e 6 e os 4 blocos da grande indústria.
  - Os saves "v7", "v8" e "v9" da base são reconhecidos pela contagem de números e abrem.
- **Motor dos capítulos** (`chapter.rs`):
  - Cada etapa diz a história do prefeito (`veio.<cap>.story.<n>`) antes do porquê técnico.
  - Quando `Spec::voices` dá um nome, vem também o comentário de um morador (`veio.<cap>.voice.<n>`).
  - Cada etapa concluída faz o prefeito reagir. O fim de capítulo tem fogos e o prefeito comemora.
- **Capítulo 5** (grande indústria) e **capítulo 6** (a cidade):
  - Têm 8 e 6 histórias e 12 comentários: Seu Bento, Mestre Lumen, Dona Cida, Seu Anselmo, a pequena Aurora, Seu Benedito e Dona Lia.
  - A abertura e o fim foram reescritos com os fios da história: o trato, a mina fechada do avô (que a Mk.II reabre), as casas vazias do começo e o trato cumprido no fim do capítulo 6.
- **Capítulo 1:**
  - Depois do favor, o Seu Bento comenta as chapas e as engrenagens, e a Dona Cida comenta a encomenda e a refeição no Lar (`veio.onboarding.voice.<n>`).
  - Correção: o anúncio da etapa 2 liberava a receita da prensa antes do favor. Agora ela só vem com o Bento.
- `frentes/HISTORIA.md`:
  - personagens Bento e Cida;
  - o capítulo 1 com 16 etapas e o favor;
  - uma seção "Como a história aparece nas etapas".

## Evidência
- E1: `python tools/check_rust.py` → 384 testes passando. Testes novos:
  - `chapters_tell_a_story_per_step_with_a_villager_and_end_with_fireworks`;
  - `the_press_recipe_waits_for_the_favor_and_the_neighbours_come_back_in_the_factory`;
  - a leitura do save "v9" da base.
- Os 3 ignorados passam com `--ignored --test-threads=1`. `python tools/check_project.py` ok.
- J: não testado em jogo.

## Toca no compilado
- Pacote `release/Veio-0.38.0.zip` no lugar do 0.37.0.
