# Consolidação 0.16.1

## Entradas preservadas

- Base fornecida `E:/minefac-0.16.0.zip`: bundle verificado, commit `3cd563d096f67d20728732391a329348b4340f3a`.
- `13758bd7859602b50bd7028377035fe005806d5d`: planejamento incremental dos capatazes, cota compartilhada e continuidade da fila anterior durante o recálculo.
- `fd2a44602577ec9abf0688b7bb1f5174b3d0d2d1`: casa do fundador, reinício correto dos estados temporários e checkpoint do mapa.
- Tutorial local: oito objetivos observados em jogo, persistência por jogador/mundo e transportes por esteiras, braços, funis e carregadores.
- Realismo opcional, opção sem shaders e identidade visual das máquinas.
- `MineFac-progressao-fases.zip`: os dois patches foram aplicados sobre sua base `cf37004`, preservando autoria, e a branch resultante foi fundida à consolidada.
- Branches remotas: todas as pontas conferidas são ancestrais desta build. Lista de nomes e hashes em `frentes/branches-consolidadas.json`.

## Decisões de integração

A casa, o sono noturno dos trabalhadores, os quartos da vila, quartel, contratos industriais, terminais, drones e caravanas ficam ativos. O relógio roda somente no servidor; o renascimento mantém a fila adiada corrigida pela revisão da 0.16.0.

O lote 2 antigo do Grok foi registrado por merge sem reaplicar seu código: lotes posteriores já substituem suas otimizações. A documentação histórica foi recuperada. As outras branches foram integradas por ancestralidade ou merge efetivo.

A progressão por fases substitui os tempos anteriores da prensa e do montador: uma jazida normal alimenta uma prensa; duas linhas alimentam um montador de engrenagens. As receitas da prensa seguem o último pacote fornecido, inclusive ouro e alternativas. Os contratos e recompensas da 0.16.0 permanecem. Avanços já obtidos em mundos existentes não são revogados. No encerramento do tutorial, a mensagem aponta para as 32 engrenagens e explica a razão da linha.

Os contadores de produção da 0.16.0 contam produtos, incluindo produtividade. O tutorial usa ciclos efetivamente concluídos para certificar o consumo automático e não gasta créditos extras pelo produto bônus. O estado de descanso noturno foi preservado; os estados de caravana foram movidos para índices distintos.

Os guias foram regenerados após a fusão, com paginação adicional no sumário de logística. A validação de tamanho cobre os quatro livros. O tutorial principal continua na tela; livros são consulta opcional.

O mapa finito em `work-in-progress/finite-world` é apenas checkpoint de desenvolvimento, fora do manifesto ativo. Jazidas conectadas estão ativas; geração de um mapa finito garantido ainda não está implementada nesta build.

## Verificações

- Conteúdo: 518 JSON, traduções e 63 texturas 16×16; quatro livros com ajuste de página.
- Formas de máquinas: 34 registros e 27 silhuetas distintas.
- Regras Rust e progressão: 206 testes aprovados, zero falhas. Quatro cenários são opcionais na execução comum; os dois cenários de tutorial abaixo foram executados separadamente e aprovados.
- Cenários isolados: tutorial salvo/reaberto/troca de mundo e transporte misto com braços, funis e carregadores sem esteiras.
- Launcher: seis testes, incluindo escolha de gráficos, preservação das opções pessoais e cópias dos arquivos originais.
- Compilação do executável nativo: em verificação nesta máquina; ainda sem teste em partida.
- Tipos nativos: conferência contra os metadados reais do Horizon `66f3981`; não equivale a teste em partida.

Nenhum mundo ou conta foi apagado. O jogo não foi aberto automaticamente para testes.
