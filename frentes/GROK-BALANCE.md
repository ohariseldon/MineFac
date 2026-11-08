# Grok — balanceamento: a primeira linha pede uma fábrica

Base: 0.15.0 (`e60dee3`). Números de `tools/production.json`, regenerados em `production/data.rs`. O guia só mudou o texto da prensa e da engrenagem. Sem bump de versão.

## O que estava fácil demais

Uma extratora normal tira 1 cobre a cada 40 ticks (0,5/s). A prensa fazia a chapa em 30 ticks (0,67/s) e o montador a engrenagem em 40. Uma de cada máquina esvaziava o veio e ainda sobrava tempo. O jogador montava a linha do tutorial e não tinha motivo para uma segunda prensa, um divisor ou uma esteira ramificada.

| Veio de cobre | Extratora | Prensas que ela enche | Montadores |
|---|---|---|---|
| Impuro (1–2 blocos) | 80 ticks, 0,25/s | 1 | — |
| Normal (3–6) | 40 ticks, 0,5/s | 2 | 1 (duas chapas) |
| Puro (7–9) | 20 ticks, 1/s | 4 | 2 |

Achar um veio maior passa a valer a pena: a pureza, que já existia, agora muda o tamanho da linha.

## O que mudou

- Cobre e ferro brutos: prensa de 30 para **80** ticks. Ouro bruto: 40 para **120**.
- Lingote já pronto: 15 para **40** ticks. Continua o atalho (duas vezes o bruto), para quem já fundiu na fornalha vanilla.
- Engrenagem: montador de 40 para **80** ticks. Duas prensas enchem um montador.

Energia por máquina não muda (prensa 8 FE/t, montador 12, extratora 6). Uma linha de 2 extratoras + 2 prensas continua em 28 FE/t: duas rodas com duas faces (32) ou quatro cata-ventos no nível do mar cobrem, e a caldeira (120) continua outro patamar. Um gerador a carvão (40) roda a primeira engrenagem (extratora + duas prensas + montador = 34 FE/t).

Esteiras (2,5 e 5 itens/s), braços, comida, `TASK_COST` e o aço (120 ticks) ficam como estão. A Mk.III pura (4 itens/s) ainda pede a esteira rápida.

## Visual

No teto das máquinas elétricas, uma cruz da cor do setor gira enquanto trabalha e fica parada quando ociosa. Uma marca na borda aponta a saída. Luzes, módulos e o desenho da Tesla continuam. O desenho ocioso segue em cache (`keep`); só o giro usa `spin`.

## Teste no mundo

1. Cobre pequeno: uma prensa acompanha a extratora.
2. Cobre de veio normal: a prensa enche e a extratora para; a segunda prensa destrava.
3. Duas prensas numa esteira para um montador: a engrenagem sai no ritmo das chapas, sem chapas paradas.
4. De cima, a marca na tampa mostra para onde a máquina aponta, e a cruz gira só quando há energia e insumo.

Sem migração de save. Máquinas no meio de uma receita só ficam mais lentas no próximo ciclo.
