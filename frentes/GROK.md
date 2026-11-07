# Ordem para Grok: desempenho e balanceamento

Leia primeiro [COORDENACAO.md](../COORDENACAO.md), [README.md](../README.md) e [CONTRIBUTING.md](../CONTRIBUTING.md). Esta é a frente atribuída pelo usuário. Parta da `main` atualizada e trabalhe em `grok/performance-balance`, com commits e PR.

## Objetivo

Deixar a simulação leve em fábricas maiores e ajustar produção, energia, logística e progressão para criar escolhas interessantes, com ferramentas de escala e pouco trabalho repetitivo. Identifique onde o código faz trabalho desnecessário e entregue correções sustentadas por evidência.

## Desempenho

1. Mapear os custos de esteiras, consultas a blocos, conexões e distribuição de energia, máquinas, agricultura, animais, poluição e desenho. Distinguir custos por tick, por atualização de topologia, por entidade e por quadro.
2. Usar medições reproduzíveis quando houver ambiente adequado. Registrar cenário, quantidade de máquinas/entidades, ferramenta, versão e antes/depois. Se só houver o substituto offline, limitar as conclusões à lógica e ao número de operações medido; não alegar FPS ou ganho no Horizon real.
3. Priorizar varreduras repetidas, alocações evitáveis, consultas redundantes, contenção de registros e trabalho em áreas descarregadas. Reutilizar registros e caches apenas com invalidação correta ao colocar, girar, remover, trocar de mundo ou descarregar objetos.
4. Preservar conservação de itens e energia, justiça entre entradas e rapidez de resposta perceptível. Uma otimização não pode congelar curvas, fios ou atribuições depois que o jogador muda a fábrica.

A atualização das curvas está em `logistics.rs`; postes em `power.rs`; produção em `production/`; poluição e defesa em módulos próprios. O Cloud está implementando trabalhadores, portanto evite uma refatoração geral de IA e combine alterações em registros ou agendamento com a interface dessa frente.

## Balanceamento

1. Levantar vazões, tempos de receita, consumo e geração de FE, capacidades, pureza de jazidas, módulos, custos de construção, valor de alimentos e recompensas dos projetos.
2. Identificar gargalos involuntários, ciclos que criam recursos sem custo pretendido, receitas sem utilidade, fontes de energia dominantes e custos que incentivem grind. Diferencie problemas demonstrados de hipóteses para teste pelo usuário.
3. Ajustar números em lotes pequenos, com exemplos de fábrica antes/depois. Dar utilidade a alternativas de energia, terreno, processamento e logística sem tornar uma única solução obrigatória.
4. Preparar parâmetros para a progressão jogador → trabalhador → coordenador. Custos e ritmos devem ser discutidos a partir do sistema entregue pelo Cloud; não implemente uma segunda versão de trabalhadores.

`tools/production.json` é a fonte dos dados industriais; regenere `native/api/src/production/data.rs` com `tools/build_production.py` quando alterar essa tabela. Outros números ainda estão em Rust. Atualize receitas, ajuda, guia e documentação afetados; não edite somente um arquivo gerado.

## Limites e entrega

- Não mude IDs, significado dos campos NBT, protocolo dos menus ou regras de save como efeito colateral de uma otimização. Mudanças necessárias devem ter migração e contrato explícito.
- Não remova funcionalidades nem reduza silenciosamente área, alcance, vazão ou frequência para declarar uma melhoria de desempenho. Explique qualquer troca perceptível e mantenha-a separada da correção técnica.
- Preserve o tutorial e o objetivo de escalar com ferramentas, em vez de exigir mais cliques ou coleta manual.
- Entregue o diagnóstico, as alterações concretas, a comparação reproduzível e as limitações num documento curto no PR. Registre riscos de gameplay e pontos de teste.
- Faça as verificações necessárias ao concluir cada lote; evite repetir testes de jogo a cada ajuste. Termine com commits e PR, sem incrementar a versão consolidada por conta própria.

O primeiro lote (custos de tick, esteiras estáveis, cabos ociosos e energia renovável) está em [GROK-LOTE.md](GROK-LOTE.md). O terceiro (postos da 0.11.0, comida e varredura) está em [GROK-LOTE3.md](GROK-LOTE3.md).
