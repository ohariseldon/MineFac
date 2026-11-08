# Otimização da simulação — novembro de 2026

Este lote otimiza a base consolidada 0.15.0 (`e60dee3`). Mantém receitas,
velocidades, capacidades, alcances, ordem de atendimento e formato dos saves.
Não muda arte, modelos ou interface do jogador.

## Mudanças implementadas

- Atualização numérica do NBT modifica o valor existente, sem reconstruir a chave
  UTF-16 a cada mudança. Campos frequentes usam chaves estáticas e deixam de
  disputar o mutex do cache de nomes raros.
- Cabos e postes não montam dados de uma tela que não possuem. Centro de controle
  e marcos de zona publicam seus 13 valores diretamente em arrays fixos,
  preservando os índices e os valores negativos das metas especiais.
- Publicação dos filtros verifica o texto emprestado, sem criar uma String para
  descobrir se há filtro. A transferência continua usando o mesmo filtro.
- Transporte retorna antes de montar o layout fora de sua fase de transferência.
  A cadência original de oito ticks e a alternância entre posições permanecem.
- Rede elétrica percorre as listas de área e fios diretamente no NBT. Só aloca uma
  nova lista de offsets quando o conteúdo muda. Preserva a rotação do primeiro
  consumidor atendido, a conservação de energia e o atendimento único de cada fio.
- Conexões de postes verificam a posição no ranking dos seis mais próximos,
  interrompendo a busca ao encontrar seis candidatos melhores. Evitam construir
  e ordenar uma lista para cada vizinho; conservam distâncias, desempates por
  posição e a união simétrica das escolhas dos dois postes.
- Planejamento do coordenador reutiliza ItemStacks dos materiais disponíveis,
  compartilhando o identificador do item. Só cria a String persistida quando
  emite uma tarefa. Materiais incompatíveis são rejeitados antes de montar o
  layout de slots; os limites dos slots e o contrato atual continuam obrigatórios.
- A chave NBT é derivada do tipo público `Compound`: a API real do Horizon não
  reexporta `JavaString`, embora o substituto offline o faça. Isso corrige a
  incompatibilidade de importação sem exigir alteração no motor.

## Medição reproduzível

Nota posterior: a reconstrução em execução passou a usar etapas em
`power/rebuild.rs`; o cenário `dense_pole_links` deste benchmark histórico mantém
o resolvedor anterior como referência de equivalência. Ele não mede o agendador
incremental. Os limites e testes do novo caminho estão em
[frentes/REDES-INCREMENTAIS.md](frentes/REDES-INCREMENTAIS.md).

O mesmo vale para `foreman_95_node_queue`: ele mantém o planejador anterior
somente como referência histórica sob `cfg(test)`. O caminho de execução foi
substituído pelas etapas descritas em
[frentes/CAPATAZES-INCREMENTAIS.md](frentes/CAPATAZES-INCREMENTAIS.md).

Referência instrumentada: `b4c09cf`, sem as otimizações de execução deste lote.
Código otimizado medido: `e4166a6`. Os relatórios guardam os SHAs completos em
[performance/before.json](performance/before.json) e
[performance/after.json](performance/after.json).

São medianas de três execuções em release com Rust 1.99.1, Windows x86_64 GNU,
no mesmo computador. Cada cenário aquece quatro chamadas antes da contagem.
As contagens são por cenário inteiro, não por bloco ou tick. Resultados:

- Publicação de 2.000 cabos por 20 ticks: 480.000 consultas de chave eliminadas.
  Já não havia alocações nesse cenário; o ganho vem de retirar trabalho sem uso.
- 20.000 publicações de um braço com filtro: 100.000 alocações passam a zero.
- 20.000 atualizações das métricas de uma prensa: 40.001 alocações passam a uma
  (20 bytes), correspondente à primeira inserção de um campo da amostra.
- 30.000 rejeições de pedra numa prensa: 60.000 alocações passam a zero.
- Rede com 256 postes, 40 ticks, topologia em cache: 88.789 alocações passam a zero.
- 100 cálculos de conexões com 80 postes vizinhos: 96.500 alocações passam a 1.500,
  redução de 98,4%; bytes alocados passam de 72.112.800 para 1.024.800.
- 40 atualizações de uma fila sobre 95 estruturas: 16.248 alocações passam a
  10.038, redução de 38,2%. A fila ainda tem alocações para snapshots e tarefas.

Os tempos também caíram nos sete cenários; os valores em nanossegundos estão
nos JSONs. São simulações offline com contadores, sem renderização, acesso real
a chunks ou IA completa. Não medem FPS, RAM residente, mundo inteiro ou limite
de estruturas. `allocated_bytes` representa tráfego de alocação, não memória
simultaneamente ocupada. Não somar os cenários para estimar o custo de uma fábrica.

As duas versões produziram os mesmos controles de trabalho:
`power_energy=129500 callbacks=33792` e `queue_tasks=8 source_reads=1045`.
Eles são controles resumidos; a equivalência de ordenação, slots, contratos,
inventários e atendimento também é coberta pela suíte de regressão.

Com Rust e o linker no PATH, em um checkout limpo da revisão desejada:

```powershell
python tools/check_performance.py --output .build/performance.json
```

O comando executa os sete cenários três vezes e falha se os controles diferirem
entre amostras. Para comparar versões, executar separadamente em cada revisão.
O alocador e os contadores ficam sob `cfg(test)` e não entram no jogo.

## Verificações desta entrega

Em checkout isolado de `e4166a6`, sem as alterações locais de tutorial:

- `python tools/check_rust.py`: 113 testes passaram; duas ferramentas de prévia e
  medição ficam ignoradas por padrão. O benchmark foi executado separadamente.
- `python tools/check_project.py`: 475 JSONs, 93 páginas e 58 texturas conferidos.
- `tools/check_native_api.py`: compilação de metadados contra a API real já
  construída pelo launcher (`libhorizon_client-e6f5e808dcf5c228.rmeta`), sem avisos.

Os novos testes comparam as conexões com o algoritmo anterior em grades de
densidades e tiers diferentes, incluindo empates; verificam rotação da área,
arrays vazios/incompletos, semântica numérica dos saves e os filtros de material
em 40 tipos de bloco. Outro teste exige zero alocações e zero acessos ao mutex
de nomes raros em publicações e rede estáveis após aquecimento.

Para conferir novamente a API real sem reconstruir o Horizon:

```powershell
python tools/check_native_api.py --deps CAMINHO_DOS_DEPS --rustc CAMINHO_DO_RUSTC
```

Usar os metadados e o compilador da mesma compilação do launcher. Essa checagem
valida tipos, não linkedição nem funcionamento dentro do jogo. Não foi feita uma
nova compilação completa do executável nem medição de FPS nesta entrega.

## Próxima medição no jogo

Medir tempo de simulação e memória numa fábrica salva, com chunks carregados e
quantidades registradas de máquinas, postes e trabalhadores. Avaliar separadamente
fábrica ociosa, produção contínua e reconstrução da rede. A fila de trabalhadores,
buscas no mundo, caminhos da IA, sincronização e renderização ainda precisam dessa
medição antes de se prometer capacidade ou escolher a próxima otimização.
