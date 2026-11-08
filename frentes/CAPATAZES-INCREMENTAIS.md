# Planejamento incremental dos capatazes

Base: main `cf37004`, com fios incrementais e orçamento global já integrados.
O planejador da fila passa a avançar em `workforce/foreman/planning.rs`.

## Trabalho dividido

- Coleta um bloco da rede por etapa. Copia ID, inventário, estágio do contrato do
  HUB e filtro de exportação dos terminais da frente paralela de transportes;
  não copia energia, caminhos, edições pendentes ou todo o NBT.
- Separa máquinas e armazéns durante a coleta, preservando a ordem original de
  cada grupo. Não ordena a rede inteira ao terminar.
- Examina uma saída por etapa para montar as fontes. Fontes já reservadas na
  fila inicial não entram de novo; ItemStacks compartilham o identificador do item.
- Examina uma combinação fonte/entrada por etapa. Guarda os cursores de bloco,
  slot e fonte para continuar, sem repetir a busca inteira no próximo tick.
- Fontes usadas são marcadas, sem remover elementos e deslocar o vetor inteiro.
  A mesa continua tendo prioridade; armazéns seguem depois das máquinas e não
  enviam para outros armazéns. Materiais e slots respeitam o contrato atual.

Cada atendimento tem até 64 etapas. A categoria global `Planning` agora reserva
2.048 etapas por tick entre todos os capatazes e dimensões, até 32 atendimentos
completos. Um lote que termina cedo também reserva 64 unidades. A descoberta da
rede continua usando sua cota separada de varredura dos trabalhadores.

O início continua na cadência de quatro ciclos do capataz. Enquanto há um plano
parcial, a continuação ocorre também entre os ciclos de dez ticks, sem executar
a IA inteira de novo, cobrar comida adicional ou registrar a mesa a cada frame.
Sem trabalhador atribuído ou com ausência detectada, a continuação para. Troca
de trabalhador invalida o plano parcial. Durante um plano longo, a descoberta
de novos blocos dessa mesa aguarda a conclusão do plano atual.

## Publicação e carregadores em andamento

A fila antiga e a lista de postos continuam disponíveis durante a preparação.
Novas propostas ainda não têm número definitivo nem reservam itens reais.
Publicar usa uma cauda limitada a 17 unidades: até oito propostas, com até duas
leituras de inventário por proposta, e a publicação. Se o atendimento não tem
essas unidades restantes, a cauda inteira espera o próximo atendimento.

A cauda roda numa única chamada, sem atravessar ticks. Ela relê as tarefas
atualmente marcadas por carregadores, mantendo seus donos e IDs. Tarefas que
foram concluídas durante a preparação não são recriadas a partir do snapshot
antigo. Uma proposta que conflita com uma reserva atual de origem ou destino
é descartada. Reservas de entregas já em andamento também são respeitadas.

Antes de adicionar cada proposta, conferir novamente produto, quantidade,
slots de saída/entrada, espaço, compatibilidade, estágio do HUB e regra de
armazém para armazém. Se o estoque diminuiu, reduzir a quantidade; se a entrega
não serve mais, descartá-la. Os IDs são atribuídos usando o contador atual da
mesa, não um contador capturado vários ticks antes. Ao concluir, publicar fila,
postos e contadores juntos. Não se retira produto, energia ou comida durante
o planejamento. Os carregadores continuam validando suas transferências reais.

Em sobrecarga, novas recomendações podem chegar mais tarde ou uma fila conter
menos propostas após as validações. Não há recuperação de todo o tempo atrasado
num pico de execução. O processamento de receitas, o transporte elétrico e as
esteiras não foram alterados.

## Save, memória e integração

`PlanningPending` é um campo opcional da mesa; a fila existente mantém seu formato.
Os snapshots e cursores do plano ficam em memória e não aumentam o save com
uma cópia de toda a rede. Depois de carregar, um plano pendente recomeça da área
conhecida, mantendo a fila anterior. Um rodízio examina no máximo oito registros
por tick global para recolher trabalhos abandonados. A memória continua crescendo
com a quantidade de planos e seus recortes, sem garantia de RAM constante.

O orçamento mede unidades de trabalho, não milissegundos de CPU. Clonar um
inventário, alocar memória e percorrer as reservas globais ainda tem custo.
Consultas de contratação, descoberta de mesas pelos carregadores e a IA vanilla
não foram transformadas em etapas por este lote.

Ao ampliar `operations::accepts_material`, manter o snapshot de `Node::new`
compatível: na main o HUB exige NBT para essa decisão; a frente de transportes
também usa `FreightExport`, já preservado nos tipos de terminal conhecidos. Se
um novo filtro depender de outro campo, copiá-lo explicitamente e testar a equivalência.
Se aumentar `MAX_QUEUE`, ajustar a cauda e o quantum: uma asserção de compilação
impede uma publicação maior que seu atendimento.

## Verificação

- Resultados comparados com o planejador anterior em fábricas estáveis, com
  estágios distintos de contrato, pastor, fontes em armazém e prioridade das máquinas.
- 64 capatazes em redes de 95 blocos incompatíveis: todos terminam; no máximo
  64 etapas por atendimento e 2.048 etapas/leituras de entidades por tick global.
  São 6.080 leituras de coleta, sem reler fontes e destinos durante a busca longa.
- A fila antiga permanece até a conclusão. Uma publicação sem unidades suficientes
  não consulta o mundo nem modifica a fila.
- Reserva adquirida durante a preparação preservada; reserva concluída não
  reaparece. Estoque reduzido limita o lote; contrato alterado descarta a proposta.
- Mudança de trabalhador reinicia o plano; limpeza de memória permite recomeçar
  usando o estado salvo. As regressões de transporte e de fios continuam cobertas.

O planejador inteiro anterior permanece apenas sob `cfg(test)`, como oráculo e
para o benchmark histórico `foreman_95_node_queue`. Esse benchmark não mede o
novo agendador. As contagens acima são verificações offline, sem promessa de FPS.

Resultado final: 132 testes aprovados, duas ferramentas opcionais ignoradas;
conferência de 475 JSONs, 93 páginas e 58 texturas aprovada. A checagem de tipos
com a API real do launcher passou usando `libhorizon_client-3e68bc59c9e837cc.rmeta`.
Sem reconstrução completa do executável ou teste de FPS no jogo nesta entrega.

## Frentes acompanhadas

A atualização inicial trouxe `cursor/grok-quartel-c7d9` (`a6d2744`), com quartel,
milícia e defesa. Foram conferidas as alterações de `workforce/mod.rs`: acrescentam
um posto e rotinas de combate, sem mudar o planejador de capatazes. A integração
futura deve preservar a continuação entre ciclos e a manutenção global deste lote.
Não aplicar a cota de planejamento no meio de uma ação de combate ou entrega.

Transportes entre zonas (`90721ca`), economia, moradia, noite e pesquisa de jazidas
permanecem em suas branches; não foram fundidos nesta otimização. O trabalho
local de tutorial, mapa, launcher e integração de mundos foi preservado.

Na atualização final, transportes avançou para `8b8e8d9`, com caravanas, e apareceu
`codex/machine-identity` (`d396f69`), com modelos/colisões e ajustes de navegação.
Foram conferidas as diferenças compartilhadas. Os terminais passam sua configuração
de exportação a `accepts_material`; o snapshot deste planejador preserva esse campo
para a integração futura. Um teste verifica essa preservação sem copiar NBT alheio.
Essas branches continuam separadas: não se declara validada aqui sua integração completa.
