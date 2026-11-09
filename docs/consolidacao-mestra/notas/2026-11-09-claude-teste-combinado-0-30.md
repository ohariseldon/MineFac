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
