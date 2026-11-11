# Mapa 5 · Costa oeste (foco)

Mar a oeste, uma muralha de montanhas a leste; os rios descem da muralha até o mar. O litoral é úmido
e o interior, atrás das serras, seco. Planta em `tools/mapa/preview_maps.py` (`concept_5`); grade e
detalhe em `tools/mapa/costa_oeste.py`.

## A grade dos 9 blocos

Cada bloco tem 5.000 x 5.000 blocos. Imagem: [grade.png](grade.png).

| Bloco | x | z | O que tem | Situação |
|---|---|---|---|---|
| NO · Noroeste | -7500 a -2500 | -7500 a -2500 | mar frio | espera |
| N · Norte | -2500 a 2500 | -7500 a -2500 | neve, taiga e o rio do norte | espera |
| NE · Nordeste | 2500 a 7500 | -7500 a -2500 | a Muralha de Pedra gelada | espera |
| O · Oeste | -7500 a -2500 | -2500 a 2500 | o Mar do Oeste e a ilha dos cogumelos | espera |
| **C · Centro** | -2500 a 2500 | -2500 a 2500 | a vila, o Morro da Mina e a Serra do Veio | **em detalhe** |
| L · Leste | 2500 a 7500 | -2500 a 2500 | a Muralha de Pedra e os passos para o interior | espera |
| SO · Sudoeste | -7500 a -2500 | 2500 a 7500 | mar morno e quente | espera |
| S · Sul | -2500 a 2500 | 2500 a 7500 | litoral quente: selva, savana e deserto | espera |
| SE · Sudeste | 2500 a 7500 | 2500 a 7500 | o interior seco: deserto e terras áridas | espera |

O zoom de um bloco usa os mesmos cortes de clima do mapa inteiro: mostra os mesmos biomas, só com
mais detalhe de relevo e de borda.

## Bloco C · Centro (proposta para conversar)

Imagem: [bloco_C.png](bloco_C.png). Planta da vila de perto: [../previas/vila.png](../previas/vila.png).

### O lugar

- **A vila** fica na **Planície do Veio**, uma planície pequena e amena (uns 600 x 700 blocos)
  encaixada entre três coisas:
  - o **Morro da Mina**, ao norte;
  - o **Rio Veio**, a leste;
  - a **mata**, a ~150 blocos a oeste.
- **Terraços:** a vila desce em degraus de 2 blocos até o rio.
- **O Morro da Mina** (prado, de x -650 a 850, crista em z ≈ -480) é o primeiro contraforte da
  serra. O rio o atravessa no **Passo do Rio**. Na encosta sul ficam:
  - a **mina do avô**, fechada, a ~300 blocos da praça;
  - a **pedreira**.
- **A Serra do Veio**, a nordeste (z ≈ -1.800), tem neve no alto, picos e um pouco de cerejeiral
  nas encostas, longe da vila. Embaixo dela fica a **escuridão profunda**, para a cidade
  ancestral.
- **O Rio Veio** desce da serra, passa pelo leste da vila, faz uma curva para sudoeste e atravessa
  o **Brejo do Rio** (pântano e um pouco de manguezal) até a **Foz**, com praia, no canto sudoeste
  do bloco. Na foz está o **cais velho**, por onde o minério descia o rio quando a mina funcionava;
  é a porta do "mundo lá fora" (capítulo 7).
- **As matas em volta:**
  - a oeste, a **Mata Escura** (floresta escura);
  - a noroeste, o **Pinhal do Norte** (taigas antigas);
  - a leste, o **Bosque das Bétulas**;
  - a sudeste, a **Taiga Velha**;
  - ao sul, os **Bosques do Sul**, que esquentam até selva esparsa perto da borda.

### Recursos garantidos perto da vila

O subsolo é o do Minecraft; estes pontos o plano reserva, porque a história precisa deles:

| Recurso | Onde | Para quê |
|---|---|---|
| Cobre | afloramento a ~350 blocos a nordeste da praça, no terraço da margem | a primeira extratora (etapa 5) perto da oficina |
| Ferro | no Morro da Mina, junto da mina do avô | o ferro à mão e, depois, a mina reaberta |
| Carvão | veio raso na borda da mata, junto da carvoaria | o gerador e o Seu Lino |
| Argila | o barreiro na curva do rio e a foz | a Dona Zefa e o telhado do coreto |
| Pedra | a pedreira do Morro | a fonte da praça e a prefeitura de pedra |

### Onde a vila cresce

- **Distrito industrial** (capítulos 1 a 5): o terraço a leste, entre a vila, o rio e o morro.
  Tem chão plano, água para a roda d'água e o cobre embaixo.
- **Lavouras e pastos**: a planície a sudoeste da praça.
- **Expansão da cidade** (capítulos 4 e 6): ao sul, do outro lado da curva do rio, com pontes.

### Estradas

- **Estrada da Mina**: da praça até a mina do avô.
- **Trilha da Mata**: até a carvoaria.
- **Trilha da Serra**: sobe o rio até a Serra do Veio.
- **Estrada velha do Rio**: acompanha o rio até o cais velho, na foz.

### Regras do bloco

- **Perto da vila:**
  - 600 blocos em volta da vila sem ravinas e sem bocas de caverna;
  - nenhuma variante rara de bioma (cerejeiral, girassóis, bétulas antigas, taigas antigas) a menos
    de ~1.000 blocos: o raro fica para quem explora.
- **Ravina permitida:** a **Fenda da Serra**, a nordeste, a ~2.300 blocos da vila. É a primeira
  aventura subterrânea.

### Para conversar

1. **Nomes:** Planície do Veio, Morro da Mina, Rio Veio, Passo do Rio, Mata Escura, Pinhal do
   Norte, Bosque das Bétulas, Taiga Velha, Bosques do Sul, Brejo do Rio, Foz e Fenda da Serra.
2. **O cais velho na foz** como porta do capítulo 7 (porto e exportação).
3. **Onde fica a cidade grande:** ao sul, do outro lado do rio; ou na margem leste, junto da
   indústria?
4. **A distância até o mar:** a foz fica a ~2.500 blocos da vila. Perto o bastante para ser uma
   viagem, longe o bastante para não ser o quintal.
