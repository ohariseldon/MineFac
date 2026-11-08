# Grande indústria

Pedido do Luiz: fechar as lacunas do Veio em relação a Factorio e Satisfactory, levantadas no
documento "Veio: lacunas em relação a jogos de fábrica". Para as razões, ele pediu os mecanismos
desses dois jogos. Para a fábrica longe do jogador, escolheu a âncora de chunk com custo de
energia. A vila operária (`native/api/src/workforce/`) não foi tocada.

## O que entrou

| Lacuna | Como ficou | Código |
|---|---|---|
| Progressão por volume | Laboratório de pesquisa com 4 pesquisas (128 a 256 peças cada) que liberam as receitas do ramo seguinte | `research.rs` |
| Cadeia mais funda | Motor, processador, núcleo de automação, fabricadora (3 entradas) e kit Mk.II | `tools/production.json`, `upgrades.rs` |
| Fábrica longe do jogador | Âncora de chunk: 32 FE/t, `forceload` do próprio chunk | `industry.rs` |
| Razões visíveis | Itens por minuto em cada receita do guia e acima de cada slot do painel | `presentation.rs`, `tools/build_tutorial.py` |
| Estatísticas e alertas | Monitor da fábrica: máquinas por estado, 9 itens com produção e gasto por minuto, aviso na barra a cada 30 s | `industry.rs` |
| Projeto final | Torre de transmissão em 4 fases na central de contratos, entrega acumulada | `contracts.rs` |
| Itens menores | Esteira expressa (10/s), túnel de esteira (até 6 blocos), prancheta (copiar configuração), lava nos canos | `logistics.rs`, `logistics_controls.rs`, `production/water.rs` |

Tudo é explicado no terceiro livro, o **Guia da Grande Indústria** (20 páginas), entregue ao
obter a primeira chapa de aço. O guia principal ficou com 100 páginas, o máximo.

## Razões (Satisfactory e Factorio)

- **Guia:** cada linha de receita mostra o ritmo de uma máquina sem módulo em itens por minuto,
  na entrada e na saída, e o ciclo em segundos: `40 Cobre bruto > 40 Chapa de cobre /min · 1,5s`.
- **Painel:** acima de cada slot, o que a máquina pede e entrega por minuto na receita ativa, já
  com módulo, Mk.II e bônus de produtividade. A receita ativa viaja no slot 8 dos dados do painel
  (`RecipeIndex`), livre nas máquinas de receita.
- **Fábrica inteira:** o monitor soma a produção e o gasto medidos de cada máquina, como as
  estatísticas de produção do Factorio. Consumo maior que a produção aparece em vermelho.

## Pesquisa

| Pesquisa | Pede | Libera |
|---|---|---|
| Metalurgia | 128 aço, 128 engrenagens | bateria, geotérmico, vapor, poste de aço, fazenda de árvores, rancho, pacotes de esteira e cabo, monitor, túnel |
| Eletrônica | 128 circuitos, 128 molduras | solar, broca II, Tesla, módulos de velocidade e eficiência, lâmpada, turbina hidrelétrica, âncora |
| Automação | 64 motores, 64 processadores | fabricadora, broca III |
| Escala | 32 núcleos de automação, 256 aço | esteira expressa |

Essas receitas saíram dos avanços de obter aço, circuitos e módulos (o quartel continua nos
circuitos, por ser da vila). O laboratório consome 1 item de cada entrada a cada 10 ticks com
24 FE/t. O progresso é do mundo, em `<mundo>/veio/pesquisa.txt`: vários laboratórios somam e
quebrar um não perde nada. Ao concluir, o servidor roda `advancement grant @a only
veio:research/<id>` (gatilho `minecraft:impossible`).

## Mk.II

Kit feito na fabricadora. Botão direito numa máquina de receita ou broca grava `Tier = 2`:
ciclo × 0,5, energia por tick × 2 (a mesma energia por produto) e poluição × 1,25, somados ao
módulo. Agachar com a chave tira o módulo primeiro, depois devolve o kit.

## Monitor e alertas

Cada máquina com energia (fora braços, Tesla, laboratório e âncora) se registra a cada 20 ticks,
espalhada pela posição, com estado e fluxos (`Rate` × receita ativa; brocas pelo minério do slot).
O registro esquece quem ficou 200 ticks sem notícia. O monitor resume a dimensão a cada 20 ticks.
Os 9 itens viajam compactados nos números do painel (índice de 7 bits, produção e gasto de
12 bits, até 4.095/min). Com máquina parada (saída cheia, sem energia ou desgaste), avisa na barra
de ação dos jogadores da dimensão com a posição de uma delas, no máximo a cada 600 ticks.

## Âncora de chunk

Com 32 FE/t, roda `forceload add` no próprio chunk (com `execute in` fora do Overworld). Sem
energia por 100 ticks, roda `forceload remove`. A lista de chunks forçados fica em
`<mundo>/veio/ancoras.txt`; um chunk cuja âncora não rodar em 400 ticks (bloco quebrado) é solto
pela manutenção do tick do mod. Só o servidor força chunks.

## Grande projeto

Depois dos 4 contratos, a central pede a torre de transmissão em 4 fases (estágios 4 a 7), antes
da exportação sem fim:

| Fase | Pede | Paga |
|---|---|---|
| 1 | 200 aço, 100 módulos | 32 esmeraldas, 4 módulos de velocidade |
| 2 | 300 circuitos, 100 motores | 48 esmeraldas, 4 de produtividade |
| 3 | 150 processadores, 50 núcleos | 64 esmeraldas, 8 kits Mk.II |
| 4 | 200 núcleos, 50 kits Mk.II | 2 estrelas do Nether, 2 sinalizadores |

Recolhe até 4 peças por tick de cada slot; o progresso fica em `Total` e `Info`. A última fase
concede `veio:grand_project`. Mundos que já exportavam entram na fase 1.

## Logística

- **Esteira expressa:** lote 4 a cada 8 ticks (10 itens/s), mesmas curvas e rampas.
- **Túnel de esteira:** dois na mesma direção, até 6 blocos. A entrada entrega direto à saída
  (2 por vez); os túneis alternam entrada e saída ao longo da linha.
- **Prancheta:** agachado num braço, divisor ou unificador copia filtro e prioridade; em pé cola.
  A cópia é uma só por mundo aberto.
- **Lava nos canos:** a bomba encostada em lava parada (sem água) puxa 5 mB/t por fonte. Cada cano
  leva um fluido (`Fluid`); o geotérmico aceita 1.000 mB como um balde, antes dos baldes do slot.

## Testes e o que conferir no jogo

Testes novos: `industry.rs` (monitor e âncora), `research.rs` (2), `contracts` no `tests.rs`
(grande projeto), túnel em `logistics/tests.rs`, prancheta em `logistics_controls.rs` e lava em
`production/water_tests.rs`. Conferir no jogo: o `forceload` pelo console do Horizon, a concessão
dos avanços de pesquisa, o desenho dos números acima dos slots e do monitor, e o túnel numa
linha real.
