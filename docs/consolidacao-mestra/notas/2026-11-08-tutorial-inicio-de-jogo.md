# Início de jogo: casa, prefeito, terreno e as 8 etapas da fábrica

- **Data / frente / autor:** 2026-11-08 · tutorial · Claude, thread "Tornar o início de jogo claro e envolvente"
- **Onde:** branch `claude/project-thread-hckdtj` (Veio 0.30.0), PR sobre o #54 (`claude/trusting-ritchie-sc9ty8`); a branch também junta o topo do #47 (`l94frq`), do #51 (`mrtok9`) e o #56 (`szmjbg`)
- **Sistemas:** tutorial e casa do fundador (cartões SIS no 03_atlas do #55) · **FUN:** prólogo e prefeito do tutorial são novos, sem ID
- **Tipo:** código · conteúdo

## Por quê
Luiz (2026-11-08): "tá muito confuso hoje qual que é o objetivo do usuário, o que ele deveria fazer". Ele quer testar o loop: começar, se estabelecer em casa, falar com o prefeito, saber o que fazer, quando e onde; o prefeito pede para planificar uma área e pôr as máquinas. O tutorial do #54 (0.28.0/0.28.1) já ensinava a fábrica em 8 etapas, mas começava direto em "coloque um gerador", sem objetivo nem lugar.

## O que mudou
- **Prólogo** (`native/api/src/onboarding/intro.rs`), antes das 8 etapas:
  1. Chegada: título na tela e o objetivo do jogo no chat; a barra de ação leva até a casa do fundador (distância e rumo, 8 direções). Sem casa em 3 min, segue sem ela.
  2. Em casa: apresenta a cama (renascer), o painel (centro de controle) e o Lar; o **Prefeito Otávio** (`veio:mayor`, tipo próprio com molde de aldeão, parado e invulnerável) aparece na varanda, 2 blocos a oeste da porta.
  3. Clique no prefeito: 4 falas (papel da fábrica e da vila) e a missão: **nivelar o terreno da oficina**, 11 x 11, escolhido pelos mapas de altura a 26/34/42 blocos do centro da vila, o mais plano, sem água, fora da casa. Nível = altura do meio.
  4. Terreno: a cada 2 s (só com o jogador a até 48 blocos) conta colunas fora do nível, a cavar e a aterrar; faíscas verdes na borda, chamas onde cavar, faíscas azuis onde aterrar. Pronto com até 4 colunas fora (folga para um tronco ou uma flor alta).
- **Fábrica:** as 8 etapas do #54 seguem iguais; a etapa 1 pede o gerador "no terreno da oficina". O fim do tutorial manda voltar ao prefeito, que fecha com título, som, XP (uma vez) e os próximos passos (quadro da fábrica, contratar com esmeralda, crescer a vila, wiki J).
- **Prefeito como "o que eu faço agora?":** a qualquer momento, clicar nele repete a etapa atual (título, porquê e passos).
- **Save** `veio-onboarding-3` (+ prólogo, última conversa e terreno). Saves v1/v2 de quem já tinha começado a fábrica pulam o prólogo; quem já tinha terminado não precisa voltar ao prefeito.
- Integração: `interact_entity` ficou um só (prefeito, visitante da agenda e escolha do aldeão do #56); a cópia duplicada do gancho no substituto saiu. Duas frases do #56 que estavam só no JSON (`veio.worker.hired`, `veio.worker.none`) foram para `tools/build_tutorial.py`, senão o gerador as desfazia.

## Evidência
- E1: `python tools/check_rust.py` → 364 testes passando (5 novos do prólogo: rumos, contagem cavar/aterrar, escolha do terreno, saves antigos e v3, fluxo prefeito > terreno > fábrica > fim); os 3 ignorados do tutorial passam com `--ignored --test-threads=1`. `python tools/check_project.py` ok (só os avisos antigos do Guia da Vila).
- J: não testado em jogo. A confirmar no PC: o tipo `veio:mayor` com molde `minecraft:villager` (o visitante usa `wandering_trader`), o seletor `@e[type=veio:mayor]` e o significado exato das alturas de `surface_heights` (o nível é medido pelo próprio mapa, então só a coerência importa).

## Toca no compilado
- O prefeito do tutorial é um personagem, não o "prefeito" contratado na prefeitura (bloco). Decidir com o Luiz se um vira o outro.
- Pacote `release/Veio-0.30.0.zip` no lugar do 0.29.0.
