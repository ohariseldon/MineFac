# Cidade viva integrada e aumentada cerca de 5 vezes

- **Data / frente / autor:** 2026-11-09 · claude · thread "Testar a build combinada no PC"
- **Onde:** zip `cidade-viva.zip` enviado pelo dono (patch `50f1bd4` sobre `17f229f`, 0.34.0), juntado na `claude/teste-combinado`
- **Sistemas:** spawn, cidade, população, ruínas, centro de controle · **FUN:** nova, sem ID
- **Tipo:** código · conteúdo · decisão do dono

## O que mudou
- Entra a cidade pronta do Veio (`tools/build_town.py`, `workforce/town.rs`, `founders.rs`), a casa do jogador redesenhada e a população de 20 a 500 com o casal fundador. Detalhes no `LEIA-ME` do zip e em `frentes/SOCIEDADE.md`.
- **Pedido do dono:** a cidade era pequena para a escala do jogo. O centro desenhado à mão (`CORE` = 44) continua igual. Em volta dele, até a cerca agora em `HALF` = 96, vieram bairros:
  - 48 quadras: de quatro casas, ou de sobrados com loja e casas;
  - 6 praças de bairro com laguinho;
  - 4 lavouras ao sul e 4 galpões a leste;
  - ruas de terra entre as quadras e avenidas pavimentadas até a cerca nos quatro sentidos, com portão também ao norte.
  São cerca de 140 prédios novos (eram uns 23) e 300 camas (eram 30). A cidade continua nascendo com 20 moradores: encher as casas fica com o jogador.
- Com a cidade maior, mudaram:
  - as ruínas foram para fora da cerca: `ruins::RING` passou a ser `HALF`+14 a `HALF`+54, e canteiros que caem dentro do quadrado da cidade são pulados;
  - o alcance do centro (`center::RADIUS`) foi de 64 para `HALF`+64 (160), para cobrir a cidade e as ruínas;
  - o jogador novo aparece em `spawn::APPROACH` = `HALF`+12 (108 blocos), já fora da cerca.
- Na junção, o `ruins::arrive` passou a ser chamado depois do `town::build`, como o zip pedia. Saiu uma duplicata de `world_dir` no host de teste. README, wiki e guia ficaram com o texto da cidade (teto de 500).

## Evidência
- E1: `python3 tools/check_rust.py` → 410 passam. `check_project.py` e `check_performance.py` passam.
- E1: `python3 tools/build_town.py` → 31.556 comandos, 300 camas e 301 lugares (eram cerca de 5 mil comandos).
- Ainda não testado em jogo. Riscos: a função roda num tick só (65.536 é o limite padrão de `maxCommandChainLength`), e a cidade de 193x193 precisa de todos os chunks carregados (distância de visão de pelo menos 7 chunks).

## Toca no compilado
- R (risco): pico de lag ao erguer a cidade. Se pesar, dividir `veio:town/build` em partes por tick.
- O alcance do centro de controle mudou (64 para 160), o que vale para postos, zonas, armazéns e obras.
