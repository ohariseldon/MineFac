# Mapa 5 · Costa oeste (foco)

Planta final do mundo inteiro: [mapa_final.jpg](mapa_final.jpg) (15.000 x 15.000 blocos, 4 blocos por
pixel) e uma prévia leve em [mapa_final_previa.jpg](mapa_final_previa.jpg). A planta em dados, para o
gerador e para o mod: [plano.json](plano.json). A vila:
- de perto no estágio 1: [vila_estagio1.png](vila_estagio1.png);
- o plano de crescimento: [vila_crescimento.png](vila_crescimento.png).

**Ferramentas:**
- `tools/mapa/costa_oeste.py`: o mundo, a revisão dos rios, as estruturas e o `plano.json`;
- `tools/mapa/vila_estagio1.py`: a vila no estágio 1;
- `tools/mapa/vila_crescimento.py`: o crescimento da vila, com a checagem de conflitos.

Os biomas usam a mesma tabela e o mesmo equilíbrio das prévias (`tools/mapa/preview_maps.py`): os
53 biomas estão no mapa.

## A ideia

A costa oeste da América do Sul, em escala de jogo:
- **a oeste,** o "Pacífico" e um litoral de planícies e cidades (o "Chile"), onde fica a vila;
- **no meio,** a Muralha de Gelo (os "Andes");
- **do outro lado da muralha,** uma bacia úmida em que os rios correm para leste e terminam no
  **Pantanal do Leste** (a "Amazônia" e o "Pantanal").

Só o oeste tem mar. Ao norte, a leste e ao sul o continente segue até a muralha de bedrock, como uma
fatia de um continente maior.

## Geografia

**A costa, de norte a sul:**
- **O Golfo do Norte** e os **fiordes e penhascos** (Noruega, Escócia): um planalto baixo que cai
  direto no mar frio, com quatro fiordes. O Rio do Norte despenca num deles, nas **Cataratas do
  Fiorde**.
- **Os Lençóis:** dunas brancas com lagoas entre elas.
- **A Foz do Veio:** uma fenda larga que entra uns 1.500 blocos na terra, até ~1.350 blocos da
  vila. O rio chega em três braços (o delta), a água vai alargando até virar mar, e na boca fica a
  **Ilha Grande** (como Marajó). O **cais velho** fica junto do delta.
- **A Ponta do Paredão:** uma península alta, esticada para o mar, com a Cascata do Paredão.
- **O Mar das Ilhas** (um Caribe): água turquesa e rasa, recifes de coral, ilhotas com coqueiros e
  praia em volta, e praias brancas no continente, com o deserto logo atrás.

**No interior:**
- **A Lagoa da Restinga:** uma lagoa no meio do deserto, cercada de manguezal, onde o Rio do Sul
  termina.
- **A Ilha da Bruma:** no mar aberto a oeste, alta, de campos de cogumelos.

**As serras e as matas:**
- **As serras:**
  - a **Muralha de Gelo**, de norte a sul;
  - a **Serra do Veio**, que sai da muralha para oeste e é a divisa de clima entre a **Mata Escura**,
    ao sul, e o **Pinhal do Norte**, ao norte;
  - os **Montes do Norte**;
  - a **Serra do Sul**, entre as **Terras Vermelhas**.
- **Ao norte,** o Planalto Gelado (neve e picos de gelo).
- **Na bacia leste,** taigas ao norte e selva e bambu ao sul.

## Os rios

Cada rio é uma árvore: nascentes pequenas viram córregos, os córregos se juntam em afluentes, e os
afluentes no rio principal. A largura cresce com a soma de tudo o que chegou: um fio de 2 blocos na
nascente, 40 ou mais perto da foz.

| Rio | Afluentes | Foz |
|---|---|---|
| Rio Veio | Ribeirão da Serra (com o Córrego do Gelo), Rio Claro (com o Arroio da Pedra e o Arroio Fundo), Córrego do Morro, Rio das Bétulas (com a Sanga Clara), Riacho da Mata (com a Grota Escura) | o delta da Foz do Veio |
| Rio do Sul | Rio Seco de Cima (com o Córrego Vermelho), Rio Manso, Sanga do Oásis | a Lagoa da Restinga |
| Arroio das Palmas | — | o Mar das Ilhas |
| Rio do Norte | Riacho dos Montes | as Cataratas do Fiorde |
| Grande Rio do Leste | Rio do Sul do Leste e oito igarapés que descem da muralha, um deles com o próprio afluente | o Lago do Pantanal |

O Rio Veio corre entre a Serra do Veio e o Morro da Mina e atravessa o morro no **Passo do Rio**,
logo ao norte da vila.

### A revisão física (automática)

`costa_oeste.py` confere os rios a cada vez que gera o mapa. Hoje sai sem problemas.

**O que ela confere:**
- **A água só desce.** O perfil de cada rio é o ponto mais baixo do terreno da nascente até ali.
  - Se um morro mais alto que a nascente está no caminho, a nascente de verdade é o alto antes dele.
  - Na confluência, o afluente chega abaixo do rio que o recebe; na foz, no nível do mar ou do lago.
- **O rio cava o vale.** O terreno em volta é rebaixado: estreito e fundo nas serras, largo e raso na
  planície. Onde o vale corta a serra, o bioma vira o de encosta, e não o de pico.
- **Cachoeiras** aparecem onde o perfil cai mais de 9 blocos em menos de 60. Ao todo são 40: as 9
  com nome e as que o relevo pediu.
- **Nenhum rio cruza outro** fora da confluência.
- **Nenhum passa pelo mar** antes da foz.

**O que a primeira rodada da revisão corrigiu:**
- três cruzamentos (a Sanga Clara com o Rio das Bétulas e com o Rio Manso, e o Rio Manso com o Rio do
  Sul);
- o Rio Veio e o Rio Claro correndo por cima da crista do Morro da Mina;
- o Arroio das Palmas subindo do litoral para o interior (agora desce direto para o mar);
- a Grota Escura, refeita;
- rios cortando morros em cânions de quase 200 blocos.

## Estruturas

São 59 estruturas, todas com o bioma conferido pelas regras do Minecraft. Uma estrutura fora do
bioma certo vai para o lugar válido mais perto, até 1.500 blocos; dez foram movidas. Nenhuma fica a
menos de 700 blocos da vila, fora as da história.

- **Do Minecraft, na superfície:**
  - 9 vilas (dos Campos, do Planalto, do deserto, do Oásis, da savana, da Chapada, da taiga, do Leste
    e nevada);
  - 2 templos do deserto e 2 da selva, a mansão (na Mata Escura), 2 cabanas de bruxa (Brejo e
    Pantanal) e 2 iglus;
  - 2 postos de pillagers, 2 ruínas de trilha, 3 portais em ruínas e 2 poços do deserto.
- **Do Minecraft, no mar:** 2 monumentos oceânicos, ruínas submersas, naufrágios (um na praia) e um
  tesouro enterrado.
- **No subsolo:**
  - a cidade ancestral, sob a muralha;
  - 2 câmaras de desafio;
  - 3 fortalezas do End, no anel de 1.300 a 2.800 blocos, como no Minecraft;
  - mina abandonada, geodo e fósseis.
- **Do Veio, ligadas à história:** o **Acampamento dos Saqueadores** (de onde vêm as ondas), o cais
  velho e a mina do avô.
- **Do Veio, extras** (ideias de estruturas da comunidade, como o farol, o forte, a torre, as
  catacumbas e a taverna, que aqui seriam construídas pelo próprio Veio):
  - o farol abandonado (Ilhota do Farol) e o forte em ruínas (Ponta do Paredão);
  - a torre de vigia (Serra do Veio), o mosteiro (Muralha) e o observatório (Montes do Norte);
  - a vila de pescadores abandonada (Lençóis), a ponte de pedra antiga e o moinho velho;
  - a taverna da encruzilhada, as catacumbas sob a Mata Escura, o templo submerso (Mar das Ilhas) e o
    santuário da Bruma.

## Perto da vila

- **Argila:** o barreiro grande fica no **Brejo do Rio**, a meio caminho entre a vila e a foz (~1.200
  blocos), e também no delta.
  - O Minecraft põe manchas pequenas de argila em qualquer rio raso. Para a argila ser difícil de
    verdade, o gerador vai tirar essas manchas num raio em volta da vila.
- **Outros recursos garantidos:**
  - **cobre** aflorando ao lado do terreno da oficina;
  - **ferro** no Morro da Mina;
  - **carvão** junto da carvoaria;
  - **pedra** na pedreira do morro.
- **Proteção:** 600 blocos sem ravinas nem bocas de caverna em volta da vila, e nenhum bioma raro a
  menos de ~1.000 blocos.

## A vila e os estágios

**Estágio 1** ([vila_estagio1.png](vila_estagio1.png)): a vila em declínio, como o jogador a encontra.
- **A praça:** a fonte seca e quebrada e o coreto sem telhado.
- **As casas:** 7 habitadas, 2 em obra, 1 abandonada e 4 lotes vazios.
- **Os moradores dos pedidos:** a vendinha vazia, a carvoaria apagada, o forno da olaria rachado e um
  pasto com uma vaca só.
- **O terreno:** terraços descendo para o rio, com escadas de pedra.

**O crescimento** ([vila_crescimento.png](vila_crescimento.png)): a vila cresce por anéis
irregulares que ligam as ruas de terra do começo e formam quarteirões tortos. O que existe no
estágio 1 fica no lugar (pode ganhar andar, telhado ou toldo), e o novo entra nos espaços livres.
`vila_crescimento.py` confere que nenhuma rua, casa ou prédio novo passa por cima de algo que já
existe; hoje, nenhum conflito.

| Estágio | Quando | O que acrescenta |
|---|---|---|
| 2 · A vila respira | os pedidos da vila e o capítulo 2 (a noite) | fonte com água, coreto com telhado, as casas em obra prontas, a abandonada reformada, o Anel da Praça, lanternas, a muralha de margem a margem do rio |
| 3 · A vila operária | capítulo 3 | prefeitura, postos de lavoura e de pastor, mesa do capataz, casas nos 4 lotes vazios, o começo do Anel de Fora |
| 4 · A vila cresce | capítulo 4 (fase Vila) | o Anel de Fora fecha os quarteirões; poço novo, parque, a vendinha vira loja no mesmo lugar, 14 casas novas |
| 5 · A grande indústria | capítulo 5 | o distrito industrial em volta do terreno da oficina, o quartel e a torre Tesla |
| 6 · A cidade | capítulo 6 (fase Cidade) | ruas de pedra, a ponte e o bairro da margem, a Rua do Sul para a expansão, a praça cívica |

**As trocas:**
- **Quando:** as do estágio acontecem longe dos olhos do jogador (a mais de 32 blocos) ou com um
  efeito de obra. As grandes, uma por dia de jogo.
- **Como no jogo:** cada prédio vira um conjunto de modelos de estrutura, um por estágio.

## O que já está no jogo

- **Os moradores com nome dormem.** O prefeito e os vizinhos vão à cama mais perto ao anoitecer e
  voltam de manhã (`onboarding/rest.rs`). No mapa fixo, cada um mora num dos prédios do estágio 1.
- **As ondas de Saqueadores vêm do lado do acampamento.** Num mundo aleatório, o lado sai do
  centro da vila; no mapa fixo, ele vem do acampamento da planta.
- **Sem patrulhas de pillagers do Minecraft.**
- **Chão limpo onde a vila nasce:** a limpeza tira a serrapilheira e os arbustos junto com as
  árvores.

**Desde a 0.40.0:** a vila do estágio 1 é erguida no jogo, bloco a bloco, com o acabamento e as
trocas de estágio ([../../vila/README.md](../../vila/README.md)).
- Os moradores dos pedidos ficam na frente da própria casa.
- O prefeito fica na praça.
- A casa do fundador e o terreno da oficina têm lugar marcado no plano.

**Ainda não está no jogo:**
- o gerador que usa esta planta (o rascunho está em `work-in-progress/finite-world`);
- as casas novas do estágio 4 em diante, e o parque, o quartel, o distrito e a praça cívica.

## Histórico

A divisão em 9 blocos ([grade.png](grade.png)) e o zoom do bloco central ([bloco_C.png](bloco_C.png))
serviram para a primeira rodada de detalhe.
