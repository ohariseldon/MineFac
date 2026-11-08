# Orçamento global de trabalho de fundo

Base: main `bffce28`, incluindo o lote 8 do Grok. Este lote introduz
`native/api/src/budget.rs` e conecta as tarefas existentes ao mesmo relógio de
simulação. Nenhuma mudança de arte, receita, custo de energia ou geração.

## Cotas por tick, compartilhadas entre dimensões

- Descoberta da área elétrica: 512 unidades; cada lote de um poste reserva oito.
  No máximo 64 lotes por tick no jogo inteiro, em vez de oito consultas por cada
  poste sem limite agregado. O lote final também reserva oito unidades.
- Agricultura: 384 unidades; colheitadeira e semeadora reservam 12 por tentativa,
  lâmpada oito e silvicultora 24. São pesos de trabalho por lote, não uma contagem
  exata de consultas nem milissegundos de CPU.
- Reconstrução das conexões de postes: 4.096 etapas por tick, em lotes de até 64
  por atendimento. Coleta, ranking e preparação dos fios agora são retomáveis;
  detalhes em [REDES-INCREMENTAIS.md](REDES-INCREMENTAIS.md).
- Coordenação: 2.048 etapas por tick, em lotes de até 64 por capataz. Coleta,
  descoberta de fontes e busca entre fontes e destinos são retomáveis. A fila
  anterior fica disponível até a publicação da próxima; detalhes em
  [CAPATAZES-INCREMENTAIS.md](CAPATAZES-INCREMENTAIS.md).
- Descoberta de alvos dos trabalhadores: 512 células por tick no conjunto dos
  postos. `scan_named` reserva o tamanho da sua etapa; se adiada, devolve a última
  lista completa e mantém o cursor.
- Busca dos carregadores por mesas/tarefas: 1.024 unidades por tick, em lotes
  de até 32. Usa cadastro por região e cursores; detalhes em
  [CARREGADORES-INCREMENTAIS.md](CARREGADORES-INCREMENTAIS.md). Renovar uma tarefa
  já assumida usa uma consulta direta limitada e não disputa essa cota de descoberta.

Cada categoria tem reserva própria. Um campo enorme não toma a cota dos fios ou
dos trabalhadores. As cotas valem para todos os blocos e dimensões que usam a
mesma simulação, não se renovam a cada bloco/chunk/dimensão e só são repostas no
gancho global `tick`. Valores centralizados em `LIMITS`, sem novo menu.

## Espera, justiça e continuidade

Quem não cabe na cota entra numa fila FIFO por categoria. Chamadas repetidas do
mesmo bloco renovam uma única entrada, sem acumular tarefas por tick atrasado.
Um trabalho novo não passa na frente dos blocos que já estão esperando.
Entradas sem novas tentativas expiram em três ticks nas buscas contínuas e em
21 ticks nas consultas de trabalhadores, chamadas em ciclos mais espaçados.
Isso libera a fila quando o bloco é quebrado, perde energia ou descarrega.

A fila guarda identidade e último contato; não guarda inventários, resultados
de transferências ou cópias do mundo. A memória cresce com os blocos pendentes,
não com o tempo de espera. Não há um teto de fila que torne os últimos blocos
permanentemente inelegíveis. A capacidade alocada pode reter seu pico histórico;
este lote não declara um limite fixo de RAM.

Postes persistem `TopologyPending`; a silvicultora persiste `TerrainPending` e
volta a tentar nos ticks seguintes sem perder uma tentativa por causa da fase de
cinco ticks. As outras buscas continuam dos seus próprios cursores. Não há
execução acelerada de todo o atraso quando uma cota libera espaço.

As últimas áreas e conexões completas permanecem ativas. Transferência de
energia, esteiras, braços, entregas já atribuídas e processamento de receitas
não passam pelo orçamento. A lista de alvos dos trabalhadores continua utilizável
quando a descoberta é adiada. Edições agrícolas já pagas continuam saindo pela
fila existente. A espera não retira itens ou FE.

Sob sobrecarga, reconhecimento de máquinas e disponibilidade de novas tarefas
demoram mais; a produção agrícola pode cair porque há menos tentativas de
trabalho. O limite anterior de aproximadamente três segundos para descoberta de
área não vale sob fila global saturada. A velocidade das esteiras e das receitas
não foi alterada para disfarçar esse efeito.

## Integração com as outras frentes

O lote 8 do Grok já está na main. A frente Cloud consultada não tinha commits
novos além da integração anterior. Economia/produtividade e moradia permanecem
nas branches próprias: não foram fundidas como parte deste orçamento.
A pesquisa `research/connected-veins` contém projeto e protótipo, sem implementação
de geração no runtime; também não foi integrada aqui.

Na atualização final também apareceram `cursor/grok-lan-c7d9` (entrada na vila
do anfitrião), `cursor/grok-noite-c7d9` (ataques noturnos e novos caches de alvos)
e a revisão de moradia `19a311e`. Foram conferidas separadamente. A noite altera
poluição e objetivos de mobs; precisa de medição própria e não deve ser anunciada
como coberta por estas cotas de fábrica. Nenhuma delas foi mesclada neste lote.

Para novas tarefas de manutenção, usar `budget::request` antes de consultar o
mundo ou avançar o cursor. Se negar, preservar o estado, consultar caches completos
quando possível e tentar novamente no ciclo adequado. Não chamar `begin_tick`
de uma máquina nem renovar cotas por dimensão. Não aplicar a cota a metade de
uma transação ou cobrar ingredientes antes da admissão. Novas categorias ou lotes
maiores exigem um peso compatível e teste de progresso, para não criar uma tarefa
que nunca caiba em sua reserva.

## Evidência e limites

Verificação em checkout isolado: 122 testes aprovados, duas ferramentas opcionais
ignoradas; 475 JSONs, 93 páginas e 58 texturas conferidos. Compilação de metadados
contra a API real já construída pelo launcher aprovada sem avisos. Sem recompilar
o executável completo nem abrir o jogo nesta entrega.

- Mil trabalhos concorrentes em ordem fixa: todos recebem atendimento; uso por
  tick nunca supera a cota e a diferença de atendimento fica em até um lote.
- Mil postes em uma descoberta completa: no máximo 512 leituras de área em cada
  tick, com as mesmas 74.000 células úteis visitadas e todas as varreduras concluídas
  dentro dos 220 ticks do cenário. Sem reduzir alcance ou abandonar blocos.
- Uma dimensão não renova a cota consumida pela outra; repetir `begin_tick` no
  mesmo tick não repõe saldo. Categorias independentes continuam elegíveis.
- Retentativas de 9.999 blocos não duplicam entradas; desaparecimento e recuo do
  relógio liberam as filas.
- Energia continua sendo entregue enquanto área e topologia aguardam admissão;
  o cenário conserva os 200 FE. Máquinas adiadas preservam energia, sementes e
  cursor e retomam quando a cota permite. Alvos antigos dos trabalhadores continuam
  disponíveis durante o adiamento da varredura.

É um orçamento global **das tarefas conectadas**, por quantidade de trabalho,
não um temporizador capaz de interromper qualquer código Rust. A reconstrução
individual de fios e a preparação das filas dos capatazes agora avançam por
etapas. A publicação de até oito tarefas usa uma etapa final limitada, com
validação dos inventários e preservação das reservas atuais.

Callbacks básicos por bloco, renderização, IA/pathfinding do Minecraft, difusão
da poluição e geração de terreno não estão sujeitos a estas cotas. Não há garantia
de FPS ou de que qualquer fábrica, de qualquer tamanho, caiba em um tick. O próximo
passo é medir o tempo de tick no Horizon real e o custo das demais tarefas.
