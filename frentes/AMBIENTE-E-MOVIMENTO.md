# Ambiente sonoro e movimento mecânico

Continuação de `codex/machine-audio`, na branch
`codex/factory-ambience-motion`. Sem integração de outras sessões, publicação ou
mudança de versão. Conserva receitas, custos, velocidades de transporte e saves.

## Fundo e feedback

Esteiras comuns/rápidas, divisores e unificadores agora têm um ruído de rolagem
muito baixo. Movem-se e fazem som mesmo vazios; o volume vazio cai a 65% do
perfil. Estados de bloqueio e pausa interrompem a renovação. Isso corresponde ao
movimento das travessas e não depende de haver uma pilha visível.

Moinho, extratoras, geradores e drones também usam fundos de 2,4 segundos,
renovados a cada 48 ticks, com afinação fixa: evita oscilações de duração e
frequência entre segmentos. Há duas variações, fades de 160 ms, filtro de
agudos de 850–1.500 Hz e RMS de até aproximadamente 0,10 antes do volume do jogo.
Os perfis ficam entre 0,055 e 0,11. A intenção é um fundo macio, próximo de ASMR;
a qualidade subjetiva precisa ser ouvida com música, ambiente e outras máquinas
no jogo. Fades longos deixam uma respiração suave na emenda; não se trata de um
loop perfeito sem variação de amplitude.

Batidas, cortes e ruídos agrícolas foram reduzidos; conclusões tocam a 60% do
volume do perfil. Conclusões não são alertas. Estado 13 (avaria/desgaste) novo
emite um toque metálico curto com legenda “Máquina precisa de reparo”. Não há
sirene contínua, aviso por saída cheia ou aviso por espera normal de energia e
material. Avaria que já estava salva não dispara de novo ao carregar.

Até três fundos por ouvinte deixam espaço para um efeito pontual; a avaria pode
usar uma quinta voz. Permanecem os limites de duas admissões por dimensão/tick,
um representante por família/célula de 8 blocos e 256 registros globais. Avisos
têm cooldown de 400 ticks por célula e no máximo 128 registros. Avisos negados
pela cota não ficam numa fila atrasada. São limites de feedback, sem efeito na
produção. Não há novas buscas de mundo.

Todos os sons continuam posicionais, mono, com atenuação de 8 blocos e categoria
**Blocos**. Mais perto há mais volume; longe deixam de ser audíveis. Um segmento
já iniciado pode durar até 2,4 segundos depois de parar a máquina; os demais
efeitos têm até 0,9 segundo (menos de um segundo na afinação mínima). O Horizon
não oferece aqui um identificador de loop individual para interromper a instância.

Assets existentes CC0 de Kenney; nenhum pacote novo ou som do Minecraft foi
incorporado. São agora 15 eventos/30 arquivos. Origem e licenças em
[CREDITS.md](../CREDITS.md) e `third_party/audio/kenney/`.

## Animações

`motion.rs` contém ciclos de oito poses com interpolação suave no relógio de
renderização e no tick parcial. Não cria oito mensagens de rede nem oito
texturas por máquina. Giro contínuo usa o `spin` do GPU, que oferece mais poses
intermediárias que oito quadros fixos.

- Prensa: descida, permanência na compressão e retorno da cabeça.
- Extratoras: broca e aletas; rotação aumenta nas Mk.II e Mk.III.
- Montador/integrador: mesa rotativa, em sentidos distintos.
- Cortadora/silvicultora: lâmina de serra.
- Britador: dois rolos em sentidos opostos.
- Moinho: prato de moagem; semeadora: distribuidor com três pequenos copos.
- Colheitadeira: tambor com barras de coleta; rancho: grade do alimentador.
- Eletrônica: cabeça deslizante na mesa; cozinha: misturador.
- Fornos: grelhas com movimento e cor quente enquanto trabalham.
- Geradores: volante frontal, parado sem queima. A assinatura de sincronização
  agora inclui apenas a transição de queima, não cada tick do contador.
- Drone: quatro hélices com pares em sentidos opostos. O corpo interpola entre
  snapshots de progresso, limitando extrapolação a quatro ticks. Pausa/bloqueio
  congelam o movimento. `FlightVisualAt` é um novo campo cosmético opcional;
  mundos anteriores funcionam sem migração. Ele não governa entregas ou carga.
- Aerogerador/roda d'água: mantêm giro contínuo conforme vento/água; a rotação
  natural pode continuar com acumulador cheio.
- Esteiras: travessas/carga seguem o caminho, curvas e rampas com tick parcial.
  Braços: oito poses interpoladas na ida e volta, sem alterar tempo de entrega.

Armazéns, móveis, baterias e painéis sem peças mecânicas permanecem estáticos.
As peças novas recebem folgas nos hitboxes existentes, geradas pela mesma fonte
`machine_forms.py`, preservando um ponto de colocação e as quatro orientações.
Rotações continuam em cache no GPU. Prensa, eletrônica, grelhas e alimentador
ativos redesenham no CPU a cada quadro; medir o custo numa fábrica grande antes
de declarar desempenho final. Não há nova simulação de entidades por hélice.

## Verificação

- 162 testes offline passam; dois previews anteriores opcionais ignorados.
- O novo probe de desenho executa os callbacks de produção de 31 máquinas/peças,
  testa que se movem e param, e permite capturar oito poses de cada uma.
- Ciclos são contínuos entre ticks, transferência preserva extremos e ordem,
  voo tem limite de extrapolação e respeita pausa, inclusive na volta do relógio
  modular. Gerador sem combustível não anima apenas por ter energia guardada.
- Teste de vinte renovações de três fundos sem perder cadência, reserva de voz
  para efeito/avaria, esteira vazia, silêncio bloqueado e cooldown de avaria.
- 30 OGGs decodificados: duração, mono/Vorbis 22.050 Hz, sinal, fades e RMS;
  hashes/licenças/referências verificados. 507 JSONs conferidos, 34 formas e suas
  orientações/folgas conferidas.

Prévia de áudio: `python tools/preview_machine_audio.py`.
Prévia geométrica animada (Pillow, opcional):

```sh
VEIO_MOTION_PREVIEW="$PWD/.build/machine-motion.json" python tools/check_rust.py
python tools/preview_machine_motion.py
```

Saídas em `.build/machine-audio.html` e `.build/machine-motion.gif`. A prévia
usa as chamadas de desenho capturadas, mas não simula luz/texturas/culling do
Horizon. São oito poses para revisão, não uma gravação do jogo ou medição de FPS.

Ainda precisa da compilação pelo launcher e teste dentro do jogo: ouvir em
fábrica densa e tranquila, comparar aproximação/afastamento, mutar Blocos,
pausar/bloquear/reparar máquinas, conferir hélices nas quatro orientações e
medir fluidez/custo. Nenhuma publicação ou build jogável foi feita nesta etapa.
