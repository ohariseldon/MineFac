# Ordem para Codex local: expansão e integração

Leia [COORDENACAO.md](../COORDENACAO.md). Esta frente continua a expansão das funcionalidades, máquinas e relações entre sistemas, fortemente influenciada por Factorio e Satisfactory e integrada ao mundo procedural do Minecraft.

## Escopo

- Continuar logística: braços de transferência, filtros, prioridades, transporte e ferramentas para montar fábricas em escala.
- Expandir máquinas e cadeias quando elas criarem usos e escolhas concretas no mundo: recursos do terreno, redstone, energia, agricultura, povoados, Nether, armazenamento e projetos de infraestrutura.
- Integrar cada novidade com visual legível, controles, estados de espera, receitas, tutorial e progressão. Preservar curvas, rampas, unificador e os recursos industriais recebidos na 0.9.
- Criar tarefas e pontos de integração que o sistema de trabalhadores do Cloud possa consumir sem duplicar sua IA ou seu mecanismo de atribuição.
- Receber as melhorias de Grok, resolver conflitos e preparar versões consolidadas na `main`.

## Ordem inicial de trabalho

1. Incorporar o feedback do usuário ao testar a 0.9.1. Priorizar falhas que impeçam construir ou entender uma fábrica.
2. Entregar braços de transferência com origem e destino visíveis, consumo de energia, conservação de itens e estado de bloqueio.
3. Evoluir filtros, prioridades e ferramentas de escala. Expandir produção com objetivos úteis, articulando os parâmetros com o balanceamento do Grok.
4. Integrar o primeiro trabalhador agrícola e, depois, as tarefas de outros setores, conforme os contratos entregues pelo Cloud.

Essa lista é planejamento. Braços e trabalhadores ainda não estão implementados na base 0.9.1.

## Integração e versões

Trabalhar em `codex/expansion` durante entregas paralelas. Manter alterações em arquivos compartilhados pequenas e identificar dependências entre PRs. Integrar sem sobrescrever trabalho dos outros agentes e preservar a autoria dos commits.

Após consolidar um lote, atualizar versões e documentação, executar as verificações adequadas e fazer uma compilação final pelo launcher. Confirmar que a instância e a `main` correspondem à entrega. Deixar o teste dentro do jogo com o usuário, salvo pedido diferente. Não recompilar a cada iteração.

Todos os downloads, caches, builds e instâncias locais devem permanecer no E:. Contas e mundos não entram no Git. Mudanças em saves, IDs ou contratos entre sistemas precisam ser tratadas de forma explícita antes da instalação.
