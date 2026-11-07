# Coordenação das três frentes

Orçamento compartilhado de manutenção da fábrica: ler
[frentes/ORCAMENTO-GLOBAL.md](frentes/ORCAMENTO-GLOBAL.md) ao acrescentar buscas,
varreduras de rede ou planejamento. O relógio global repõe as cotas; máquinas e
dimensões não devem repô-las. Transferências e transações de inventário continuam
fora desse adiamento.

Divisão atual definida pelo usuário após a integração da 0.9.1. Ela substitui a divisão anterior entre produção na nuvem e logística local. As ordens abaixo são trabalho futuro, não funcionalidades já implementadas.

## Responsáveis

- **Cloud — aldeões trabalhadores e coordenação:** implementar a passagem da operação pelo jogador para a delegação a aldeões, começando por fazendas e estações de trabalho, com coordenadores setoriais numa etapa posterior. Ordem completa: [frentes/CLOUD.md](frentes/CLOUD.md).
- **Grok — desempenho e balanceamento:** investigar custos reais de simulação, corrigir ineficiências e calibrar números e progressão com justificativas. Ordem completa: [frentes/GROK.md](frentes/GROK.md).
- **Codex local — expansão e integração:** continuar máquinas, funcionalidades, logística e suas conexões com o mundo do Minecraft; integrar as três frentes e preparar as versões jogáveis. Ordem completa: [frentes/CODEX.md](frentes/CODEX.md).

Geração e distribuição continuam sendo sistemas distintos no código. Essa separação técnica não representa mais uma atribuição exclusiva de toda a produção ao Cloud: a nova frente dele é o sistema de trabalhadores.

## Fluxo compartilhado

1. Partir da `main` atualizada. A base de código desta rodada é a **0.9.1**, commit `26599ecd7d8e94f9889a84da42b3928ad3156a9e`; este documento e as ordens acrescentam a coordenação da rodada.
2. Trabalhar em branches próprias: `cloud/villagers`, `grok/performance-balance` e `codex/expansion`. Ambos os agentes da nuvem podem commitar e publicar suas branches.
3. Usar commits pequenos e descrever alterações em arquivos compartilhados. Abrir PR para `main`; o Codex local integra e resolve os conflitos. Evitar commits concorrentes diretamente na `main`, substituição integral de arquivos compartilhados e qualquer push forçado.
4. Mudanças em contratos entre sistemas devem vir acompanhadas da interface proposta e de uma implementação compatível. Se houver dependência, deixar a integração explícita no PR, sem bloquear o trabalho independente.
5. Não incrementar versões nem regerar todos os assets por rotina. A versão consolidada e a compilação final ficam com a integração local. Quando um gerador mudar, incluir suas saídas relevantes no mesmo commit.
6. Ao concluir um lote, informar o que mudou, como usar, verificações realizadas, limitações e requisitos de migração. Separar teste offline, conferência da API real, compilação pelo launcher e teste dentro do jogo.

O usuário testa a versão jogável. Evitar recompilar ou abrir o jogo a cada iteração. Dados pessoais, mundos, executáveis e caches ficam fora do Git; os arquivos pesados locais permanecem no E:.

## Arquivos e interfaces

A preparação comum de postos está descrita em [frentes/INTERFACE_POSTOS.md](frentes/INTERFACE_POSTOS.md). Cloud deve usar essa consulta como ponto de integração para tarefas e inventários; ela não implementa atribuições nem substitui suas transações.

- `native/api/src/logistics.rs`: transporte atual e estado das esteiras. Ponto de entrada para os próximos trabalhos locais de itens.
- `native/api/src/power.rs`: postes e fios aéreos, mantidos localmente.
- `native/api/src/farm.rs`: colheitadeira; compartilhada quando trabalhadores ou novas funções agrícolas exigirem integração.
- `native/api/src/lib.rs`: reúne ganchos, registros e distribuição adjacente de energia. Receitas e jazidas estão em `production/`. Evite substituí-lo inteiro em uma entrega; envie alterações pequenas e descreva quais funções mudou.
- `native/api/src/workforce/`: trabalhadores (Cloud). Posto, atribuição exclusiva por UUID, ordens de caminhada, objetivo `Work` dos aldeões e tarefas por setor (`farmhand.rs`). Estados 18-23 do painel. Detalhes e contratos tocados em [TRABALHADORES.md](TRABALHADORES.md).
- `presentation.rs`, `tutorial.rs`, traduções e geradores de conteúdo são compartilhados. Preserve as entradas do outro lado ao acrescentar conteúdo.

O transporte consulta `output_slots` para descobrir quais slots uma máquina permite esvaziar. Produção define suas entradas, saídas e filtros em `layout`/`accepts`. O transporte entrega itens por `Host::insert_item` e desconta da origem somente a quantidade aceita. Receita, consumo de ingredientes e criação de produtos continuam na produção. Estoque cheio deve parar o fluxo sem destruir itens.

A energia compartilhada permanece em `energy` no NBT, em FE, respeitando `capacity`. Produtores adicionam energia; redes transferem; consumidores descontam seu custo. Não mude nomes de campos, capacidades ou regras de acesso sem explicar o impacto nos saves e na rede. Mantenha os IDs existentes.

## Mapa da produção recebida na 0.9

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

Curvas, rampas, indicação de bloqueio e unificador já foram entregues na 0.9.1. As prioridades seguintes de logística são planejamento:

1. Introduzir braços de transferência com origem/destino visíveis, reserva de energia e comportamento seguro quando o destino encher.
2. Evoluir filtros e prioridades de divisores e unificadores.
3. Revisar alcance, conexões, capacidade e indicação visual da rede elétrica existente.

As novas ferramentas devem facilitar escala e manutenção. O tutorial deve acompanhar cada entrega.

## Alternativa por ZIP se o acesso ao GitHub falhar

O fluxo principal agora é commit e PR. O intercâmbio por ZIP continua disponível como alternativa, com o SHA exato da base e uma lista de arquivos alterados e removidos.

O pacote 0.8 já contém a 0.7; não aplique a 0.7 novamente sobre ele. Os dois patches originais foram integrados em sequência, preservando autoria e remoções de arquivos antigos.

Para continuar na nuvem, use o pacote consolidado mais recente entregue pelo agente local. O ZIP de intercâmbio inclui `BASELINE.txt`, com o commit de referência. Mantenha esse arquivo junto à próxima entrega para permitir uma comparação de três versões e preservar trabalho local posterior.

Ao devolver trabalho, envie o código completo ou um patch, a referência usada, uma lista dos arquivos alterados/removidos e as verificações realizadas. Diferencie verificação com o substituto offline, verificação contra a API real, compilação pelo launcher e teste dentro do jogo. Não inclua contas, mundos, compiladores ou caches.

Faça as verificações necessárias ao concluir uma entrega. O usuário prefere testar os lotes no jogo e pediu para evitar testes repetidos a cada ajuste.
