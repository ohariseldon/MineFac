# 08/11/2026 — PR #47: correções do primeiro teste real no PC

Primeira compilação real da build combinada (#47 + #51 + #52 + #53) pelo Horizon Launcher, no PC do Luiz (RTX 3050, Horizon 66f3981, perfil performance). Os testes offline passavam. O jogo real encontrou os problemas abaixo.

| O que mudou | Por quê | Commit |
|---|---|---|
| `with_nbt_key`, `FastHasher` e `FastHash` em lib.rs passam de `pub(super)` para `pub(crate)` | lib.rs é a raiz do crate e o rustc recusa `super` ali (E0433) | 3f7cb28 |
| `ladders::climb` recebe `on_ladder`, e quem chama usa `m.on_climbable(cx.w)` | No Horizon real, `Mob::on_climbable` é método que lê o mundo, não campo (E0615). O substituto offline tinha o campo errado e foi corrigido | 3f7cb28 |
| A junta dos canos usa a textura `copper_block` | `waxed_copper_block` não existe no vanilla, e o jogo mostrava a textura ausente | df9b3a7 |
| O launcher grava `pickedShaderPack:veio-industrial` (nome da pasta), e a revisão do perfil sobe para 2 | O Horizon junta o valor direto à pasta shaderpacks. `horizon:veio_industrial` dava "os error 2" e o Realismo nunca carregava | 53bccaa |
| O launcher apaga `shaderpacks/veio-industrial` antes de copiar o pacote | O Horizon recusa pacote com `.wgsl` que o manifesto não cita, e a cópia por cima deixava shaders da 2.1 | 4b2133d |

Fora do #47: o pacote Realismo passou para 3.0.1, com 4 saídas de pós-processamento, que é o limite do Horizon (`MAX_POST_TARGETS = 4`). Isso está no #51, commit f85b918. Ele carregou no jogo.

## Pendências
- O `launcher.exe` do repositório ainda é o antigo. O novo foi compilado no PC (`E:\Veio-fix`, SHA256 C8ACCD1E…), mas não foi publicado.
- Sons do Veio não tocam no jogo. Os arquivos estão instalados. Em investigação: lista de ouvintes vazia (`server_players`) ou fila de sons do Horizon.
- O céu do Realismo 3.0.1 está "quebrado" e o brilho e o contraste estão estranhos, segundo o Luiz. Aguardando capturas. A correção é da frente de visual (#51).
- Lição: o substituto offline do Horizon pode divergir da API real. Ao acrescentar API nele, confira no fonte do 66f3981.
