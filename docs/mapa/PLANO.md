# Mapa fixo do Veio: planta e plano

Pedido de 11/11/2026: um mapa único, sempre igual, com todos os biomas do Minecraft distribuídos de forma
equilibrada nos 15.000 x 15.000 blocos, transições naturais, a vila inicial integrada ao terreno e uma vila
que muda de aparência conforme a história avança.

**Decidido em 11/11:** o [MAPA.md](../../MAPA.md) agora diz que os mapas são fixos e desenhados. Os
cinco conceitos ficam no [catálogo](conceitos/README.md) como opções do jogador ao criar o mundo; o
**mapa 5, Costa oeste**, é o foco, começando pelo [bloco central](costa-oeste/README.md).

## As cinco prévias

Geradas por `tools/mapa/preview_maps.py` e `tools/mapa/preview_village.py`. As imagens ficam em
[previas/](previas/).

| Conceito | Ideia | Dificuldade vem de |
|---|---|---|
| 1 · Continente clássico | frio ao norte, quente ao sul, seco a oeste, úmido a leste; mar em volta | distância e clima |
| 2 · Vale entre serras | a vila num vale; duas serras com passos; atrás de cada uma, outro clima | atravessar as serras |
| 3 · Ilha-mãe e arquipélago | a vila numa ilha temperada; os climas extremos em ilhas | o mar (barco) |
| 4 · Anéis de dificuldade | o centro é ameno; cada anel para fora é mais extremo; serra em anel com 4 passos | a distância do centro |
| 5 · Costa oeste | mar a oeste, muralha de montanhas a leste; rios descem ao mar | subir a muralha; interior seco |

Os cinco têm os 53 biomas do 1.21 (50 de superfície e 3 de caverna). O Jardim Pálido entra se a versão do
Horizon o tiver.

## Como o mapa vira o mundo do jogo (a "seed")

Uma seed não se desenha: ela é só o número que alimenta o gerador. O mapa fixo sai de três camadas:

1. **Planta de clima** (o que estas prévias desenham): continentalidade, temperatura, umidade, montanha,
   erosão e estranheza em escala de continente. São os mesmos campos que o gerador do Minecraft 1.18+ usa
   para escolher o bioma e a altura do terreno. A planta vai num arquivo pequeno com o jogo: uma grade
   de 64 em 64 blocos ocupa uns 235 x 235 pontos por campo.
2. **Seed fixa** para o detalhe: o ruído fino do vanilla (formato dos morros, cavernas, minérios,
   aquíferos, árvores) entra por cima da planta. Mesma seed e mesma planta dão o mesmo mundo sempre.
3. **Marcos colocados pelo mod** em coordenadas fixas: a vila e os prédios dela, a mina do avô e, mais
   tarde, as obras da história.

No motor, isso entra no gerador que o rascunho de `work-in-progress/finite-world` já intercepta
(`OverworldGenerator`). A planta substitui o clima amostrado em escala grande e mantém o ruído de detalhe
do vanilla. O subsolo continua o do Minecraft: mesmas cavernas, minérios por altura e deepslate. Os
biomas de caverna seguem as regras do vanilla:
- espeleotemas longe do mar;
- cavernas exuberantes onde é úmido;
- escuridão profunda sob as serras, longe da vila, por causa da cidade ancestral.

## Equilíbrio dos biomas

- **Mesma área por faixa de clima.** Cada uma das 5 faixas de temperatura e das 5 de umidade ocupa a
  mesma fração da terra (`equalize` no gerador de prévias). O norte continua mais frio que o sul, mas
  nenhum clima engole o mapa.
- **Variantes raras com fatia fixa.** Picos de gelo, girassóis, floresta florida, bétulas antigas,
  pinheiros antigos, bambu, cerejeiral, terras áridas erodidas e arborizadas, colinas de cascalho e
  costões rochosos ficam com uma fatia fixa da região-mãe deles (de 25% a 50%).
- **Faltam:** rios (pela natureza, estreitos), pântanos e manguezais ainda ficam pequenos (menos de 1 km²).
  No gerador de verdade, as várzeas vão ganhar largura.
- **Dificuldade pela distância:**
  - perto da vila (até ~1.500 blocos): biomas temperados, com ferro, carvão, cobre, madeira e argila;
  - no meio: neve, savana, selva, deserto e floresta escura;
  - longe: terras áridas, picos de gelo, cogumelos, bambu, picos irregulares e a escuridão profunda.

## A vila inicial

Planta: [previas/vila.png](previas/vila.png).

- **O terreno.** Fica numa planície ampla, numa mancha de clima ameno com borda irregular, sem disco
  artificial. Tem uma floresta a ~500 blocos a oeste, a Serra do Veio a ~150-700 blocos ao norte e um
  rio a ~150 blocos a leste.
- **Terraços** descem para o rio em degraus de 2 blocos (no máximo 4 entre a praça e a margem), com
  escadas onde as ruas atravessam.
- **Moradores onde o ofício faz sentido:**
  - a oleira na margem do rio, onde está a argila;
  - o carvoeiro na borda da floresta;
  - o vaqueiro com o pasto na planície;
  - o ferreiro perto da praça, na estrada que sobe para a mina do avô;
  - a lavradora ao lado da roça;
  - a vendinha na praça;
  - o terreno da oficina no terraço da margem, com água para a roda d'água.
- **Raio de 600 blocos** sem ravinas e sem bocas de caverna. Abaixo de y = 20, as cavernas seguem
  normais.
- **No código hoje:** os moradores ficam a ~10 blocos da praça (`errands.rs`, `favors.rs`). Com o mapa
  fixo, passam a ter coordenadas próprias.

## Estágios da vila (próxima etapa, depois de escolher o mapa)

Cada prédio da vila é um conjunto de modelos (arquivos de estrutura), um por estágio. Uma tabela liga
cada troca a um evento da história. Dois ritmos:

- **Pontual**, quando um pedido termina:
  - os pedregulhos do prefeito consertam a fonte (seca e quebrada → limpa → com água);
  - os tijolos cobrem o coreto;
  - a vendinha reabre com toldo e prateleiras;
  - a carvoaria volta a soltar fumaça;
  - a olaria ganha o forno novo.

  A troca acontece com um efeito curto de obra (andaime, partículas, som), de preferência quando o
  jogador está a mais de 32 blocos ou de costas.
- **Gradual**, ao longo dos capítulos:
  - ruas de terra → cascalho → pedra;
  - lotes vazios recebem fundação e depois a casa;
  - as casas abandonadas são reformadas ou demolidas;
  - postes e luz na praça;
  - a prefeitura de pedra (capítulo 4).

  Uma obra por dia de jogo, no máximo, para a vila mudar sem "pular".

**Estágio 0** (a vila em declínio, a planta acima):
- 7 casas habitadas;
- 2 em obra;
- 1 abandonada;
- 4 lotes vazios;
- a fonte seca;
- o coreto sem telhado;
- ruas de terra.

## Próximos passos

1. ~~Escolher um conceito~~: o mapa 5. ~~Atualizar o MAPA.md~~: feito.
2. Detalhar o bloco C do mapa 5 com o Luiz (nomes, recursos, onde a vila cresce).
3. Passar a planta para o formato que o gerador lê e testar no motor real: comparar a imagem do mundo
   gerado com a planta e conferir a cobertura de biomas, estruturas e recursos.
4. Modelos da vila no estágio 0 e a tabela de estágios, começando pela fonte.
