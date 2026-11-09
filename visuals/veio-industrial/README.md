# Veio — Realismo 3.0

Visual original para o Veio: céu com atmosfera, nuvens, estrelas e sol próprios, plantas e
água em movimento, luz colorida de tochas, fornos e máquinas quentes, brilho, raios de sol,
estações do ano nas folhas e a fumaça da fábrica no céu. Mantém os modelos, texturas e regras
do Veio. Não contém código, imagens ou materiais de outros jogos ou pacotes de shaders.

O launcher oferece **Realismo leve — pensado para 8 GB** e **Leve — sem shaders** antes de abrir o jogo.
`Escolher-Visual.cmd` abre essa escolha de novo; `Jogar-Leve.cmd` inicia sem o pacote.
Também dá para desligar o pacote em Opções > Vídeo > Shaders.

## 3.1: céu físico e cor limpa

Depois do primeiro teste no jogo (céu "quebrado", brilho e contraste esquisitos), o céu e a cor
foram refeitos com técnicas publicadas e usadas há anos, em vez de fórmulas próprias:

- **Céu físico:** dispersão simples numa atmosfera esférica com ar (Rayleigh), névoa (Mie) e
  ozônio, com os coeficientes da Terra de Hillaire (EGSR 2020) e Bruneton. A luz até o sol
  também é calculada, então o pôr do sol avermelha sozinho e a sombra da Terra sobe no
  anoitecer. Depois, exposição fixa e a curva de filme ACES (ajuste de Narkowicz).
- **Brilho (bloom):** o céu claro do dia não brilha mais; só o sol e o que passa do branco. No
  chão, lâmpadas, lava, fogo e janelas acesas.
- **Cor:** sem a curva de contraste forte, sem tonalização dividida e com o tom dourado mais
  leve. A imagem fica perto das cores do Minecraft, com um pouco mais de saturação.
- Água e sombras não mudaram.

**Texturas realistas embutidas (3.1.1):** o Realismo instala e liga sozinho o pacote
`visuals/texturas/Veio-Texturas-JG-128x.zip`, uma adaptação de **PBR JG** (Jason J. Gardner,
CC BY-NC-SA 4.0): texturas fotográficas com mapas LabPBR de relevo, oclusão e brilho por
material, que este shader lê. O original é 256x; o Veio reduz para 128x para caber na meta de
8 GB (cerca de 64 MB) e mantém a água do jogo. O pacote não vai para o git: gere com
`python tools/build_realistic_textures.py`, que baixa do Modrinth e confere o SHA-512. O launcher põe o
pacote em `resourcepacks/`, liga por último na lista (os seus pacotes continuam) e sobe o
orçamento de texturas para 1024 MiB. No Leve ele sai.

**Controles na tela (3.3.0):** Brilho, Contraste e Saturação na página Imagem das opções do
shader; 1 é o ajuste do Veio.

**Claridade e contraste (3.2.5):** níveis como um fotógrafo ajusta: ponto de preto que tira o
véu, meios-tons mais claros, curva S mais firme e um pouco mais de cor. A opção "Cor e clima da
imagem" controla tudo isso (0 desliga).

**Sem contorno branco (3.2.4):** as nuvens em meia tela deixavam 1 a 2 pixels de céu limpo em
volta de cada silhueta e entre as folhas da grama, que pareciam um neon branco; agora o texel
calcula nuvem se houver céu em qualquer pixel ao redor. Nitidez mais suave e borda de nuvem perto
do sol que não estoura em branco.

**Nuvens claras e ar mais limpo (3.2.3):** bases de nuvem cinza-claro (luz que volta do chão e
mais luz espalhada dentro), raios de sol só como um toque sobre o terreno, névoa de distância
mais longe e mais fraca, e faces de costas para o sol com luz do céu suficiente para mostrar a
textura.

**Brilho do Minecraft nas texturas (3.2.2):** o PBR JG é feito para shaders PBR, que acendem a
cena bem mais; com a luz do jogo a pedra ficava com 57% do brilho do original e a grama com 76%.
A ferramenta agora mede o brilho médio de cada textura do Minecraft 26.3 e ajusta a do JG com
uma curva de luz até bater, sem estourar os claros. Minérios, plantações, folhas e água ficam
originais. Nuvens de chuva ficam cinza da neblina, não carvão.

**Adaptação do olho e nuvens lisas (3.2):** o brilho segue o lugar onde o jogador está, pela luz
em volta do olho que o jogo suaviza em ~2 s (a mesma técnica do BSL e do Complementary com
`eyeBrightnessSmooth`): em caverna, quarto escuro ou ao ar livre à noite a exposição abre até 2×
(em luz, não na imagem codificada); de dia fica em 1. Ao sair da caverna para o sol, a imagem
começa clara e assenta. As nuvens trocaram o deslocamento por célula (que serrilhava) por um
ruído suave, e a ampliação da meia tela passou a um filtro 3×3.

**Céu estável (3.1.3):** o Luiz viu o céu "bugar" ao andar e pular. A causa: as passadas de
nuvens e de luz no ar montavam a direção do olhar com um ponto a 0,1 bloco do olho, e o balanço
da câmera ao andar (que o Horizon põe na projeção) desloca o olho alguns centímetros; a direção
errava dezenas de graus a cada passo. Agora a direção é a diferença de dois pontos do mesmo raio,
imune ao balanço. As nuvens também deixaram o ruído preso à tela (que "andava" ao mexer a câmera)
por um preso às próprias nuvens, ganharam mais passos (24/36), somem no ar com a distância
mostrando o céu de verdade por trás, e à noite não brilham mais que o céu. Pontilhado de 1/255
contra faixas no céu escuro; a fumaça da fábrica não pinta o horizonte à noite.

**Noite (3.1.1):** o primeiro teste foi à noite e saiu escuro, lavado e roxo. A luz do céu à noite
não escurece mais o que o jogo já ilumina; o olho se adapta ao ar livre à noite (+55%) e as cores
desbotam um pouco, como na visão noturna de verdade; o céu noturno ficou azul-escuro em vez de
preto, com estrelas mais visíveis; a lua clareia mais neutra; os tons azuis da noite e do inverno
caíram para um terço; a vinheta caiu de 0,35 para 0,2.

## 3.0.1

O jogo recusava o pacote ("a pack may write 4"): os efeitos agora cabem em quatro alvos de pós.
Mesmo visual; o brilho passou a ser lido direto da imagem, em um quarto da tela.

## 3.0: o salto

- **Nuvens volumétricas:** cúmulos de verdade, entre 560 e 1.150 blocos de altura, com base
  escura, topo iluminado, bordas prateadas contra o sol e luz dourada no fim da tarde. Luz por
  dentro da nuvem (lei de Beer com efeito "powder" e espalhamento múltiplo aproximado). Meia tela.
- **Sombra das nuvens no chão:** manchas que andam com o vento sobre o terreno.
- **Profundidade nas texturas (parallax):** as frestas entre pedras, tijolos e tábuas afundam de
  verdade e mudam com o ângulo da câmera.
- **Sombras suaves:** disco de Poisson girado; no Alto a penumbra cresce com a distância até o
  que faz a sombra, como a do sol de verdade.
- **Água espelhada ligada por padrão.**
- **Cor de cinema:** hora dourada mais forte, curva de contraste maior, saturação e
  tonalização dividida (sombras frias, luzes quentes).

## 2.1: cara de jogo novo

- **Relevo das texturas:** perto da câmera, o detalhe de cada bloco vira relevo, e o sol de lado
  revela pedras, tábuas e rebites. Com um pacote de recursos LabPBR, usa os mapas de normal dele.
- **Sol mais forte, sombra mais azul:** a luz direta ganha peso contra a luz do céu, como numa
  tarde de verdade.
- **Sombra de cantos (oclusão de ambiente):** cantos, frestas, o pé das paredes e o chão embaixo
  das máquinas escurecem. Calculada em meia tela com as normais do relevo e ampliada sem halo.
- **Luz volumétrica:** o raio de visão atravessa o ar e pergunta ao mapa de sombra se o sol chega
  ali. Os feixes passam entre folhas, janelas e chaminés mesmo com o sol fora da tela, com
  neblina de manhã, no outono, na chuva e na fumaça da fábrica.
- **Reflexos na tela:** chão molhado depois da chuva, metal e vidro refletem lâmpadas, máquinas e
  céu que estão na imagem.
- **Água:** refração do fundo com as ondas, absorção do vermelho com a profundidade, cáusticas
  dançando no fundo raso, ondinhas de vento e o brilho do sol bem fino.
- **Nitidez:** um filtro que respeita o contraste deixa as texturas nítidas sem serrilhar.

## Níveis de qualidade

A opção **Qualidade** decide o custo de uma vez só:

| | Leve (8 GB) | Equilibrado (padrão) | Alto |
|---|---|---|---|
| Nuvens | 2 camadas | 4 camadas | 5 camadas |
| Brilho (bloom) | desligado | ligado | ligado |
| Luz colorida | desligada | ligada | ligada |
| Plantas e água balançando | desligadas | ligadas até 48 blocos | ligadas até 48 blocos |
| Relevo, sombra de cantos, reflexos na tela, refração | desligados | ligados (8 amostras de cantos, 12 passos de reflexo) | ligados (12 amostras, 24 passos) |
| Luz no ar | desligada | 8 passos pelo mapa de sombra | 16 passos |

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
- O pacote: sombra de cantos, luz no ar e reflexos rodam em meia resolução; brilho em um quarto. Relevo custa duas leituras de textura por pixel até 32 blocos. Nuvens analíticas no
  céu, sem volume; sem traçado de raios nem histórico temporal. No Leve nada disso roda.
- O `budget.gpu_ms` (4,5 ms no Equilibrado) é um objetivo, não uma medição. A meta de 8 GB considera o
  computador inteiro. Placa de vídeo, resolução e tamanho da fábrica pesam mais que a RAM.

## Desenvolvimento

Shader API 0.1 do Horizon. `python tools/check_shaders.py` compõe cada estágio com o prelúdio
da API e valida com o `naga` (instale com `cargo install naga-cli`). Também confere se cada
opção tem seu `override` e se as páginas citam opções que existem. Os arquivos `.wgsl` ficam
na raiz do pacote. A redistribuição deste shader original é permitida pela licença MIT incluída.
