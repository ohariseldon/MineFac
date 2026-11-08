# Consolidação — candidata em integração

Aguardando o encerramento de todas as frentes antes de gerar a build única final. A release 0.16.1 anterior não contém as entregas recebidas depois de seu empacotamento.

## Entradas preservadas

- Base fornecida `E:/minefac-0.16.0.zip`: bundle verificado, commit `3cd563d096f67d20728732391a329348b4340f3a`.
- `13758bd7859602b50bd7028377035fe005806d5d`: planejamento incremental dos capatazes, cota compartilhada e continuidade da fila anterior durante o recálculo.
- `fd2a44602577ec9abf0688b7bb1f5174b3d0d2d1`: casa do fundador, reinício correto dos estados temporários e checkpoint do mapa.
- Tutorial local: oito objetivos observados em jogo, persistência por jogador/mundo e transportes por esteiras, braços, funis e carregadores.
- Realismo opcional, opção sem shaders e identidade visual das máquinas.
- `MineFac-progressao-fases.zip`: os dois patches foram aplicados sobre sua base `cf37004`, preservando autoria, e a branch resultante foi fundida à consolidada.
- `MineFac-quadro-da-fabrica.zip`: pacote substituto de fases aplicado em branch própria sobre `6f8a64b`; inclui quadro, licenças, pedidos do dia e censo.
- Cloud inicialmente em `a5f0eb5` / `7208324`: aldeões guiados pelo cérebro vanilla, descanso e carga visível na mão.
- Realismo 1.1: `3106849` e `2bc62ec`, incluídos por merge; não exige recompilar código nativo.
- `MineFac-megafabricas.zip`: importado sobre `a5f0eb5`. Os quatro patches anteriores foram confirmados equivalentes por `git range-diff`; o quinto acrescenta copiadora, marco, elevador e escadas (`e0ee148` na branch importada).
- Cloud até `5217b62`, incluindo `494d3e7` e `604369d`: combate baseado em vida real, plantão do quartel, otimização de ticks e painéis, sincronia de esteiras e busca incremental de tarefas dos carregadores.
- Branches remotas: todas as pontas conferidas são ancestrais desta build. Lista de nomes e hashes em `frentes/branches-consolidadas.json`.

## Decisões de integração

A casa, o sono noturno dos trabalhadores, os quartos da vila, quartel, contratos industriais, terminais, drones e caravanas ficam ativos. O relógio roda somente no servidor; o renascimento mantém a fila adiada corrigida pela revisão da 0.16.0.

O lote 2 antigo do Grok foi registrado por merge sem reaplicar seu código: lotes posteriores já substituem suas otimizações. A documentação histórica foi recuperada. As outras branches foram integradas por ancestralidade ou merge efetivo.

A progressão por fases substitui os tempos anteriores da prensa e do montador: uma jazida normal alimenta uma prensa; duas linhas alimentam um montador de engrenagens. As receitas da prensa seguem o último pacote fornecido, inclusive ouro e alternativas. Os contratos e recompensas da 0.16.0 permanecem. Avanços já obtidos em mundos existentes não são revogados. No encerramento do tutorial, a mensagem aponta para as 32 engrenagens e explica a razão da linha.

Os contadores de produção da 0.16.0 contam produtos, incluindo produtividade. O tutorial usa ciclos efetivamente concluídos para certificar o consumo automático e não gasta créditos extras pelo produto bônus. O estado de descanso noturno foi preservado; os estados de caravana foram movidos para índices distintos.

Os guias foram regenerados após a fusão, com paginação adicional no sumário de logística. A validação de tamanho cobre os quatro livros. O tutorial principal continua na tela; livros são consulta opcional.

O mapa finito em `work-in-progress/finite-world` é apenas checkpoint de desenvolvimento, fora do manifesto ativo. Jazidas conectadas estão ativas; geração de um mapa finito garantido ainda não está implementada nesta build.

## Verificações

- Conteúdo: 558 JSON, traduções e 68 texturas 16×16; quatro livros com ajuste de página.
- Formas de máquinas: 34 registros e 27 silhuetas distintas.
- Regras Rust e progressão: 248 testes aprovados, zero falhas. Oito cenários são opcionais na execução comum; os cenários de tutorial abaixo foram executados separadamente e aprovados.
- Cenários isolados: tutorial salvo/reaberto/troca de mundo, transporte misto com braços, funis e carregadores sem esteiras, e prioridade do objetivo do tutorial sobre os alertas do quadro.
- Launcher: seis testes, incluindo escolha de gráficos, preservação das opções pessoais e cópias dos arquivos originais.
- Executável nativo da integração anterior, antes das novas entregas do quadro e dos aldeões: compilado pelo Horizon Launcher sobre 66f3981, perfil performance (fast, otimizado), incluindo mod-veio 0.16.1 e horizon-native-game. Etapa de compilação concluída em 7 min 14 s, sem falhas. O jogo não foi aberto.
- API real: a integração anterior passou contra o Horizon `66f3981`; nova conferência final pendente após as últimas entregas. Os metadados do cache estão sendo atualizados por outra frente, por isso a candidata ainda não é anunciada como compilada.

Nenhum mundo ou conta foi apagado. O jogo não foi aberto automaticamente para testes.

## Compatibilidade do quadro com o tutorial e a vila

O tutorial continua com um objetivo por vez. Os alertas e as mensagens diárias do quadro aguardam o tutorial visível terminar ou ser ocultado. O painel do quadro continua acessível normalmente. Um cenário isolado verifica que o aviso de máquina danificada não substitui o objetivo e reaparece quando o tutorial é ocultado.

O censo e os pagamentos temporários do quadro são limpos ao iniciar outro mundo, junto do estado temporário da casa. O quartel mantém o turno noturno para defender a vila; os postos civis seguem o descanso introduzido pela nova frente de aldeões. O cancelamento das ordens limpa a navegação e o destino do cérebro; a consulta das ordens mantém o intervalo limitado da otimização.

As entregas de fase do quadro, os contratos da central e os exercícios do tutorial têm funções distintas. A conclusão dos exercícios encaminha ao quadro para continuar a progressão.

## Revisão da leva 0.17.2 e megafábricas

A rotina de tick única da frente de desempenho também executa as observações do tutorial e do censo. A observação detalhada de ciclos do tutorial é ativada apenas enquanto há jogadores em exercícios. O painel mantém as leituras específicas de caravanas, drones, quadro e farol opcional, sem perder a atualização eficiente da base 0.17.2.

As chaves de estado introduzidas por quadro, elevadores e copiadora entraram na tabela de chaves compiladas. O quadro reaproveita a chave de presença do censo. A animação de transporte mantém seus campos próprios; as esteiras preservam a otimização que evita sincronizar mudanças apenas de quantidade.

O quartel usa o modelo de combate da 0.17.0 em diante: dano real, cura do guarda e memória de plantão para não fugir nem dormir em serviço. As ordens mantêm o cancelamento de navegação e a limpeza do alvo antigo. A troca de classe usa agachar com esmeralda, conforme o comportamento mais recente. A correção anterior de ondas sem local carregado foi preservada: não anuncia nem conta uma onda que não conseguiu criar inimigos.

Projetos e ajustes de máquinas em espera são limpos ao trocar de mundo. O retorno da copiadora vai ao jogador que clicou, mesmo se outra pessoa estiver mais perto em LAN. Há testes específicos para os dois casos, além dos testes de cobrança e recusa sem gastar materiais.

A candidata recebeu código novo desde a última compilação nativa completa. A compilação e o ZIP únicos finais aguardam o encerramento de todas as frentes, como solicitado.
