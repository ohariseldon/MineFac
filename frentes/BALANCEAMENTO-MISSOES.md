# Balanceamento das missões — base 0.15.0

Continuação da frente Grok sobre a `main` e60dee3. Este lote cobre os quatro
projetos da central, a exportação e a passagem entre eles. Não altera números
de energia, aldeões, combate ou máquinas já ajustados nos outros lotes.

## Diagnóstico e decisões

- Oficina: mantém 24 chapas de cobre + 8 engrenagens. Ensina duas etapas de
  produção e devolve logística para ampliar a fábrica. Não aumentar o início.
- Entrepostos: 16 → 24 aços e 8 → 12 módulos. Pede uma linha de aço e montagem
  mais estabelecida, sem exigir pilhas múltiplas ou transporte manual adicional.
- Povoado: 12 → 24 circuitos e 24 → 32 molduras. Antes tinha menos trabalho de
  processamento que Entrepostos apesar de vir depois. Agora os dois têm esforço
  agregado semelhante, mas Povoado exige a nova cadeia eletrônica.
- Farol: 32 → 48 circuitos, conservando a estrela do Nether e as recompensas.
  Continua sendo o desafio final opcional, com exploração e produção eletrônica.
- Exportação: conserva 4 circuitos + 4 aços por 4 esmeraldas + 2 frascos.
  O problema demonstrado era o acesso, não um rendimento excessivo comprovado.
  Agora abre depois de Povoado, sem obrigar a enfrentar o Wither.

Todas as entradas cabem numa pilha e todas as recompensas nos dois slots da
central. Não há prazo, reinício de entrega, exigência de produção contínua ou
dependência da futura coordenação por aldeões. O desafio continua sendo montar
as cadeias, sustentar energia e escoar os produtos.

## Comparação reproduzível

Fonte: receitas principais (sem `alt`) de `tools/production.json`, sem módulos,
a 20 ticks/s. Para cada produto, somar ticks e FE de sua receita e dos
componentes Veio, multiplicados pelas quantidades; dividir pela quantidade de
saída de cada receita. Matérias-primas vanilla entram prontas. Exemplo: aço =
2 prensas de 30 ticks + liga de 120 ticks = 180 ticks = 9 segundos de máquina;
energia = 2 × 30 × 8 + 120 × 16 = 2.400 FE.

| Pedido | Segundos de máquina antes → depois | FE antes → depois |
|---|---:|---:|
| Oficina | 76 → 76 | 13.440 → 13.440 |
| Entrepostos | 232 → 348 | 57.600 → 86.400 |
| Povoado | 198 → 348 | 34.800 → 61.920 |
| Farol (só circuitos) | 336 → 504 | 62.080 → 93.120 |
| Exportação (uma remessa) | 78 → 78 | 17.360 → 17.360 |

Esses valores são soma de trabalho de máquinas, não duração de missão ou tempo
de sessão. Excluem extração, construção, combustível, deslocamento, aquisição
da estrela, transporte, filas e espera por energia. Linhas paralelas reduzem
tempo de produção; jazidas impuras e energia insuficiente aumentam espera.
Não demonstram duração real nem dificuldade percebida. A rota alternativa de
circuitos continua disponível para não tornar o Nether obrigatório no comércio.

## Seleção, saves e interface

Os estágios NBT 0–4 mantêm os mesmos projetos. No estágio 3, a central oferece
exportação por padrão. Esvaziar entradas e colocar a estrela primeiro no slot 1
seleciona o Farol; o slot 2 passa a aceitar circuitos. A conclusão avança para 4,
como antes. Exportar no estágio 3 incrementa `Total`, sem concluir o Farol.
O jogador precisa retirar recompensas incompatíveis antes de trocar pedidos.

Mantidos IDs, inventários, campos de progresso e os 13 slots de dados do menu.
O slot 12, antes sem uso específico na central, informa apenas a seleção do
Farol. Slots 8 (estágio) e 9 (exportações) preservados. Guia em português e inglês
explica a seleção, e o painel mostra o pedido ativo e o contador de exportações.

Saves antigos nos estágios 0–2 passam a usar os novos custos para pedidos ainda
não concluídos. Itens já inseridos são preservados, mas podem precisar de
complemento. Estágio 3 com estrela já inserida continua no Farol; sem estrela,
troque eventuais circuitos da entrada 2 por aço para exportar. Estágio 4 continua
exportando. Nenhuma recompensa concluída é retirada ou entregue novamente.

## Validação e teste de partida

`python3 tools/check_project.py`: 475 JSON, 94 páginas, traduções e 58 texturas.
`python3 tools/check_rust.py`: 111 testes aprovados, 1 prévia opcional ignorada.
Novos testes cobrem exportar antes do chefe sem avançar estágio, seleção e
conclusão do Farol, filtros, dados do painel e atomicidade de todos os projetos
com material insuficiente ou segunda saída cheia.

A compilação pelo launcher e o teste em jogo não foram executados. Avaliar em
uma partida: tempo gasto construindo versus esperando, disponibilidade de aço
para expansão durante Entrepostos, rotas de circuito sem Nether e troca de
exportação/Farol com saídas ocupadas. Ajustar pelo resultado da partida antes de
afirmar que a dificuldade está fechada. Desmontar e recriar centrais ainda
reinicia projetos: comportamento preexistente, sem trava global por jogador.
