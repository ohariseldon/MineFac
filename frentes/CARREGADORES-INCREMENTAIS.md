# Busca incremental de mesas e tarefas

Iniciada sobre `a5f0eb5` (0.16.1); integrada com a main `280bd3b` (0.17.2) antes
da publicação. Mantém as otimizações de NBT, ticks, desenho e esteiras da nuvem.

## Caminho novo

O cadastro antigo guardava todas as mesas num vetor. Cada renovação removia
entradas e criava uma String; cada busca percorria esse vetor, copiava a lista
de mesas próximas e reconstruía filas completas ao marcar uma tarefa.

O cadastro agora usa uma árvore ordenada por dimensão, chunk e posição. Atualizar
uma mesa modifica seu horário no próprio registro. A busca consulta somente os
chunks que podem tocar o raio de 16 blocos, sem montar um vetor de candidatos.
Cada etapa avança uma entrada do cadastro, uma região vazia ou uma tarefa da mesa.
Os cursores continuam no atendimento seguinte.

`HaulSearch` reserva 1.024 unidades por tick global, entre todas as dimensões,
em atendimentos de até 32 unidades. Mesas distantes não geram consultas ao mundo
nem uma passagem linear pelo cadastro inteiro. As consultas ordenadas ainda têm
custo logarítmico no tamanho do cadastro; não é uma promessa de CPU constante.

Na seleção final há uma seção limitada a nove unidades: uma consulta à mesa e
verificação de até oito tarefas. Ela confere novamente ID, dono, validade da
reserva e alcance. Uma tarefa já assumida pelo carregador naquela mesa tem
prioridade sobre uma nova. Se a fila mudou de ordem, o ID protege contra pegar
a tarefa errada. Marcar/renovar altera apenas `by` e `seen` no NBT; não reconstrói
a fila e preserva campos adicionais de outros sistemas.

## Entregas em andamento

Uma referência direta guarda mesa, ID e índice da tarefa atual. Renovar a tarefa
usa uma consulta à mesa, primeiro pelo índice conhecido; se ele mudou, examina
no máximo as oito entradas do protocolo para localizar o mesmo ID. Esse caminho
não disputa a cota de descoberta, evitando que buscas ociosas façam uma reserva
ativa expirar. Ele não assume a tarefa de outro carregador.

O posto persiste `HaulTaskDesk` e `HaulTaskId`. Depois de reabrir, o carregador
pode restaurar a tarefa mesmo antes do primeiro registro da mesa, mas somente
se a fila real ainda confirmar que a reserva é dele e está válida. Uma tarefa
concluída não reaparece a partir dessa referência. Saves anteriores continuam
fazendo uma busca normal, sem esses campos opcionais.

Tarefas novas exigem uma mesa recentemente ativa. Uma tarefa já assumida pode
continuar sendo renovada mesmo se o capataz parou; a renovação confirma a reserva
na mesa real. Não se copia nem se cria carga durante a busca. A coleta e a entrega
dos itens continuam nos caminhos existentes, com suas validações e limites.

## Justiça, descanso e memória

Depois de uma entrega, o cursor começa após a última mesa utilizada e completa
a volta pela região. Isso cria rodízio entre mesas próximas, em vez de favorecer
sempre a primeira do cadastro. Dentro da mesa, mantém a prioridade da reserva
própria e da primeira tarefa livre observada.

Busca incompleta não inicia uma viagem própria em paralelo: mantém a prioridade
da fila do capataz. Ela continua entre os ciclos normais, enquanto houver
trabalhador presente, sem carga a entregar e fora dos estados de descanso/fome.
`HaulSearchPending` guarda apenas essa indicação. Um resultado vazio é reutilizado
por dez ticks, até aproximadamente meio segundo a 20 TPS, antes de procurar de novo.

Cadastro e cursores têm limpeza em rodízio: cada tick examina até oito registros
de cada estrutura, sem limpar tudo numa passada. Memória depende das mesas e
carregadores conhecidos, não do número de retentativas. Cursores e índices ficam
em memória; apenas a referência pequena da tarefa vai para o save.

## Verificações

Na base final 0.17.2: 152 testes aprovados e cinco ferramentas opcionais ignoradas;
484 JSONs, 93 páginas e 59 texturas conferidos. A checagem com a API real ficou
limitada pelo cache de compilação incompatível do launcher (`E0463` ao carregar
`horizon_client`); não foi reconstruído o motor para substituir esse cache.
Sem compilação completa do executável nem medição de FPS nesta entrega.

- 64 carregadores procurando entre 225 mesas próximas: todos concluem a busca,
  sem superar 32 unidades por atendimento nem 1.024 unidades/consultas por tick.
- Dez mil mesas distantes: zero consultas ao mundo e até dez etapas na busca local
  vazia. Repetir dentro do intervalo de cache não executa nova varredura.
- Reserva atual renovada com uma consulta mesmo com a cota de descoberta esgotada;
  campos adicionais da tarefa preservados.
- Troca de dono e mudança do índice da tarefa conferidas antes da marcação; rodízio
  entre três mesas verificado sem duplicar reservas.
- Retomada de uma referência salva e rejeição de referência a tarefa concluída.
- Mesa substituída deixa o cadastro; limpeza tem limite por rodada. Busca em segundo
  plano não interrompe descanso, fome nem carga em entrega.

São testes offline de lógica e limites de trabalho. Reservas já ativas têm custo
próprio fora da cota de descoberta; movimentação, contratação e pathfinding do
Minecraft não foram transformados em etapas por este lote. Não há promessa de FPS
ou de custo total constante para qualquer quantidade de trabalhadores.

## Frentes acompanhadas

A frente Cloud publicou e integrou 0.16.1, 0.17.0, 0.17.1 e 0.17.2 durante o lote,
incluindo carga nas mãos, quartel/defesa e otimizações de desempenho. A base final
incorpora essas mudanças com seu histórico. O conflito na leitura de `Queue` foi
resolvido mantendo o decodificador compartilhado, as chaves prontas e os campos
originais. O hash rápido do orçamento e as mudanças de desenho foram preservados.

Transportes/caravanas e identidade visual continuam em branches independentes.
O trabalho local de launcher, tutorial e mundos não foi incluído nem sobrescrito.

Novos chamadores devem usar `foreman::search_task` e distinguir `Pending` de
`Empty`: apenas uma busca concluída sem resultado permite iniciar o planejamento
local alternativo. O caminho direto de renovação não deve ser adiado pela cota
de descoberta, pois isso faria uma entrega ativa perder sua reserva.
