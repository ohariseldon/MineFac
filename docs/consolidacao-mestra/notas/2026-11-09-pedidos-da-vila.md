# Os pedidos da vila: a vila em declínio apresenta as áreas do jogo

- **Data / frente / autor:** 2026-11-09 · tutorial / vila · Claude, thread "trusting-ritchie"
- **Onde:** branch `claude/trusting-ritchie-sc9ty8` (Veio 0.38.0)
- **Sistemas:** tutorial (prólogo em diante), moradores com nome, trocas vanilla · **FUN:** sem ID novo
- **Tipo:** código · conteúdo
- **Junta antes** a linha `claude/project-thread-hckdtj` (b46b13b: 16 etapas, favor da prensa, festas,
  resmungos, noite antiga) com os capítulos 2 a 6 desta linha (commit 1cf0f64).

## Por quê
Luiz (2026-11-09): aprofundar o começo em volta da vila decadente que o jogador veio reconstruir, usando
os moradores para apresentar as áreas do jogo: a vendinha sem insumo e sem freguês, o vaqueiro com uma vaca
só, a energia da carvoaria, o ferreiro pedindo ferro, a prefeitura com o telhado caindo (barro para o
telheiro). Em níveis: primeiro o telhado; enquanto ele está aberto, o prefeito sugere os outros.

## O que mudou
- `onboarding/errands.rs`: motor de pedidos com tarefas `Meet`, `Bring` (qualquer item da lista, somando),
  `Herd` (animais perto do morador, pelo registro do rancho), `Build` (tijolos perto do prefeito), `Place`
  (um bloco) e `Trade` (uma troca vanilla).
- 5 pedidos (ver a tabela na wiki, "Primeiros passos"): telhado (Dona Zefa, `veio:potter`), vendinha
  (Seu Zé, `veio:grocer`), rebanho (Seu Tonho, `veio:cowherd`, e a vaca Mimosa), carvoaria (Seu Lino,
  `veio:collier`), forja (Seu Bento, depois do favor; o clique no Bento com o favor feito vai para os pedidos).
- Nível 1: 10 s depois do primeiro encontro, o prefeito conta da vila em declínio e pede o telhado
  (`errands::queue`). Nível 2: a cada conversa com o prefeito, uma sugestão nova (`errands::suggest`).
  Nível 3: com os 5 feitos, "A vila respira" (título, fogos, 100 de experiência).
- Faíscas verdes em quem tem algo para o jogador; perto (8 blocos), a barra mostra o pedido; o caderno em
  `/veiotutorial pedidos`. Os pedidos correm em paralelo ao tutorial (laço próprio no fim do `tick`), menos na
  primeira noite.
- Recompensas: itens (telhas, carvão vegetal, tesoura, cordas), receitas (roda d'água, posto de pastor,
  marcador de cercado), experiência e as trocas vanilla de cada morador (`data merge Offers`).
- Resmungos dos aldeões: assuntos `errands` e `revived` (`barks.rs`).
- **Save:** uma linha própria no fim, `pedidos a b c d e tijolos marcas`, sem mudar a versão (v11). Saves
  sem ela abrem com os pedidos por fazer.
- **Save v11** (a junção): v7, v8 e v9 das duas linhas se distinguem pela contagem de números.

## Evidência
- E1: `python tools/check_rust.py` → 383 testes. Novo:
  `the_village_errands_teach_bricks_commerce_cattle_and_power_in_any_order` (a pausa antes do telhado, uma
  sugestão por conversa, forja só depois do favor, entrega parcial devolvida, telhas só perto do prefeito,
  trocas, rebanho, roda d'água, "a vila respira", o save com e sem a linha).
- J: não testado em jogo. A confirmar no PC: a troca vanilla nos tipos com template de aldeão, o evento de
  troca chegar com o tipo `veio:grocer`, a Mimosa no pasto, onde os quatro aparecem na vila.

## Efeito na economia
- As trocas vanilla dos moradores criam esmeraldas (como a da Dona Cida). Elas entram na economia do Veio só
  se o jogador depositar no cofre. Os pedidos não dão esmeralda direto.

## Para a outra frente
- Nada nas 16 etapas mudou. Os pedidos não ocupam a barra do tutorial, só perto do morador.
- Bento: `favors::talk` continua igual; `onboarding::neighbor_talk` manda o clique para `errands` quando o
  favor está feito.
