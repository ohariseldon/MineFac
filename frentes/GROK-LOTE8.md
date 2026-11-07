# Grok — lote 8: censo da vila e filtro do divisor

Base: 0.15.0 (`e60dee3`), com braços, filtros e o lote 7 já integrados. Medições com o substituto offline. Não alegam FPS no Horizon.

## Mapa de custos (antes)

| Caminho | Quando | Custo observado |
|---|---|---|
| `villager_positions` | cada posto, a cada 10 ticks, e cada despensa | mesmo com o censo fresco: lista de todos os mobs, ordenação e **cópia** das posições |
| `recent_villagers` | essa consulta | montava a lista de animais e aldeões para depois ficar só com aldeões |
| `number("LogisticsPaused")` | cada esteira, cada tick | chave fora da tabela fixa: mutex da tabela extra |
| `split` | cada divisor, a cada 8 ticks | `String` do filtro só para comparar o item |

O censo da 0.14.0 já evitava reler 150 posições. Quem perguntava de novo ainda copiava o vetor e reconstruía a lista de ids passando pelos animais.

## Correções

1. **Censo.** A lista pronta é compartilhada. Uma consulta no mesmo intervalo não copia as posições. O conjunto de aldeões continua a invalidar o censo antes dos 100 ticks, como antes.
2. **Aldeões.** A busca lê só os aldeões do registro, sem montar a lista de animais.
3. **Chaves da 0.15.0.** `LogisticsPaused`, `RoutingPriority`, `LogisticsFilter`, `Swing` e `PreferRight` entram na tabela fixa. Esteira parada não disputa o mutex das chaves raras.
4. **Divisor.** O filtro compara o item direto no NBT. A `String` só aparece quando o braço ou o desenho precisam do nome.

## O que permanece

Teto de 150, janela de 100 ticks do censo, 1 item por transferência do braço, 40/64/60 FE, ritmo de 8/20 ticks, prioridade e filtro do divisor, pausa por redstone no mesmo tick da esteira. Sem bump de versão (continua 0.15.0). Sem migração de save.

## Teste pelo usuário

1. Vila grande: contratar e achar o trabalhador continua no mesmo ritmo (o censo ainda pode ter até 5 s).
2. Divisor com filtro de cobre: cobre sai à esquerda, o resto à direita, e não vaza quando o lado certo enche.
3. Alavanca na esteira: pausa em até 8 ticks, como na 0.15.0.

## Verificação

- `python tools/check_project.py`
- `cargo +1.90.0 test --offline -p veio-check` — 108 testes.
- Sem compilação pelo launcher e sem teste no jogo.
