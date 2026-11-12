# A vila do Veio

A vila que o jogador encontra num mundo novo, desenhada bloco a bloco. O Veio a ergue no lugar da
vila do Minecraft (`place structure minecraft:village_plains`, que vinha antes). As posições são as
do plano do estágio 1 ([../mapa/costa-oeste/vila_estagio1.png](../mapa/costa-oeste/vila_estagio1.png))
e do plano de crescimento ([../mapa/costa-oeste/vila_crescimento.png](../mapa/costa-oeste/vila_crescimento.png)).

| Prévia | O que mostra |
|---|---|
| [vila_estagio1_3d.png](vila_estagio1_3d.png) | a vila inteira no estágio 1 |
| [vila_estagio2_3d.png](vila_estagio2_3d.png) | a vila inteira no estágio 2 (a vila respira) |
| [predios_1_casas.png](predios_1_casas.png) | as 7 casas habitadas, por fora e por dentro |
| [predios_2_moradores.png](predios_2_moradores.png) | as casas e oficinas dos moradores dos pedidos |
| [predios_3_obras_e_novas.png](predios_3_obras_e_novas.png) | a obra, a ruína, o lote, o poço e as casas novas |
| [predios_4_prefeitura.png](predios_4_prefeitura.png) | a prefeitura nos estágios 3, 4 e 6 |
| [predios_5_praca.png](predios_5_praca.png) | a praça nos estágios 1 e 2 |
| [predios_6_acabamento.png](predios_6_acabamento.png) | o mesmo prédio nos estágios 1, 2 e 4 |

## As ferramentas

- **`tools/vila/modelo.py`:** as peças de montar.
  - As peças: paredes, vigas, telhado de duas águas, janela, veneziana, porta, cama, mesa e cadeira.
  - A rotação de 90 em 90 graus, que gira também `facing`, `axis` e os lados.
  - A ligação de cercas, muros e vidraças aos vizinhos (o `setblock` não liga sozinho).
  - Os comandos `fill` e `setblock`, juntando os blocos iguais seguidos.
  - A checagem dos estados de bloco.
- **`tools/vila/predios.py`:** os modelos.
  - As 7 casas, os 6 moradores, o pasto, a roça e a praça.
  - O poço, a obra, a ruína, o lote e a prefeitura dos estágios 3, 4 e 6.
  - As variações de material das casas novas.
  - Os padrões de rua e o acabamento por estágio.
- **`tools/vila/build_village.py`:** põe os modelos nos lugares do plano. Gera:
  - as funções do datapack (`content/data/veio/function/village/`);
  - a tag do chão de rua (`content/data/veio/tags/block/road_ground.json`);
  - o plano para o Rust (`native/api/src/workforce/town_plan.rs`).
  - Com `--check`, só confere (o `tools/check_project.py` roda assim).
- **`tools/vila/previa.py <pasta>`:** as prévias desta pasta.

Depois de mudar um modelo, rode:

```bash
python3 tools/vila/build_village.py
```

```bash
python3 tools/vila/previa.py docs/vila
```

## Como a vila sobe no jogo (`workforce/town.rs`)

1. **O lugar.** Num mundo novo, quem chega pela primeira vez espera uns 5 segundos (os chunks em
   volta carregam). O Veio escolhe:
   - um terreno plano, seco e sem árvores;
   - a mais de 150 blocos de qualquer aldeão (longe das vilas do Minecraft).
   - Ali fica o centro (a fonte da praça), salvo em `<mundo>/veio/vila.txt`.
   - O jogador aparece a 44 blocos, fora dos prédios.
2. **As peças.** A cada meio segundo, até 3 peças do estágio atual. Cada uma:
   - mede o piso: a mediana do chão sob as paredes, menos um;
   - roda a função dela: tira as árvores em volta, limpa o volume, põe grama onde falta chão e terra
     embaixo, pedregulho sob as paredes (aparece onde o chão desce), os blocos, e um aldeão por
     cama até o número de moradores da casa.
   - Uma peça em chunk não carregado espera o jogador chegar perto.
   - Nenhuma peça nasce em cima de um jogador.
3. **As ruas e o verde,** depois que as peças do estágio estão de pé, em pedaços de 32 x 32.
   - Cada bloco de rua cai no chão do lugar (`positioned over motion_blocking_no_leaves`) e só troca
     terra, grama ou a rua do estágio anterior.
   - No estágio 1, o tronco que estiver no meio da rua sai.
   - O verde são árvores e canteiros (`place feature`), longe das ruas e dos lugares de todos os
     estágios.
4. **Os estágios** sobem com a história:
   - 2: os pedidos da vila atendidos;
   - 3: o capítulo da vila;
   - 4: o da cidade;
   - 6: o da metrópole.
   - Uma troca no mesmo lugar usa o mesmo piso, limpa a peça antiga e só acontece com todos os
     jogadores a mais de 32 blocos.

**Mundos antigos:** quem já tinha chegado (tem a tag `veio_village`) e não tem `vila.txt` continua com
a vila do Minecraft de antes. Ninguém ganha uma vila nova por cima dela.

**Quem mora onde:**
- **Os moradores com nome:**
  - Seu Bento, Dona Cida, Seu Zé, Dona Zefa, Seu Lino e Seu Tonho ficam na frente da própria porta;
  - o prefeito Otávio fica na praça, entre a fonte e o coreto (o telhado do coreto conta perto dele);
  - à noite, cada um vai para a cama mais perto (`onboarding/rest.rs`), que agora é a da casa dele.
- **A casa do fundador** (`tools/build_house.py`) ocupa o lugar dela no plano, ao norte da praça.
- **O terreno da oficina** é o quadrado livre a nordeste, que o prefeito indica.

## O estágio 1 e o que melhora depois

| | Estágio 1 (a chegada) | Estágio 2 (a vila respira) | Estágio 4 (a vila cresce) | Estágio 6 (a cidade) |
|---|---|---|---|---|
| Praça | pedregulho com musgo e falhas, fonte seca e quebrada, coreto sem telhado com telhas no chão, uma lanterna | pedra, fonte com água, coreto com telhado de tijolo, bancos e cinco lanternas | | |
| Casas | uma vidraça quebrada, teia no beiral, mato junto da parede | vidro novo, lanterna na porta, jardineiras e flores | venezianas, banco ao lado da porta, arbustos floridos | |
| Obras | duas casas em obra (meia parede, andaime, material) e uma abandonada | prontas, e a abandonada reformada | | |
| Lotes | quatro lotes com marcas de fundação no capim | | (casas novas no estágio 3) | |
| Ruas | terra batida, cascalho solto e mato no meio, sem lanterna | cascalho com lanternas a cada 28 blocos | pedregulho e andesito com meio-fio, lanterna a cada 20 | tijolo de pedra com meio-fio polido, lanterna a cada 14 |
| Prefeitura | | | sobrado com varanda e salão do conselho (o gabinete nasce no estágio 3) | o paço municipal, com escadaria, colunas e torre do sino |
| Poço | seco, telhadinho caído | | com água, telhado e lanterna | |
