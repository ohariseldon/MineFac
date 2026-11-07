# Escala, energia e custo de simulação

Pedido do usuário: expansão gradual por infraestrutura e energia, com simulação
barata para fábricas grandes. Este lote parte da main `65cab50`, integra o lote 8
do Grok (`a8e4741`) e acrescenta limites locais para trabalho de descoberta.

## O que foi implementado

- Censo de aldeões compartilhado por `Arc`, sem copiar todas as posições a cada
  consulta. Filtros do divisor comparados diretamente no NBT. Integração do Grok
  preserva os caches e a atualização numérica do lote local anterior; remove as
  definições duplicadas que uma fusão textual deixaria.
- Colheitadeira, semeadora e silvicultora verificam o custo mínimo possível antes
  de consultar o mundo. Sem energia suficiente, não varrem o terreno e mostram
  falta de energia. A colheitadeira mantém o cursor; a semeadora continua exigindo
  sementes válidas. A retomada ocorre na próxima fase normal de trabalho quando
  a energia volta, sem um cooldown adicional.
- Edições agrícolas já pagas e enfileiradas continuam sendo enviadas antes dessa
  pausa. Nenhum produto, semente ou energia é consumido pelo caminho de espera.
- Descoberta da área do poste verifica no máximo oito células por tick por poste.
  Antes a varredura consultava até 74 blocos no poste de madeira e 146 no de aço
  em um só tick. A volta completa usa 10 e 19 ticks, respectivamente.
- A última área completa continua alimentando as máquinas durante a descoberta.
  A lista nova substitui a antiga ao terminar, preservando a ordem original.
  Dois buffers são reutilizados; cursor e resultado parcial sobrevivem ao save.
  Novos campos opcionais `AreaScan` e `AreaFound` ficam apenas nos postes; saves
  anteriores iniciam a primeira varredura automaticamente.

O ciclo de início continua em 40 ticks. A descoberta inicial leva até 19 ticks
carregados; depois de uma alteração, a publicação completa pode levar até cerca
de três segundos a 20 TPS. É uma troca explícita de latência por pico menor.
O alcance, a potência e a transferência de energia por tick não diminuíram.
O limite de oito vale para a **varredura da área**, não para validar postes
vizinhos, recalcular fios ou percorrer consumidores já conhecidos.

O rancho mantém a coleta gratuita de itens do chão: pausá-lo inteiro por falta
de energia removeria uma mecânica existente. A lâmpada de cultivo já verifica
energia antes da busca. Trabalhadores continuam usando alimento e as regras de
atribuição da frente Cloud, sem uma segunda IA ou custo elétrico acrescentado.

## Como a energia controla a expansão

Geração, combustível, vazão dos fios, espaço e materiais são freios de progressão.
A rede atual transfere estoques locais de FE; ela ainda não tem um disjuntor global
que calcula a demanda da fábrica inteira. Esse mecanismo não foi acrescentado
como efeito colateral da otimização.

No [Satisfactory](https://satisfactory.wiki.gg/wiki/Power), demanda acima da
produção com baterias esgotadas derruba a rede. Isso limita operação, mas não é
uma garantia de orçamento de CPU. Mesmo estruturas desligadas, redes e entidades
podem custar processamento. Por isso o jogo precisa tanto de progressão quanto
de limites explícitos de trabalho por atualização.

Direção para as próximas entregas: geração inicial presa ao terreno/combustível;
fontes avançadas e módulos compram capacidade; módulos de eficiência permitem
mais produção com a mesma energia. Priorizar máquinas melhores sobre duplicar
centenas de estruturas. Antes de criar disjuntores e prioridades por setor,
implementar um cadastro de redes invalidado corretamente, contabilizar oferta e
demanda uma vez por rede e preservar conservação de energia. Não deduzir demanda
percorrendo toda a fábrica a cada tick.

## Frentes conferidas

- Lote 8 do Grok: integrado neste lote, com autoria e histórico preservados.
- `grok/mission-balance`: projetos, produtividade e contagem dos produtos.
  Inspecionado; não integrado aqui porque altera economia e progressão.
- `cursor/grok-vilas-c7d9`: camas, crescimento e roteiro de cidade. Inspecionado;
  não integrado aqui porque altera moradia e mensagens. O teto existente de 150
  aldeões permanece; este lote não acrescenta um teto de máquinas.
- A branch Cloud consultada não tinha novos commits além da main. Alterações
  locais de onboarding/tutorial foram preservadas em outro checkout.

## Verificação e limites

Os testes verificam ausência de consultas ao terreno durante 120 ticks sem
energia e retomada das três máquinas, limite de oito consultas por passo,
ordem e cobertura completa dos dois alcances, retomada de varredura salva,
publicação apenas ao terminar e remoção de consumidor antigo. A transferência
continua durante a varredura e conserva os 200 FE do cenário de teste.

Validação do lote: 116 testes aprovados (duas ferramentas opcionais ignoradas),
conferência de conteúdo aprovada (475 JSONs, 93 páginas, 58 texturas) e compilação
de metadados contra a API real já construída pelo launcher, sem avisos. Sem nova
linkedição do executável nem teste em partida nesta entrega.

Essas contagens são verificações offline. O custo total ainda cresce com o número
de estruturas carregadas. Não há garantia de FPS ou teto global de CPU: callbacks
do motor, reconstrução densa de fios, IA, sincronização e renderização ainda
precisam de medição no Horizon antes de se estabelecer um orçamento global.
