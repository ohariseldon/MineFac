# Experiência do jogador: o Quadro da fábrica

Base: branch `claude/progressao-fases` (fases, unidade de 30/min e veios conectados,
ver [PROGRESSAO-FASES.md](PROGRESSAO-FASES.md)). Pedido do usuário: aprimorar a
experiência com o que Satisfactory, Factorio e Punch Club fazem bem, nas quatro
frentes — metas visíveis, gargalo legível, rotina e primeira hora —, com tela nova.

## O que cada jogo ensina e onde entrou

| Jogo | Lição | No Veio |
|---|---|---|
| Satisfactory | O HUB mostra o próximo marco, quanto falta e o que ele libera; entregar peças é o ritual de avançar | **Quadro da fábrica**: fase atual, barra de entrega, o que a licença libera, recompensa na saída |
| Factorio | O jogo aponta a máquina parada e o motivo antes que a linha inteira pare | **Censo de gargalos** no quadro e **alertas** na barra de ação para quem está perto |
| Punch Club | O dia tem um começo e um fim; há uma meta curta com prazo e um balanço | **Pedido do dia** da vila, aviso ao amanhecer e balanço ao anoitecer |
| Os três | A primeira meta aparece no primeiro minuto | O quadro chega com o guia; na fase 1 ele vira uma lista de montagem que se marca sozinha |

## Quadro da fábrica (`veio:factory_board`)

Recebido com o Guia ao entrar no mundo; receita liberada desde o início
(baú, 2 lingotes de cobre, graveto). Entrada à esquerda, recompensa à direita,
ambas automatizáveis por esteira e braço.

| Fase | Entregar | Recompensa | Efeito |
|---|---|---|---|
| 1. Primeira fábrica | 16 chapas de cobre | 32 esteiras | — |
| 2. Oficina | 32 engrenagens | Licença da oficina | concede o avanço `engineering` |
| 3. Aço | 16 chapas de aço | Licença do aço | concede o avanço `steel` |
| 4. Eletrônica | 16 circuitos | Licença da eletrônica | concede o avanço `circuits` |
| 5. Profundidade | 32 circuitos | 2 diamantes | material da Mk.III |

As quantidades são as mesmas dos avanços da fase; segurar a pilha no inventário
continua valendo (`requirements: [["stock", "permit"]]`). O quadro dá um ritual e
um caminho automatizável, sem criar um segundo sistema de desbloqueio.

Ao concluir uma fase: som, mensagem para quem está a até 48 blocos e o **próximo
gargalo** em uma frase (energia, carvão, silício, transporte). A saída cheia
segura a fase e o excedente fica na entrada; nada se perde.

Painel, de cima para baixo: fase e nome; entregue/total com barra; o que libera
(ou a recompensa); na fase 1, a lista Energia · Extratora · Prensa · Montador,
verde quando a máquina existe a até 64 blocos; pedido do dia; e o estado da
fábrica (máquinas trabalhando, ou paradas por energia, saída cheia e dano).
A diagramação foi simulada com as larguras da fonte do Minecraft, nos dois
idiomas e em todas as combinações de fase e pedido: a última linha termina
acima do rótulo do inventário.

## Pedido do dia

A cada amanhecer o quadro sorteia um pedido entre os dois níveis mais recentes
da sua fase (chapas → engrenagens e pão → molduras e aço → módulos e carne →
circuitos), de 16 a 64 itens, pagando 1 a 6 esmeraldas. Vale até o próximo
amanhecer, quando o progresso zera. A vila paga **um pedido por dia por
dimensão**: um segundo quadro que fechar o mesmo pedido mostra que já foi pago.

Quem estiver a até 96 blocos de um quadro recebe no chat:

- **Amanhecer:** o número do dia e o pedido.
- **Anoitecer:** itens produzidos desde o amanhecer, andamento do pedido e,
  se houver, as máquinas paradas para resolver amanhã.

Carregar um mundo não anuncia nada; a contagem começa no carregamento.

## Gargalos e alertas

`milestones/census.rs` registra consumidores e geradores de energia quando o
estado muda e, sem mudança, a cada 100 ticks (fase espalhada pela posição).
Classificação, com tolerância para não acusar oscilações normais:

| Estado | Vira alerta depois de |
|---|---|
| Sem energia, sem combustível | 10 s |
| Saída cheia | 60 s (linha equilibrada para quando o estoque enche) |
| Danificada | na hora |

A cada 5 s, quem tem máquinas paradas a até 32 blocos recebe uma linha na barra
de ação. Um problema novo avisa na hora; o mesmo problema só se repete depois de
um minuto. Sem paradas, nada aparece.

## Custo

- Por bloco da fábrica, por tick: uma comparação de estado. A trava do censo só
  é tomada na mudança de estado ou a cada 100 ticks; a contagem de produção é
  atômica.
- Quadro: uma leitura do censo por segundo.
- Tick global: alertas a cada 100 ticks (jogadores × máquinas registradas),
  ritmo do dia a cada 20.
- Nada lê o mundo; entradas sem atualização por 400 ticks saem do registro.
- `check_performance.py`: verificações idênticas nos 7 cenários.

## Integração

Novo módulo `native/api/src/milestones/` (`mod.rs`, `census.rs`, `tests.rs`).
Linhas acrescentadas em arquivos compartilhados, todas no padrão de `workforce`:

- `lib.rs`: `mod milestones`; `is_factory_block`, `layout`, `has_item_inputs`,
  `output_slots`, `accepts`, `tick_factory`; `census::observe` depois de
  `tick_factory` em `block_entity_tick`; `census::tick` no tick global.
- `operations.rs`: entrega automática aceita só o item da fase ou do pedido atual.
- `presentation.rs`: slots da tela, desenho e dados do painel.
- `tutorial.rs`: rótulos dos slots; a dica de estado não aparece no título do quadro.
- Conteúdo: bloco, menu, modelo, textura frontal, três licenças (itens e
  texturas), receita, `root` libera a receita, avanços de fase aceitam a licença,
  `tutorial/start` entrega o quadro (`loot_table/tutorial/board.json`),
  guia (página inicial, ficha do quadro na categoria Projetos) e traduções.

Sem mudança de IDs, slots, campos ou contratos existentes. Campos novos só no
NBT do quadro (`Phase`, `Delivered`, `Day`, `Order`, `OrderDone`, `OrderPaid`
e as leituras `D*`). Fusão com `grok/mission-balance`: o mesmo conflito
preexistente em `presentation.rs` e as páginas geradas do guia; ver
PROGRESSAO-FASES.md.

## Validação

- `python3 tools/check_rust.py`: 151 testes aprovados sobre a 0.16.0 (8 novos do quadro: fases,
  saída cheia, pedido diário, pagamento único por dimensão, censo, alertas,
  amanhecer e anoitecer).
- `python3 tools/check_project.py`: 492 JSON, 94 páginas, 62 texturas.
- `python3 tools/check_performance.py`: 7 cenários, verificações idênticas.
- Não feitos: compilação pelo launcher e teste em partida.

## Limites e o que observar na partida

- Alertas e o pagamento diário por dimensão vivem em memória: reiniciar o jogo
  zera a repetição dos alertas e permitiria receber o pedido do dia de novo.
- Em LAN, as mensagens montadas no servidor (nomes de item no chat) saem no
  idioma do anfitrião.
- O censo só vê máquinas em chunks carregados.
- Conferir no jogo: legibilidade do painel, se os alertas incomodam (60 s de
  saída cheia pode ser cedo para quem estoca de propósito), se o pedido do dia
  pesa a favor ou contra o comércio da central, e o tempo até a primeira fase.
