# Interface de postos para trabalhadores e logística

Preparação da integração, acrescentada sobre a 0.9.1. O sistema de aldeões trabalhadores continua sendo implementado pelo Cloud. Esta interface não atribui aldeões, não exige mão de obra para máquinas antigas e não altera a execução de receitas.

## Consulta comum

`native/api/src/operations.rs` oferece:

- `profile(block)`: setor e tipo do bloco (`Workstation`, `Transport` ou `Infrastructure`). O tipo não é uma autorização de atribuição; o adaptador de tarefas deve dizer quais atividades implementa.
- `inspect(&entity) -> Option<StationView>`: estado da estação, motivo de espera, entradas e saídas com índices e referências às pilhas, energia quando aplicável e serviços que merecem inspeção.
- `accepts_material(&entity, slot, stack)`: compatibilidade de material com a entrada. Respeita o pedido atual da central e os filtros de produção. Não verifica espaço, distância nem acesso pela face.
- `accepts_delivery(host, event)`: validação ao entregar por uma face. É usada pelo gancho real de inserção e mantém o roteamento separado das entradas A/B do unificador.
- `advice_key(status)`: orientação traduzida para o jogador. O painel a mostra ao passar o cursor no estado da máquina; a ajuda também informa o setor.

As leituras usam o estado já publicado pela simulação. Energia vazia não é tratada automaticamente como falta de energia: uma máquina pode estar ociosa por outro motivo. Estados desconhecidos continuam desconhecidos e não viram uma ordem de trabalho inventada.

## Uso pelo Cloud

Obtenha uma entidade carregada pelos recursos reais do Horizon e consulte `operations::inspect`. Associe a consulta à identidade do mundo, dimensão e posição mantida no seu registro. O resultado tem referências emprestadas; não deve ser armazenado como estado persistente nem convertido numa cópia do inventário que depois sobrescreva a entidade.

Os serviços são oportunidades de inspeção, não tarefas executadas. `CheckInputs` requer descobrir os materiais e quantidades da receita ou tarefa real. `InspectPower`, `InspectResource` e `InspectEnvironment` não autorizam criar energia, jazidas, plantas ou animais. `InspectControl` não autoriza desligar a redstone do jogador. `CollectOutputs` só indica que existem produtos; um destino válido ainda precisa ser encontrado.

Antes de movimentar itens, confirme novamente existência do posto, atribuição, presença e caminho do trabalhador, face, compatibilidade e espaço. Use as operações reais de inventário do host e respeite o restante devolvido por inserção. Não desconte itens usando somente uma consulta anterior. A transação entre inventário do aldeão e estação ainda pertence à implementação de trabalhadores; esta interface não fornece uma transferência atômica entre dois inventários.

Esteiras, divisores e unificadores são `Transport`: a consulta não gera serviços de retirada para eles. Assim, um trabalhador genérico não passa a esvaziar arbitrariamente linhas em funcionamento. Infraestrutura também não vira posto de trabalho por ter energia ou um slot interno vazio.

## Pontos de integração

O Cloud pode implementar adaptadores de tarefa em `workforce/`, sem reescrever a simulação de produção. Grok pode revisar os custos da consulta e do agendamento. A consulta custa proporcionalmente aos slots de uma estação e não procura blocos, entidades ou aldeões ao redor. Ela não é executada por um novo laço global nem em todos os ticks.

Não há campos novos de NBT nem alteração nos 13 valores enviados pelos menus. Atribuição, exclusividade e progresso de tarefas deverão ter esquema próprio de persistência, definido pelo Cloud. Não reutilize `Status`, `Info` ou `Module` para representar um aldeão atribuído.

Ao registrar uma máquina nova, mantenha seus contratos em `layout`/`accepts` e inclua seu perfil em `operations::profile`. Atualize o setor e a ajuda quando necessário. Os textos das dicas são mantidos em `tools/build_operations.py`, chamado por `tools/build_tutorial.py`.

## Entrega e validação

Verificações específicas cobrem estoques de transporte fora das tarefas, filtro do pedido atual, saídas protegidas, consulta sem consumo, energia ociosa sem falha inventada e preservação do controle por redstone. Esta preparação não instala uma versão nova enquanto o usuário testa a 0.9.1; a compilação completa e as novas dicas entram no próximo lote consolidado.
