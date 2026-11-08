# Restrição obrigatória do mundo

Decisão de produto de 7 de novembro de 2026. Vale para a geração de novos mundos
do Veio e para qualquer mudança futura no gerador, na exploração ou nos portais.
O usuário autorizou adaptar a distribuição do terreno para garantir o conteúdo.

**Estado: requisito definido; geração finita, muralha e garantia de conteúdo ainda
não implementadas nem validadas no jogo.** Este documento não altera os saves.

O código experimental e os próximos passos estão no
[checkpoint do mapa finito](work-in-progress/finite-world/CONTINUAR.md). Esse
rascunho está fora do bundle nativo e permanece desativado no jogo.

## Tamanho e limite

- O Overworld tem **15.000 × 15.000 blocos**, uma área quadrada de 225 milhões de
  colunas. Este tamanho substitui a estimativa inicial de 15 minutos correndo.
- O centro é fixado em `(0, 0)`: o interior horizontal vai de `-7.500` inclusive a
  `7.500` exclusive em X e Z. O spawn inicial e a vila inicial ficam no interior;
  mudar o spawn ou construir um centro de controle não desloca a borda.
- Há uma muralha contínua de **bedrock**, fora do interior útil, nas quatro faces
  e nos quatro cantos. Ela ocupa toda a altura construível da dimensão, inclusive
  o subsolo. Não se pode atravessá-la por mineração, explosões ou portais.
- A muralha é a representação física do limite. Uma restrição de coordenadas no
  servidor também impede sair por cima, por baixo, voando, montado, usando
  pérolas, fruta do coro, teletransporte ou mudança de dimensão.
- Geração, carregamento, construção, entidades e simulação respeitam o limite.
  Chunks que cortam a borda tratam suas colunas individualmente: 15.000 não é
  múltiplo de 16. O gerador pode usar uma vizinhança técnica limitada para luz e
  dependências; isso não cria terreno explorável ou expansão persistente externa.

Para dimensionamento, usando aproximadamente 5,6 blocos/s em corrida contínua,
15.000 blocos levam cerca de **44 minutos e 39 segundos**. Relevo, fome, água,
saltos, equipamentos e rotas mudam o tempo real; o tamanho em blocos é a regra.

## Conteúdo garantido em cada mundo

O mapa precisa oferecer **todos os biomas naturais de cada dimensão e todos os
recursos e caminhos de progressão obtíveis em sobrevivência** da versão de
Minecraft suportada pelo Horizon. Isso vale para cada mundo criado, não apenas
para uma seed escolhida ou para a média de muitas seeds. Conteúdo exclusivo de
comandos/criativo, como barreiras e blocos de comando, não é requisito de geração.

O catálogo obrigatório vem dos registros reais da versão usada: biomas,
estruturas, recursos, fontes de criaturas e dependências de obtenção. Uma
atualização do jogo precisa atualizar o catálogo e validar novamente a cobertura;
uma lista manual desatualizada não pode certificar o mundo.

- **Superfície:** todos os biomas terrestres, fluviais e oceânicos, incluindo os
  raros, com terreno, vegetação, água, clima e fontes de criaturas correspondentes.
- **Subsolo:** todos os biomas de cavernas e as faixas de altura necessárias para
  minérios, geodos, recursos e estruturas subterrâneas.
- **Estruturas:** acesso às famílias e variantes que oferecem conteúdo distinto,
  incluindo aldeias, fortaleza do End, monumento oceânico, mansão, cidade ancestral,
  câmaras de desafio e sítios de arqueologia. A caixa inteira de cada estrutura
  obrigatória deve caber no interior, com acesso utilizável.
- **Nether:** todos os seus biomas e recursos, fortalezas, bastiões e os caminhos
  necessários à progressão, dentro do seu próprio limite.
- **End:** acesso ao dragão, aos biomas das ilhas externas e às cidades com navio,
  élitros e demais recursos de sobrevivência. O End tem limite próprio e não usa a
  escala de portais do Nether; seu dimensionamento precisa preservar esses acessos.

Um bioma presente em uma única célula ou pintado sobre terreno incompatível não
cumpre a regra. As regiões precisam ter área/volume e acesso suficientes para seu
conteúdo funcionar. A disponibilidade não significa recursos infinitos: consumo,
raridade, distâncias e exploração continuam fazendo parte do jogo. As jazidas
industriais renováveis já existentes mantêm suas regras.

## Geração com variação e cobertura obrigatória

1. A seed determina o relevo, as formas, a distribuição e as variações do mundo.
   A geração continua variando entre seeds; não se reduz a uma única seed fixa.
2. Antes de liberar o mundo ao jogador, o gerador monta um plano finito por
   dimensão, reservando regiões adequadas para os biomas e para o conteúdo
   obrigatório. Pode adaptar a distribuição vanilla para cumprir essa cobertura.
3. O plano reserva estruturas e recursos com suas dependências: altura, terreno,
   bioma, espaço, criaturas e acesso. Não basta conferir nomes de biomas.
4. Uma validação determinística confirma a cobertura e seus locais. Faltas são
   corrigidas no plano antes da geração dos chunks; uma tentativa que não cumprir
   as regras não pode ser apresentada como um mundo válido ou completo.
5. O plano aprovado, sua versão, os limites e a referência ao catálogo ficam
   salvos com o mundo. Carregar chunks em outra ordem, visitar outra dimensão ou
   reabrir o jogo não muda o plano nem repete estruturas ou recompensas.
6. A geração detalhada acontece sob demanda. Não é necessário carregar ou manter
   todos os 225 milhões de colunas na memória para garantir a cobertura do plano.

Uma amostragem de alguns pontos, uma seed conhecida ou o comando `worldborder`
isolado não comprovam essa garantia. A integração precisa validar o resultado do
gerador real, incluindo biomas subterrâneos, estruturas e obtenção de recursos.

## Nether e portais

A referência geométrica é **8 blocos no Overworld para 1 no Nether**. Assim, o
quadrado de 15.000 blocos corresponde a **1.875 blocos de lado** no Nether,
centrado em `(0, 0)`, com semiextensão de `937,5`. Esse é o teto inicial da área
correspondente; não aplicar 15.000 blocos também ao Nether. A cobertura obrigatória
do Nether precisa ser garantida nessa área pela geração adaptada.

- Os limites lógicos podem usar coordenadas fracionárias. O posicionamento dos
  blocos de bedrock e o arredondamento de coordenadas negativas não podem aumentar
  a área permitida ou deixar frestas. Não arredondar o lado para 1.876 ou 1.888.
- A conversão 8:1 fornece apenas o alvo inicial. A procura de um portal existente,
  a criação de uma saída e a posição final do viajante são validadas separadamente
  contra o interior da dimensão de destino.
- A moldura, a área do portal, o corpo do viajante e o espaço de saída devem caber
  no interior, com margem para chegada segura. Procurar candidatos também respeita
  a borda; um candidato externo não pode vencer por estar mais perto.
- Uma saída que cairia fora da muralha é reposicionada para um local válido dentro
  da dimensão de destino. Preservar altura segura e acesso; não colocar viajantes
  dentro da bedrock, de blocos sólidos, na lava ou no vazio.
- Se não houver saída segura disponível, a transferência não é concluída. O
  viajante permanece na origem e recebe uma mensagem clara; não nasce fora do mapa.
- A regra vale nos dois sentidos, para jogadores e outras entidades, inclusive
  veículos e passageiros. Persistência e reentrada não podem contorná-la.

O Nether não precisa ter o mesmo tempo de travessia do Overworld. Sua escala é
definida pela correspondência espacial e pela segurança dos portais.

## Compatibilidade e desempenho

Não encolher, cortar ou regenerar mundos existentes automaticamente. Um save
anterior precisa de migração explícita e backup antes de receber limites; uma
bedrock nova não pode apagar construções, inventários ou entidades. A garantia de
conteúdo aplica-se ao plano de geração dos novos mundos, não retroativamente a
terreno já gerado.

O limite contém o crescimento da exploração, dos arquivos e do trabalho acumulado
de geração. **Não garante aumento de FPS ou redução imediata de RAM**: esses custos
dependem principalmente dos chunks carregados, da renderização e da simulação
ativa. Manter carregamento sob demanda, distância de simulação limitada e os
cuidados de [PERFORMANCE.md](PERFORMANCE.md). Não varrer o mapa por tick, manter a
muralha inteira carregada ou pré-gerar tudo como condição de entrada.

## Critérios de aceite da implementação

- Várias seeds, incluindo casos extremos, cumprem o catálogo completo no gerador
  real. Verificar cobertura efetiva, regiões utilizáveis e acesso à progressão;
  testes de constantes ou mocks não bastam para certificar a geração.
- Faces, cantos, chunks parciais, topo e subsolo impedem saída sem lacunas.
  Nenhum crescimento persistente ocorre além da vizinhança técnica delimitada.
- Testar portais em ambos os sentidos nas quatro faces e nos quatro cantos,
  coordenadas negativas, portais existentes fora do limite, saída obstruída,
  passageiros, reinício e entrada multijogador. O destino final sempre é seguro.
- Spawn, renascimento, centro de controle, pérolas, fruta do coro, élitros e comandos
  de teletransporte não colocam entidades além do limite.
- Abrir um save antigo não altera seus dados. Reabrir um mundo novo conserva o
  plano e não duplica conteúdo. Atualizar o mod não redesenha chunks existentes.
- Medir geração e uso de memória junto à borda e numa fábrica ativa antes de
  anunciar ganho de desempenho. Separar verificação offline, compilação contra o
  Horizon real e teste dentro do jogo.

## Dependência do motor

A API da base de compilação documentada do Veio (`66f3981`) oferece eventos de
carregamento de chunks e comandos, mas não o gancho `native_chunk_generate` para
substituir o terreno durante a geração. Portanto, somente acrescentar lógica aos
eventos atuais do mod não implementa este contrato. A entrega precisa integrar um
gerador compatível no motor ou atualizar a base do Horizon, conferir a API real e
validar a geração completa. Não tratar uma muralha instalada após o carregamento
como prova de mundo finito ou de disponibilidade de todo o conteúdo.
