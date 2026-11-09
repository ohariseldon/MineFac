# Primeiro ato com um favor: a prensa do Seu Bento pelo campo da Dona Cida

- **Data / frente / autor:** 2026-11-09 · tutorial · Claude, thread "Tornar o início de jogo claro e envolvente"
- **Onde:** branch `claude/project-thread-hckdtj` (Veio 0.37.0), PR #61
- **Sistemas:** tutorial (prólogo e etapa 2), aldeões com nome · **FUN:** sem ID novo
- **Tipo:** código · conteúdo
- **Continua** `2026-11-09-tutorial-festas-prefeito.md`. O capítulo 3 (cidade) não mudou.

## Por quê
Luiz (2026-11-09): o primeiro loop deve ficar mais fácil. Em vez de entregar tudo pronto (gerador, prensa e cabos), ele quer uma sequência de fatos: o prefeito monta o gerador, a prensa está com alguém da vila, e essa pessoa pede uma side quest que ajuda a comunidade, como uma fazenda para baixar o preço da comida.

## O que mudou
- O prefeito agora só monta o gerador e libera a receita dele. A prensa e os cabos saem do favor.
- **Favor** (`onboarding/favors.rs`). Ele ocupa a etapa 2 ("Ligar uma máquina") até a prensa chegar, em `Progress::favor`, de 0 a 6:
  1. O prefeito manda falar com o **Seu Bento, o ferreiro** (`veio:smith`, a leste da praça).
  2. O Bento tem uma prensa velha, mas o pão está caro porque a **Dona Cida, a lavradora** (`veio:farmer`, a oeste), não dá conta da roça. Ele pede que você ajude a Cida.
  3. A Cida dá 16 sementes, 32 farinhas de osso e uma enxada de pedra. Ela pede 12 pés de trigo a até 10 blocos dela, que a barra conta a cada 5 s.
  4. Você colhe e entrega 12 trigos. Se faltar, ela devolve o que pegou.
  5. A Cida passa a vender pão mais barato: a troca dela vira 6 pães por esmeralda (`data merge Offers`), e a fala conta que antes eram 2. O clique nela depois disso abre a troca vanilla.
  6. O Bento entrega a prensa e 16 cabos, libera as duas receitas e dá 30 de experiência, com o título "Favor feito!".
- A barra mostra "Favor · ..." com a seta e a distância até a pessoa.
- O prefeito, quando clicado, lembra quem procurar.
- Os aldeões comuns comentam o favor (assunto `favor`).
- Os dois vizinhos são tipos novos com template de aldeão (`tools/build_content.py`), em lugares fixos a partir do centro da vila (o chão mais baixo e seco entre 4 colunas). As cópias a mais são removidas como as do prefeito.
- **Save v9:** guarda `favor`.
  - Os saves v1 a v8 que já passaram do pedido de recursos ficam com o favor feito, porque já tinham as receitas.
  - Os que ainda não passaram fazem o favor.
- Wiki (artigo do começo) e README.

## Evidência
- E1: `python tools/check_rust.py` → 376 testes passando. O teste novo `the_press_comes_from_the_smith_after_a_wheat_field_for_the_farmer` cobre:
  - a ordem das conversas e os presentes;
  - a contagem do campo (o trigo longe não conta);
  - a entrega parcial devolvida;
  - a troca do pão;
  - a prensa e os cabos;
  - o clique que abre a troca;
  - o save e os saves antigos.
- Os 3 ignorados passam com `--ignored --test-threads=1`. `python tools/check_project.py` ok.
- J: não testado em jogo. A confirmar no PC:
  - se a troca vanilla abre num tipo de mod com template de aldeão;
  - o `data merge` das ofertas;
  - se `host.block` lê o trigo plantado;
  - onde os dois aparecem na vila.

## Efeito na economia
- Por enquanto o preço mais baixo é o da troca da Dona Cida.
- As lojas do Veio (`market.rs`) só chegam no capítulo 3. Quando chegarem, o trigo do campo que entra no estoque baixa o preço pela regra de cobertura que já existe.

## Toca no compilado
- Pacote `release/Veio-0.37.0.zip` no lugar do 0.36.0.
