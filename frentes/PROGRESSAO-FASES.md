# Progressão em fases e balanceamento por unidade de jazida

Base: `main` cf37004 (0.15.0 + orçamento global e topologia incremental).
Referência de projeto: [JAZIDAS-CONECTADAS.md](JAZIDAS-CONECTADAS.md), branch
`research/connected-veins`. Compatível com `grok/mission-balance`; ver o fim.

Pedido: progressão rápida, aos trancos, com gargalos, como no Satisfactory e no
Factorio, usando o “mapa” das jazidas conectadas como referência.

## Princípio

1. **A jazida é a unidade.** Uma extratora Mk.I em jazida normal entrega
   30 itens/min. Toda máquina básica processa exatamente essa unidade, então as
   linhas fecham com números inteiros de máquinas, sem sobras de 3:4.
2. **Saltos por pilha.** Dentro de uma fase, cada desbloqueio pede uma peça.
   Passar de fase pede uma pilha que só uma linha completa entrega em 1–3 min:
   o tranco é rápido, mas exige provar que a linha funciona.
3. **Cada fase revela o próximo gargalo**: jazida → energia → carvão/vapor →
   silício → transporte. A energia de cada linha típica foi ajustada para encher
   exatamente uma fonte da fase anterior.

## Classes de jazida (o mapa)

Minérios medem o veio conectado em 3D a partir do bloco sob a extratora (faces,
não diagonais; comum e ardósia contam juntos). Pedra, areia e outras jazidas
infinitas continuam no 3x3, sem busca.

| Blocos conectados | Classe | Vazão | Mk.I | Mk.II | Mk.III |
|---|---|---:|---:|---:|---:|
| 1–2 | impura | 0,5× | 15 | 30 | 60 |
| 3–6 | normal | 1× | 30 | 60 | 120 |
| 7–15 | pura | 2× | 60 | 120 | 240 |
| 16+ | industrial | 4×, potência 2× | 120 | 240 | 400 |

Itens/min, sem módulos. A Mk.III industrial arredonda o ciclo para cima (3 ticks):
400/min, acima da esteira rápida (300/min) — é o gargalo de transporte da fase 5,
e a extratora pausa sem perder itens. Diamante, esmeralda e detritos param na
pura até medirmos a geração. A classe industrial puxa o dobro de FE/t, então o
FE por minério é o mesmo da pura: o prêmio é vazão por máquina, não energia.

A busca para em 16 blocos (no máximo 97 leituras), roda a cada 200 ticks como
antes e pede 24 unidades da cota global de terreno (`budget::Lane::Terrain`).
Sem cota, a extratora mantém a última classe e tenta no próximo ciclo; se ainda
não tinha leitura, tenta a cada tick sem bloquear a produção. Chunk descarregado
lê como ar: a classe pode sair menor, nunca carrega terreno.

## Receitas: antes → depois

| Receita | Antes | Depois | Por máquina |
|---|---|---|---:|
| Prensa (cobre, ferro) | 30 t, 8 FE/t | 40 t, 8 FE/t | 30/min |
| Cortadora: fiação | 1 → 1, 20 t, 6 FE/t | 1 → **2**, 40 t, 8 FE/t | 60 fiações/min |
| Cortadora: moldura | 30 t, 8 FE/t | 40 t, 8 FE/t | 30/min |
| Triturador: silício | 50 t | 40 t | 30/min |
| Forno de ligas: aço | 120 t, 16 FE/t | 80 t, 20 FE/t | 15/min |
| Integrador: módulo | 60 t, 16 FE/t | 80 t, 20 FE/t | 15/min |
| Montador de circuitos | 60 t, 12 FE/t | 80 t, 20 FE/t | 15/min |

Montador de engrenagens, ouro, alternativas, fornalha, moinho e cozinha ficaram
como estavam. Prensa mais lenta não atrasa o início: a extratora (30/min) já era
o limite da primeira linha.

## Linhas que fecham

| Linha | Máquinas | Saída | FE/t | Fonte que enche |
|---|---|---:|---:|---|
| Primeira fábrica | 1 extratora, 1 prensa, 1 montador | 15 engrenagens | 26 | roda d'água (32) |
| Engrenagens | 2 extratoras, 2 prensas, 1 montador | 30 engrenagens | 40 | 1 gerador |
| Aço | extratora de ferro + de carvão, 1 prensa, 1 forno | 15 aços | 40 | 1 gerador |
| Módulos | 3 extratoras, 3 prensas, 1 montador, 1 cortadora, 2 integradores | 30 módulos | 102 | caldeira (120) |
| Circuitos | extratora de cobre + de quartzo, 1 prensa, 1 cortadora, 1 triturador, 2 montadores | 30 circuitos | 78 | caldeira (120) |

Os testes em `production/balance_tests.rs` derivam esses números de
`tools/production.json`, `extraction.rs` e `generation.rs`. Mudar um tempo ou
potência sem fechar a razão faz o teste falhar e mostra qual linha quebrou.

## Árvore de avanços

| Fase | Salto | Antes | Depois | Com uma linha |
|---|---|---|---|---|
| 2 → oficina | `engineering` | 1 engrenagem | 32 engrenagens | ~2 min (15/min) ou ~1 min (30/min) |
| 3 → aço | `steel` | 1 chapa de aço | 16 chapas | ~1 min |
| 4 → eletrônica | `circuits` | 1 circuito | 16 circuitos | ~1 min |

Os demais nós seguem pedindo uma peça, para o tutorial e os ramos paralelos
(braços, vila, lavoura) continuarem rápidos. Usa o predicado vanilla
`count.min` de `inventory_changed`, já usado pelo avanço dos oito módulos e
suportado pelo Horizon de referência (`advancements.rs`, filtro por pilha).
O passo 6 do guia e as descrições dos avanços dizem quanto juntar.

Mundos existentes: avanços já concedidos continuam concedidos e as receitas
continuam liberadas. A nova exigência vale só para quem ainda não chegou ao nó.

## Efeito nos projetos da central

Mesmo método da frente Grok (soma de ticks e FE de receita e componentes, sem
extração), aplicado aos pedidos de `grok/mission-balance`:

| Pedido | Segundos de máquina | FE |
|---|---:|---:|
| Oficina | 76 → 96 | 13.440 → 16.640 |
| Entrepostos | 348 → 360 | 86.400 → 94.080 |
| Povoado | 348 → 368 | 61.920 → 83.840 |
| Farol (só circuitos) | 504 → 480 | 93.120 → 126.720 |
| Exportação | 78 → 72 | 17.360 → 19.520 |

O trabalho de máquina fica parecido (±25%); a energia sobe 10–36%. É o desejado:
o gargalo do meio do jogo é energia, não espera por máquina lenta. Não muda os
pedidos nem as recompensas.

## Integração

- Arquivos de código: `production/extraction.rs` (pureza, busca, potência),
  `production/mod.rs` (rótulo industrial no painel e registro dos testes),
  `production/balance_tests.rs` (novo), `tests.rs` (um tempo de teste do
  integrador). Gerados: `production/data.rs`, guia e traduções.
- `grok/mission-balance` muda outras linhas de `extraction.rs` e `mod.rs`
  (`output_amount`, `bonus_ready`, `finish_cycle`); as alterações daqui não se
  sobrepõem. Os arquivos gerados (`guide.json`, traduções, `data.rs`) devem ser
  regerados depois da fusão: `python tools/build_production.py` e
  `python tools/build_tutorial.py`.
- As tabelas de taxa em `BALANCEAMENTO-ECONOMIA.md` (Grok) para prensa, aço e
  circuito ficam desatualizadas; os números desta página as substituem.
- Sem novos IDs, slots, campos NBT ou contratos. `Info` da extratora passa a
  aceitar 4 (industrial).

## Validação

- `python3 tools/check_rust.py`: 135 testes aprovados, 2 prévias ignoradas.
- `python3 tools/check_project.py`: 475 JSON, 93 páginas, traduções e texturas.
- `python3 tools/check_performance.py`: 7 cenários, verificações idênticas.
- Não feitos: compilação pelo launcher, teste em partida e medição de FPS com
  muitas extratoras em veios grandes.

Em partida, conferir: tempo até 32 engrenagens em jazida impura e normal; se a
roda d'água deixa claro que falta energia; quantas extratoras industriais uma
seed comum oferece (a geração ainda é vanilla — veios de 16+ são raros fora de
cobre e ferro grandes); e se 400/min da Mk.III industrial frustra ou motiva.
