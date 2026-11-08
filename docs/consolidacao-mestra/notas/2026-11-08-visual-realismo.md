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
