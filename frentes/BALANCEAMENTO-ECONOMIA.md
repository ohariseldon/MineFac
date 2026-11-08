# Economia e retorno da progressão — 0.15.0

Continuação do balanceamento das missões, agora no sentido de recompensa por
etapa de desenvolvimento. Números derivados de `production/extraction.rs`,
`production/generation.rs`, `upgrades.rs` e `tools/production.json`.

## Critério

O crescimento vem de reinvestir: encontrar uma jazida melhor, duplicar máquinas,
melhorar a extração e aumentar o aproveitamento em várias etapas. O próximo
desafio deve ser alimentar, energizar e transportar essa produção. Aumentar
somente os pedidos prolongaria a espera. Não é uma promessa de crescimento
exponencial ilimitado: cada multiplicador encontra capacidade e custos reais.

## Quanto minério sai hoje

Máquinas alimentadas continuamente, sem módulo e sem bloqueio, a 20 ticks/s.
As jazidas são infinitas; a dificuldade é a taxa de extração, não seu esgotamento.

| Produto por minuto | Mk.I impura/normal/pura | Mk.II impura/normal/pura | Mk.III impura/normal/pura |
|---|---:|---:|---:|
| Cobre, ferro, ouro, carvão, redstone, lápis, quartzo e materiais comuns | 15 / 30 / 60 | 30 / 60 / 120 | 60 / 120 / 240 |
| Diamante e esmeralda | não extrai | 1,5 / 3 / 6 | 3 / 6 / 12 |
| Obsidiana | não extrai | 3,75 / 7,5 / 15 | 7,5 / 15 / 30 |
| Ametista | não extrai | 7,5 / 15 / 30 | 15 / 30 / 60 |
| Detritos ancestrais | não extrai | não extrai | 1,5 / 3 / 6 |

São unidades do produto da tabela: ouro do Nether entrega pepita, não lingote.
Fórmula: `1200 × quantidade / duração_em_ticks`. Pureza é 1–2, 3–6 ou 7–9
blocos iguais no 3×3 inferior. Cada melhoria dobra a vazão; passar da impura à
pura quadruplica. Combinadas, Mk.III pura rende 16× uma Mk.I impura.
Essas taxas foram preservadas: já oferecem uma progressão forte.

Em minério comum normal, cada unidade custa 240 FE na Mk.I/II e 300 FE na III.
A III compra espaço e vazão, não eficiência energética. Diamantes/esmeraldas
custam 20× o tempo comum; detritos, 40×. Pureza reduz o tempo e o custo por item.
Não aumentar indiscriminadamente esses rendimentos antes de medir o impacto
nos equipamentos vanilla e na economia da vila.

## Recompensa dos módulos

| Escolha | Retorno | Contrapartida |
|---|---|---|
| Velocidade | ciclo 40% menor, aproximadamente +66,7% de vazão | +80% de potência nominal e +50% de emissão por tick ativo |
| Eficiência | potência nominal −40%, emissão −50% | mesma velocidade e mesmos ingredientes |
| Produtividade, antes | lote extra a cada 10 ciclos (+10%) | ciclo +15%, potência +40% |
| Produtividade, agora | lote extra a cada 5 ciclos (+20%) | mantém ciclo +15%, potência +40% e emissão +20% |

Antes, a produtividade tinha vazão teórica de `1,10 / 1,15 = 95,7%` da máquina
normal. Agora, `1,20 / 1,15 = 104,3%`: pequeno ganho de vazão e bom ganho de
aproveitamento, sem competir com velocidade. Energia teórica por produto:
`1,15 × 1,40 / 1,20 = 1,342×`, aproximadamente +34,2%. Arredondamentos de ticks
e FE inteiros mudam os resultados, sobretudo nos ciclos curtos de extratoras.
Exemplo real: prensa de cobre com produtividade usa 35 ticks e 11 FE/t;
5 ciclos entregam 6 chapas por 5 cobres, a 1.925 FE no total.

O crescimento acumulado vem da cadeia: quatro etapas com produtividade rendem
um fator teórico de `1,2^4 = 2,0736` no aproveitamento de uma mesma ramificação,
contra `1,1^4 = 1,4641` antes. Isso não dispensa os outros ingredientes ou a
capacidade de cada estágio. Uma etapa lenta ainda limita a linha inteira.

Exemplo sem módulos: uma extratora normal Mk.I fornece 30 cobres/min; uma
prensa processa 40/min e um montador produz até 30 engrenagens/min, gastando
60 chapas/min. A linha inicial rende **15 engrenagens/min**, limitada pela
extração. Mk.II normal alimenta 60 cobres/min, mas exige duas prensas para
atingir as **30 engrenagens/min** do montador. Mk.III normal abre 120/min,
pedindo três prensas e dois montadores para **60 engrenagens/min**. Esses
exemplos exigem energia, entrada contínua e transporte compatíveis.

Uma esteira comum leva 150 itens/min, a rápida 300. Mk.III pura sem módulos
entrega 240: merece uma esteira rápida. Com velocidade, chega a 400/min e
ultrapassa até a rápida; precisa de rotas paralelas/retirada adequada. Isso é
um gargalo esperado, não perda autorizada de itens.

## Retorno da energia, comida e comércio

- Carvão rende 16.000 FE no gerador, aproximadamente 66,7 extrações comuns
  normais Mk.I/II se toda a energia for só para a extratora. Processar o minério
  reduz esse total. Vapor fornece 120 FE/t e cerca de 31.920 FE por carvão
  (266 ticks inteiros), quase o dobro: recompensa infraestrutura e água.
- Água/vento continuam opções limpas iniciais: 32 FE/t cobrem o exemplo do
  Grok de duas extratoras e duas prensas (28 FE/t). Duplicar linhas exige ampliar
  geração e distribuição, em vez de combustível automaticamente infinito.
- Solar tem pico de 16 FE/t, não uma entrega constante. À noite, a recompensa
  depende dos acumuladores; não comparar pico solar com geração permanente.
- Pão industrial usa 2 trigos, contra 3 na bancada. Venda de pão dá 6 pontos
  contra 2 dos trigos, mas exige moinho, cozinha e 640 FE por pão. Com aldeões
  próximos, cada pão rende 9 pontos: 20 pães → 3 esmeraldas. Agricultura, energia
  e processamento passam a sustentar o crescimento da vila.
- Exportação industrial preserva seu retorno e agora abre sem o chefe final.
  Isso é uma rota alternativa à venda de comida; não há evidência de partida
  suficiente para reduzir a recompensa de ambas ou inflar todos os custos.

## Correções necessárias para o retorno ser confiável

1. Antes, uma saída com 63 itens recebia o item normal e perdia o bônus que
   excedesse 64. Agora reserva espaço para o lote inteiro antes de gastar
   energia ou ingredientes. Escoar a saída retoma a mesma produção.
2. Frações de produtividade pertencem ao produto (`BonusItem`). Não é mais
   possível acumular ciclos baratos de carvão e transferir o bônus para sucata
   de netherita. Trocar de produto reinicia a fração ao completar o novo ciclo.
3. `Produced`, usado pelo medidor de itens/min, passa a contar unidades reais,
   incluindo lotes e bônus. Oito biscoitos deixam de aparecer como uma unidade.
   `Total` mantém a semântica de ciclos para não reinterpretar saves.
4. O texto do módulo de velocidade agora diz ciclo 40% menor. A afirmação
   anterior de “40% mais rápida” não correspondia à vazão implementada.

## Saves, validação e limites

IDs, receitas de construção, campos existentes e slots dos menus preservados.
Em saves sem `BonusItem`, a fração antiga sem identidade é descartada na
próxima conclusão com produtividade; nenhum item de inventário é retirado.
Contadores de ciclos mantidos. O primeiro intervalo de telemetria depois da
atualização pode misturar a contagem antiga e nova; os intervalos seguintes
usam itens reais. Os módulos já instalados passam a aplicar +20% automaticamente.

Testes offline exercitam os três níveis e as três purezas com produtividade,
saída cheia sem consumo, retomada sem perda, troca carvão/netherita sem bônus
emprestado e lotes de oito biscoitos no medidor. Foram aprovados 117 testes,
com uma prévia opcional ignorada; a conferência de conteúdo passou (475 JSON,
94 páginas, 58 texturas). Não medem FPS ou a dificuldade percebida no Horizon.

Validar em partida uma expansão cobre → engrenagens, a primeira linha de aço,
uma extratora III pura em esteira rápida, comida alimentando vila e exportação
sem enfrentar o Wither. Registrar tempo construindo versus esperando, energia
e estoques antes de ajustar custos de construção ou rendimentos raros. Este lote
oferece crescimento mais recompensador; não declara a economia inteira fechada.
