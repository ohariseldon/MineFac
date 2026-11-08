# Continuação: obras, funcionários, ruínas e proteção

Branch `cloud/obras-planta` (base: `origin/main` 542b0a0). Testes: `python tools/check_rust.py` (178 ok).

## Feito
- Plantas redesenhadas com interiores (~25% maiores), 9 prédios + Centro de pesquisa (`lab`).
- `workforce/staff.rs`: funcionário no canteiro concluído (cozinheira, professor, ferreiro, bibliotecário,
  carteiro, estalajadeiro, cientista); visitantes nos assentos no horário (alunos crescem na escola);
  pesquisas (Agronomia, Logística, Nutrição, Pedagogia, Engenharia: cientista instala módulos).
- Canteiro com 3 slots (comida, entrada, produto). Efeito da obra só vale **funcionando**
  (`site::near().operating`); centro mostra "de pé · parado" (`veio.center.project.idle`, bit 26).
- Projeto `lab` (bit 512, Vila), pesquisas nos bits 20+ dos efeitos (`projects::research_bit`).

## Feito na continuação (branch `claude/project-thread-6fbc15`)
1. **Traduções** (pt_br e en_us, em `tools/build_tutorial.py`): `veio.site.no_staff`, `veio.site.next.<role>`,
   `veio.center.project.idle`, `veio.project.lab.name/effect`, `veio.research.<key>.name/effect`. O aviso de
   pesquisa concluída da vila usa **`veio.research.lab_done`**: `veio.research.done` já era da pesquisa da
   grande indústria (`research.rs`, 1 argumento). Menu do canteiro com 3 slots em `tools/build_content.py`, ajuda
   dos slots, rótulos do centro nos slots 6 (obra) e 7 (decreto). Guia da vila: Funcionários, Quem trabalha,
   Centro de pesquisa, Engenharia, Ruínas e território, planta do centro de pesquisa e um índice das plantas.
2. **Ruínas** (`workforce/ruins.rs`): como no desenho acima, com estas escolhas: até 3 ruínas de obras de nível 0
   (refeitório, escola, ferraria, praça); raios 50/56/62 (dentro do anel 48–80, para o centro de controle no meio
   da vila alcançar a 64); só mundos novos (tempo de jogo até 24000); `execute if block ... #minecraft:replaceable
   run setblock` no lugar do `keep` (a grama baixa da planície faria o `keep` falhar); `<mundo>/veio/ruins.txt`
   guarda que já foi feito e os canteiros pendentes (sobrevive a reabrir o mundo). Carimbo: camadas 0-3 com
   90/70/45/20%, sem ar, móveis (PROPS), blocos protegidos ou que se apoiam em outros.
3. **Proteção** (`workforce/territory.rs`, ganchos em `lib.rs` e no substituto): território = planta + 3 em todos
   os eixos, para `Fixed` e BUILDING/DONE; `Building` ganhou `area`, `fixed` e `counts` (ruína à espera só
   publica o território; `near()` só conta `counts`). O canteiro do jogador pode ser quebrado (o prédio sai do
   registro na hora); o da ruína não. Numa ruína à espera, blocos que o construtor não derruba podem sair.
4. **Docs**: TRABALHADORES.md (décima entrega) e README. Checks: `check_rust`, `check_project`,
   `check_performance` passam.

## Falta
- Testar no jogo (nada foi testado no jogo): ruínas aparecendo e sendo reconstruídas, ganchos de proteção e as
  assinaturas reais deles no Horizon (em especial o retorno de `explosion_detonate` e o tipo de `face`).
- Zip consolidado (fora desta continuação).

## Integração com a main 0.22.0 (feita)
- Centro de controle com 8 espaços: 0 comida, 1-3 metas, 4 cofre, 5 saque (da 0.21), **6 obra**, **7 decreto**
  (`projects::SITE`/`DECREE` mudaram de 5/6 para 6/7).
- Estados do canteiro mudaram para 36-39 (`site::SITE_*`); 32-35 são das conexões externas (0.23).
- Resumo do centro: linha 4 é a da companhia (caixa e exportação do dia, clique saca 1 esmeralda);
  a comida foi para a linha de aldeões. `DA5` traz (fila, ocupação+1) e `DA6` a exportação do dia (`center::refresh`).
- Economia fechada: `center::add_coins` é borda (esmeraldas de verdade); recompensas de pedidos e o
  rendimento da Prefeitura saem do caixa público (`center::reward`). Sem prefeitura ou sem saldo, só prestígio.
- Menu do canteiro com 3 espaços em `tools/build_content.py`: feito.
