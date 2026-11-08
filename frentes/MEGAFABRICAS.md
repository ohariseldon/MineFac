# Megafábricas: copiadora, marco de obra e fábricas verticais

Pedido do usuário: a "copiadora" do Satisfactory (Blueprint Designer) para montar
megafábricas, com projetos que o jogador monta e cola pagando os componentes do
próprio inventário; fábricas verticais em que os aldeões sobem e descem escadas de
mão para pegar e entregar recursos, com a alternativa de um elevador de carga.

## Copiadora (`veio:blueprint_designer`)

O volume acima da copiadora é a prancheta: um quadrado de 5, 9, 15 ou 21 blocos
centrado nela, com até 24 de altura, marcado no mundo por uma moldura ciano.

| Ação | Efeito |
|---|---|
| Papel na mão + clique | Copia a prancheta. O chat resume blocos, materiais e o que ficou de fora |
| Agachado + chave | Muda o tamanho da prancheta |
| Chave | Gira a copiadora (define a frente do projeto) |
| Mão vazia | Resumo: tamanho e projeto guardado |

Copia blocos Veio e vanilla com orientação (`facing`, `axis`, `half`, `type`,
`shape`, `hinge`, `face`, `part`, `rotation`) e os ajustes das máquinas Veio:
filtro (`LogisticsFilter`), prioridade de divisores/unificadores
(`RoutingPriority`), sentido do elevador (`Down`) e direção manual das esteiras.
Nunca copia inventários, energia, módulos ou progresso. Fica de fora: minérios,
detritos ancestrais, fluidos, fogo, portais, bedrock, geradores de monstro,
blocos técnicos, plantações e a própria copiadora/marco. Teto de 6.000 blocos.

O projeto fica no NBT da copiadora (paleta + deslocamentos), sobrevive a salvar e
recarregar, e **não** é sincronizado com os clientes: eles recebem só o tamanho
para desenhar a moldura.

## Marco de obra (`veio:build_marker`)

Coloque o marco onde a cópia deve nascer, no lugar equivalente ao da copiadora
(a obra sobe a partir do bloco acima dele). Ele usa a copiadora com projeto mais
próxima, até 256 blocos, e desenha a moldura verde. A diferença de orientação
entre o marco e a copiadora gira a obra em passos de 90°.

| Ação | Efeito |
|---|---|
| Mão vazia | Orçamento no chat: origem, tamanho, giro, até 10 materiais e o primeiro obstáculo |
| Agachado + chave | Constrói, cobrando do inventário de quem clicou |
| Chave | Gira a obra |

Sem espaço livre (ar e vegetação rasteira) ou sem material, nada é cobrado: o
chat aponta o bloco que atrapalha ou lista o que falta. Portas e camas são pagas
uma vez; tochas de parede custam tochas; placas de parede, placas. Ajustes de
máquina que não puderem ser aplicados no mesmo tick (entidade ainda não criada)
ficam numa fila que o marco termina nos ticks seguintes.

## Fábricas verticais

### Escadas de mão (aldeões)

A navegação dos mobs não planeja escadas, como no vanilla. O carregador planeja:

1. A cada 20 s, procura escadas de mão no próprio andar (área 13×13, cota de
   trabalhadores), mede cada coluna (até 24 blocos acima e abaixo) e as alturas em
   que dá para sair dela para um piso: espaço livre de 2 blocos ao lado, com chão
   firme. Guarda até 4 colunas e 6 andares.
2. A varredura de máquinas passa a cobrir cada andar ligado (e a camada acima e
   abaixo dele), com mais leituras por ciclo para a volta levar o mesmo tempo, e o
   limite de máquinas atendidas dobra quando há mais de um andar.
3. Toda ordem de caminhada passa por `ladders::route`: no mesmo andar, segue
   direto; noutro andar, vai até a escada mais conveniente no próprio andar, sobe
   ou desce até a altura de pé do andar do alvo e então sai para ele.
4. O objetivo `Work` faz a subida de verdade: com o aldeão na escada
   (`on_climbable`) e o passo na mesma coluna, aplica os valores do vanilla
   (0,2 subindo, queda limitada a 0,15 descendo), centralizando na coluna.

Regra de construção: a escada precisa chegar até a altura de pé do andar de cima
(o primeiro bloco acima do piso), como para o jogador.

### Elevador de carga (`veio:cargo_lift`)

Empilhe elevadores numa coluna. Cada bloco passa um item a cada 4 ticks ao
vizinho de cima (ou de baixo, em descida): 5 itens/s, a vazão da esteira rápida.
O último bloco entrega pela frente, como uma esteira, em esteira, máquina, baú ou
outro elevador. Entra item por qualquer face. Sem energia; redstone pausa.
Agachado + chave inverte o sentido; a chave gira a saída. Saída cheia segura o
item sem perda.

## Desbloqueio e receitas

| Bloco | Receita | Avanço |
|---|---|---|
| Elevador de carga (4) | chapa de aço, 2 esteiras, engrenagem | Aço (fase 3) |
| Copiadora | 2 circuitos, 2 chapas de aço, bancada, papel | Eletrônica (fase 4) |
| Marco de obra (4) | graveto, papel, corante azul-claro | Eletrônica (fase 4) |

Novos avanços: "Para cima" (elevador) e "Copiar e colar" (copiadora). O guia
ganhou a página "Megafábricas" e as fichas dos três blocos (98 de 100 páginas).

## Integração

Novos: `native/api/src/blueprint/` (`mod.rs`, `tests.rs`), `native/api/src/lift.rs`,
`native/api/src/workforce/ladders.rs`. Linhas em arquivos compartilhados:

- `lib.rs`: `mod`, `is_factory_block`, `layout`, `has_item_inputs`, `accepts`,
  `tick_factory` (elevador e copiadora antes da logística), clique (copiadora e
  marco antes da mão vazia; elevador antes da chave), etiqueta para o cliente sem o
  projeto, caixa de desenho da moldura.
- `presentation.rs`: slot e linhas do painel do elevador; desenho da moldura.
- `tutorial.rs`: o elevador não rotula slots, como as esteiras.
- `workforce/mod.rs`: `ladders` no módulo; descoberta no `step` do carregador;
  `area_position_in` e camadas por andar em `scan_named`; ordens de `work` via
  `ladders::route`; subida no objetivo `Work`.
- `workforce/hauler.rs`: leituras por ciclo e limite de máquinas por andares.
- Substituto offline: `Mob.velocity` e `Mob.on_climbable` (nomes do Horizon).

Sem andares descobertos, a varredura dos postos é idêntica à anterior (mesmas três
camadas, mesma ordem).

## Validação

- `python3 tools/check_rust.py`: 164 testes aprovados, 12 novos:
  copiar (estados, ajustes, minério de fora), salvar/carregar, giro de posições e
  propriedades, colar cobrando o orçamento exato e girando, recusa sem cobrança
  (material e obstáculo), clique completo copiadora → marco → obra, itens de custo;
  coluna e saídas da escada, andares e camadas, rota subindo/descendo/no mesmo
  andar, subida física; elevador subindo a 5 itens/s e descendo com destino cheio.
- `python3 tools/check_project.py`: 513 JSON, 98 páginas, 63 texturas.
- `python3 tools/check_performance.py`: 7 cenários, verificações idênticas.
- Não feitos: compilação pelo launcher e teste em partida.

## O que conferir no jogo (riscos conhecidos)

- **Subida dos aldeões:** depende de a física do Horizon respeitar a velocidade
  aplicada no objetivo enquanto o cérebro roda depois. Se o aldeão travar no pé
  da escada, o próximo passo é segurar o alvo do cérebro durante a subida.
- **Colar máquinas:** `edit_blocks` precisa criar a entidade dos blocos Veio; os
  ajustes esperam na fila até 40 tentativas. Conferir filtros e sentidos depois.
- Obras grandes reenviam chunks aos jogadores (como a silvicultura); 6.000 blocos
  de uma vez podem dar um engasgo curto.
- O orçamento e as mensagens vão a quem clicou, inclusive em LAN. A cobrança usa o inventário do jogador que executou a ação.
- Projetos não viajam entre mundos (não há item com dados na API atual).
