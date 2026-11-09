# Capítulo da vila com festas marcadas, prefeito vestido e aldeões que só comentam

- **Data / frente / autor:** 2026-11-09 · tutorial · Claude, thread "Tornar o início de jogo claro e envolvente"
- **Onde:** branch `claude/project-thread-hckdtj` (Veio 0.36.0), PR #61
- **Sistemas:** tutorial (capítulo 2 da vila), prefeito, aldeões · **FUN:** sem ID novo
- **Tipo:** código · conteúdo
- **Continua** `2026-11-09-tutorial-atos-dicas-diario.md`. O capítulo 3 (cidade) não mudou.

## Por quê
Luiz (2026-11-09): o capítulo 2 com eventos como no Stardew Valley, para dar urgência ("a gente vai ter que fazer isso por causa do evento"); o prefeito se veste diferente e reage; os outros aldeões são "minions" que falam pouco, e o prefeito é quem se comunica de verdade.

## O que mudou
- **Festas** (`onboarding/festival.rs`), contadas em dias do jogo (`day_time / 24000`):
  - Ao abrir o capítulo, o prefeito marca a **Festa da Colheita** para 3 dias depois, com título, sino e 2 falas. Ela pede `village >= 5` (posto, contratar, marco, primeira colheita). Recompensa: 16 pães e 60 de experiência.
  - Depois dela vem a **Visita do Governador**, 4 dias depois, que pede o capítulo inteiro. Recompensa: 1 bolo e 120 de experiência.
  - Se a vila não estiver pronta no dia, a festa sai fraca: o prefeito fica bravo e remarca para 2 dias depois. Não há castigo.
  - A barra do capítulo soma "Festa ... em N dia(s)", e clicar no prefeito lembra da data.
  - Economia fechada: as recompensas são só itens e experiência, sem esmeralda.
- **Save v8:** grava `festival` e `festival_day`. Saves v1 a v7 abrem com a festa por marcar.
- **Prefeito** (`intro.rs`):
  - A roupa é aplicada uma vez por mundo (`dress`), também ao prefeito de um save antigo: VillagerData taiga (casaco de pele), cartógrafo nível 5 (insígnia de mestre) e nome dourado em negrito. Ele também ganha um brilho `wax_on` em volta.
  - Ele nasce já como taiga.
  - Uma textura própria não dá por dados no Horizon, porque o template de aldeão usa a textura do vanilla.
  - **Reações** (`react`): acena quando você chega, faz corações e "celebrate" quando entrega o gerador, quando a fábrica fica pronta e quando uma festa dá certo, e mostra a nuvem brava com "no" quando falta material ou a festa sai fraca.
- **Aldeões comuns** (`onboarding/barks.rs`):
  - Com o jogador a até 6 blocos, um aldeão solta um resmungo curto, em cinza itálico, sobre o assunto do momento: prefeito, recursos, terreno, festa (na véspera e no dia), fábrica, vila ou livre.
  - São 4 falas por assunto, no máximo 1 por jogador a cada 40 s.
  - O gancho fica no tick do aldeão em `lib.rs`, amostrado a 1/97.
- Wiki (artigo da vila) e README falam das festas e de quem fala.

## Evidência
- E1: `python tools/check_rust.py` → 374 testes passando.
  - Testes novos: `village_festivals_set_a_date_reward_a_ready_village_and_move_a_weak_one` (data, véspera, festa fraca remarcada, festa boa com pão e fogos, Governador com bolo, save) e `common_villagers_only_comment_and_the_mayor_reacts_and_dresses_apart`.
  - Os 3 ignorados passam com `--ignored --test-threads=1`. `python tools/check_project.py` ok.
- J: não testado em jogo. A confirmar no PC:
  - se o `data merge` de VillagerData pega na entidade `veio:mayor`;
  - o `§7§o` nas falas dos aldeões;
  - se dormir pula os dias como esperado (a festa é checada a cada segundo).

## Toca no compilado
- Pacote `release/Veio-0.36.0.zip` no lugar do 0.35.0.
