# Lote integrado de logística

Este lote amplia o PR dos braços, compatibilizado com a consolidação 0.12.0, a fila do coordenador, o pastor e o lote 3 do Grok. A unidade da entrega é uma rede que abastece, transporta, separa, prioriza, armazena e pausa itens, com ferramentas de escala e leitura de vazão. A IA, os contratos de trabalhadores e os ajustes globais de balanceamento continuam com Cloud e Grok.

## Recursos implementados

- Braço básico: alcance de um bloco em cada lado, até 1 item/s, 40 FE por item entregue.
- Braço rápido: alcance de um bloco em cada lado, até 2,5 itens/s, 64 FE por item entregue. Motor azul.
- Braço longo: busca a dois blocos atrás e entrega a dois à frente, até 1 item/s, 60 FE por item. Motor vermelho. Os inventários intermediários não participam da transferência.
- Filtro em qualquer braço: usa o tipo de item da amostra. Não diferencia encantamentos ou outros dados de dois itens do mesmo tipo. O conteúdo transferido mantém seus dados originais pelo host.
- Divisor equilibrado, com prioridade esquerda/direita e uso da outra saída quando a preferida recusa. O modo filtrado separa o item selecionado à esquerda e os demais à direita, sem desviar para a via errada quando a correta enche.
- Unificador equilibrado ou com prioridade A/B. Uma entrada vazia ou rejeitada permite atender a outra. O modo equilibrado mantém a alternância existente; prioridade é uma escolha explícita e pode adiar indefinidamente a outra entrada se a preferida estiver sempre abastecida.
- Redstone pausa a transferência automática de braços, esteiras, divisores e unificadores. Os transportadores reavaliam o sinal em até oito ticks; itens permanecem nos estoques. Inventários ainda podem ser acessados por outros mecanismos, como funis e braços.
- Painéis mostram vazão medida, modo de roteamento e presença de filtro. Marcadores coloridos mostram prioridade e separação; o item copiado aparece como ícone no mundo. Esteiras bloqueadas ou pausadas deixam de animar como se estivessem movimentando carga.
- Receitas e avanços para as variantes; guia com controles e quatro páginas que explicam a rede como um conjunto.

## Controles

1. Botão direito abre o painel; chave com botão direito gira.
2. Agachar e usar um item num braço ou divisor copia o filtro sem gastar a amostra.
3. Agachar com mão vazia limpa o filtro. Nos braços, agachar com chave também limpa.
4. Agachar com chave num divisor ou unificador alterna equilibrado → esquerda/A → direita/B → equilibrado. Num divisor filtrado, essa ação primeiro remove o filtro e volta ao equilíbrio.
5. Nos unificadores, A é a lateral esquerda quando se olha para a saída; B é a direita. Nos divisores, esquerda/direita seguem a mesma referência.

## Roteiro para testar uma fábrica completa

Use o criativo para montar os cenários rapidamente; a revisão de custos de sobrevivência é uma etapa própria.

1. Misture cobre e ferro brutos numa esteira e copie cobre como filtro de um divisor. Encaminhe cada saída à sua prensa. Encha o destino do cobre e confira que cobre não aparece na linha do ferro. Limpe o filtro e confira o retorno ao modo indicado no painel.
2. Alimente carvão num divisor sem filtro, com prioridade para o gerador e um baú na outra saída. O gerador deve receber primeiro; quando sua entrada recusar, o baú recebe o excedente.
3. Junte uma linha principal e um estoque de reserva num unificador. No equilíbrio, as duas entradas têm vez. Na prioridade A, o estoque B entra quando A não puder entregar. Use esteira rápida na saída para comportar duas comuns.
4. Use dois braços rápidos para alimentar uma esteira de 5 itens/s; compare com braços básicos. Passe um braço longo por uma esteira intermediária, com baús a dois blocos em cada lado, e confira que ele usa apenas os baús distantes.
5. Instale filtros nos braços de abastecimento e ligue uma alavanca à rede. Confirme pausa, retorno e conservação de itens. Depois de uma transferência, salve durante a animação e reabra: ela não deve se repetir.
6. Ligue uma esteira ou braço à saída do posto do lavrador. Produtos podem sair; a comida reservada à entrada do posto continua protegida pelas regras de inventário. O braço não acessa o inventário pessoal dos aldeões.

## Desempenho, dados e compatibilidade

Os braços continuam usando `Host::move_items` com filtros e faces, de forma atômica. Uma recusa não retira o item nem cobra energia. Consultas aos inventários são escalonadas por posição e limitadas pelo ritmo do braço. Não há busca global de máquinas ou aldeões.

`logistics_controls.rs` centraliza `LogisticsFilter`, `RoutingPriority`, interação e marcadores. `Info`, reservado à informação do transporte, codifica prioridade nos dois bits inferiores e presença de filtro no bit 2, sem aumentar os 13 campos dos menus. `LogisticsPaused` guarda o sinal amostrado das esteiras. `Produced` passa a contar transferências reais nos transportadores para a métrica comum, sem incrementar de novo a produção das máquinas.

As escolhas são salvas em NBT. Blocos existentes começam equilibrados e sem filtro. Nenhum ID antigo mudou. O perfil de todos os braços permanece `Transport`, fora das tarefas genéricas dos aldeões. Custos e ritmos estão em `inserters::tier` para a revisão do Grok.

Validação final após integração da 0.12.0: 93 regras offline aprovadas, compilação de metadados contra a API real `66f3981`, 427 JSON, 99 páginas e 55 texturas conferidos. Compilação completa pelo launcher e conferência visual no jogo estão pendentes. A versão e o ZIP oficial permanecem 0.12.0 até a consolidação; para testar o código da branch use `launcher.exe --rebuild`.

O guia está em 99 das 100 páginas suportadas pelo Horizon. A próxima expansão de conteúdo deve revisar a organização do guia antes de acrescentar páginas.

## Visão das próximas rodadas

As áreas restantes formam lotes integrados: construção em escala e cópia de configurações; transporte a longa distância e cruzamentos; armazenamento e reposição por metas; transmissão elétrica e diagnóstico de gargalos; combinação entre redes automáticas e tarefas dos trabalhadores. Essas frentes ainda são planejamento. Cada rodada deve entregar uma rede utilizável e documentada, com o tutorial e as verificações fechados juntos.
