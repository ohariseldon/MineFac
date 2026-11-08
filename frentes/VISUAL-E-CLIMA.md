# Visual Realismo 2.0 e clima

Pedido do Luiz: "a gente quer um visual bonito", com imersão no mundo e não só números, sem
pesar o PC de 8 GB. Esta frente cobre o pacote de shaders, o que o mod informa a ele e o clima
que afeta energia e lavoura.

## Pacote de shaders (`visuals/veio-industrial`, 2.0.0)

Céu próprio (atmosfera, sol com escurecimento da borda, estrelas, nuvens analíticas a 1.200
blocos), sombras, brilho e raios de sol em meia/um quarto de resolução, plantas e água
balançando, luz colorida de tochas, lava, fornos e partículas de fogo, reflexos opcionais,
folhagem por estação, fumaça da fábrica no céu e na névoa. Três níveis: Leve (8 GB),
Equilibrado e Alto. Detalhes e custo em `visuals/veio-industrial/README.md`.

Validação offline: `python tools/check_shaders.py` (precisa do `naga`, `cargo install naga-cli`).
Os prelúdios da Shader API 0.1 ficam em `tools/horizon_shader_0_1/`.

## 2.1: cara de jogo novo

Pedido seguinte do Luiz: "aprimorar o realismo visual... dar uma impressão de um Minecraft 2030".
Usa o que a Shader API 0.1 já oferece e o pacote ainda não usava:

- `set_surface` no material (normal com relevo, suavidade, 1 em folhagem) → `surface_at` no pós.
- `ao.wgsl` (meia tela; na 3.0.1 virou parte de `light.wgsl`): oclusão de ambiente por hemisfério, 8/12 amostras, ampliada no
  `finish.wgsl` pela distância de cada texel (alvos são Rgba16Float, então o g guarda a distância).
- `rays.wgsl`: luz volumétrica pelo `shadow_map`, 8/16 passos; sem sombras volta ao raio na tela.
- `ssr.wgsl` (meia tela): reflexo na tela em superfícies lisas ou molhadas, 12/24 passos e 3
  bissecções.
- Material: relevo procedural (ou LabPBR), sol mais forte contra sombra azulada, água com
  refração por `opaque_color`, absorção por canal e cáusticas.
- `finish.wgsl`: nitidez que respeita o contraste.

Tudo some no Leve. Nada disso foi visto no jogo ainda: só validado pelo `naga`.

## 3.0: "radical, excepcional, sem perder a essência"

Pedido do Luiz com fotos de referência (pôr do sol com nuvens grandes, lago espelhado):
`clouds.wgsl` (nuvens volumétricas em meia tela, 16/28 passos e 3/5 rumo ao sol; prévia em numpy
usada para acertar luz e forma), sombra das nuvens no material, parallax por luminância
(8/14 passos), PCF de Poisson com PCSS simples no Alto, reflexo planar da água ligado por padrão,
gradação de cinema. Custo alvo 7 ms no Equilibrado.

## 3.0.1: limite de 4 saídas de pós

No PC do Luiz o Horizon recusou o pacote inteiro: "6 [[post]] outputs; a pack may write 4" (a
2.1 tinha 6, a 3.0 tinha 7). Agora são 4 saídas e 5 passes:

- `blur_h.wgsl` → `glow_h` (1/4): limiar de brilho lido direto do `scene_color` (4 leituras por
  toque, cobrindo os 4×4 pixels) e meia gaussiana horizontal. `blur_v.wgsl` → `glow` (1/4).
- `light.wgsl` → `light` (1/2): r = oclusão de ambiente, g = distância, b = quanto o ar manda de
  sol. A cor dos raios sai em `finish.wgsl` (`veio_rays_colour`).
- `sky_mirror.wgsl` → `sky_mirror` (1/2): onde há céu, nuvens volumétricas (alfa 0..1); no chão,
  reflexo na tela (alfa 0..-1). `veio_sky_mirror` no `finish.wgsl` amplia à mão e só soma
  texels do tipo certo, para nuvem não vazar na borda do telhado.
- `tools/check_shaders.py` agora barra mais de 4 saídas e saídas repetidas.

O Horizon também recusa o pacote se sobrar um `.wgsl` que nenhum estágio usa ("ao.wgsl is not used by any
stage or in shared"). O launcher apaga os `.wgsl` da versão anterior antes de copiar, e o
`check_shaders.py` barra arquivo não usado.

## Pelo motor, sem depender do shader

Luiz: "mexer no motor do jogo... realismo mais integrado, de um jeito nativo" (visual). Funciona
também no visual Leve, sem pacote de shaders:

- `scenery.rs` (cliente): gancho `biome_colors` pinta grama, folhas, serrapilheira e água de cada
  bioma pela estação; `refresh_biome_colors` só quando a mistura das estações anda um quarto
  (~4 reconstruções por virada; cada uma custa alguns segundos dos workers do Horizon).
  Biomas nevados, quentes e de fora do Overworld não mudam. Gancho `render_fog`: neblina nas
  manhãs úmidas (outono e primavera, mais nos vales) e fumaça parda perto de fábricas.
- `winter.rs` (servidor): com chuva abaixo de 0 °C, neve cobre aos poucos o chão natural e as
  copas perto dos jogadores (6 colunas por jogador a cada meio segundo, num raio de 40), engrossa
  no frio forte e a água parada congela abaixo de −2 °C. Cada bloco que o Veio cobriu fica em
  `<mundo>/veio/inverno.txt`; no degelo (acima de 2 °C) só esses voltam ao que eram, então a
  neve dos biomas nevados do vanilla nunca é tocada. Biomas quentes ficam de fora pela cor da
  grama da coluna. Edições em lote com filtro (`only`): nunca sobrescreve o que o jogador pôs.
- O pacote de shaders recebe `frame.mods[1].x = 1` e então só varia a cor de árvore em árvore,
  sem aplicar a estação de novo; a chuva abaixo de zero vira granizo e neve branca.

Ganchos novos (`biome_colors`, `render_fog`, `refresh_biome_colors`) seguem o catálogo do
Horizon (`native-hooks.json`) e o stub offline, mas ainda não foram compilados contra o Horizon real.

## O que o mod manda (native/api/src/ambience.rs)

- `shader_params`: `frame.mods[0] = [fase do ano 0..4, temperatura/50, fumaça 0..1, 1]`.
- `visuals`: até 8 luzes coloridas das máquinas quentes ligadas a até 32 blocos da câmera,
  com oscilação de chama. O registro é alimentado no tick das máquinas, uma vez por segundo
  cada (com defasagem por posição), e esquece máquinas paradas há 3 s.
- Fumaça, vapor ou brasas saem das máquinas quentes a cada 2 s, só com jogador a até 40 blocos.
- Num servidor dedicado o cliente recebe só as partículas; as luzes dependem do registro no
  mesmo processo (um jogador).

## Clima (native/api/src/climate.rs)

- Ano de 4 estações de 6 dias (24 dias, ~8 h de jogo), contado de `day_time`.
- Temperatura: média da estação (15/25/13/1 °C, sem salto na virada) ± 5,5 °C no dia,
  −1 °C a cada 10 blocos acima de y 64, −3 °C na chuva; subsolo estável (~12 °C);
  Nether 48 °C, End −8 °C.
- Solar: ×1,18 no verão a ×0,62 no inverno e −0,4%/°C acima de 25 °C. Vento: ×0,85 a ×1,2.
  Rios (turbina): ×1,35 na primavera, ×0,75 no verão. Roda d'água congela abaixo de −3 °C.
- Aspersor: abaixo da temperatura mínima da cultura (trigo 3 °C … melão 15 °C) a lavoura para;
  nos 4 °C acima dela cresce na metade; com telhado (luz do céu < 15) vale 20 °C (estufa).
  Calor acima de 25 °C gasta +1 mB de água por bloco a cada 5 °C.
- As máquinas releem o clima a cada 2 s e guardam em `Climate`; o painel mostra
  "Estação · °C" no solar, vento, roda d'água, turbina e aspersor.

## Medição

`python tools/check_performance.py`: 8 cenários, verificações idênticas às de antes; os 7
antigos sem mudança de alocações ou leituras de NBT. Novo: `ambience_1000_hot_machines_second`
(mil geradores ligados, 3 quadros de luzes por tick) em ~40 µs por tick e 3 alocações pequenas
por tick.
