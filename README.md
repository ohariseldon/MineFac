# Veio 0.16.0

**Build consolidada** (rodada de integração e revisão feita pelo Claude, com o trabalho pendente das três frentes):

- **Casa do fundador:** a vila já vem com a casa do jogador: centro de controle, lareira, sofá, cama ao lado do painel e um mural com o mapa das redondezas. Em casa, perto do painel, as fábricas rendem 15% mais; a primeira cama é a da casa e fora dela só cabe mais uma; refeições e banquetes aceleram o trabalho e as máquinas. Detalhes em [TRABALHADORES.md](TRABALHADORES.md), sexta entrega.
- **Mobs e quartel (Grok, revisado):** saqueadores vêm atrás de quem está na fábrica; à noite, monstros caçam aldeões; o quartel treina milícia, guarda, arqueiro e mago. Lavradores e pastores dormem à noite. Sétima entrega do TRABALHADORES.md.
- **Transporte entre zonas (Codex):** terminais ferroviários e portos de drones ([TRANSPORTES.md](TRANSPORTES.md)).
- **Grok:** convidados da LAN vão para a vila do anfitrião, a vila só cresce com cama livre, [a primeira linha pede duas prensas](frentes/GROK-BALANCE.md), [projetos industriais e produtividade por produto](frentes/BALANCEAMENTO-MISSOES.md).
- **Correções da revisão:** o relógio do mod andava em dobro no jogo de um jogador só (tempos, nascimentos e ondas da noite saíam errados); renascer no centro da vila não funcionava; o quartel não trocava de classe com a mão vazia; 30 páginas dos guias passavam do tamanho do livro e ficavam cortadas.

A 0.15.0 foi a build consolidada anterior: vila operária completa (Cloud), braços de transferência e controles de fluxo (Codex) e os lotes 1 e 3 a 8 do Grok.

Histórico: a 0.14.0 fez as vilas crescerem com a fábrica até 150 aldeões (com um censo de posições para manter o custo baixo), põe o jogador no centro da vila ao entrar e ao renascer, e leva os sistemas da vila para um segundo livro, o Guia da Vila Operária, com sua própria aba de avanços; inclui o lote 6 do Grok. A 0.13.0 trouxe zonas de lavoura e de cercado com níveis de expansão, celeiro e silo, o centro de controle (painel de gestão com metas) e vilas que crescem até 30 aldeões, além dos lotes 4 e 5 do Grok. A 0.12.0 acrescentou a fila de tarefas do capataz e o posto do pastor, e inclui o [lote 3 do Grok](frentes/GROK-LOTE3.md) (varreduras mais baratas e comida que custa 2 pontos por tarefa). A 0.11.0 trouxe o carregador e o capataz, e a 0.10.0 consolidou as três frentes da rodada sobre a 0.9.1:

- **Cloud — trabalhadores:** [aldeões contratados](TRABALHADORES.md). Zonas de lavoura e cercado, celeiro, silo e centro de controle; posto do lavrador (ara, planta e colhe), posto do pastor (cuida dos animais), posto do carregador (abastece máquinas e recolhe produtos) e mesa do capataz (alimenta e recontrata os postos e monta uma fila de transferências que os carregadores executam). As entregas da 0.11.0 à 0.15.0 aguardam teste no jogo.
- **Codex — interface de postos:** [consulta comum de estações](frentes/INTERFACE_POSTOS.md), com setor, inventários, motivo de espera e dica ao passar o cursor sobre o estado da máquina. O posto do lavrador já está registrado nela.
- **Grok — lote 1:** [ticks mais leves e energia renovável utilizável](frentes/GROK-LOTE.md): roda d'água, cata-vento e biomassa reforçados; menos trabalho repetido em cabos, esteiras e sensores. **Lote 3:** [postos da 0.11.0](frentes/GROK-LOTE3.md): varredura por fileira, pão paga 3 tarefas, trigo cru não fecha o ciclo. **Lote 4:** [inspeções do carregador e NBT do pastor](frentes/GROK-LOTE4.md): 3 máquinas por ciclo; o pastor relê só o alvo. **Lote 5:** [fila do capataz](frentes/GROK-LOTE5.md): uma leitura por bloco da rede; 3 postos por ciclo. **Lote 6:** [zonas da 0.13.0](frentes/GROK-LOTE6.md): 9 blocos/ciclo na lavoura grande; um teste de água por ciclo. **Lote 7:** [tick sem alocação repetida](frentes/GROK-LOTE7.md): chaves NBT reaproveitadas; a difusão da poluição corre uma vez por dimensão.

**Para jogar no Windows:** depois de clonar, abra `launcher.exe` na raiz do repositório (ou extraia `release/Veio-0.16.0.zip` em qualquer pasta e abra o `launcher.exe` de dentro). Ele compila o código nativo pelo Horizon Launcher quando a versão muda, instala o conteúdo na instância **Veio** e abre o launcher. Se a instância não existir, ele a cria (`horizon-launcher instance create Veio`) e depois abre o jogo direto. Antes, instale o Horizon Launcher, entre com a conta e faça o Prepare uma vez. Na primeira partida, ative o Veio em Opções > Mods e reinicie o jogo. Para gerar de novo: `python tools/package.py`.

**Na mesma rede:** quem hospeda abre o mundo, aperta Esc, escolhe **Abrir para a LAN** e **Iniciar mundo LAN**. O mundo continua no computador dele. Quem entra, no mesmo Wi-Fi, abre **Multijogador**: o mundo aparece na lista. Cada pessoa usa a própria conta do Minecraft e a mesma versão do Veio. As máquinas, a vila e os itens são os do anfitrião. Quem chega pela primeira vez vai para a vila que já existe, em vez de ganhar outra.

A integração passou na conferência de conteúdo e nos testes offline (161 regras do mod e 2 do launcher). A compilação nativa pelo Horizon Launcher e o teste dentro do jogo ainda precisam ser feitos na máquina do usuário; o `launcher.exe` executa essa compilação. Ordens das frentes: [Cloud](frentes/CLOUD.md), [Grok](frentes/GROK.md) e [Codex](frentes/CODEX.md); acordo em [COORDENACAO.md](COORDENACAO.md).

Código do projeto **MineFac**. Para continuar o desenvolvimento localmente ou na nuvem, leia [CONTRIBUTING.md](CONTRIBUTING.md). A verificação de conteúdo funciona com Python, e a do módulo Rust funciona sem internet, sem instalar o jogo.

Automação industrial no mundo procedural do Minecraft, executada no Horizon. Abra a instância **Veio** pelo launcher. No criativo, as abas **Veio – Indústria**, **Veio – Energia**, **Veio – Logística** e **Veio – Mundo** reúnem o conteúdo.

A fábrica faz parte do mundo do Minecraft: extrai das jazidas vanilla, queima combustíveis vanilla, leva energia por postes e fios, cuida da lavoura, polui e atrai ataques noturnos, defende a base e exporta para os povoados em troca de esmeraldas.

## Transporte entre zonas

A integração de [terminais, drones e caravanas](TRANSPORTES.md) permite ligar zonas por cor e definir os produtos de ida e volta. Aldeões abastecem e retiram as cargas pela fila do capataz. Vagões com baú usam funis; drones levam até 16 itens a até 64 blocos; cavalos, burros/mulas e porcos caminham entre terminais com capacidades e rações diferentes. Um Guia de Transportes é entregue ao obter o primeiro terminal. A entrega aguarda compilação pelo launcher e teste no jogo; rotas terrestres de até 512 blocos dependem de áreas simuladas.

## Tutorial dentro do jogo

Ao entrar no mundo, cada jogador recebe uma vez o **Guia do Veio**. Selecione o livro na barra e abra com o botão direito. O sumário é clicável. Comece pela primeira fábrica de cobre e siga até a primeira engrenagem. As páginas finais servem de consulta para uso e fabricação das máquinas.

- Abra **Avanços > Primeira fábrica** para acompanhar materiais obtidos. Recolha os produtos no inventário para registrar os marcos.
- Abra uma máquina com botão direito. Os rótulos mostram entradas e saídas. Passe o cursor sobre um slot vazio para saber qual material colocar.
- Passe o cursor no **?** do painel para consultar funcionamento e receita. No inventário, máquinas e componentes também mostram sua função.
- Perdeu o guia? Junte **um livro e um papel** na bancada para fabricar outro.
- O guia acompanha o idioma do jogo, com português brasileiro e inglês. As opções de idioma ficam nas configurações do Horizon.

O começo usa receitas sem posição fixa na bancada, uma extratora diretamente sobre minério de cobre, gerador com carvão e cabos encostando em cada máquina. Monte `extratora > esteira > prensa > baú` na mesma altura. A prensa usa **cobre bruto**, e o montador transforma **duas chapas de cobre** em uma engrenagem.

## Energia

- O gerador fornece até 40 FE/t e queima os combustíveis da fornalha vanilla, na mesma proporção: carvão ou carvão vegetal rende 16.000 FE, tronco ou tábua 3.000 FE, vara de blaze 24.000 FE, bloco de alga seca 40.000 FE e bloco de carvão 160.000 FE. Madeira do Nether não queima. O combustível fica em espera quando o reservatório enche.
- O gerador geotérmico fornece até 80 FE/t. Um balde de lava rende 64.000 FE e devolve um balde vazio no slot de saída. A saída pode ser automatizada com esteiras e funis.
- Cada painel solar fornece até 16 FE/t ao meio-dia. Precisa de luz do céu e funciona no Overworld. Chuva, tempestade e horário reduzem a geração.
- Cada acumulador armazena 30.000 FE. A receita entrega quatro unidades. Use bancos de acumuladores para atravessar a noite.
- **Postes** levam a energia. Cada poste se liga sozinho aos postes vizinhos por fios aéreos e alimenta as máquinas, geradores e acumuladores da sua área. O poste de madeira alcança 8 blocos, alimenta num raio de 2 blocos e passa 120 FE/t por fio. O poste de aço alcança 16 blocos, raio de 3 e 400 FE/t. Cada poste se liga aos até seis vizinhos mais próximos.
- Cabos continuam funcionando como ligação curta entre blocos encostados. Máquinas armazenam até 2.000 FE.
- Esteiras têm acionamento mecânico. A comum transporta até 2,5 itens/s e a rápida até 5 itens/s.

Os painéis de inspeção mostram energia, progresso e motivo de espera. Os estoques, o progresso, a carga e a queima em andamento são salvos no mundo. A energia armazenada pertence ao bloco colocado. Use a chave para mudar sua orientação.

## Extração e processamento

As extratoras trabalham diretamente sobre as jazidas existentes do Minecraft. As jazidas permanecem disponíveis para sustentar a fábrica.

- Cobre e ferro, comuns ou de ardósia, produzem minério bruto.
- Ouro, comum ou de ardósia, produz ouro bruto.
- Carvão, comum ou de ardósia, produz carvão para alimentar geradores e ligas.
- Redstone, comum ou de ardósia, produz pó de redstone.
- Lápis-lazúli, comum ou de ardósia, produz lápis-lazúli.
- Quartzo do Nether produz quartzo, e ouro do Nether produz pepitas de ouro.
- Diamante e esmeralda, comuns ou de ardósia, só com a Mk.II, 20 vezes mais devagar: uma pedra a cada 20 segundos e 4.800 FE por unidade.

A extratora básica produz uma unidade a cada 2 segundos e consome 6 FE/t enquanto trabalha. A Mk.II produz uma unidade por segundo e consome 12 FE/t. As duas gastam 240 FE por unidade extraída.

As linhas de produção usam estas transformações.

- Prensa. Um cobre bruto vira uma chapa de cobre, um ferro bruto vira uma chapa de ferro e um ouro bruto vira um lingote de ouro.
- Montador. Duas chapas de cobre viram uma engrenagem.
- Cortadora. Uma chapa de ferro vira uma moldura. Uma chapa de cobre vira uma fiação de cobre.
- Integrador. Uma moldura e uma engrenagem viram um módulo mecânico.
- Triturador. Um quartzo vira uma pastilha de silício. Um osso vira quatro farinhas de osso. Pedregulho vira cascalho e cascalho vira areia.
- Forno de ligas. Duas chapas de ferro e um carvão ou carvão vegetal viram uma chapa de aço.
- Montador de circuitos. Duas fiações de cobre e uma pastilha de silício viram uma placa de circuito.

Máquinas aguardam material, energia e espaço na saída. Os ingredientes são consumidos juntos quando o ciclo termina. Dedique máquinas a materiais específicos para manter as saídas organizadas.

## Progressão

1. Cobre abre a primeira fábrica, com extratora, prensa e esteiras.
2. Engrenagens abrem a oficina industrial, com chave, triturador e forno de ligas.
3. Aço libera acumuladores, geotermia, montagem eletrônica e construção em lote.
4. Módulos mecânicos liberam a colheitadeira.
5. Quartzo e fiação permitem produzir circuitos, que liberam solar, extração Mk.II e a torre Tesla.
6. O pedido de oito módulos mecânicos continua liberando o lote de 32 esteiras rápidas.

Os avanços entregam as receitas. Em um mundo anterior, recolha novamente os materiais do novo ramo para ativar os novos avanços.

## Construção e logística

Selecione a **chave de montagem** e clique com o botão direito para girar um bloco de Veio. A orientação muda mantendo o inventário, o progresso e a carga. Com a mão vazia, o botão direito abre o painel da máquina.

A saída das máquinas aponta para longe de quem colocou o bloco. Esteiras podem fazer curvas pela orientação do próximo segmento. O divisor alterna as duas saídas laterais e desvia os itens quando uma delas recusa a carga.

Uma receita inicial entrega 16 esteiras. Com aço, uma chapa de aço, uma de cobre e dois gravetos entregam 64 esteiras comuns. Uma chapa de aço e duas fiações entregam 64 cabos. Os lotes permitem ampliar a fábrica com menos operações manuais.

## Visual

As máquinas têm [formas próprias por função](VISUAL-MAQUINAS.md): prensa aberta com pistão, cortadora de bancada, triturador com funil, forno, caldeira, tanques, prateleiras e consoles. São 34 modelos revisados, mantendo um ponto de colocação por máquina. Cobre, ferro, madeira e ardósia combinam com detalhes discretos em verde, azul e terracota. As caixas de seleção e colisão acompanham os formatos e sua orientação.

As luzes de estado e a placa do módulo ficam em alojamentos frontais. As máquinas que escoam produtos têm bico traseiro; as demais usam conexões metálicas. O painel solar usa as células do sensor de luz do dia, e o cabo mantém isolamento de lã preta e emenda de cobre.

Os painéis seguem as telas de baú e fornalha: fundo cinza em relevo, slots rebaixados, seta de progresso entre entrada e saída e texto escuro. Uma luz na tela e nas quatro faces da máquina indica o estado.

Todas as texturas próprias são desenhadas em texto em `tools/build_textures.py`, e os modelos saem de `tools/build_content.py`, com as formas definidas em `tools/machine_forms.py`. As referências a texturas do Minecraft são resolvidas pela instalação do jogador. Créditos em [CREDITS.md](CREDITS.md).

## Desempenho

O código evita trabalho por tick:

- Postes se encontram por um registro em memória dividido em chunks e recalculam vizinhos e área a cada 2 segundos, em ticks escalonados. No tick comum, só percorrem essas listas.
- A poluição é somada na máquina e entregue ao chunk uma vez por segundo; a difusão roda a cada 64 ticks, uma vez por dimensão e `game_time`.
- Cabos e geradores sem carga não consultam vizinhos. Esteiras estáveis guardam uma assinatura da vizinhança e só relêem curvas quando um vizinho muda.
- Painéis solares, rodas e cata-ventos relêem o mundo a cada 2 segundos, não a cada tick.
- A torre Tesla consulta apenas os hostis vistos no tick anterior.
- Fios, máquinas paradas e rotores não são redesenhados a cada quadro: o jogo guarda a geometria até o estado mudar, e as peças giratórias giram na placa de vídeo.

## Instalação e dados

A instância precisa do pacote em content/ e do módulo Rust em native/api/. A compilação pelo launcher instala o módulo na instância Veio. Alterações que registram novos blocos exigem reiniciar o jogo.

Os dados do launcher ficam em E:\GPTmodMine\.launcher, os mundos e instâncias em E:\GPTmodMine\.game e a compilação em E:\GPTmodMine\.build. Essas pastas estão excluídas do controle de versão.

O mundo **Veio03-Laboratorio** contém a demonstração da versão anterior. Os novos equipamentos estão disponíveis nas abas criativas e na progressão da 0.4. Blocos colocados quando apenas o pacote de conteúdo estava instalado podem precisar ser recolocados para criar seu inventário.

Mundos da 0.6 e da 0.7 continuam válidos: nenhum bloco ou item mudou de nome, e estoques, progresso e energia ficam nas máquinas. Os painéis reposicionaram os slots na tela, sem mudar o que cada slot guarda.

A validação de jogabilidade desta expansão fica com o jogador. O desenvolvimento desta versão concentra as mudanças antes da compilação, seguindo a preferência por evitar testes no jogo a cada iteração.

## Projetos, indicadores e redstone

A versão 0.5 acrescenta uma central de contratos, indicadores de produção e comando por redstone.

A central é fabricada com um baú, uma engrenagem, um lingote de ferro e um de cobre. Tem duas entradas à esquerda e duas saídas à direita. Os contratos são concluídos quando os ingredientes e o espaço para todas as recompensas estão disponíveis.

1. Oficina. Entregue 24 chapas de cobre e oito engrenagens para receber 64 esteiras e 32 cabos.
2. Entrepostos. Entregue 24 chapas de aço e 12 módulos mecânicos para receber 64 trilhos e 16 trilhos elétricos.
3. Povoado. Entregue 24 circuitos e 32 molduras para receber oito funis e oito esmeraldas.
4. Farol industrial. Entregue uma estrela do Nether e 48 circuitos para receber um farol e duas extratoras Mk.II. É um projeto opcional depois da infraestrutura.
5. Exportação contínua. Depois do projeto do povoado, sem exigir o farol, a central aceita sem fim quatro circuitos e quatro chapas de aço, e entrega quatro esmeraldas e dois frascos de experiência por remessa. O painel conta as entregas.

Para selecionar o farol opcional, esvazie as duas entradas e coloque primeiro a estrela na entrada 1; depois, os 48 circuitos na entrada 2. Sem a estrela, a central exibe e aceita exportações. Retire recompensas anteriores antes de trocar. Após concluir o farol, a central continua exportando.

O progresso é salvo na central colocada. Sua desmontagem inicia uma central nova. A chave permite reorientá-la mantendo o progresso. Ao mudar de contrato, retire sobras das entradas e direcione os materiais do projeto seguinte. As recompensas podem sair por funis ou pelo bocal frontal.

As máquinas de produção passam a mostrar itens produzidos por minuto e porcentagem de tempo ativo. A primeira leitura usa a amostra parcial; depois as leituras fecham a cada minuto de simulação com a máquina carregada. Falta de materiais, falta de energia, saída cheia e suspensão por redstone contam como tempo de espera. Uma linha equilibrada pode esperar ao atingir o estoque desejado.

Um sinal de redstone pausa a produção e preserva o progresso. A saída continua escoando produtos. Alavancas permitem comando manual; comparadores e circuitos de redstone podem transformar estoque em sinal de parada. O painel informa a suspensão.

## Produção (0.9)

**Jazidas e pureza.** A extratora lê o quadrado 3x3 logo abaixo dela. Com 1 ou 2 blocos da mesma jazida, ela é impura e trabalha na metade da velocidade. De 3 a 6 blocos é normal. De 7 a 9 é pura e trabalha no dobro. Além dos minérios, há jazidas infinitas de pedra, ardósia, areia, areia vermelha, cascalho, argila, netherrack, areia das almas, pedra-negra e pedra do End. Obsidiana e ametista exigem a Mk.II; detritos ancestrais, a Mk.III. A Mk.III é quatro vezes mais rápida que a Mk.I.

**Receitas.** Todas as receitas de máquina estão em `tools/production.json` e aparecem no guia.

| Máquina | Faz |
|---|---|
| Fornalha elétrica | Funde e assa como a fornalha vanilla, 2,5x mais rápido: minérios, vidro, pedra, tijolo, carvão vegetal, carnes e batata |
| Moinho | Trigo vira farinha, cana vira o dobro de açúcar, plantas e troncos viram biomassa |
| Cozinha industrial | Pão, biscoito, bolo, torta de abóbora, cenoura e maçã douradas, sopa de beterraba e ensopado de cogumelo |
| Alternativas | Silício de areia no forno de ligas (sem ir ao Nether), chapas a partir de lingotes, circuito com redstone |

**Energia.**

| Fonte | Energia | Condição |
|---|---|---|
| Roda d'água | 8 FE/t por lado com água corrente, até 32 | Não polui |
| Cata-vento | 8 FE/t no nível do mar, até 32; y 110 dá 16 e chuva ajuda | Precisa de céu aberto; não polui |
| Gerador a biomassa | 40 FE/t, polui menos que o carvão | Queima biomassa, plantas e feno |
| Caldeira a vapor | 120 FE/t e o dobro de energia por combustível | Precisa de água parada ao lado |

**Lavoura.**
- A colheitadeira também corta cana, cacto e bambu acima da base, e colhe abóbora e melancia presas a um caule.
- A **semeadora** planta sementes, cenouras, batatas, beterrabas, caules e fungo do Nether em solo vazio.
- A **lâmpada de cultivo** adianta um estágio de uma planta a cada 4 ticks, com 50 FE cada, e ilumina a lavoura.
- A **silvicultora** planta mudas numa grade com passo de 3 blocos, derruba árvores crescidas, replanta a mesma espécie e entrega troncos, gravetos e maçãs.

**Rancho.** Com ração, reproduz vacas, ovelhas, porcos, galinhas, coelhos e cabras até 10 adultos por espécie. Também:
- tosquia ovelhas;
- ordenha com baldes vazios;
- recolhe ovos, penas, couro e carnes do chão;
- abate o excedente acima do teto. Animais com nome nunca são abatidos.

**Armazém do povoado.** Compra comida e paga uma esmeralda a cada 60 pontos de valor. Comida pronta vale mais que o ingrediente cru: um pão vale 6, contra 1 de cada trigo. Com 3 ou mais aldeões a até 48 blocos, o armazém paga 50% a mais. Construir a vila perto da fábrica compensa.

## Gargalos e melhorias

A fábrica cresce resolvendo um gargalo de cada vez: a extração limita a prensa, a energia limita as máquinas, o fio de madeira limita a energia que chega longe, a poluição limita o tamanho que a base aguenta sem defesa.

Os **módulos** entram com o botão direito numa máquina de receita ou extratora, um por máquina. A chave, com o jogador agachado, devolve o módulo.

| Módulo | Ciclo | Energia por tick | Poluição | Efeito |
|---|---|---|---|---|
| Velocidade | 60% | 180% | 150% | Mais produção por máquina, 8% mais energia por produto |
| Eficiência | 100% | 60% | 50% | Menos energia e metade da poluição |
| Produtividade | 115% | 140% | 120% | Um lote extra a cada cinco ciclos (+20%) |

Produtividade reserva espaço para o lote extra antes de trabalhar: saída cheia pausa sem perder o bônus. A fração pertence ao produto e reinicia ao concluir uma receita de outro produto. O painel conta itens produzidos, incluindo lotes e bônus; o contador interno de ciclos permanece separado.

O painel mostra o módulo, a poluição emitida por minuto e a integridade. Uma placa colorida no suporte frontal indica o módulo instalado.

## Poluição e Saqueadores

Máquinas trabalhando poluem o chunk onde estão: gerador 6 por segundo, forno de ligas e extratora Mk.II 4, extratora, triturador e geotérmico 2, demais máquinas 1. A cada 3,2 segundos a poluição se espalha aos chunks vizinhos e 2% se dissipa. Cada colheita da colheitadeira absorve 3 unidades.

1. Quando um chunk passa de 400, o chat avisa os jogadores próximos e explica o que fazer.
2. Ataques só acontecem um dia inteiro depois desse aviso, só à noite e só se o chunk mais poluído passar de 1.200. Ao anoitecer, a barra de ação avisa.
3. A onda surge na cara de quem está a até 64 blocos da fábrica (ou a 18 blocos da máquina, se não houver ninguém): quatro **Saqueadores** na primeira noite, crescendo com a poluição e com as noites, até doze. É no máximo uma onda por noite. O som é o de invasão.
4. Eles correm até o jogador e batem como zumbi. Sem luta corpo a corpo, vão aos defensores do quartel (o guarda primeiro) e, sem soldado por perto, aos outros aldeões: quatro golpes matam um trabalhador, o guarda aguenta oito, e o posto fica parado até outra contratação. Sem jogador nem aldeão a 28 blocos, golpeiam a máquina no mesmo ritmo. À noite, zumbis, esqueletos e aranhas também caçam aldeões na mesma altura (os da caverna não sobem atrás deles), e um aldeão que passa 30 s sem apanhar se recupera. Esqueletos com o jogador a até 16 blocos continuam atirando nele. Lavradores e pastores dormem em casa à noite e voltam ao amanhecer. Eles queimam ao sol.
5. Máquina com integridade zerada para, sem perder estoque. Um **kit de reparo**, com o botão direito, recupera 50%.

Muros vanilla barram o caminho, a torre Tesla abate Saqueadores, o quartel treina milícia, guarda, arqueiro e mago, e lavouras e módulos de eficiência reduzem a poluição. No modo pacífico não há ataques. A poluição fica em memória e recomeça ao reabrir o mundo.

## A fábrica no mundo

A versão 0.7 liga a produção ao que acontece em volta da base.

**Colheitadeira.** Verifica uma área de 9x9 em volta de si, numa camada abaixo, na própria e numa acima. Quando trigo, cenoura, batata, beterraba, fungo do Nether, frutas silvestres ou cacau amadurecem, ela colhe e replanta no mesmo lugar, gastando 120 FE por colheita. Os produtos ficam em três saídas e seguem pelo bocal até esteiras, funis ou baús. Com farinha de osso no slot de adubo, adianta dois estágios de uma planta por farinha. Uma volta completa leva cerca de quatro segundos.

**Torre Tesla.** Abate o monstro hostil mais próximo num raio de 10 blocos, com 2.400 FE por disparo e intervalo de dois segundos. Guarda 20.000 FE. Aldeões, animais e chefes não são alvos. Como os painéis solares param à noite, a defesa depende de acumuladores ou de geradores. Os itens dos monstros caem no chão, e os ossos viram farinha no triturador, alimentando a lavoura. Um sinal de redstone desliga a torre.

**Esmeraldas.** A exportação contínua da central transforma a produção eletrônica em esmeraldas para negociar com os aldeões.

## Direção de jogo

As metas combinam domínio do fluxo, eficiência e construção no mundo. Primeiro, uma linha entrega produtos; depois, o jogador estabiliza essa entrega; por fim, usa a produção em um projeto de infraestrutura.

Uma meta de produção sustentada pode pedir 20 circuitos por minuto durante três minutos, com pausas e retomada livres. Uma meta energética pode pedir abastecimento durante uma noite usando solar e acumuladores. Essas condições de avaliação contínua fazem parte do planejamento posterior aos contratos de entrega da 0.5.

A integração com Minecraft deve criar usos para a fábrica.

- Redstone controla a produção por estoque.
- Trilhos e carrinhos ligam jazidas, bases e entrepostos.
- Funis e fornalhas participam das linhas, incluindo produção de carvão vegetal.
- Comércio com aldeões oferece destino para entregas e esmeraldas.
- Nether fornece quartzo e lava para rotas industriais.
- Faróis permitem transformar exploração e construção em um projeto de fábrica.

Os contratos implementados entregam materiais e recompensas, e a lavoura e a defesa já reagem ao mundo. A avaliação de construções no terreno, rotas ferroviárias e presença de povoados pertence às próximas etapas. Rotas alternativas de produção devem criar escolhas entre exploração, energia e complexidade, como obter silício de quartzo ou desenvolver uma rota de areia com mais etapas.
