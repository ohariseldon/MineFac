# Água, lavoura e rios (0.20.0)

Pedido do Luiz: lavoura com irrigação e necessidades reais, sem refazer as plantas do
Minecraft; bomba, cano e usinas de verdade; nascentes no alto de algumas montanhas formando
rios que descem até o mar com quedas para represar; tudo explicado no guia sem refazê-lo.
A vila operária (`native/api/src/workforce/`) não foi tocada.

## Blocos

| Bloco | Código | O que faz |
|---|---|---|
| Bomba d'água | `production/water.rs` (`pump`) | 25 mB/t por fonte de água parada encostada, até 4; 6 FE/t; tanque de 4.000 mB |
| Cano d'água | `pipe` | Liga as seis faces, sem energia; 1.000 mB; nivela pela metade da diferença, até 200 mB/t por face |
| Aspersor | `sprinkle` | Lavoura 9x9 em volta, na altura das plantas; tanque de 4.000 mB; slot de adubo |
| Turbina hidrelétrica | `turbine`, `measure_dam`, `hydro_power` | Mede a represa atrás e a saída na frente a cada 40 ticks |

A água fica no NBT `Water` (mB) de cada bloco. A bomba e os canos empurram para vizinhos que
recebem (cano, aspersor, caldeira a vapor); a bomba nunca recebe, então a água não volta.
A caldeira a vapor aceita água encanada (5 mB/t enquanto queima) quando não tem água parada ao
lado. Usei um sistema próprio de água em vez da API de tanques do Horizon para não depender de
assinaturas que o substituto offline ainda não espelha.

## Lavoura

`needs(planta)` dá a água por estágio, o adubo por estágio e a idade madura:

| Planta | Água/estágio | Solo/estágio | Estágios |
|---|---|---|---|
| Cenoura | 45 mB | 4 | 7 |
| Trigo | 60 mB | 6 | 7 |
| Batata | 70 mB | 6 | 7 |
| Abóbora (caule) | 80 mB | 7 | 7 |
| Melancia (caule) | 90 mB | 7 | 7 |
| Beterraba | 150 mB | 14 | 3 |
| Flor-tocha | 60 mB | 5 | 1 |

O aspersor varre 3 posições por tick (orçamento de terreno), deixa a terra arada com umidade 7
(2 mB por bloco, metade na chuva sob céu aberto) e, para cada planta regada, sorteia um estágio:
1 em 10 com adubo no solo, 1 em 30 só com água. O sorteio é um hash fixo da posição e do tempo.
Farinha de osso vale 40 de adubo e biomassa 20; o solo guarda até 400 e repõe do slot abaixo de
200. Fungo do Nether fica de fora. Sem aspersor, nada muda no crescimento vanilla.

## Hidrelétrica

`hydro_power(queda, espelho, vazão) = 6 × queda × min(espelho, 40)/40 × (1 + min(vazão, 12)/4)`,
com queda limitada a 32 e teto de 320 FE/t. Queda abaixo de 3 ou espelho abaixo de 12 blocos
não gera. A queda é a coluna de água parada atrás da turbina menos a coluna da saída na frente
(água represada dos dois lados não conta). O espelho é uma amostra 7x7 no topo do lago, 3 blocos
para dentro. A vazão vem do rio do Veio mais próximo (até 10 blocos), então represar um rio
grande rende até o triplo de uma represa de água parada.

## Rios (`native/api/src/rivers.rs`)

O Horizon não lê worldgen de data pack e `native_chunk_generate` trocaria o terreno inteiro.
Por isso os rios são traçados por cima do terreno vanilla da seed, logo depois da geração:

1. `chunk_load` com `new = true` marca o chunk como recém-gerado e o põe na fila de varredura.
2. A varredura acha o pico do chunk. Acima de y 100, com chance de 20% a 50% (hash da posição)
   e a pelo menos 40 blocos de outra nascente, brota uma nascente 2 blocos abaixo do pico,
   enfeitada com pedregulho musgoso.
3. Cada frente desce pela vizinha mais baixa (8 direções, curvas de no máximo 45°). Encontrando
   outro rio, junta-se a ele e soma a vazão até o mar. Num buraco, procura a passagem de menor
   corte (até 8 blocos, alcance 24); sem passagem, a água acumula como lagoa.
4. O nível só desce, em degraus de pelo menos 3 blocos (`HOLD + 1`): são as quedas.
5. A vazão é o número de nascentes somadas mais uma unidade a cada 48 blocos percorridos.
   Raio do leito e profundidade crescem com a raiz da vazão (raio até 4, profundidade até 4).
6. O leito é escavado com `edit_blocks` (2 chunks por tick): ar acima, água, fundo de cascalho ou
   areia, barrancos de terra e grama onde o terreno é mais baixo que a água.
7. As frentes esperam chunks não gerados e continuam quando o jogador explora. O estado fica em
   `<mundo>/veio/rios.txt`, salvo a cada 1.200 ticks e ao trocar de mundo.

Só chunks gerados nos últimos 6.000 ticks são escavados; colunas com blocos de construção (vilas,
blocos de mod) são contornadas. Um rio que encontra terreno antigo termina ali.

## Limitações e o que conferir no jogo

- `chunk_load`, `Host::world_dir` e `Host::read_columns` entraram no substituto offline copiados
  do código do Horizon, mas só a compilação pelo launcher confirma as assinaturas.
- Num mundo já explorado, os rios só aparecem nas áreas novas.
- O traçado roda só no servidor (sondado com `surface_heights`). Em multijogador com vários
  mundos, o estado é por pasta de mundo.
- Testes: `production/water_tests.rs` (9) e `rivers/tests.rs` (10), em terreno sintético com
  montanha cônica e vale.
