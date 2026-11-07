# Braço de transferência — primeiro lote

Desenvolvido sobre a consolidação 0.10.0, em `codex/inserters`. O launcher, o pacote de release e os sistemas de trabalhadores permanecem sob os contratos da base. Este lote acrescenta `veio:inserter` e não altera a IA dos aldeões nem os números de produção e energia definidos pelo Grok.

## Uso

Coloque o braço no espaço entre dois inventários ou esteiras, olhando para o destino. Ele retira do bloco imediatamente atrás e entrega no bloco imediatamente à frente, na mesma altura. O bico de cobre marca a frente. A chave gira o braço. Conecte energia por cabo ou pela área de um poste; uma alavanca pode pausá-lo por redstone.

Exemplos de montagem: baú → braço → prensa; colheitadeira ou posto do lavrador → braço → esteira; esteira → braço → baú. O braço também pode ligar duas esteiras paralelas com um espaço entre elas. As faces e os filtros do destino continuam valendo, inclusive entradas A/B dos unificadores e slots de máquinas vanilla.

Uma transferência move até um item por segundo a 20 ticks/s e custa 40 FE apenas quando aceita. A reserva usa o padrão de 2.000 FE do mod. Fonte vazia, destino cheio, item recusado ou inventário indisponível não consomem energia e não retiram itens. O braço não tem um inventário de carga para o jogador abastecer.

Receita sem posição fixa: um pistão, uma engrenagem, um lingote de ferro e duas fiações produzem dois braços. A receita é liberada ao obter fiação. O bloco aparece na aba Logística e na categoria de logística do guia. Seu painel mostra energia, atividade, progresso da animação e vazão. Quando está ocioso, orienta a conferir origem e destino; não distingue qual dos dois impediu a transferência.

## Integração

`native/api/src/inserters.rs` usa `Host::move_items` do Horizon (API 48). O host executa a transferência respeitando extração, inserção, faces e pilhas; itens recusados permanecem na origem. A animação sinaliza uma transferência já concluída: não existe pilha temporária clonada na mão do braço.

Os NBT novos pertencem ao bloco novo: `Swing` guarda o restante da animação, `MovedAt` seu instante inicial, e `Produced` a contagem para a vazão comum. Energia, `Progress` e `Duration` usam os campos existentes. Reabrir o mundo durante a animação não executa de novo uma transferência concluída. Redstone congela a animação e impede novas transferências; ao liberar, o braço continua sem cobrar outra vez.

As consultas aos dois inventários são escalonadas por posição e limitadas a uma por segundo. O desenho interpola o movimento pelo relógio do cliente. A assinatura de sincronização usa `MovedAt` para não enviar um pacote a cada passo da animação. Sem trabalho, o desenho fica em cache. Não há varredura de aldeões ou de inventários ao redor.

O perfil é `Sector::Logistics` / `StationKind::Transport`: o braço recebe energia, mas não cria tarefas de retirada para aldeões nem expõe slots de carga. Pode retirar produtos do posto agrícola pelas regras normais do inventário; não acessa o inventário pessoal de um aldeão nem seu esquema de atribuição.

## Entrega e teste

Foram aprovadas 67 regras offline (seis cenários novos do braço), a verificação de conteúdo e a compilação de metadados contra a API real `66f3981`. Os cenários cobrem energia, redstone, destino cheio ou filtrado, fonte vazia, quatro orientações, conservação e retomada sem repetição após salvar.

A compilação completa pelo launcher e a conferência visual no jogo estão pendentes. A versão consolidada permanece 0.10.0: esta branch não atualiza `release/Veio-0.10.0.zip` nem recompila o `launcher.exe`. Para testar o código da branch, feche o jogo e execute o launcher da raiz com `--rebuild`; abrir apenas o ZIP antigo não inclui o braço. A próxima consolidação deverá atualizar a versão e o pacote para ativar o rebuild automático normal.

Filtros configuráveis, braços rápidos e ajuste de alcance são trabalho posterior. Este lote entrega o braço básico, sua receita, visual, controles, painel e guia.
