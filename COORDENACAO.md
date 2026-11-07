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

## Produção (agente da nuvem)

- `native/api/src/production/`: extração (`extraction.rs`), motor de receitas (`mod.rs`), geração (`generation.rs`), semeadora e lâmpada (`crops.rs`), silvicultura (`forestry.rs`), rancho e registro de animais e aldeões (`ranch.rs`), armazém do povoado (`pantry.rs`) e testes (`tests.rs`).
- `native/api/src/production/data.rs` é gerado por `tools/build_production.py` a partir de `tools/production.json` (receitas, jazidas, combustíveis e valores de comida). Não edite o `.rs` à mão.
- `native/api/src/farm.rs`: colheitadeira, agora também cana, cacto, bambu, abóbora e melancia.

Interfaces que a produção oferece ao resto do código (todas em `production/mod.rs`): `is_machine`, `consumes`, `generates`, `burns`, `layout`, `outputs`, `accepts`, `tick`, `emission`, `accepts_modules`, `screen_slots`, `roles`, `panel_lines`, `shows_progress`, `draw`.

Pontos de contato em arquivos compartilhados, mantidos pequenos:

- `lib.rs`: as listas `is_factory_block`, `needs_power`, `is_power_block`, `layout`, `output_slots` e `accepts` consultam a produção no fim. As receitas e jazidas saíram de `tick_factory`, que agora chama `production::tick`. Em `tick_power`, só o trecho de geração virou `production::generation::produce`; a distribuição está intacta. O estado sem energia vem de `production::generation::empty_status`.
- `lib.rs`: nova função `is_generator(block)`, usada por `power.rs` no lugar de `matches!(… GENERATOR | SOLAR | GEOTHERMAL)`. Novas fontes de energia entram por ela; a rede só precisa puxar de quem ela aponta.
- `lib.rs`: `entity_tick_pre` também chama `production::ranch::observe`; `block_entity_render_bounding_box` também chama `production::generation::bounds`; `visual_signature` inclui o NBT `Info`.
- `presentation.rs` e `tutorial.rs`: consultam `production::screen_slots`, `panel_lines`, `roles` e `draw` antes das regras antigas. Os estados 14 a 17 (`water`, `wind`, `seeds`, `animals`) foram acrescentados ao fim de `STATUS_KEYS`. O slot de dados 12 mostra o NBT `Info` das máquinas de produção (pureza, FE/t gerado, animais, aldeões); na torre Tesla segue sendo a poluição local.
- `pollution.rs`: máquinas novas emitem por `production::emission`; quem queima combustível polui só enquanto há queima (`production::burns`).
- `upgrades.rs`: módulos valem também em `production::accepts_modules`.
- `defense.rs`: nova `clock()` só de leitura, usada pelo registro de animais.
- `tools/build_tutorial.py`: uma página por máquina, índices por categoria e páginas de receitas geradas do `production.json`. As entradas de logística foram preservadas na categoria "Logística".

Itens que entram e saem das máquinas novas seguem `layout`/`outputs`, então esteiras, divisores e futuros braços funcionam sem casos especiais.

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
