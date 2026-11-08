# Vila que vira cidade

Pedido do usuário para a frente Cloud. O Grok já fechou a regra de moradia. Este texto é o que falta para a vila parecer uma cidade, sem inventar um sistema paralelo de habitantes.

## O que já vale

- A população fica entre 30 e 150. O teto sobe com postos, zonas, armazéns e o centro de controle.
- Cada aldeão precisa de uma cama. Conta a cabeceira (`part=head`), não o pé. A volta das camas lê 16 blocos por tick num raio de 32, em três camadas.
- A vila que o jogo coloca no começo (`village_plains`) já traz camas para quem está lá. Crescer além disso é o jogador construir mais casas.
- Sem cama livre, ninguém nasce. A cada 10 s, quem está perto ouve: “N aldeões e M camas. Construa mais casas.” (`veio.village.need_houses`).

## O que construir

Não é um gerador que enche o mapa de prédios. É um conjunto de casas que o jogador coloca, no estilo do Minecraft, e a vila passa a usar.

1. **Casas diferentes.** Quatro ou cinco plantas pequenas (madeira, pedra, telhado, uma cama, uma porta), com profissão visível: fazenda, ferraria, cartografia, templo. Cada uma cabe numa cama que o contador já entende. Receita com materiais da fábrica (tábuas, pedra, vidro, uma cama), não um bloco mágico que teleporta gente.
2. **Mercado.** Uma praça com bancas. O comércio vanilla continua: o aldeão sem ordem de trabalho negocia. A banca é o lugar, não uma segunda loja.
3. **Vida.** Aldeões sem tarefa andam entre a cama, o sino e a banca. Quem está contratado continua no posto. Não criar outra IA de trabalho.
4. **Aviso no centro.** O painel do centro de controle pode repetir “faltam casas” junto com a barra de ação. O texto já existe.

A vila inicial não precisa ser demolida. As casas novas entram na borda, conforme a população pede.
