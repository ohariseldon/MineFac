# Veio 0.6

Código do projeto **MineFac**. Para continuar o desenvolvimento localmente ou na nuvem, leia [CONTRIBUTING.md](CONTRIBUTING.md). A verificação de conteúdo funciona com Python, sem instalar o jogo.

Automação industrial no mundo procedural do Minecraft, executada no Horizon. Abra a instância **Veio** pelo launcher. No criativo, as abas **Veio – Indústria**, **Veio – Energia** e **Veio – Logística** reúnem o conteúdo.

## Tutorial dentro do jogo

Ao entrar no mundo, cada jogador recebe uma vez o **Guia do Veio**. Selecione o livro na barra e abra com o botão direito. O sumário é clicável. Comece pela primeira fábrica de cobre e siga até a primeira engrenagem. As páginas finais servem de consulta para uso e fabricação das máquinas.

- Abra **Avanços > Primeira fábrica** para acompanhar materiais obtidos. Recolha os produtos no inventário para registrar os marcos.
- Abra uma máquina com botão direito. Os rótulos mostram entradas e saídas. Passe o cursor sobre um slot vazio para saber qual material colocar.
- Passe o cursor no **?** do painel para consultar funcionamento e receita. No inventário, máquinas e componentes também mostram sua função.
- Perdeu o guia? Junte **um livro e um papel** na bancada para fabricar outro.
- O guia acompanha o idioma do jogo, com português brasileiro e inglês. As opções de idioma ficam nas configurações do Horizon.

O começo usa receitas sem posição fixa na bancada, uma extratora diretamente sobre minério de cobre, gerador com carvão e cabos encostando em cada máquina. Monte `extratora > esteira > prensa > baú` na mesma altura. A prensa usa **cobre bruto**, e o montador transforma **duas chapas de cobre** em uma engrenagem.

## Energia

- O gerador a carvão fornece até 40 FE/t. Cada carvão ou carvão vegetal rende 16.000 FE. O combustível fica em espera quando o reservatório enche.
- O gerador geotérmico fornece até 80 FE/t. Um balde de lava rende 64.000 FE e devolve um balde vazio no slot de saída. A saída pode ser automatizada com esteiras e funis.
- Cada painel solar fornece até 16 FE/t ao meio-dia. Precisa de luz do céu e funciona no Overworld. Chuva, tempestade e horário reduzem a geração.
- Cada acumulador armazena 30.000 FE. A receita entrega quatro unidades. Use bancos de acumuladores para atravessar a noite.
- Cabos e máquinas armazenam até 2.000 FE. Cabos e acumuladores equilibram a porcentagem de carga e alimentam máquinas adjacentes pelas seis faces. Para começar, use redes curtas.
- Esteiras têm acionamento mecânico. A comum transporta até 2,5 itens/s e a rápida até 5 itens/s.

Os painéis de inspeção mostram energia, progresso e motivo de espera. Os estoques, o progresso, a carga e a queima em andamento são salvos no mundo. A energia armazenada pertence ao bloco colocado. Use a chave para mudar sua orientação.

## Extração e processamento

As extratoras trabalham diretamente sobre as jazidas existentes do Minecraft. As jazidas permanecem disponíveis para sustentar a fábrica.

- Cobre e ferro, comuns ou de ardósia, produzem minério bruto.
- Ouro, comum ou de ardósia, produz ouro bruto.
- Carvão, comum ou de ardósia, produz carvão para alimentar geradores e ligas.
- Redstone, comum ou de ardósia, produz pó de redstone.
- Quartzo do Nether produz quartzo.

A extratora básica produz uma unidade a cada 2 segundos e consome 6 FE/t enquanto trabalha. A Mk.II produz uma unidade por segundo e consome 12 FE/t. As duas gastam 240 FE por unidade extraída.

As linhas de produção usam estas transformações.

- Prensa. Um cobre bruto vira uma chapa de cobre, um ferro bruto vira uma chapa de ferro e um ouro bruto vira um lingote de ouro.
- Montador. Duas chapas de cobre viram uma engrenagem.
- Cortadora. Uma chapa de ferro vira uma moldura. Uma chapa de cobre vira uma fiação de cobre.
- Integrador. Uma moldura e uma engrenagem viram um módulo mecânico.
- Triturador. Um quartzo vira uma pastilha de silício.
- Forno de ligas. Duas chapas de ferro e um carvão ou carvão vegetal viram uma chapa de aço.
- Montador de circuitos. Duas fiações de cobre e uma pastilha de silício viram uma placa de circuito.

Máquinas aguardam material, energia e espaço na saída. Os ingredientes são consumidos juntos quando o ciclo termina. Dedique máquinas a materiais específicos para manter as saídas organizadas.

## Progressão

1. Cobre abre a primeira fábrica, com extratora, prensa e esteiras.
2. Engrenagens abrem a oficina industrial, com chave, triturador e forno de ligas.
3. Aço libera acumuladores, geotermia, montagem eletrônica e construção em lote.
4. Quartzo e fiação permitem produzir circuitos, que liberam solar e extração Mk.II.
5. O pedido de oito módulos mecânicos continua liberando o lote de 32 esteiras rápidas.

Os avanços entregam as receitas. Em um mundo anterior, recolha novamente os materiais do novo ramo para ativar os novos avanços.

## Construção e logística

Selecione a **chave de montagem** e clique com o botão direito para girar um bloco de Veio. A orientação muda mantendo o inventário, o progresso e a carga. Com a mão vazia, o botão direito abre o painel da máquina.

A saída das máquinas aponta para longe de quem colocou o bloco. Esteiras podem fazer curvas pela orientação do próximo segmento. O divisor alterna as duas saídas laterais e desvia os itens quando uma delas recusa a carga.

Uma receita inicial entrega 16 esteiras. Com aço, uma chapa de aço, uma de cobre e dois gravetos entregam 64 esteiras comuns. Uma chapa de aço e duas fiações entregam 64 cabos. Os lotes permitem ampliar a fábrica com menos operações manuais.

## Visual

Os modelos usam painéis metálicos, grades, faixas de segurança e detalhes de esteira do conjunto CC0 **Platformer Pack – Industrial**, de Kenney. Os ícones de componentes, a interface escura e as cores das máquinas seguem a mesma linguagem industrial. As luzes indicam atividade, bloqueio e falta de recursos.

A geometria dos blocos e a lógica do Veio pertencem a este projeto. As referências a texturas do Minecraft são resolvidas pela instalação do jogador. Créditos e licença estão em [CREDITS.md](CREDITS.md).

## Instalação e dados

A instância precisa do pacote em content/ e do módulo Rust em native/api/. A compilação pelo launcher instala o módulo na instância Veio. Alterações que registram novos blocos exigem reiniciar o jogo.

Os dados do launcher ficam em E:\GPTmodMine\.launcher, os mundos e instâncias em E:\GPTmodMine\.game e a compilação em E:\GPTmodMine\.build. Essas pastas estão excluídas do controle de versão.

O mundo **Veio03-Laboratorio** contém a demonstração da versão anterior. Os novos equipamentos estão disponíveis nas abas criativas e na progressão da 0.4. Blocos colocados quando apenas o pacote de conteúdo estava instalado podem precisar ser recolocados para criar seu inventário.

A validação de jogabilidade desta expansão fica com o jogador. O desenvolvimento desta versão concentra as mudanças antes da compilação, seguindo a preferência por evitar testes no jogo a cada iteração.

## Projetos, indicadores e redstone

A versão 0.5 acrescenta uma central de contratos, indicadores de produção e comando por redstone.

A central é fabricada com um baú, uma engrenagem, um lingote de ferro e um de cobre. Tem duas entradas à esquerda e duas saídas à direita. Os contratos são concluídos quando os ingredientes e o espaço para todas as recompensas estão disponíveis.

1. Oficina. Entregue 24 chapas de cobre e oito engrenagens para receber 64 esteiras e 32 cabos.
2. Entrepostos. Entregue 16 chapas de aço e oito módulos mecânicos para receber 64 trilhos e 16 trilhos elétricos.
3. Povoado. Entregue 12 circuitos e 24 molduras para receber oito funis e oito esmeraldas.
4. Farol industrial. Entregue uma estrela do Nether e 32 circuitos para receber um farol e duas extratoras Mk.II. É um projeto opcional depois da infraestrutura.

O progresso é salvo na central colocada. Sua desmontagem inicia uma central nova. A chave permite reorientá-la mantendo o progresso. Ao mudar de contrato, retire sobras das entradas e direcione os materiais do projeto seguinte. As recompensas podem sair por funis ou pelo bocal frontal.

As máquinas de produção passam a mostrar itens produzidos por minuto e porcentagem de tempo ativo. A primeira leitura usa a amostra parcial; depois as leituras fecham a cada minuto de simulação com a máquina carregada. Falta de materiais, falta de energia, saída cheia e suspensão por redstone contam como tempo de espera. Uma linha equilibrada pode esperar ao atingir o estoque desejado.

Um sinal de redstone pausa a produção e preserva o progresso. A saída continua escoando produtos. Alavancas permitem comando manual; comparadores e circuitos de redstone podem transformar estoque em sinal de parada. O painel informa a suspensão.

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

Os contratos implementados entregam materiais e recompensas. A avaliação de construções no terreno, rotas ferroviárias e presença de povoados pertence às próximas etapas. Rotas alternativas de produção devem criar escolhas entre exploração, energia e complexidade, como obter silício de quartzo ou desenvolver uma rota de areia com mais etapas.
