# Sociedade da vila (0.18.0)

A vila deixa de ser só mão de obra. Cada família mora numa **casa**, recebe **salário** dos postos onde
seus adultos trabalham, compra **comida e bens** nas **lojas** e vive melhor ou pior conforme o que a
fábrica entrega. O **prefeito** mede tudo isso e decide até onde a vila pode crescer.

O objetivo é um planejador de automação com uma vila viva (Factorio/Satisfactory com um pouco de
Stardew Valley): o jogador continua livre para construir onde quiser, mas a geografia e o bem-estar
viram restrições reais. Tudo é simulado por família e por bloco, nunca por aldeão a cada tick.

## Ciclo da economia

```
esmeraldas do jogador ─▶ cofre da prefeitura ─▶ salários ─▶ carteira da casa
        ▲                                                         │
        └──── caixa da loja (60 moedas = 1 esmeralda) ◀── compras ┘
                         ▲
        comida e bens que a fábrica e os trabalhadores põem na loja
```

- **Moeda:** moedas virtuais. 1 esmeralda = 60 moedas (o mesmo preço da despensa: 60 pontos de comida
  por esmeralda). Comida custa 1 moeda por ponto de comida (pão = 6). Bens custam o valor da tabela.
- **Salário:** a cada turno da casa (1 min, 1/20 do dia), cada morador empregado ganha 2 moedas,
  pagas pelo cofre da prefeitura. Sem dinheiro no cofre, o salário fica devendo e a renda cai.
- **Consumo:** cada morador come 1 ponto de comida por turno (20 por dia) e gasta conforto
  (bens) aos poucos. A casa compra quando a despensa baixa: vai à loja mais próxima com estoque, paga
  da carteira e um morador ocioso anda até a loja.
- **Loja:** posto com um comerciante contratado. Recebe estoque à mão, por funil ou por carregador.
  Vende às casas e ao jogador. A cada 60 moedas no caixa sai 1 esmeralda no slot do caixa.
- **Comércio com o jogador:** o comerciante tem a tela de troca do Minecraft com o que a loja tem
  (uma oferta por item, preço em esmeraldas, estoque real). Cada troca tira o item da loja e põe o
  valor no caixa.

## Casas e bairros

- **Casa** (bloco): até 4 moradores. Os aldeões a até 10 blocos viram moradores (cada aldeão mora em
  uma casa só). Despensa de 3 espaços (comida que a família comprou; o jogador também pode doar).
- **Qualidade da moradia:** lotação, saneamento (poço a até 24 blocos), energia (poste a até 12),
  comércio (loja a até 40) e ar (poluição do chunk). Casa com mais de 6 vizinhas a 16 blocos fica
  "apertada": a vila densa pesa no bem-estar, e bairros espalhados funcionam melhor.
- **Bairros:** grupos de casas a até 24 blocos umas das outras. O relatório do prefeito mostra cada
  bairro com população e IDH, então serviços precisam chegar a cada bairro (capilaridade).

## Prefeito e plano diretor

A **prefeitura** é um posto com o prefeito contratado. Cobre 128 blocos. Ele não é inimigo: é o Estado
cuidando do bem-estar, e por isso põe limites indiretos.

| Fase | Para chegar | Libera |
|---|---|---|
| 1. Povoado | (sem prefeitura) | zonas até nível 2, até 30 aldeões, 4 alvarás de casa |
| 2. Vila | prefeitura, 12 moradores, 3 famílias, IDH 0,40 | zonas até 3, 60 aldeões, 10 alvarás |
| 3. Cidade | 36 moradores, 9 famílias, IDH 0,55, PIB 400/dia, emprego 60% | zonas até 4, 100 aldeões, 24 alvarás |
| 4. Metrópole | 80 moradores, 20 famílias, IDH 0,70, Gini até 0,45, 3 bairros | 150 aldeões, 48 alvarás |

- **Alvarás:** só casas com alvará recebem moradores. A partir da fase 2, uma casa só ganha alvará com
  saneamento; da 3, também com energia; da 4, também com comércio perto.
- **Licença suspensa:** se o IDH cair abaixo do exigido pela fase atual, o prefeito congela novos
  níveis de zona e nascimentos até a vila se recuperar.
- **Pedidos:** o prefeito sempre aponta o pior problema (fome, salário atrasado, sem saneamento,
  desemprego, sem-teto, lotação, poluição) com números, no painel e no relatório (clique com livro).

## Indicadores (prefeitura)

- **População:** moradores, adultos, crianças, famílias, sem-teto (aldeões a 128 blocos sem casa).
- **Emprego:** adultos empregados / adultos.
- **PIB (moedas/dia):** produção (tarefas dos trabalhadores x 2 moedas) + comércio (vendas das lojas),
  medido no último dia.
- **IDH (0 a 1):** média geométrica de quatro dimensões por família, ponderada pelos moradores:
  alimentação (despensa e variedade), moradia (alvará, lotação, ar), serviços (saneamento, energia,
  comércio) e renda (renda por morador frente ao custo de vida).
- **Gini:** desigualdade da renda por morador entre as famílias.
- **Global:** a soma de todas as prefeituras da dimensão aparece no relatório.

## Custo

Casa: um turno por minuto, lê só o censo de posições que já existe e no máximo 4 NBT. Loja: atualiza
as ofertas do comerciante só quando o estoque muda. Prefeitura: soma registros em memória a cada
turno. Nada roda por aldeão a cada tick.

## Integração com a frente de obras (outro agente)

A frente de obras da vila (prestígio, biblioteca, correio, estalagem, prefeitura, escola, mercado...)
ainda não está no `main`. Quando chegar: a prefeitura daqui deve virar a mesma obra, os serviços
(escola, refeitório, mercado) entram como novos itens de "serviços" do IDH, e o prestígio pode somar
às exigências das fases.
