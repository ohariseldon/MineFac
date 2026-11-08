# Pesquisa: jazidas conectadas e exploração industrial

Pedido: aplicar a inspiração de Shapez ao Minecraft. Encontrar uma jazida com
vários blocos conectados deve justificar uma operação de extração maior, com
menos instalações pequenas espalhadas. Interpretação: **jazidas mais espaçadas
entre si, minério concentrado e conectado dentro de cada jazida**.

Este documento é uma proposta; não muda geração, extração ou mundos existentes.
O protótipo Python anexo só testa a busca em um mapa artificial.

## Recomendação

Combinar duas mudanças independentes:

1. A extratora mede o minério conectado em 3D, começando pelo bloco abaixo dela.
   Uma jazida rica aumenta sua vazão até um teto. Isso já funciona como ideia
   para mundos existentes, sem gerar minério novo.
2. Em terreno ainda não gerado, criar depósitos industriais maiores e menos
   frequentes, preservando pequenas ocorrências para o início da exploração.
   A raridade, altura e dimensão dependem do minério.

Não basta aumentar a geração: o reconhecimento atual pode ignorar um veio
vertical. Não basta mudar a leitura: a distribuição vanilla continua decidindo
onde há jazidas boas para construir.

## O que existe hoje

`production/extraction.rs` lê nove posições no plano 3×3 abaixo da extratora.
Conta blocos da mesma família, sem verificar caminhos entre eles. Os níveis
são 1–2 blocos = metade da vazão, 3–6 = normal, 7–9 = dobro. Isso pode contar
minérios separados e ignorar um veio que desce ou dobra para fora do plano.

A jazida não se esgota. Cada extratora tem inventário, energia e produção
independentes. Hoje, instalar duas máquinas na mesma jazida dobra a produção;
não existe uma capacidade compartilhada do depósito.

## Conexão proposta

- Dois blocos se ligam por uma das seis faces. Diagonal sozinha não conecta.
- Minério normal e sua variante de ardósia pertencem à mesma família.
- Ferro não conecta diamante; ar, pedra e cabos não são pontes de minério.
- Contar com busca em largura (fila e conjunto de visitados), incluindo os
  blocos acima, abaixo e nas laterais do bloco de origem.
- Restringir o mecanismo a famílias explícitas de **minérios**. Pedra, areia
  e netherrack não devem iniciar uma busca pelo componente inteiro do terreno.
- Parar assim que alcançar o último limiar útil. Mostrar “16+ conectados”,
  não um tamanho exato que não foi medido.

Sugestão inicial para experimentar, ainda não calibrada em partida:

| Blocos conectados | Vazão relativa à extratora normal |
|---|---:|
| 1–2 | 0,5× |
| 3–6 | 1× |
| 7–15 | 2× |
| 16+ | até 4× para minérios industriais comuns |

Assim, uma jazida com cinco/seis diamantes ligados já rende o dobro de uma
isolada. Uma sétima ligação leva ao nível seguinte. O ganho depende da jazida,
não de construir sete extratoras. As três primeiras faixas preservam os valores
atuais, mas passam a contar conectividade real.

Para diamante, esmeralda e detritos, começar com teto de **2×**, já existente,
até medir o efeito de jazidas maiores na progressão vanilla. O teto 4× de cobre,
ferro e carvão deve exigir mais potência nos níveis altos, além de saída capaz
de escoar o produto; o multiplicador exato de FE fica para o lote de balanceamento.
Não multiplicar a eficiência energética por quatro acidentalmente.

Exemplo em diamante, Mk.II sem módulos e com energia/saída livres: 1–2 blocos
conectados → 1,5 diamante/min; 3–6 → 3/min; 7+ → 6/min. A pesquisa não altera
esses valores no jogo. Minérios comuns podem receber a faixa industrial nova.

Vazões maiores que a capacidade da esteira devem pausar a extratora sem perda.
O painel precisa mostrar classe da jazida, quantidade conectada observada,
itens/min, potência e eventual limite de transporte. Isso explica a recompensa
de encontrar uma jazida melhor.

## Geração: viabilidade confirmada no Horizon de referência

Foi lido o código do Horizon **66f3981f522b24c250b21523a70080965b940424**,
referência indicada no `CONTRIBUTING.md`; não se assume a API da última `main`.

O carregador aceita arquivos de `configured_feature` e `placed_feature` no
conteúdo do mod. Os modificadores de bioma permitem adicionar e remover
features. O gerador já implementa `minecraft:ore`, tamanho de 0 a 64 e alvos de
substituição; a colocação aceita raridade, contagem, altura e filtro de bioma.

Locais previstos, sem substituir arquivos de bioma completos:

```text
content/data/veio/worldgen/configured_feature/industrial_copper.json
content/data/veio/worldgen/placed_feature/industrial_copper.json
content/data/veio/neoforge/biome_modifier/industrial_ores.json
```

Usar `minecraft:ore` para depósitos agrupados; `minecraft:scattered_ore` espalha
tentativas individuais e não é a primeira escolha para este objetivo.
Mas `size` é um parâmetro geométrico: **não garante aquela quantidade de blocos
gerados, nem um único componente conectado**. Pedra substituível, cavernas e
outras features interferem. Medir os componentes depois da geração.

Diminuir tentativas e aumentar `size` é uma boa primeira experiência. Não
preserva automaticamente a quantidade de minério por região: o resultado não
é simplesmente `tentativas × size`. Quando o novo modelo estiver medido,
reduzir apenas as features pequenas correspondentes para controlar a oferta
total, sem remover todos os minérios vanilla de uma vez.

`rarity_filter` dá espaçamento **estatístico**; não garante uma distância mínima
entre depósitos. Para garantir um centro por região, separado dos vizinhos,
seria necessário outro algoritmo/feature regional. Esta pesquisa não confirmou
um contrato nativo de worldgen e seed suficiente para prometer essa segunda
opção como mudança somente no mod. Não usar tick de extratora para gerar mundo.

Plano de medição: várias seeds, áreas equivalentes e as mesmas faixas de altura;
comparar minérios por região, distribuição do tamanho conectado e distância até
uma jazida industrial. Separar disponibilidade física de minério da vazão das
extratoras infinitas. Amostrar região com margem para medir componentes que
atravessam bordas e não contar fragmentos como depósitos separados.

## Chunks, cache e custo

A API de referência expõe `level`, `level_block` e `level_is_loaded` através
de `LevelRef`. A leitura simples `Host::block` retorna ar onde nada está
carregado; por isso, não basta usá-la e declarar a jazida pequena.

Consultar apenas a dimensão/epoch corretas e posições carregadas. Marcar uma
busca como incompleta ao encontrar uma fronteira desconhecida; manter a última
classe verificada ou uma estimativa conservadora explicitamente provisória.
Não forçar carregamento de chunks para medir minérios.

Com limite útil 16, o protótipo faz no máximo 97 consultas de bloco por busca;
não é uma varredura volumétrica. Consultar carregamento também tem custo na
implementação real. Fazer a busca na instalação e na invalidação, com orçamento
global por tick, em vez de repetir em cada máquina a cada tick. Eventos
`block_changed`, `chunk_load`, `chunk_unload` e `world_lifecycle` existem na
referência e permitem invalidar o cache e evitar misturar mundos.

Registrar a fronteira consultada: um bloco removido divide uma jazida; um
bloco inserido pode unir duas. Invalidar também quando uma fronteira antes
descarregada fica disponível. Uma releitura periódica escalonada pode servir
de recuperação; não substituir os eventos por scans contínuos de todos os blocos.

O substituto offline atual não demonstra essas APIs e eventos em execução:
copiar as assinaturas reais necessárias, testar e compilar pelo launcher.
Nenhum método novo foi adicionado ao substituto neste lote de pesquisa.

## Várias máquinas e minério colocado pelo jogador

Há duas regras possíveis, que não devem ser confundidas:

- **Primeira entrega recomendada:** bônus por riqueza conectada, com teto por
  extratora. Uma máquina em uma boa jazida substitui várias em jazidas pequenas.
  Outras máquinas continuam comprando mais vazão com construção/energia.
- **Evolução opcional:** uma capacidade total por depósito, dividida justamente
  entre máquinas. Evita multiplicação sem limite no mesmo veio, mas exige um
  identificador persistente da jazida e tratamento de união/divisão, chunks e saves.

Uma busca que para em 16 NÃO descobre o componente inteiro; seu menor bloco
visitado não é um identificador seguro para compartilhar capacidade. Usar
metadados de geração ou um índice completo separado para essa evolução.

Blocos vanilla podem ser colocados com Toque Suave. Contar conexões físicas
permite enriquecer artificialmente uma jazida. Não há marca de procedência
natural no ID do bloco. Para a primeira entrega, assumir isso como montagem
permitida, com ganho limitado, custo de obter os blocos e energia. Se a intenção
for tornar a exploração insubstituível, usar depósitos industriais identificados
na geração e limitar o bônus especial a eles. Isso precisa de projeto de
persistência; `block_changed` sozinho não prova a origem natural de um bloco antigo.

## Mundos existentes e ordem de entrega

1. Implementar e validar conexão 3D na extração existente, sem reescrever terreno.
   Testar faixas, formatos verticais, diagonais, limite, energia, saída cheia,
   troca de mundo, eventos e fronteiras de chunk. Atualizar guia/painel.
2. Acrescentar e medir geração industrial em chunks novos, conservando início
   acessível. Validar seeds no Horizon real antes de reduzir minério vanilla.
3. Se necessário, adicionar prospecção com indicação aproximada de direção e
   riqueza. O custo de explorar passa a render uma descoberta legível.
4. Só implementar capacidade compartilhada se o teste revelar spam de máquinas;
   ela não é pré-requisito para colher a melhoria de uma jazida conectada.

Terreno já gerado permanece intacto. A geração nova aparece em chunks ainda
não gerados; apenas descarregar/recarregar um chunk antigo não o torna novo.
Não preencher automaticamente minério dentro de bases ou minas existentes.

## Evidência e fontes

O experimento `python3 frentes/prototipos/jazidas_conectadas.py` passou em
**7 casos**: veio vertical/com curva, diagonais e intervalos vazios, famílias
normal/ardósia, parada por limite, borda descarregada, divisão ao quebrar uma
ponte e exclusão de pedra. É teste de algoritmo em Python, não medição de FPS,
de geração Minecraft nem validação de integração no jogo.

Fontes primárias consultadas:

- [Shapez 1: miner.js](https://github.com/tobspr-games/shapez.io/blob/master/src/js/game/systems/miner.js).
  O código mostra extratoras encadeadas repassando itens até uma saída. Não
  demonstra bônus automático pela área conectada da jazida. A proposta acima
  adapta a experiência descrita pelo usuário, sem atribuir essa regra exata ao jogo.
- [Horizon: carregamento de conteúdo e features](https://github.com/horizon-rust/horizon/blob/66f3981f522b24c250b21523a70080965b940424/crates/horizon-client/src/mod_content.rs).
- [Horizon: modificadores de biomas](https://github.com/horizon-rust/horizon/blob/66f3981f522b24c250b21523a70080965b940424/crates/horizon-client/src/mod_content_biomes.rs).
- [Horizon: geração de minério](https://github.com/horizon-rust/horizon/blob/66f3981f522b24c250b21523a70080965b940424/crates/horizon-world/src/worldgen/ore_feature.rs).
- [Horizon: modificadores de colocação](https://github.com/horizon-rust/horizon/blob/66f3981f522b24c250b21523a70080965b940424/crates/horizon-world/src/worldgen/placement.rs).
- [Horizon: consultas a níveis e chunks carregados](https://github.com/horizon-rust/horizon/blob/66f3981f522b24c250b21523a70080965b940424/crates/horizon-client/src/game/native_level_refs.rs).
- [Horizon: eventos de blocos, chunks e ciclo do mundo](https://github.com/horizon-rust/horizon/blob/66f3981f522b24c250b21523a70080965b940424/crates/horizon-client/src/game/native_hooks.rs).
- [Horizon: catálogo dos contratos nativos](https://github.com/horizon-rust/horizon/blob/66f3981f522b24c250b21523a70080965b940424/native-hooks.json).
