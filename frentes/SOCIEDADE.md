# Sociedade da vila (0.18.0, contas e preços na 0.19.0)

A vila deixa de ser só mão de obra. Cada família mora numa **casa**, recebe **salário** dos postos onde
seus adultos trabalham, compra **comida e bens** nas **lojas** e vive melhor ou pior conforme o que a
fábrica entrega. O **prefeito** mede tudo isso e decide até onde a vila pode crescer.

O objetivo é um planejador de automação com uma vila viva (Factorio/Satisfactory com um pouco de
Stardew Valley): o jogador continua livre para construir onde quiser, mas a geografia e o bem-estar
viram restrições reais. Tudo é simulado por família e por bloco, nunca por aldeão a cada tick.

## Duas contas que não se misturam (0.19)

O jogador é a **companhia**: um empresário que contrata e produz. A prefeitura é o **governo**.

| | Companhia (sua) | Prefeitura (pública) |
|---|---|---|
| Onde fica | cofre do centro de controle (sede) | cofre da prefeitura |
| Entra | esmeraldas que você deposita; as lojas devolvem esmeraldas no caixa | impostos; verba inicial de 300 moedas; aportes voluntários |
| Sai | salário de quem trabalha nos seus postos (lavrador, carregador, pastor, capataz, quartel, comerciante, administrador) e os impostos | salário do prefeito (funcionário público), manutenção dos poços e assistência às famílias pobres |
| Sem dinheiro | salários atrasados (a família fica sem renda) | prefeito sem salário; poços sem manutenção deixam de contar como saneamento |

```
companhia ──salários──▶ famílias ──compras──▶ lojas (caixa: esmeraldas para você)
    │
    └──impostos (fase: 0%, 8%, 12%, 16% da produção e das vendas)──▶ prefeitura
                                     prefeito, poços, assistência ◀──┘
```

- **Moeda:** 1 esmeralda = 60 moedas, nas duas contas.
- **Imposto:** a cada turno, a prefeitura cobra a taxa da fase sobre a produção (tarefas x 2) e as
  vendas da sua área, da conta da companhia. O que faltar vira **dívida**, cobrada primeiro no
  turno seguinte. Dívida maior que um dia de imposto suspende as licenças (zonas e nascimentos).
- **Assistência:** com mais de 200 moedas no cofre público, famílias com renda baixa recebem 1 moeda
  por morador por turno: o imposto volta como bem-estar e reduz o Gini.
- **Funcionário público:** o prefeito. Os demais trabalhadores são da companhia.

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
