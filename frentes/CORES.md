# Cores: corantes, tintas e peças pintadas

Base: `50f1bd4` (cidade viva, sobre a 0.34.0 do PR #61). Algumas peças passam a pedir cor, e a cor
vem de uma fábrica própria, que se liga à produção final.

## A linha

| Etapa | Máquina | Receita |
|---|---|---|
| Pigmento | Moinho | flor vermelha ou beterraba → 2 corantes vermelhos; flor amarela → 2 amarelos; farinha de osso ou lírio → 1 branco; bolsa de tinta ou rosa de Wither → 2 pretos; centáurea → 2 azuis; tulipa laranja → 2 laranjas |
| Pigmento | Triturador | lápis-lazúli → 2 corantes azuis; carvão → 1 preto (o cacto na fornalha elétrica já dava verde) |
| Tinta | **Misturador de tintas** (novo, cobre) | 2 corantes + 1 bola de argila → 2 latas (7 cores); alternativas: tinta azul + amarela → 2 verdes, vermelha + amarela → 2 laranjas |
| Pintura | **Cabine de pintura** (nova, ferro) | 4 engrenagens + 1 lata preta/branca → 4 engrenagens pretas/brancas; 4 chapas de ferro + vermelha/amarela → 4 chapas vermelhas/amarelas; 2 chapas de aço + azul → 2 chapas de aço azuis; 8 lãs brancas, terracotas ou vidros + 1 lata → 8 tingidos |
| Bens finais | Integrador | 2 engrenagens pretas + 1 branca → relógio de parede; moldura + chapa amarela → 2 placas de sinalização |
| Bens finais | Fabricadora | 2 chapas vermelhas + 2 molduras + 4 engrenagens pretas → bicicleta |
| Cadeia funda | Fabricadora | **kit Mk.II** pede 8 engrenagens **pretas** (antes: comuns); **núcleo de automação** pede 4 chapas de aço **azuis** (antes: aço) |

Uma lata pinta 4 peças: um misturador abastece três cabines (razão inteira, como nas outras linhas).

## Onde entra no jogo

- **Desbloqueio:** avanço "Fábrica de cores" (filho do aço) libera as duas máquinas. Depois vêm
  "Pintado na linha" (primeira peça pintada) e "Bens de consumo" (relógio, placa ou bicicleta).
- **Mercado:** 13 produtos novos no fim de `trade::PRODUCTS` (índices antigos preservados): 5 tintas
  (camada 1), 5 peças pintadas e a placa (camada 2), relógio e bicicleta (camada 3). Entram nos
  contratos e nos preços das lojas.
- **Conforto:** relógio (90 moedas, 60 de conforto) e bicicleta (210, 150) entram em `GOODS`: as
  famílias compram nas lojas, então a cidade passa a puxar a fábrica de tinta.
- **Guias:** o guia principal está no limite de 100 páginas, então as receitas de cor (`"book":
  "industry"` no `production.json`) vão para o **Guia da Grande Indústria**: páginas "Cores",
  "Linha de pintura", receitas por máquina e as duas máquinas. Wiki (tecla J e `docs/wiki`) com o
  artigo "Cores e tintas" e textos de todos os itens.
- **Visual e som:** texturas das 7 latas, peças pintadas, relógio, placa e bicicleta; frentes das
  máquinas; silhuetas (misturador como a cozinha, cabine como o montador); névoa lilás na cabine;
  sons reaproveitados (cozinha e montador). A cabine polui como uma fornalha (solvente).

## Arquivos

- `tools/production.json` (receitas, grupos de pigmento, campo `book`), `data.rs` gerado.
- `production/mod.rs`: `PAINT_MIXER`, `PAINT_BOOTH` nas tabelas (como a cozinha: 2 entradas, 1 saída).
- `operations.rs` (setor Indústria), `ambience.rs`, `society/trade.rs`, `society/mod.rs` (`GOODS`).
- Geradores: `build_content.py`, `build_textures.py`, `machine_forms.py`, `machine_audio.json`
  (só a tabela nativa foi regerada; os .ogg não mudam), `build_tutorial.py`, `build_wiki.py`,
  `wiki_articles.py`.
- Testes novos em `production/tests.rs`: da flor à chapa vermelha, mistura de cores e tingimento,
  a cadeia funda e os bens pedem peças pintadas, e cada ingrediente tem um slot só nas máquinas que
  dividem itens (a esteira entrega no primeiro slot que aceita).

## Saves e balanceamento

Nada a migrar. Quem já tinha kits Mk.II e núcleos fica com eles; as próximas receitas pedem as peças
pintadas. Os números (tempo, energia, preço, conforto) são iniciais, para a frente Grok calibrar.
