# Fabricante e receitas por mistura

> **Integrado em 2026-11-09 na `claude/teste-combinado`, adaptado à versão atual.** No jogo a máquina se chama **Combinadora** (`veio:manufacturer`), para não confundir com a Fabricadora (`veio:fabricator`) da Grande Indústria. O motor (`veio:motor`, "Motor elétrico") é um item só: a Combinadora tem receitas novas para ele, e a do integrador continua valendo. A Combinadora é liberada pela pesquisa de Eletrônica e tem estrutura 3x3. As receitas dela ficam na wiki e no Guia da Grande Indústria, não no guia principal (que tem o limite de 100 páginas). Detalhes em `docs/consolidacao-mestra/notas/2026-11-09-claude-combinadora.md`.

Base: `53267af` (0.25.0). Lote de produção: a mistura de ingredientes passa a decidir o produto,
no espírito das máquinas de várias entradas e das receitas alternativas do Satisfactory.

## O que muda no jogo

- **Fabricante** (`veio:manufacturer`, nível aço): quatro entradas, uma saída e um slot de
  **sobra** (subproduto). Liberado pelo avanço dos circuitos. Montagem: integrador, duas chapas
  de aço, circuito e pistão.
- **Receita pela mistura.** Sem escolha no painel, a máquina faz a receita pronta com mais
  ingredientes: chapa + engrenagem dá placa reforçada; chapa + engrenagem + fiação dá a placa
  costurada, que rende o dobro. Vale para todas as máquinas de receita; nas antigas o resultado
  não muda (nenhuma receita delas é subconjunto de outra com prioridade diferente).
- **Fixar a receita.** A linha `< receita >` do painel é um botão: metade esquerda volta,
  metade direita avança; a posição 0 é a automática. Fixada, a máquina espera os ingredientes
  daquela receita e não troca sozinha.
- **Subprodutos.** O aço com escória devolve cascalho; os trilhos devolvem pepitas de ferro.
  Sobra cheia para a máquina (estado "saída bloqueada"), como no Satisfactory: o jogador
  precisa dar destino a ela (o cascalho volta a areia no triturador).
- **Peças novas** com uso: placa reforçada, rotor e motor. Entram no mercado (camadas 2, 3 e 4,
  no fim de `PRODUCTS`) e nas receitas de funis e carrinhos. Avanço "Linha pesada" ao montar um
  motor.

| Receita | Entradas (slots 1-4) | Produto | Sobra |
|---|---|---|---|
| Placa reforçada | 4 chapa de ferro, 2 engrenagem | 1 placa reforçada | |
| Placa costurada (alt) | + 4 fiação | 2 placas reforçadas | |
| Rotor | 1 chapa de aço, 4 engrenagem | 1 rotor | |
| Rotor de cobre (alt) | 6 chapa de cobre, 4 engrenagem, 2 fiação | 1 rotor | |
| Motor | 1 placa reforçada, 1 rotor, 8 fiação | 1 motor | |
| Motor elétrico (alt) | 1 placa reforçada, 1 rotor, 4 fiação, 1 circuito | 2 motores | |
| Aço com escória (alt) | 3 chapa de ferro, 2 carvão, 1 areia | 2 chapas de aço | 1 cascalho |
| Pistões | 1 chapa de ferro, 4 pedregulho, 3 tábuas, 1 redstone | 2 pistões | |
| Trilhos | 4 chapa de ferro, 2 graveto | 24 trilhos | 3 pepitas |
| Funis | 1 placa reforçada, 1 baú | 2 funis | |
| Carrinho | 1 placa reforçada, 2 engrenagem | 1 carrinho | |
| Pó de concreto | 4 cascalho, 1 farinha de osso, 4 areia | 10 pó de concreto branco | |

Números iniciais, para a frente de balanceamento calibrar.

## Contratos e arquivos tocados

- `production/mod.rs`: `Recipe` ganhou `id` e `byproduct`; `find` escolhe a receita mais completa;
  novas `picked`, `click`, `press`, constantes `MANUFACTURER`, `PICK_SHIFT`, `PICK_PREVIOUS`, `PICK_NEXT`.
  `process` grava `Info` (só no fabricante) e usa o segundo slot de saída para a sobra.
- `tools/production.json`: campos `id` e `by`; grupo `@planks`. `build_production.py` gera os ids
  (os sem `id` viram `máquina_produto`), confere entradas por máquina, sobra só onde há slot e
  **um slot fixo por ingrediente no fabricante** (a esteira entrega no primeiro slot que aceita).
- `lib.rs`: `screen_click` e `menu_button` repassam o fabricante para `production::click`/`press`.
- `presentation.rs`: moldura de botão atrás da linha da receita do fabricante.
- `operations.rs`: fabricante no setor Indústria.
- `workforce/society/trade.rs`: três produtos acrescentados ao fim (índices antigos preservados).
- Conteúdo gerado: `build_content.py`, `build_textures.py`, `build_tutorial.py` (ajuda, slots,
  nomes das receitas, sobra nas páginas de receitas). Guia: 97 páginas.

## Saves e migração

Nada a migrar. O NBT novo é `Pick` (0 por padrão). Receitas novas de uma máquina entram no fim
da lista dela, porque `Pick` guarda a posição.

## Verificações

- `python tools/check_rust.py`: 180 testes, 4 novos (mistura maior vence, sobra e bloqueio,
  receita fixada, rota dos slots e cliques).
- `python tools/check_project.py`: JSON, guia, traduções das receitas e texturas.
- `python tools/check_performance.py`: cenários idênticos.
- Falta: compilação pelo launcher e teste no jogo (clique no painel, esteiras nos quatro slots).
