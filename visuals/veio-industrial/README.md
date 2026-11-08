# Veio — Realismo leve 1.1

Visual industrial original inspirado na direção de arte de fábricas ao ar livre:
luz quente e sombras frias discretas, céu natural, materiais com rugosidade e brilho
dependente do ângulo, chuva e água com profundidade. Mantém os modelos, texturas e regras do Veio. Não contém
código, imagens ou materiais de Satisfactory ou de outros pacotes de shaders.

O launcher oferece **Realismo leve — 8 GB** e **Leve — sem shaders** antes de
abrir o jogo. A escolha fica salva. `Escolher-Visual.cmd` abre essa escolha novamente;
`Jogar-Leve.cmd` permite iniciar sem carregar este pacote desde a primeira partida.
Também pode desligar o pacote em Opções > Vídeo > Shaders.

## Aparência

- O sol aquece a luz no começo e fim do dia; o luar usa a mesma sombra próxima com
  intensidade menor. Luz de tochas mantém sua contribuição sem criar fontes novas.
- Ferro e cobre têm brilho metálico; pintura tem brilho largo e discreto; vidro e
  gelo recebem destaques mais estreitos. Cobre oxidado mantém a pátina fosca.
- Superfícies voltadas para cima e expostas ao céu escurecem e ganham brilho na
  chuva. A umidade acompanha o tempo de secagem do Horizon. Folhagem fica sem essa
  camada brilhante e recebe luz transmitida suave quando iluminada por trás.
- Água rasa fica mais transparente; profundidade aumenta a cor e a absorção.
  O brilho do céu depende do ângulo. A ondulação de luz diminui com a distância
  para evitar cintilação; mergulhar mantém o tratamento subaquático do jogo.
- Uma névoa de distância suave dá profundidade à fábrica e à paisagem. O efeito
  considera a luz na câmera, chuva e efeitos de visão; os painéis próximos mantêm
  seu contraste. A suavização de bordas escolhe uma direção para preservar detalhes.

O brilho do céu é uma aproximação analítica. Não reproduz objetos refletidos do
cenário e não exige uma segunda câmera. A aparência ainda usa a geometria de blocos
do Minecraft; esta entrega desenvolve a direção visual industrial do Veio.

## Custo previsto

- Sombra de sol/lua de 1024 × 1024, alcance de 48 blocos e uma comparação bilinear por
  fragmento exterior. Esmaece antes do limite, evitando uma borda brusca.
- Um pós-processamento com cor, névoa e suavização espacial de cinco amostras.
- A água faz uma leitura da profundidade já copiada pelo renderizador para o
  material nativo. A névoa usa a profundidade fornecida ao pós-processamento.
- Nenhuma passagem de reflexão, luz volumétrica, bloom, SSAO, traçado de raios,
  subdivisão de geometria, histórico temporal ou textura adicional de recursos.
  Esta revisão não solicita novos alvos de renderização nem mapas de materiais.
- Nenhum código por tick, varredura da fábrica ou nova fonte dinâmica de luz.
- Sombras, água, metais, chuva, atmosfera e suavização podem ser reduzidos nas opções do pacote.
  Desligar Sombras elimina a passagem, além das consultas no material.
  A página Qualidade das sombras também permite 512 e alcance de 32 blocos.

O `budget.gpu_ms` é um objetivo, não uma medição ou um teto automático. A meta de
8 GB considera o computador inteiro; não reserva 8 GB para o jogo. O limite de
texturas do perfil é uma preferência do Horizon, não um limite de RAM total.
Placa gráfica, resolução e tamanho da fábrica também determinam o desempenho.
Esta revisão foi feita sem retomar os testes no jogo ou a comparação da fábrica
grande, conforme solicitado. A meta de 8 GB continua sem homologação completa.

Os ajustes iniciais controlam a distância de visão e simulação, os trabalhadores
de preparação de terreno e a resolução interna. Ajustes posteriores no jogo são
preservados nas próximas aberturas. O perfil só é reaplicado quando escolhido de
novo; as opções originais ficam em arquivos com sufixo `.antes-do-veio`.

## Desenvolvimento

Shader API 0.1 do Horizon. Valide com `horizon-shader check` e, para produzir um
`.hshader`, com `horizon-shader pack`. A versão em pasta funciona diretamente.
O perfil Leve não seleciona nem compila o pacote. A redistribuição deste shader
original é permitida pela licença MIT incluída.
