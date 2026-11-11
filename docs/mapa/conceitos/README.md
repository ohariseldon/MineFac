# Catálogo de mapas

Ao criar um mundo, o jogador vai escolher um destes mapas. Cada um é fixo: a mesma planta, a mesma
seed e os mesmos marcos sempre (regra em [MAPA.md](../../../MAPA.md)). A lista que o jogo e o
launcher vão ler está em [catalogo.json](catalogo.json).

| # | Mapa | Estado | Prévia |
|---|---|---|---|
| 1 | Continente clássico | espera | [mapa_1.png](../previas/mapa_1.png) |
| 2 | Vale entre serras | espera | [mapa_2.png](../previas/mapa_2.png) |
| 3 | Ilha-mãe e arquipélago | espera | [mapa_3.png](../previas/mapa_3.png) |
| 4 | Anéis de dificuldade | espera | [mapa_4.png](../previas/mapa_4.png) |
| 5 | **Costa oeste** | **foco** | [mapa_5.png](../previas/mapa_5.png) · [detalhe](../costa-oeste/README.md) |

**Em espera** quer dizer: a planta geral está guardada como está (a função `concept_N` em
`tools/mapa/preview_maps.py` e a prévia). Nada mais foi feito nesses mapas. Quando um deles sair da
espera, segue o mesmo caminho do mapa 5: a grade de 9 blocos e o detalhe bloco a bloco.

Os cinco cumprem a cobertura na planta: 50 biomas de superfície e 3 de caverna, com a vila numa
planície temperada no centro.
