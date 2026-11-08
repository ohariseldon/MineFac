# Veio — Realismo leve 1.0

Visual industrial original inspirado na direção de arte de fábricas ao ar livre:
luz quente e sombras frias discretas, céu natural, destaques largos nos metais e
água com ondulação de luz. Mantém os modelos, texturas e regras do Veio. Não contém
código, imagens ou materiais de Satisfactory ou de outros pacotes de shaders.

O launcher oferece **Realismo leve — 8 GB** e **Leve — sem shaders** antes de
abrir o jogo. A escolha fica salva. `Escolher-Visual.cmd` abre essa escolha novamente;
`Jogar-Leve.cmd` permite iniciar sem carregar este pacote desde a primeira partida.
Também pode desligar o pacote em Opções > Vídeo > Shaders.

## Custo previsto

- Sombra solar de 1024 × 1024, alcance de 48 blocos e uma comparação bilinear por
  fragmento exterior. Esmaece antes do limite, evitando uma borda brusca.
- Um pós-processamento com correção de cor e suavização espacial de cinco amostras.
- Nenhuma passagem de reflexão, luz volumétrica, bloom, SSAO, traçado de raios,
  subdivisão de geometria, histórico temporal ou textura adicional.
- Nenhum código por tick, varredura da fábrica ou nova fonte dinâmica de luz.
- Sombras, água, metais e suavização podem ser reduzidos nas opções do pacote.
  Desligar Sombras elimina a passagem, além das consultas no material.

O `budget.gpu_ms` é um objetivo, não uma medição ou um teto automático. A meta de
8 GB considera o computador inteiro; não reserva 8 GB para o jogo. O limite de
texturas do perfil é uma preferência do Horizon, não um limite de RAM total.
Placa gráfica, resolução e tamanho da fábrica também determinam o desempenho.

Os ajustes iniciais controlam a distância de visão e simulação, os trabalhadores
de preparação de terreno e a resolução interna. Ajustes posteriores no jogo são
preservados nas próximas aberturas. O perfil só é reaplicado quando escolhido de
novo; as opções originais ficam em arquivos com sufixo `.antes-do-veio`.

## Desenvolvimento

Shader API 0.1 do Horizon. Valide com `horizon-shader check` e, para produzir um
`.hshader`, com `horizon-shader pack`. A versão em pasta funciona diretamente.
O perfil Leve não seleciona nem compila o pacote. A redistribuição deste shader
original é permitida pela licença MIT incluída.
