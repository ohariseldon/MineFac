# Veio — Realismo 2.0

Visual original para o Veio: céu com atmosfera, nuvens, estrelas e sol próprios, plantas e
água em movimento, luz colorida de tochas, fornos e máquinas quentes, brilho, raios de sol,
estações do ano nas folhas e a fumaça da fábrica no céu. Mantém os modelos, texturas e regras
do Veio. Não contém código, imagens ou materiais de outros jogos ou pacotes de shaders.

O launcher oferece **Realismo leve — pensado para 8 GB** e **Leve — sem shaders** antes de abrir o jogo.
`Escolher-Visual.cmd` abre essa escolha de novo; `Jogar-Leve.cmd` inicia sem o pacote.
Também dá para desligar o pacote em Opções > Vídeo > Shaders.

## Níveis de qualidade

A opção **Qualidade** decide o custo de uma vez só:

| | Leve (8 GB) | Equilibrado (padrão) | Alto |
|---|---|---|---|
| Nuvens | 2 camadas | 4 camadas | 5 camadas |
| Brilho (bloom) e raios de sol | desligados | ligados, 12 passos | ligados, 24 passos |
| Luz colorida | desligada | ligada | ligada |
| Plantas e água balançando | desligadas | ligadas até 48 blocos | ligadas até 48 blocos |

Sombras (512, 1024 ou 2048; alcance de 32 a 96 blocos), reflexos, atmosfera, nuvens, vinheta
e cada efeito também têm opção própria nas páginas do pacote.

## O que o jogo informa ao shader

O mod nativo manda ao shader, uma vez por quadro, quatro números: a fase do ano, a temperatura
do ar onde está a câmera, a fumaça da fábrica por perto e um sinal de que o mod está ativo.
Com eles:

- as folhas e a grama esverdeiam na primavera, ficam fortes no verão, douradas e avermelhadas no
  outono e acinzentadas no inverno; a cor final da imagem acompanha a estação;
- o céu, as nuvens e a névoa ficam amarelados e cinzentos perto de fábricas muito poluídas;
- o céu fica mais encoberto na chuva, no outono e no inverno.

Geradores, motores a vapor, fornos, cozinhas e outras máquinas quentes acendem uma luz colorida
oscilante quando estão ligadas, e as mais próximas (até 8, num raio de 32 blocos) iluminam o
pátio. Elas soltam fumaça ou vapor a cada 2 segundos, só quando há jogador a menos de 40 blocos.

Sem o mod (pacote usado sozinho), o shader usa valores neutros: primavera, sem fumaça.

## Custo

- O mod: com mil máquinas quentes, cerca de 40 µs por tick e as 8 luzes por quadro sem
  varrer a fábrica (`python tools/check_performance.py`, cenário
  `ambience_1000_hot_machines_second`).
- O pacote: pós-processamento em meia e um quarto de resolução para brilho e raios de sol;
  nuvens analíticas no céu, sem volume; sem SSAO, traçado de raios ou histórico temporal.
- O `budget.gpu_ms` (3 ms) é um objetivo, não uma medição. A meta de 8 GB considera o
  computador inteiro. Placa de vídeo, resolução e tamanho da fábrica pesam mais que a RAM.

## Desenvolvimento

Shader API 0.1 do Horizon. `python tools/check_shaders.py` compõe cada estágio com o prelúdio
da API e valida com o `naga` (instale com `cargo install naga-cli`). Também confere se cada
opção tem seu `override` e se as páginas citam opções que existem. Os arquivos `.wgsl` ficam
na raiz do pacote. A redistribuição deste shader original é permitida pela licença MIT incluída.
