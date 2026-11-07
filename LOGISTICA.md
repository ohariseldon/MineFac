# Logística 0.8.1 — primeiro lote

Trabalho na branch `logistics/merger`, sobre a base consolidada 0.8. A produção pode continuar em paralelo; esta entrega não altera receitas de processamento, geração de energia nem agricultura.

## Unificador de esteiras

O bloco `veio:merger` junta duas linhas em uma saída frontal. Cada entrada lateral guarda sua própria pilha, sem disputar o espaço da outra. A entrada A é amarela e a B é azul. Olhando na direção da saída, A fica à esquerda e B à direita.

Coloque o bloco olhando para o destino e conecte duas esteiras pelas laterais, apontando para ele. Ponha a esteira de saída em frente à ponta com a seta. Todas as conexões ficam na mesma altura. Use a chave com botão direito para girar o conjunto; os estoques e a próxima vez de saída ficam preservados no bloco.

O unificador alterna A e B após cada item aceito pelo destino. Se uma entrada estiver vazia, a outra usa a saída. Se o destino rejeitar um material, a outra entrada ainda pode entregar um material aceito. Quando tudo bloqueia, os itens permanecem nos estoques. São até dois itens a cada oito ticks, equivalentes a **5 itens/s** a 20 ticks/s. Use uma esteira rápida na saída para aproveitar a vazão de duas esteiras comuns. Uma esteira comum na saída limita a linha a 2,5 itens/s.

As duas entradas aceitam inserção automática apenas de suas respectivas laterais. Frente, traseira, topo e base recusam entrada automática. O painel permite abastecer ou retirar itens à mão. Funis podem retirar os estoques expostos; a alternância vale para a saída automática frontal do unificador. O mecanismo não precisa de eletricidade.

## Fabricação e ajuda

- Receita sem posição fixa: duas esteiras, uma engrenagem e um lingote de ferro produzem **dois unificadores**.
- A receita é liberada ao recolher uma engrenagem.
- O bloco está na aba criativa de logística.
- O painel identifica as entradas A/B e mostra o estado de espera ou bloqueio.
- O guia contém uso e fabricação. Recolher um unificador entrega uma cópia atualizada do guia uma vez por jogador, inclusive em saves anteriores.

## Pontos de integração com a nuvem

A simulação está em `native/api/src/logistics.rs`, com verificações de alternância, bloqueio, destino filtrado e orientação em `native/api/src/logistics/tests.rs`. As alterações pequenas em `lib.rs` apenas registram o bloco, os slots e seu filtro de entrada. A apresentação e os geradores acrescentam modelo, textura, registro, receita e ajuda.

Para integrar, preserve o ID `veio:merger`, os dois slots de entrada, o campo NBT `NextInput` e o campo de movimento `MovedAt`. Os menus continuam usando os mesmos 13 valores da base 0.8. Não troque um `lib.rs` inteiro por outra versão; combine as funções alteradas.

Braços de transferência, filtros configuráveis e prioridades continuam no planejamento seguinte. Este lote entrega o unificador.

## Verificações desta entrega

Conteúdo conferido (257 JSON, 72 páginas e 39 texturas); 23 testes offline aprovados, incluindo quatro cenários do unificador; tipos conferidos contra a API real; compilação completa pelo Horizon Launcher concluída com Horizon `66f3981`, Rust 1.99.1 e perfil `dev`. O jogo não foi aberto para testes automatizados; a validação visual e de interação fica com o usuário.
