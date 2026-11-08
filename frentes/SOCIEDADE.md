# Sociedade da vila (0.18.0; contas, preços, economia fechada e comércio exterior até a 0.23.0)

A vila deixa de ser só mão de obra. Cada família mora numa **casa**, recebe **salário** dos postos onde
seus adultos trabalham, compra **comida e bens** nas **lojas** e vive melhor ou pior conforme o que a
fábrica entrega. O **prefeito** mede tudo isso e decide até onde a vila pode crescer.

O objetivo é um planejador de automação com uma vila viva (Factorio/Satisfactory com um pouco de
Stardew Valley): o jogador continua livre para construir onde quiser, mas a geografia e o bem-estar
viram restrições reais. Tudo é simulado por família e por bloco, nunca por aldeão a cada tick.

## A moeda e a economia fechada (0.21)

A **esmeralda é a moeda** (1 esmeralda = 60 moedas). Ela é rara no Minecraft (vem das trocas e,
em breve, das missões dos aldeões e da praça do mercado), então a economia não fabrica dinheiro:
ele **circula**.

```
          salários                compras (varejo)
companhia ────────▶ famílias ────────────────▶ lojas (comerciantes independentes)
    ▲                  ▲                          │
    │                  │ prefeito, assistência    │ atacado (70% do preço local)
    │                  │                          │
    └── atacado ───────┼──────────────────────────┘
                       │
                  prefeitura ◀── imposto retido na fonte em cada venda (4%, 8%, 12%, 16%)
```

| Livro | Entra | Sai |
|---|---|---|
| Companhia (cofre do centro de controle) | atacado pago pelas lojas (menos o imposto); depósitos de esmeralda | salário dos trabalhadores da companhia; saques em esmeralda |
| Lojas (cada comerciante) | varejo das famílias e do jogador (menos o imposto) | atacado à companhia; salário do próprio comerciante |
| Prefeitura | imposto na fonte; aportes em esmeralda | salário do prefeito; assistência às famílias pobres |
| Famílias (carteira da casa) | salários; assistência | compras nas lojas |

- **Identidade contábil:** todo pagamento tira de um livro e põe em outro. A soma de todos os saldos
  só muda pelas bordas: esmeraldas que entram (depósito, aporte, compras do jogador) ou saem (saque).
  O relatório da prefeitura mostra a moeda em circulação por livro e as bordas, e um teste confere que
  nada aparece nem some.
- **Imposto na fonte:** não há como deixar de pagar. Em cada venda, a parte da prefeitura sai na hora.
- **Comerciantes independentes:** o que a indústria (você, à mão, funil ou carregador) põe no estoque
  da loja é uma venda no atacado. A loja paga com o caixa, primeiro a empresa mais antiga; sem caixa, o
  valor fica fiado e é pago com as vendas. O comerciante tira o salário do caixa da loja.
- **Sem dinheiro do nada:** a verba inicial da prefeitura e a manutenção dos poços saíram (criavam ou
  sumiam com moeda). A prefeitura começa vazia e vive do imposto.
- **Lucro por empresa da holding:** a receita de cada empresa é o atacado dos itens que ela produz
  (comida, lã e couro: agro; madeira: extrativismo; minério: mineração; o resto: manufatura); os gastos
  são os salários dos trabalhadores dos postos dela e o imposto. O centro de controle mostra receita,
  gastos e lucro do dia em cada aba.
- **PIB:** consumo final (o varejo do dia). O atacado aparece como produção da indústria.

## Comércio exterior (0.23, no estilo das conexões externas de Cities: Skylines)

**O problema.** A vila é pequena: a indústria satura a demanda local rápido; a moeda fica presa
(só cresce com esmeraldas depositadas, inclusive as mineradas); e, sem referência de fora, os preços
locais podem disparar (inflação) ou desabar. Também queremos que a geografia e a logística pesem e
que o jogo puxe para produtos cada vez mais complexos.

**A solução, em quatro partes, e por que cada uma resolve:**

| Parte | Como funciona | O que resolve |
|---|---|---|
| Conexões externas | *Porto de carga* só opera com mar ou rio largo em volta (20 blocos de água); *estação de carga* só na fronteira, a 96 blocos da sede e das prefeituras. Porto: navio a cada 2 min, 9 pilhas, frete 4 + 1 por pilha. Estação: trem a cada minuto, 6 pilhas, frete 2 + 2 por pilha. | A geografia vira problema real: é preciso levar a carga até o mar ou a fronteira (esteiras, carregadores, capilaridade). O frete premia volume e valor. |
| Bolsa mundial | Cotação = valor base x complexidade (1x a 4x) x índice do dia (80% a 120%, igual para todos) x saturação (o que você exportou nos últimos dias derruba o preço; os mercados se recuperam em alguns dias). | Um sinal de preço de fora: o jogador vê o que vale exportar e não ganha inundando um item só. |
| Contratos e reputação | Até 3 contratos: um produto, um lote e um prazo (3 a 4 dias), pagando 50% a mais. Cumprir: reputação +1; deixar vencer: -1. A reputação abre os mercados de componentes (1), eletrônicos (3) e módulos (6). | Progressão no estilo do elevador espacial: metas claras que puxam para cadeias mais longas e de mais valor agregado. |
| Importação automática | Se falta comida nas lojas e há uma conexão, o comerciante importa pão a preço mundial + 20% de frete + 10% de tarifa; a tarifa fica na prefeitura, o resto sai da economia. Com conexão, o preço local nunca passa dessa paridade de importação. | Teto para os preços (fim da inflação descontrolada) e a moeda que sai quando a vila não produz. Produzir localmente fica mais barato que importar. |

Resultado: o preço local fica **entre a paridade de exportação (piso: abaixo disso, compensa
exportar) e a de importação (teto)**. Exportar traz moeda nova, importar e o frete levam moeda embora,
e tudo passa pelas bordas do `ledger`, então a identidade contábil continua valendo (testes conferem
com o porto, o frete e a importação).

**Onde ver:** a aba *Comércio exterior* do centro de controle (reputação, exportação e importação do
dia, saldo comercial e os contratos); o relatório do centro sugere os itens guardados que mais rendem
na bolsa hoje; o relatório da prefeitura separa a moeda que entrou por exportação.

**Para as outras frentes:** `society::trade::{PRODUCTS, quote, import_price, export, import}`. Produto
novo entra em `PRODUCTS` com a sua camada; a praça do mercado e as missões usam o mesmo caminho.

### Para integrar depois

- **Missões e praça do mercado (outra frente):** devem pagar em moedas pelas mesmas contas (a praça
  comprando bens da companhia = atacado; missões pagando a companhia = entrada de esmeralda como borda).
- **Balanceamento:** salário (2 moedas por turno), margem do atacado (70%), impostos e preços
  ficam para uma rodada de balanceamento.

## Preços locais (oferta e procura)

Cada loja guarda um fator de preço por item (50% a 200%). Uma vez por turno ele anda um quarto do
caminho até o alvo, calculado pela **cobertura**: estoque / procura média do dia.

- 1 dia de cobertura = 100%; metade do estoque necessário = ~140%; sem estoque = 200%.
- Estoque parado (ninguém compra) barateia 5 pontos por turno até 70%.
- Famílias que queriam comprar e não acharam (sem dinheiro ou sem estoque) somam 25 pontos ao alvo
  da comida.
- A procura vem das casas em volta, então cada vila tem seu mercado: o pão pode estar caro numa
  vila e barato em outra, e vale a pena levar comida até onde falta.
- O jogador compra pelo mesmo preço local (as ofertas do comerciante se refazem quando mudam).
- O prefeito mostra o **custo de vida** (média dos preços da comida das lojas da área).

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
