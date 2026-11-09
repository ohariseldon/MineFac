# 2026-11-08 · Realismo visual (PR #51)

Nota curta de mudança. Entra no próximo compilado da consolidação mestra (SIS-PES / visual).

| Commit | O que mudou | Por quê |
|---|---|---|
| f85b918 | Pacote `veio-industrial` 3.0.1: 4 saídas de pós (`glow_h`, `glow`, `light` = AO + raios, `sky_mirror` = nuvens no céu e reflexos no chão). `check_shaders.py` barra mais de 4 saídas. | O Horizon recusou o pacote no PC do Luiz: "6 [[post]] outputs; a pack may write 4". |
| 9eeccba | O launcher apaga os `.wgsl` da versão anterior antes de copiar; `check_shaders.py` recusa `.wgsl` que nenhum estágio usa. | O Horizon recusou de novo: "ao.wgsl is not used by any stage or in shared". O #47 também passou a refazer a pasta do zero. |

## Teste no jogo (Luiz, 2026-11-08, RTX 3050, Equilibrado, resolutionScale 85)

- A 3.0.1 carrega.
- Água: "perfeita". Sombras: "muito boas". Ficam como estão.
- Céu: "completamente quebrado". Brilho, contraste e iluminação: "esquisitos". A diferença geral parece pequena e as texturas parecem iguais.
- Direção do Luiz: em vez de inventar, refazer com técnicas conhecidas e comprovadas, bem documentadas. Pode usar pacotes de texturas de terceiros.

## 3.1.0 (próximo commit desta nota)

- `sky.wgsl`: céu físico (Rayleigh, Mie, ozônio; coeficientes de Hillaire 2020/Bruneton), 8/12/16
  passos por pixel e 2/4 rumo ao sol, exposição 20 e ACES (Narkowicz), depois gama 2,2. Sol
  com o disco atenuado pelo ar. Faixa baixa misturada à névoa do jogo para não haver costura.
  Prévia numpy: `/mnt/project-files/visual/previa-ceu-fisico.png` (meio-dia, 25°, 6°, 1°, -4°).
- `blur_h.wgsl`: o céu só brilha acima do branco; no chão o joelho subiu para 0,72–0,95.
- `finish.wgsl`: contraste 0,26 → 0,10, calor dourado ×2,1 → ×1, saturação 0,16 → 0,10, sem
  tonalização dividida, bloom 0,55 → 0,45.
- Pacote de texturas indicado (não incluído): Vanilla Normals Renewed (LabPBR, uso gratuito).
- Água e sombras intactas, porque o Luiz aprovou as duas.
- Não visto no jogo ainda: falta o Luiz testar.

## 3.1.0 no jogo (capturas do Luiz, 30 JPGs, todas à noite)

Em `/mnt/project-files/visual/capturas/3.1.0/` (e na branch `claude/capturas-visual-3.1`). Céu preto
sem estrelas, imagem escura e lavada (pixel mais claro ~80/255), neve cinza-roxa, faixa
castanho-magenta no horizonte (fumaça da fábrica + névoa), árvores distantes borradas, alguns
quadrados escuros no chão (provável artefato do mapa de sombra; não corrigido ainda). O Luiz
reforçou: realismo de "parecer real", e o melhor pacote de texturas já dentro do pacote.

## 3.1.1 (commit desta seção)

- Noite: céu da noite não escurece o que o jogo ilumina (`material.wgsl`), adaptação do olho ao
  ar livre à noite e visão noturna (`finish.wgsl`), céu noturno azul-escuro e estrelas mais
  visíveis (`sky.wgsl`), lua mais neutra (`common.wgsl`), tons azuis de noite e inverno a 1/3,
  vinheta 0,35 → 0,2.
- Texturas: `tools/build_realistic_textures.py` baixa PBR JG 1.1.0 do Modrinth (SHA-512 conferido), reduz
  256x → 128x, tira a água e grava `visuals/texturas/Veio-Texturas-JG-128x.zip` (93 MB, fora do
  git). Licença CC BY-NC-SA 4.0, crédito em `CREDITS.md` e dentro do zip. Escolha: entre os
  pacotes realistas com PBR que permitem redistribuir, foi o de melhor aparência; os mais
  famosos (Patrix, Optimum Realism, Prime's HD, RealSource) proíbem redistribuir.
- Launcher (`launcher/src/visuals.rs`, revisão de perfil 3): no Realismo copia o pacote para
  `resourcepacks/`, liga por último em `resourcePacks` e `incompatibleResourcePacks` e sobe
  `textureBudgetMiB` para 1024; no Leve remove. Teste novo no launcher.
- Falta: capturas de dia e do pôr do sol; quadrados escuros da sombra; árvores borradas ao longe
  (talvez o LOD do Horizon, não o shader).

## 3.1.2 (commit desta seção)

O Luiz explicou o defeito principal: "o céu tá com uma referência do player; quando ele se mexe ou
pula, o céu inteiro buga". O estágio `sky` do Horizon recebe só a direção do olhar
(`horizon_sky_view_ray`, conferido no código do Horizon), então o céu em si não depende da
posição. O que dependia era a passada `sky_mirror` (nuvens volumétricas em meia tela): nas
capturas clareadas, as "árvores" borradas do horizonte são cúmulos dela, e as faixas onduladas
do céu também. Ela agora vem desligada por padrão (`volumetric_clouds = false`), com as nuvens
planas do `sky.wgsl` de volta. A imagem final ganhou pontilhado triangular de 1/255 contra as
faixas de 8 bits no céu escuro, e a fumaça da fábrica não pinta o horizonte à noite. Sem vídeo
do defeito, a causa é a mais provável, não confirmada.

## 3.1.3 (commit desta seção): causa achada, nuvens religadas

O Luiz não quis as nuvens desligadas: "é muito mais fácil só consertar". Causa: `sky_mirror.wgsl`
e `light.wgsl` faziam `normalize(view_position(uv, 0.5))`. Com profundidade invertida e infinita,
0,5 é um ponto a 2 planos próximos (~0,1 bloco) do olho, e o Horizon põe o balanço da câmera
(`frame.bob`) dentro de `frame.projection` (`native_post.rs`, `world_projection(...) * bob * spin`).
A translação do balanço (centímetros) virava dezenas de graus de erro na direção. Correção: a
direção é `view_position(uv, 1e-6) - view_position(uv, 1.0)`, dois pontos do mesmo raio, e a
translação se cancela. Também: jitter das nuvens preso ao mundo (não à tela), 24/36 passos em até
3.000 blocos, saída pré-multiplicada com desbotamento por distância (o céu real aparece por trás),
luz ambiente das nuvens à noite sem o piso de 0,03. Nuvens volumétricas ligadas de novo.

## 3.2.0 (commit desta seção)

Pedido do Luiz: shaders "super realistas" como os melhores gratuitos, brilho e contraste que
dependam de onde o jogador está, nuvens menos serrilhadas, sempre leve (8 GB).

- Exposição automática por `eye_light_smooth` (técnica do BSL/Complementary): até 2× em caverna e
  à noite, 1× de dia, em luz linear (`finish.wgsl`). Substitui os dois ajustes antigos.
- Nuvens: deslocamento de passos por ruído suave em vez de hash por célula; ampliação 3×3 tenda.
- Shaders de terceiros: o Horizon roda packs Iris/OptiFine (README do Horizon; a spec dele cita
  BSL 10.1.8, Complementary Reimagined r5.9.3 e Sildur's). Licenças: Complementary e Photon
  "custom", BSL e Bliss "all rights reserved"; nenhum pode ir dentro do nosso zip. Plano: o
  launcher baixa do Modrinth no PC do jogador. Pedido de teste no PC do Luiz (Complementary
  Reimagined, Photon, BSL: carrega? FPS? capturas) antes de ligar isso no launcher.

## Shaders Iris otimizados (commit desta seção)

O Luiz não quer só baixar BSL e companhia ("o BSL é muito foda, mas é muito pesado"); quer os
shaders otimizados para o projeto. Ele afirmou que temos licença para baixar. Não foi preciso
mexer no código deles: o que pesa está nas opções, e opções são uso normal. Em
`visuals/iris-presets/` há um ajuste "Veio 8 GB" para Complementary Reimagined r5.9.3, BSL 10.1.8 e
Photon v1.4, que liga os mapas LabPBR das nossas texturas e corta o que mais custa (sombra
grande, luz colorida, TAA, GTAO, SSRT, POM com 40 amostras). Os nomes de opção foram conferidos nos
próprios packs. Próximo: medir FPS (padrão × ajuste) no PC do Luiz e o launcher baixar e ligar o
vencedor.

## 3.2.1 (commit desta seção): menos reflexo, identidade do Minecraft

Luiz (23:45): o ambiente "ficou bom, não precisa esquentar a cabeça"; alguns objetos refletem
sem necessidade (preocupação com custo quando há muita coisa na tela); minérios e plantações
devem ficar como no Minecraft, pacote de texturas só para o básico (areia, grama...); perdeu um
pouco a identidade do Minecraft.

- Reflexo na tela só a partir de lisura 0,45 (metal polido, vidro, chão molhado), antes 0,12, que
  pegava pedra e madeira comuns com os mapas do pacote; mistura 0,75 → 0,55.
- Pacote de texturas: só `textures/block/` do cenário. Ficam originais: minérios, plantações
  (trigo, cenoura, batata, beterraba, caules, cacau, fungo do Nether, frutas, flor-tocha,
  jarro), itens, criaturas, interface, partículas e água. 66 MB.
- Próximas frentes do Luiz (loop de jogo, tutorial, aldeões, forma em cruz) são de outras
  threads; não entram aqui.

## 3.2.2 (commit desta seção): texturas com o brilho do Minecraft

Luiz (23:59, com print): "ta meio ruim ainda". No print: grama verde-oliva escura, morro de pedra
lavado, nuvens de chuva carvão, folhas com chuvisco de céu. Os riscos diagonais claros à direita
são a chuva do Minecraft começando (vanilla), não defeito.

- Causa principal medida: o PBR JG é calibrado para shaders PBR, que acendem a cena bem mais.
  Com a luz do jogo, comparado ao Minecraft 26.3 (média de brilho): pedra 57%, pedregulho 57%,
  grama 76%, terra 71%, cascalho 44%, andesito 49%.
- `tools/build_realistic_textures.py` agora mede o brilho de cada textura original no client.jar
  do 26.3 (baixado da Mojang, conferido por SHA-1; só as médias, nada da Mojang entra no zip) e
  aplica uma curva de luz em linear até a média bater (sem estourar os claros). Texturas com
  desenho diferente do original (mais de 6x de diferença, ex.: caldeirão) ficam como estão.
- Folhas voltam ao original: os furos finos do 256x viravam chuvisco de céu a distância.
- Nuvens com chuva: cinza da neblina da chuva (0,95, antes 0,75) e o sol ainda passa nas partes
  finas (corte 65%, antes 80%).

## 3.2.3 (commit desta seção): nuvens claras, morro sem véu, lado de bloco visível

Print do Luiz com a 3.2.2 (00:09): grama melhor; morro lavado e azulado, nuvens escuras e
pesadas, lado de terra dos blocos quase preto.

- Nuvens: luz espalhada dentro da nuvem 0,35 → 0,6, céu de cima mais forte e luz que volta do
  chão iluminado (as bases de cúmulo reais são cinza-claro, não ardósia).
- Raios de sol sobre o terreno a 55% (olhando para o sol, lavavam o morro de branco); no céu
  continuam iguais.
- Névoa de distância começa mais longe (48 blocos ou 35% da distância de visão), no máximo 22%
  (antes 30%) e menos azul.
- Faces de costas para o sol: luz do céu 0,66 → 0,74 (de dia); a terra do lado mostra a textura.

## 3.2.4 (commit desta seção): fim da linha branca em volta das coisas

Luiz (02:31): "tudo meio brilhante", branco no canto esquerdo, "uma linha brilhante, praticamente
neon branca, ao redor de todos os objetos".

- Causa da linha (vista ampliando o print da 3.2.2): as nuvens são calculadas em meia tela, e um
  texel cujo centro caía num bloco ou numa folha de grama calculava reflexo em vez de nuvem. O
  céu ao lado (1 a 2 pixels na borda de cada silhueta e entre as folhas da grama) ficava sem
  nuvem e mostrava o céu limpo, claro, por baixo de nuvens escuras. Agora o texel calcula nuvem
  se qualquer pixel ao redor for céu; o reflexo só perde a borda do céu.
- Nitidez 0,9 → 0,5 (a mais forte deixava um contorno claro em blocos e folhas).
- Canto branco perto do sol: lóbulo de frente da nuvem menos pontudo (g 0,65 → 0,55) e luz das
  nuvens acima de 0,8 comprimida, em vez de estourar em branco.

## 3.2.5 (commit desta seção): claridade e contraste

Luiz (02:36): "a claridade, o contraste não tá legal. Não tá 100%."

Medido nos prints dele (só o mundo, sem céu): preto em 25/255 (névoa levantando o preto), cinza
médio em 74/255 e saturação mediana 0,25: imagem lavada e escura ao mesmo tempo.

- Níveis como um fotógrafo faria: ponto de preto 0,035 (0,014 à noite), meios-tons levantados
  (expoente 0,80 na luminância), curva S 0,10 → 0,35, saturação +0,10 → +0,20. Tudo escala com
  a opção "Cor e clima da imagem" (valor padrão 0,65 = ajuste completo). Simulado nos dois prints: cinza médio 74 → 88, preto mais limpo
  (comparação em /mnt/project-files/visual/comparacao-contraste-3.2.5.png).

## 3.3.0 e launcher 0.17.0 (commit desta seção): BSL no launcher e controles na tela

Luiz (02:39): "não mudou praticamente nada... Não tem como você só usar o BSL mais adaptado? Ou
colocar um jeito de eu configurar na minha tela essas questões?" (as versões 3.2.4 e 3.2.5 ainda
não foram montadas no PC dele; o teste no PC está pausado).

- Launcher: terceira opção, **Realismo máximo** (`--visual maximo`). Baixa o BSL 10.1.8 do
  Modrinth no PC de quem joga (nunca dentro do zip do Veio), confere o SHA-512 publicado (SHA-512
  próprio no launcher, sem dependências; download pelo curl que vem no Windows 10 e 11), põe o
  ajuste "Veio 8 GB" (`visuals/iris-presets/BSL_v10.1.8.zip.txt`) e escolhe
  `pickedShaderPack:BSL_v10.1.8.zip`. O pacote de texturas do Veio continua ligado (o ajuste
  liga os mapas LabPBR no BSL). Conferido no código do Horizon: pack clássico é escolhido pelo nome
  do zip e as opções ficam em `shaderpacks/<zip>.txt`, as mesmas que a tela Opções do Shader grava.
  O ajuste do Veio só é copiado se a pessoa não tiver mudado nada, ou se ela escolher o perfil de
  novo. Sem internet ou com hash errado, abre com o Realismo leve.
- Pacote Veio 3.3.0: Brilho, Contraste e Saturação na página Imagem (Opções > Shaders), 1 = ajuste
  do Veio.
- O BSL já tem na própria tela exposição, curvas de tom, saturação e vibração (página Cor).
