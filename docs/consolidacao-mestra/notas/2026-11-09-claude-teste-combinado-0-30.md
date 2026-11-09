# Branch de teste combinada com a 0.30.0 e teste no PC do dono

- **Data / frente / autor:** 2026-11-09 · claude · thread "Testar a build combinada no PC"
- **Onde:** branch `claude/teste-combinado` (commit desta nota), PC do dono (DESKTOP-2S5A1VT)
- **Sistemas:** integração de PRs, launcher, visual · **FUN:** nova, sem ID
- **Tipo:** teste · código (merges)

## O que mudou
- `claude/teste-combinado` junta #47, #51 (Realismo 3.2.3), #52, #53, #55 (docs), #56, #57, #58 (cano/cabo) e #59 (prólogo do tutorial, Veio 0.30.0). Os merges de #51, #55 e #58 entraram sem conflito, e os geradores rodaram sem diferença.
- No PC do dono, a 0.30.0 compilou sem erro (11 min) em `E:\Veio-combinado-030`, com o Realismo 3.2.3 e as texturas PBR JG 128x só de cenário (minérios, plantações e folhas ficam vanilla). Esse pacote ainda não tem o #58, que entrou na branch depois.
- A Wiki do Veio (#54, 0.29.0: tecla J no jogo e `docs/wiki/index.html`) já estava na branch por baixo do #59. Ela é gerada dos mesmos dados do jogo (`tools/build_wiki.py`). Rodada de novo depois dos merges, não mudou nada, então a wiki já cobre as telas do #57, os aldeões do #56 e o cano/cabo do #58 conforme os dados.
- Antes da 0.30.0, os mundos foram copiados para `E:\Veio-backup-mundos-20261108-211330`.

## Evidência
- E1: `python3 tools/check_rust.py` → 374 passam, 0 falham.
- J: a 0.30.0 chegou à tela de título na segunda abertura. Na primeira, a janela sumiu durante o carregamento, sem log (provavelmente fechada pelo dono). O prólogo ainda não foi testado em jogo.

## Toca no compilado
- Pendente: teste em jogo do prefeito (`veio:mayor`), do terreno 11x11, dos aldeões (#56) e das telas (#57). Se a janela voltar a sumir no carregamento, suspeitar da VRAM com `textureBudgetMiB` 1024.
- O `launcher.exe` do repositório é anterior ao código do launcher atual. O exe recompilado existe só no PC do dono.

## Adendo: 0.31.0
- A branch recebeu o #59 na 0.31.0 (`2c9c9a2`). O jogador nasce a uns 44 blocos da vila, uma seta na barra de ação aponta "Vá para a vila" e depois "Fale com o prefeito", e o prefeito fica na praça e pede recursos antes de entregar o gerador.
- Houve um conflito em `workforce/spawn.rs`: ficou a nova assinatura de `settle` (com a posição do jogador), e as ruínas do #53 continuam aparecendo na primeira chegada. As línguas foram juntadas chave a chave, sem choque.
- E1: `python3 tools/check_rust.py` → 375 passam. Os geradores rodaram sem diferença.

## Adendo: máquinas multibloco (#60)
- A branch recebeu o #60 (`a3d5d0c`), em que cada máquina monta sua estrutura grande. Houve um conflito em `lib.rs` (`visuals`): ficaram as duas chamadas, `onboarding::visuals` (contornos do tutorial) e `frames::visuals` (contorno vermelho de estrutura recusada), porque as duas só acrescentam contornos. As línguas foram juntadas chave a chave, sem choque.
- E1: `python3 tools/check_rust.py` → 384 passam. Os geradores rodaram sem diferença.
- Testar em jogo: se a estrutura do gerador cabe no terreno 11x11 da etapa 1 do tutorial.

## Adendo: 0.33.0 (#61) e polimento das máquinas (#62)
- A branch recebeu o #61 (`4a20b21`, Veio 0.33.0): o tutorial da fábrica passa a ter 16 etapas e é seguido pelo capítulo 2 da vila (prefeito Otávio), com o save `veio-onboarding-6`, que lê os saves antigos. Houve conflito só nas línguas, juntadas chave a chave e depois regeneradas pelo `build_tutorial.py`, sem choque.
- A branch recebeu também o #62 (`88f9aea`): cada máquina grande vira um modelo único cortado em blocos. Entrou sem conflito.
- E1: `python3 tools/check_rust.py` → 387 passam. Os geradores rodaram sem diferença.
- Ainda não testado em jogo. O teste no PC deve usar um mundo novo.

## Adendo: 0.34.0 (#61 com o capítulo 3)
- O #61 subiu para a 0.34.0 (`17f229f`): entrou o capítulo 3 do tutorial, a cidade, e o save passou a `veio-onboarding-7`, que lê todos os saves antigos. Entrou sem conflito.
- E1: `python3 tools/check_rust.py` → 389 passam. Os geradores rodaram sem diferença.

## Adendo: 0.35.0 (#61)
- O #61 subiu para a 0.35.0 (`cf9ab4e`): o começo ganhou 4 atos com fogos, uma dica do prefeito depois de 3 minutos parado numa etapa, uma seta de direção na barra e o livro Diário de Otávio. O capítulo 3 tem 10 etapas. O save continua `veio-onboarding-7`.
- Houve um conflito em `tools/build_tutorial.py`: ficaram tanto os rótulos de UI do #57 (`build_ui_text.add_labels`) quanto o livro do diário. Os geradores rodaram sem diferença.
- E1: `python3 tools/check_rust.py` → 391 passam.
- Testar em jogo: os fogos, o Diário de Otávio e a seta na barra.

## Adendo: 0.36.0 (#61)
- O #61 subiu para a 0.36.0 (`10994df`) e o save passou a `veio-onboarding-8`. Entraram:
  - as festas do capítulo 2, com a Festa da Colheita 3 dias depois de abrir o capítulo;
  - a roupa e as reações do prefeito;
  - comentários curtos dos aldeões comuns;
  - os documentos da história do Veio.
- Houve conflito só nas línguas, juntadas chave a chave, sem choque. Os geradores rodaram sem diferença.
- E1: `python3 tools/check_rust.py` → 394 passam.
- Testar em jogo: a festa (dormir avança os dias), o casaco e o nome dourado do prefeito, as partículas e os sons dele, e os comentários dos aldeões.

## Adendo: 0.37.0 (#61)
- O #61 subiu para a 0.37.0 (`b46b13b`), com o save `veio-onboarding-10`. O primeiro ato agora começa com um favor: o prefeito manda ao Seu Bento (ferreiro), que manda à Dona Cida (lavradora). É preciso plantar e entregar 12 trigos, e o Bento dá a prensa e 16 cabos. Entra também o capítulo "A noite".
- Houve conflito nas línguas, juntadas chave a chave, e na wiki, regenerada. A Combinadora continua.
- E1: `python3 tools/check_rust.py` → 401 passam. `check_project.py` → 100 páginas conferidas.
