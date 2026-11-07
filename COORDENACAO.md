# Produção e logística

Divisão definida pelo usuário após a entrega dos pacotes 0.7 e 0.8.

## Responsáveis

- **Agente da nuvem — produção:** extratores, processamento, construtores, receitas industriais, geração de energia (combustível, solar e outras fontes), alimentos, fazendas, plantações e balanceamento dessas cadeias.
- **Agente local — logística:** esteiras, transporte de itens, braços que transferem itens, divisores (splitters), unificadores (mergers/uniters), postes, transmissores, fios e cabos de distribuição de energia.
- **Integração combinada:** armazenamento de energia, interfaces de inventário e energia, registros, menus, tutorial, contratos, defesa e poluição. Não foram atribuídos integralmente a um dos lados pelo usuário. Alterações que cruzem essas áreas precisam explicar a interface afetada.

Geração de energia pertence à produção; distribuição pertence à logística. Os postes recebidos na 0.8 são preservados e passam à manutenção local.

## Arquivos e interfaces

- `native/api/src/logistics.rs`: transporte atual e estado das esteiras. Ponto de entrada para os próximos trabalhos locais de itens.
- `native/api/src/power.rs`: postes e fios aéreos, mantidos localmente.
- `native/api/src/farm.rs`: agricultura, mantida pela nuvem.
- `native/api/src/lib.rs`: ainda reúne receitas, geração, cabos e registros compartilhados. Evite substituí-lo inteiro em uma entrega; envie alterações pequenas e descreva quais funções mudou. A geração e os cabos ainda compartilham `tick_power`.
- `presentation.rs`, `tutorial.rs`, traduções e geradores de conteúdo são compartilhados. Preserve as entradas do outro lado ao acrescentar conteúdo.

O transporte consulta `output_slots` para descobrir quais slots uma máquina permite esvaziar. Produção define suas entradas, saídas e filtros em `layout`/`accepts`. O transporte entrega itens por `Host::insert_item` e desconta da origem somente a quantidade aceita. Receita, consumo de ingredientes e criação de produtos continuam na produção. Estoque cheio deve parar o fluxo sem destruir itens.

A energia compartilhada permanece em `energy` no NBT, em FE, respeitando `capacity`. Produtores adicionam energia; redes transferem; consumidores descontam seu custo. Não mude nomes de campos, capacidades ou regras de acesso sem explicar o impacto nos saves e na rede. Mantenha os IDs existentes.

## Próximas entregas locais de logística

Esta lista é planejamento; não representa funcionalidades já implementadas.

1. Melhorar a leitura das entradas, saídas e direção das esteiras, incluindo curvas e indicação de bloqueio.
2. Introduzir um unificador com entradas explícitas, saída única e atendimento alternado para evitar que uma linha monopolize a outra.
3. Introduzir braços de transferência com origem/destino visíveis, reserva de energia e comportamento seguro quando o destino encher.
4. Evoluir filtros e prioridades de divisores e unificadores.
5. Revisar alcance, conexões, capacidade e indicação visual da rede elétrica existente.

As novas ferramentas devem facilitar escala e manutenção. O tutorial deve acompanhar cada entrega.

## Troca por ZIP sem GitHub

O pacote 0.8 já contém a 0.7; não aplique a 0.7 novamente sobre ele. Os dois patches originais foram integrados em sequência, preservando autoria e remoções de arquivos antigos.

Para continuar na nuvem, use o pacote consolidado mais recente entregue pelo agente local. O ZIP de intercâmbio inclui `BASELINE.txt`, com o commit de referência. Mantenha esse arquivo junto à próxima entrega para permitir uma comparação de três versões e preservar trabalho local posterior.

Ao devolver trabalho, envie o código completo ou um patch, a referência usada, uma lista dos arquivos alterados/removidos e as verificações realizadas. Diferencie verificação com o substituto offline, verificação contra a API real, compilação pelo launcher e teste dentro do jogo. Não inclua contas, mundos, compiladores ou caches.

Faça as verificações necessárias ao concluir uma entrega. O usuário prefere testar os lotes no jogo e pediu para evitar testes repetidos a cada ajuste.
