# Logística 0.9.1

Integra a produção 0.9 da nuvem e preserva o unificador da 0.8.1. As receitas de produção, geração de energia e agricultura continuam no módulo de produção.

## Entrada, saída e carga (0.28)

- **Máquinas:** a face de trás (para onde a máquina "olha" longe do jogador ao ser colocada) é a saída: a produção sai por ali e nada entra (`logistics::accepts_at`, chamado em `container_accepts`). As outras faces recebem. No modelo, moldura vermelha no bico de saída e faixas verdes nas faces de entrada (`appearance::draw`); extratoras só têm a vermelha.
- **Esteiras:** não aceitam nada pela frente (a face para onde andam). O divisor só recebe por trás. Uma esteira que encosta na saída de uma máquina não faz curva para dentro dela.
- **Carga por bloco:** matéria-prima (minério, pedra, toras: `logistics::is_raw`) até 5; produto pronto até o que a esteira move por vez (comum 1, rápida 2, expressa 4; divisor, unificador e túnel 2). O Horizon insere um item por vez e pergunta ao mod a cada um, então o limite é exato. A vazão não muda: esteiras vizinhas andam em fases alternadas.

## Curvas e rampas

Esteiras comuns e rápidas passam a reconhecer conexões vizinhas. Quando existe uma única entrada e uma única saída disponível, uma curva de 90 graus é orientada automaticamente. Em bifurcações ambíguas a direção escolhida ao colocar a peça é preservada. A conexão é recalculada a cada oito ticks, escalonada pela posição.

Para mudar a altura, coloque esteiras sobre blocos em degraus, com diferença de um bloco entre vizinhas. Na subida, a peça mais baixa se inclina até a superior. Na descida, a peça mais baixa recebe uma rampa que começa na altura da peça anterior. As rampas são retas; use uma peça plana para fazer a curva antes ou depois da rampa. É preciso deixar espaço acima da peça inferior.

O transporte acompanha o nível do destino, e os itens e ripas desenhados seguem a curva ou inclinação. A colisão também acompanha o formato e a orientação. Máquinas, divisores e unificadores conservam seus pontos de conexão no mesmo nível. Esteiras não atravessam um bloco sólido para procurar uma saída embaixo.

Botão direito com a chave gira e fixa a direção de uma esteira. Agachar e usar a chave devolve a orientação automática. Estoques são preservados nas duas operações. O campo NBT `ManualDirection` guarda essa escolha; a propriedade de estado `path` guarda `straight`, `left`, `right`, `up` ou `down`. Esteiras existentes usam `straight` como padrão e reconhecem as conexões no tick seguinte.

`tools/build_belts.py`, chamado por `tools/build_content.py`, gera as variantes dos modelos e as caixas de colisão. As formas respeitam o limite de 16 caixas por estado do Horizon. `logistics::path_point` usa os mesmos pontos de entrada e saída para a animação.

O guia inclui duas páginas de construção e controles. Recolher uma esteira entrega uma cópia atualizada uma vez por jogador, inclusive em saves que já receberam versões anteriores do guia.

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

## Verificações do primeiro lote 0.8.1

Conteúdo conferido (257 JSON, 72 páginas e 39 texturas); 23 testes offline aprovados, incluindo quatro cenários do unificador; tipos conferidos contra a API real; compilação completa pelo Horizon Launcher concluída com Horizon `66f3981`, Rust 1.99.1 e perfil `dev`. O jogo não foi aberto para testes automatizados; a validação visual e de interação fica com o usuário.

Na integração 0.9.1: 371 JSON, 85 páginas, 51 texturas e 40 testes offline aprovados. Os cenários novos cobrem subida com saída cheia e teto obstruído, descida sem atravessar parede, curva automática, direção manual e continuidade dos pontos da animação.

Compila??o completa da 0.9.1 conclu?da pelo launcher com Horizon `66f3981`, Rust 1.99.1 e perfil `dev`. O jogo n?o foi aberto para testes.
