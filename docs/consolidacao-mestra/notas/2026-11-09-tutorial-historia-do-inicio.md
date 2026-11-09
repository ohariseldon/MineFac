# Início de jogo como história: vila, prefeito, recursos, máquina, terreno

- **Data / frente / autor:** 2026-11-09 · tutorial · Claude, thread "Tornar o início de jogo claro e envolvente"
- **Onde:** branch `claude/project-thread-hckdtj` (Veio 0.31.0), PR #59 (base #54); traz também o Realismo 3.2.3 do #51
- **Sistemas:** tutorial, spawn na vila, Quadro da fábrica · **FUN:** sem ID novo
- **Tipo:** código · conteúdo
- **Substitui** o roteiro de `2026-11-08-tutorial-inicio-de-jogo.md` (casa primeiro, prefeito na varanda).

## Por quê
Luiz (2026-11-09) achou o começo ainda confuso e descreveu o loop: nasce perto (não colado) de uma vila; uma seta diz "vá para a vila"; ao chegar, "fale com o prefeito"; o prefeito conta a história e ensina a pegar recursos; entregar os recursos libera a máquina; depois planificar a região; depois pôr as máquinas e usar esteiras; e daí emendar nos gargalos, para o jogo cobrar expansão e eficiência sem dizer.

## O que mudou
- **Spawn** (`workforce/spawn.rs::approach`): quem chega pela primeira vez vai para um chão seco a 44 blocos do centro da vila, do lado de onde estava (sem rumo: o sul, longe da casa). Quem já está entre 28 e 96 blocos fica onde está. O ponto de renascer continua no centro.
- **Seta** (`onboarding/intro.rs`): a barra aponta o alvo com ↑ ↗ → ... relativa ao olhar. A janela guarda posição e yaw (`client_view`, no tick da janela) e o servidor usa quando a posição bate (mundo de um jogador). Sem isso (convidado da LAN), a barra diz o rumo por extenso.
- **Contornos na tela** (gancho `visuals`, `NativeBlockRim`): dourado no prefeito, verde na borda do terreno.
- **Etapas do prólogo:** vila (chegar a 20 blocos do centro; sem vila em 3 min, segue) > prefeito (aldeão `veio:mayor` invocado na praça, no ponto mais baixo e seco a 3 blocos do centro) > recursos (16 pedregulhos, 2 lingotes de ferro, 2 de cobre; o prefeito explica como conseguir; a barra marca ✔/✘; a entrega é tudo ou nada) > o prefeito dá o **gerador** e libera as receitas de gerador, prensa e cabo > terreno 11 x 11 > fala de como dispor gerador, prensa, cabos, extratora e esteiras > 8 etapas da fábrica (a 1ª agora é pôr o gerador recebido no terreno).
- **Gargalos depois do tutorial:** a última conversa libera a receita do Quadro da fábrica; daí em diante o prefeito diz a fase do Quadro mais adiantado (`milestones::census::board_phase`), o que entregar e o próximo gargalo (`veio.board.hint.*`, que já existia).
- O id do prefeito é lembrado no tick da entidade (`mayor_seen`, que também tira cópias a mais) e o lugar dele vai no save.
- **Save** `veio-onboarding-4` (+ lugar do prefeito). A v3 é convertida (casa > vila, terreno e pronto mantidos).

## Evidência
- E1: `python tools/check_rust.py` → 365 testes passando (novos/refeitos: pedido e entrega de recursos com devolução, gerador e receitas, terreno, fim e Quadro; seta girando com o olhar; spawn perto da vila, LAN e vila gerada). Os 3 ignorados do tutorial passam com `--ignored --test-threads=1`. `python tools/check_project.py` ok.
- J: não testado em jogo. A confirmar no PC: setas Unicode na barra de ação, `NativeBlockRim` visível, o yaw lido pela janela no mundo de um jogador, `take_items`/`give_items` no clique em entidade, e o tipo `veio:mayor`.

## Toca no compilado
- O prefeito do tutorial continua sendo um personagem, distinto do prefeito contratado na prefeitura (decisão pendente com o Luiz).
- Máquinas multibloco (pedido do mesmo dia) ficam com outra thread.
- Pacote `release/Veio-0.31.0.zip` no lugar do 0.30.0.
