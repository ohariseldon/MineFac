# Ordem para Cloud: aldeões trabalhadores

Leia primeiro [COORDENACAO.md](../COORDENACAO.md), [README.md](../README.md) e [CONTRIBUTING.md](../CONTRIBUTING.md). Esta é a frente de implementação atribuída pelo usuário. Parta da `main` atualizada e trabalhe em `cloud/villagers`, com commits e PR.

## Experiência desejada

No início, o jogador constrói as linhas, configura as máquinas e cuida pessoalmente do abastecimento e da operação que exigir intervenção. As máquinas industriais já processam receitas automaticamente quando recebem materiais e energia; preserve esse comportamento.

O avanço seguinte permite delegar rotinas a aldeões. Comece por um aldeão que trabalhe numa fazenda ou estação agrícola. Depois, o sistema deve comportar aldeões de outros setores, como metalurgia, e um coordenador que organize vários trabalhadores. A referência a jogos idle descreve a progressão de trabalho manual para delegação e coordenação.

O hub de trabalho e o papel exato de um coordenador precisam ser definidos na implementação. Não presuma que a central de contratos existente deva ser substituída.

## Entregas em etapas

1. Definir trabalhador, posto de trabalho, tarefa, atribuição e estados visíveis. Criar a base em um módulo próprio, por exemplo `native/api/src/workforce/`, com interfaces pequenas para produção, inventários e telas.
2. Entregar um ciclo completo de atribuir, trabalhar, aguardar e dispensar um aldeão numa atividade agrícola. Mostrar o posto e o motivo de espera: sem tarefa, sem insumo, sem energia, destino cheio, caminho obstruído ou trabalhador ausente.
3. Salvar atribuições e progresso. Tratar aldeão morto, removido, descarregado, sem acesso ao posto ou já atribuído a outro trabalho, além de posto removido e retorno ao mundo salvo.
4. Expandir por interfaces de tarefas para outros setores. Coordenadores devem organizar uma fila ou conjunto limitado de trabalhadores; a hierarquia futura não precisa estar completa para entregar a primeira fazenda funcional.
5. Integrar controles e tutorial: explicar o que continua automático, o que exige operação e o que o trabalhador passa a fazer. Documentar o ponto de desbloqueio e os custos para revisão com Grok.

## Regras de integração

A base comum de consulta de postos e validação de entregas está em [INTERFACE_POSTOS.md](INTERFACE_POSTOS.md) e `native/api/src/operations.rs`. Ela separa postos, transporte e infraestrutura e preserva os filtros reais de entrada. Use-a no adaptador de tarefas sem transformar os serviços sugeridos em ações automáticas indiscriminadas.

- Estender aldeões usando os ganchos existentes do Horizon. Preserve comércio e comportamentos vanilla fora das atribuições do Veio. Não faça uma reescrita geral da IA vanilla nem da produção inteira.
- A atribuição deve ser explícita, permitir remoção e impedir dupla atribuição do mesmo aldeão. Um coordenador não deve fazer a mesma tarefa concorrer entre dois trabalhadores.
- Decida e documente quais ações exigem presença física e caminho acessível. Não finja trabalho físico com teletransporte de itens, aldeões ou resultados fora do alcance definido.
- Transferências e conclusão de tarefas devem conservar itens e energia, inclusive quando o destino enche, a atribuição muda ou um chunk descarrega. Não produza progresso ilimitado fora de simulação sem uma regra explícita e aprovada na integração.
- Preserve IDs e saves existentes. Não desative globalmente máquinas ou fazendas antigas para obrigar a presença de aldeões; se uma transição exigir migração, implemente compatibilidade e explique a mudança.
- Reutilize as interfaces de `production/`, `layout`, `outputs` e `accepts`. Toque em `lib.rs`, `ranch.rs`, telas e geradores apenas nos pontos necessários. Registre mudanças nesses contratos para o Codex e o Grok.
- Evite procurar todos os aldeões, máquinas e blocos a cada tick. Use registros com identidade do mundo/dimensão, invalidação e atualização escalonada; combine o desenho com a frente de desempenho.

## Critérios da primeira entrega

Um jogador consegue atribuir e dispensar um aldeão; ele realiza uma tarefa agrícola concreta; o painel informa sua atividade e impedimentos; salvar e reabrir preserva a atribuição; falta de insumo ou destino cheio não perde nem duplica itens. Aldeões não atribuídos continuam com o comportamento vanilla.

Inclua verificações de atribuição exclusiva, conservação de itens, interrupção e restauração. Confira assinaturas novas contra a API real e atualize o substituto offline quando necessário. Registre limitações sem apresentar o substituto como teste no jogo. Termine com commits, PR e instruções curtas para teste pelo usuário.
