"""Artigos da Wiki do Veio: a aba "Sistemas" da tela no jogo (`game`, pt e en, um parágrafo por
linha) e as seções do site da wiki (`web`, HTML em português, com o diagrama `diagram` de
wiki_site.py). Ao mudar uma regra do jogo, atualize o artigo dela aqui."""

ARTICLES = [
    {
        "id": "start", "icon": "veio:generator", "diagram": "start",
        "title": ("Primeiros passos", "First steps"),
        "game": ([
            "O Veio é automação industrial dentro do Minecraft: você extrai das jazidas do mundo, transforma em peças e monta linhas que trabalham sozinhas.",
            "O tutorial na tela (acima da barra de itens) leva do gerador à primeira encomenda em 8 etapas. Cada etapa mostra a receita, de onde vêm os materiais e uma lista que marca o que você já tem.",
            "• 1. Gerador com carvão  • 2. Prensa ligada por cabos  • 3. Chapas à mão  • 4. Engrenagem no montador",
            "• 5. Extratora sobre cobre  • 6. Esteira até a prensa  • 7. Esteira até um baú  • 8. Encomenda na central",
            "As receitas do Veio aparecem no livro de receitas (o livro verde do inventário) assim que são liberadas pelos avanços.",
            "Esta wiki abre com J. Clique num item para ver como fabricar, onde é produzido e onde é usado.",
        ], [
            "Veio is industrial automation inside Minecraft: mine the world's deposits, turn them into parts and build lines that run on their own.",
            "The on-screen tutorial (above the hotbar) goes from the generator to the first order in 8 steps. Each step shows the recipe, where materials come from and a checklist of what you already have.",
            "• 1. Generator with coal  • 2. Press wired with cables  • 3. Plates by hand  • 4. Gear in the assembler",
            "• 5. Extractor on copper  • 6. Belt into the press  • 7. Belt into a chest  • 8. Order at the hub",
            "Veio recipes show up in the recipe book (the green book in your inventory) as soon as advancements unlock them.",
            "This wiki opens with J. Click an item to see how to craft it, where it is made and where it is used.",
        ]),
        "web": """
<p>O Veio é um jogo de <b>automação industrial</b> dentro do Minecraft. Você extrai minério das jazidas que o próprio mundo gera, transforma em peças nas máquinas e monta <b>linhas de produção</b> que continuam trabalhando enquanto você explora. Em volta da fábrica cresce uma <b>vila</b> com trabalhadores, famílias, lojas e uma prefeitura.</p>
<h3>O tutorial</h3>
<p>Um objetivo por vez aparece acima da barra de itens. Cada etapa traz uma <b>ficha</b> no chat com a receita, de onde vem cada material e os passos; a receita é liberada no <b>livro de receitas</b> (clique nela e a grade se monta); e uma <b>lista de materiais</b> marca em verde o que você já tem. Ao concluir, título na tela, som de conquista e experiência.</p>
<table><tr><th>Etapa</th><th>Objetivo</th><th>O que você aprende</th></tr>
<tr><td>1</td><td>Colocar um gerador com carvão</td><td>Energia</td></tr>
<tr><td>2</td><td>Ligar a prensa por cabos</td><td>Rede elétrica</td></tr>
<tr><td>3</td><td>Fazer 4 chapas à mão</td><td>Entrada, progresso e saída</td></tr>
<tr><td>4</td><td>Fazer 1 engrenagem no montador</td><td>Cadeias de produção</td></tr>
<tr><td>5</td><td>Extratora sobre minério de cobre</td><td>Extração automática</td></tr>
<tr><td>6</td><td>Esteira da extratora até a prensa</td><td>Transporte</td></tr>
<tr><td>7</td><td>Esteira da prensa até um baú</td><td>Saída e estoque</td></tr>
<tr><td>8</td><td>24 chapas + 8 engrenagens na central</td><td>Encomendas e metas</td></tr></table>
<p>Comandos: <code>/veiotutorial ocultar</code>, <code>/veiotutorial mostrar</code>, <code>/veiotutorial reiniciar</code>. A wiki dentro do jogo abre com <kbd>J</kbd>.</p>
""",
    },
    {
        "id": "power", "icon": "veio:pole", "diagram": "power",
        "title": ("Energia", "Power"),
        "game": ([
            "Máquinas gastam energia (FE). Ela sai dos geradores, passa por cabos encostados ou por postes com fios e fica guardada em cada máquina (até 2.000 FE) e nos acumuladores.",
            "• Gerador: até 40 FE/t queimando combustível de fornalha (carvão 16.000 FE).",
            "• Geotérmico: até 80 FE/t com balde de lava (64.000 FE); devolve o balde.",
            "• Caldeira a vapor: 120 FE/t, dobro de energia por combustível; precisa de água parada ao lado.",
            "• Roda d'água e cata-vento: até 32 FE/t, sem poluir. Solar: até 16 FE/t ao meio-dia.",
            "• Acumulador: guarda 30.000 FE para atravessar a noite.",
            "• Poste de madeira: alcança 8 blocos, alimenta num raio de 2 e passa 120 FE/t. Poste de aço: 16 blocos, raio 3, 400 FE/t.",
            "Se uma máquina mostra 'sem energia', confira combustível, cabos encostados e o alcance dos postes.",
        ], [
            "Machines use power (FE). It comes from generators, travels through touching cables or poles with wires and is stored in each machine (up to 2,000 FE) and in batteries.",
            "• Generator: up to 40 FE/t burning furnace fuel (coal 16,000 FE).",
            "• Geothermal: up to 80 FE/t from a lava bucket (64,000 FE); returns the bucket.",
            "• Steam engine: 120 FE/t, double energy per fuel; needs still water beside it.",
            "• Water wheel and wind turbine: up to 32 FE/t, no pollution. Solar: up to 16 FE/t at noon.",
            "• Battery: stores 30,000 FE to get through the night.",
            "• Wooden pole: reaches 8 blocks, powers a radius of 2 and carries 120 FE/t. Steel pole: 16 blocks, radius 3, 400 FE/t.",
            "If a machine says 'no power', check fuel, touching cables and pole reach.",
        ]),
        "web": """
<p>Toda máquina do Veio consome <b>energia</b>, medida em FE (e FE/t, por tick; 20 ticks = 1 segundo). A energia nasce nos geradores, viaja por <b>cabos</b> encostados ou por <b>postes</b> com fios aéreos e fica guardada no buffer de cada máquina (2.000 FE) e nos <b>acumuladores</b>.</p>
<table><tr><th>Fonte</th><th>Potência</th><th>Combustível / condição</th><th>Polui?</th></tr>
<tr><td>Gerador</td><td>até 40 FE/t</td><td>combustíveis da fornalha: carvão 16.000 FE, bloco de carvão 160.000, tronco 3.000, vara de blaze 24.000</td><td>sim (6/s)</td></tr>
<tr><td>Gerador a biomassa</td><td>40 FE/t</td><td>biomassa, plantas, feno</td><td>menos que o carvão</td></tr>
<tr><td>Geotérmico</td><td>até 80 FE/t</td><td>balde de lava = 64.000 FE; devolve o balde vazio</td><td>pouco (2/s)</td></tr>
<tr><td>Caldeira a vapor</td><td>120 FE/t</td><td>dobro de energia por combustível; água parada ao lado</td><td>sim</td></tr>
<tr><td>Roda d'água</td><td>8 FE/t por lado, até 32</td><td>água corrente; congela abaixo de −3 °C</td><td>não</td></tr>
<tr><td>Cata-vento</td><td>8 a 32 FE/t</td><td>céu aberto; mais alto e com chuva rende mais</td><td>não</td></tr>
<tr><td>Solar</td><td>até 16 FE/t</td><td>luz do céu; mais no verão, nada à noite</td><td>não</td></tr>
<tr><td>Turbina hidrelétrica</td><td>varia com a queda</td><td>represe um rio</td><td>não</td></tr></table>
<h3>Levando a energia</h3>
<ul><li><b>Cabo</b>: ligação curta entre blocos encostados. Receita: lingote de cobre + graveto = 16.</li>
<li><b>Poste de madeira</b>: liga-se sozinho aos até 6 postes vizinhos a até 8 blocos; alimenta máquinas num raio de 2 blocos; 120 FE/t por fio.</li>
<li><b>Poste de aço</b>: 16 blocos, raio 3, 400 FE/t por fio.</li>
<li><b>Acumulador</b>: 30.000 FE; a receita entrega 4. Bancos de acumuladores seguram a fábrica (e a torre Tesla) à noite.</li></ul>
<p class="tip">Gargalo clássico: uma linha de engrenagens (2 extratoras, 2 prensas, 1 montador) puxa 40 FE/t, mais do que uma roda d'água dá.</p>
""",
    },
    {
        "id": "io", "icon": "veio:belt", "diagram": "io",
        "title": ("Entrada, saída e esteiras", "Input, output and belts"),
        "game": ([
            "Cada máquina tem lados fixos: faixas VERDES (frente e laterais) recebem itens; a moldura VERMELHA (atrás) solta a produção e não aceita nada.",
            "A saída fica do lado oposto a você na hora de colocar a máquina. A chave gira a máquina sem perder o que ela guarda.",
            "A esteira anda para longe de você ao ser colocada e não aceita nada pela frente. Por isso a máquina nunca devolve o produto na esteira que a alimenta.",
            "Carga por bloco de esteira: até 5 minérios (ou pedra, toras) ou 1 produto pronto (2 na rápida, 4 na expressa). A esteira transporta; para estocar, use baús.",
            "• Comum: 2,5 itens/s  • Rápida: 5 itens/s  • Expressa: 10 itens/s",
            "Esteiras fazem curvas sozinhas e sobem ou descem 1 bloco por degrau. A chave fixa a direção; agachar com a chave volta ao automático.",
            "Divisor: alterna as duas saídas laterais (filtro e prioridade com agachar). Unificador: junta duas entradas numa saída. Túnel: passa por baixo de até 6 blocos.",
            "Redstone pausa máquinas, esteiras e braços.",
        ], [
            "Every machine has fixed sides: GREEN strips (front and sides) take items; the RED frame (back) releases products and accepts nothing.",
            "The output faces away from you when you place the machine. The wrench rotates it and keeps its contents.",
            "A belt runs away from you when placed and accepts nothing from its front. So a machine never pushes its product onto the belt that feeds it.",
            "Load per belt block: up to 5 ores (or stone, logs) or 1 finished product (2 on fast, 4 on express). Belts carry; use chests to store.",
            "• Basic: 2.5 items/s  • Fast: 5 items/s  • Express: 10 items/s",
            "Belts turn on their own and climb or drop 1 block per step. The wrench locks direction; sneak with it to go back to automatic.",
            "Splitter: alternates its two side outputs (filter and priority by sneaking). Merger: joins two inputs into one. Tunnel: goes under up to 6 blocks.",
            "Redstone pauses machines, belts and arms.",
        ]),
        "web": """
<p>Toda máquina tem <b>entrada</b> e <b>saída</b> fixas, marcadas no próprio bloco:</p>
<ul><li><span class="chip green">VERDE</span> faixas na frente e nas laterais: por ali a máquina <b>recebe</b> itens (esteiras, braços, funis).</li>
<li><span class="chip red">VERMELHO</span> moldura em volta do bico de trás: por ali a <b>produção sai</b>, e nada entra.</li></ul>
<p>A saída fica do lado <b>oposto a você</b> no momento de colocar a máquina. A <b>chave de montagem</b> gira a máquina mantendo inventário, progresso e energia.</p>
<h3>Esteiras</h3>
<ul><li>Andam para <b>longe de você</b> ao serem colocadas e <b>não aceitam nada pela frente</b> — assim a máquina nunca devolve o produto na esteira que a alimenta.</li>
<li>Carga por bloco: até <b>5 de matéria-prima</b> (minério, pedra, toras) ou <b>1 produto pronto</b> (2 na rápida, 4 na expressa). Esteira transporta; quem estoca é o baú.</li>
<li>Fazem <b>curvas</b> sozinhas quando há uma entrada e uma saída vizinhas, e <b>rampas</b> de 1 bloco por degrau.</li></ul>
<table><tr><th>Peça</th><th>Vazão</th><th>Uso</th></tr>
<tr><td>Esteira comum</td><td>2,5 itens/s (150/min)</td><td>linhas básicas</td></tr>
<tr><td>Esteira rápida</td><td>5 itens/s (300/min)</td><td>depois de juntar linhas</td></tr>
<tr><td>Esteira expressa</td><td>10 itens/s (600/min)</td><td>pesquisa de escala</td></tr>
<tr><td>Divisor</td><td>—</td><td>alterna duas saídas; filtro (agachar + item) e prioridade (agachar + chave)</td></tr>
<tr><td>Unificador</td><td>5 itens/s</td><td>duas entradas, uma saída</td></tr>
<tr><td>Túnel</td><td>—</td><td>dois na mesma direção, até 6 blocos por baixo</td></tr>
<tr><td>Braços</td><td>até 1 item/s</td><td>puxam de trás e entregam à frente; 40 FE por item</td></tr>
<tr><td>Elevador de carga</td><td>—</td><td>leva itens entre andares</td></tr></table>
<p class="tip">Máquina parada com "saída cheia"? Leve a produção embora pela moldura vermelha (esteira, braço ou baú).</p>
""",
    },
    {
        "id": "extraction", "icon": "veio:drill", "diagram": "purity",
        "title": ("Jazidas e extração", "Deposits and extraction"),
        "game": ([
            "A extratora trabalha em cima de uma jazida do mundo, sem quebrá-la: a jazida nunca acaba.",
            "Em minérios, ela mede o veio ligado ao bloco logo abaixo dela (pelas faces, em 3D). Minério comum e de ardósia contam juntos.",
            "• 1-2 blocos: impura, metade da velocidade  • 3-6: normal  • 7-15: pura, dobro  • 16+: industrial, 4x (com o dobro de potência)",
            "Pedra, areia, cascalho, argila e outras jazidas infinitas leem o quadrado 3x3 abaixo.",
            "Mk.I: 1 unidade a cada 2 s, 6 FE/t. Mk.II: 1 por segundo, 12 FE/t, e extrai diamante e esmeralda (devagar). Mk.III: 4x a Mk.I e detritos ancestrais.",
            "Uma extratora comum alimenta uma prensa. Planeje a linha por razões inteiras de máquinas.",
        ], [
            "The extractor works on top of a world deposit without breaking it: the deposit never runs out.",
            "On ores it measures the vein connected to the block right below it (by faces, in 3D). Normal and deepslate ore count together.",
            "• 1-2 blocks: impure, half speed  • 3-6: normal  • 7-15: pure, double  • 16+: industrial, 4x (double power)",
            "Stone, sand, gravel, clay and other infinite deposits read the 3x3 square below.",
            "Mk.I: 1 unit every 2 s, 6 FE/t. Mk.II: 1 per second, 12 FE/t, and mines diamond and emerald (slowly). Mk.III: 4x Mk.I and ancient debris.",
            "One basic extractor feeds one press. Plan lines in whole-number machine ratios.",
        ]),
        "web": """
<p>A <b>extratora</b> fica <b>em cima</b> de uma jazida do mundo e extrai sem quebrá-la: a jazida nunca se esgota.</p>
<h3>Pureza</h3>
<p>Em minérios, a extratora mede o <b>veio conectado</b> a partir do bloco logo abaixo dela, em 3D, pelas faces (minério comum e de ardósia são a mesma jazida). A busca para em 16 blocos.</p>
<table><tr><th>Blocos conectados</th><th>Classe</th><th>Velocidade</th></tr>
<tr><td>1–2</td><td>Impura</td><td>½×</td></tr><tr><td>3–6</td><td>Normal</td><td>1×</td></tr>
<tr><td>7–15</td><td>Pura</td><td>2×</td></tr><tr><td>16+</td><td>Industrial</td><td>4× (dobro de potência)</td></tr></table>
<p>Diamante, esmeralda e detritos param na pura. Pedra, ardósia, areia, cascalho, argila, netherrack, areia das almas, pedra-negra e pedra do End são <b>jazidas infinitas</b> e leem o quadrado 3×3 abaixo.</p>
<h3>Níveis</h3>
<table><tr><th>Extratora</th><th>Ritmo base</th><th>Energia</th><th>Extra</th></tr>
<tr><td>Mk.I</td><td>1 a cada 2 s</td><td>6 FE/t</td><td>minérios comuns</td></tr>
<tr><td>Mk.II</td><td>1 por segundo</td><td>12 FE/t</td><td>diamante e esmeralda (20× mais devagar), obsidiana, ametista</td></tr>
<tr><td>Mk.III</td><td>4× a Mk.I</td><td>30 FE/t</td><td>detritos ancestrais</td></tr></table>
""",
    },
    {
        "id": "chains", "icon": "veio:assembler", "diagram": "chains",
        "title": ("Cadeias de produção", "Production chains"),
        "game": ([
            "Cada máquina transforma uma coisa noutra. Encadeadas, viram uma linha.",
            "• Prensa: cobre bruto → chapa de cobre; ferro bruto → chapa de ferro",
            "• Montador: 2 chapas de cobre → engrenagem",
            "• Cortadora: chapa de cobre → fiação; chapa de ferro → moldura",
            "• Integrador: moldura + engrenagem → módulo; aço + 4 fiações → motor",
            "• Forno de ligas: 2 chapas de ferro + carvão → aço",
            "• Triturador: quartzo → silício; osso → farinha de osso; pedregulho → cascalho → areia",
            "• Montador de circuitos: 2 fiações + silício → circuito; 2 circuitos + 2 silícios → processador",
            "• Moinho, cozinha e fornalha elétrica: farinha, pão, bolos, assados e vidro",
            "A máquina espera material, energia e espaço na saída; os ingredientes são gastos juntos no fim do ciclo. Veja cada máquina na wiki para tempos e FE.",
        ], [
            "Each machine turns one thing into another. Chained, they become a line.",
            "• Press: raw copper → copper plate; raw iron → iron plate",
            "• Assembler: 2 copper plates → gear",
            "• Cutter: copper plate → wire; iron plate → frame",
            "• Integrator: frame + gear → module; steel + 4 wires → motor",
            "• Alloy furnace: 2 iron plates + coal → steel",
            "• Crusher: quartz → silicon; bone → bone meal; cobblestone → gravel → sand",
            "• Circuit assembler: 2 wires + silicon → circuit; 2 circuits + 2 silicon → processor",
            "• Mill, kitchen and electric furnace: flour, bread, cakes, roasts and glass",
            "A machine waits for materials, power and output space; ingredients are used together at the end of the cycle. Look up each machine for times and FE.",
        ]),
        "web": """
<p>Cada máquina faz uma transformação. Encadeadas, elas formam a <b>árvore de produção</b> do jogo: do minério bruto aos núcleos de automação. O diagrama abaixo sai direto das receitas do jogo.</p>
<p>A máquina espera <b>material</b>, <b>energia</b> e <b>espaço na saída</b>. Os ingredientes são consumidos juntos quando o ciclo termina. Dedique máquinas a produtos específicos para manter as saídas organizadas, e planeje por <b>razões inteiras</b>: uma extratora Mk.I alimenta uma prensa; duas prensas alimentam um montador.</p>
<p>A tabela completa de receitas de máquina (tempo, energia, itens por minuto) está em <a href="#machines">Máquinas</a>.</p>
""",
    },
    {
        "id": "progression", "icon": "veio:factory_board", "diagram": "phases",
        "title": ("Fases e pesquisa", "Phases and research"),
        "game": ([
            "A fábrica avança em fases, como no Satisfactory: dentro de uma fase, cada desbloqueio pede uma peça; passar de fase pede uma pilha, que só uma linha funcionando entrega.",
            "• 1 Cobre: um lingote de cobre abre extratora, prensa, esteiras, moinho e energia.",
            "• 2 Oficina: 32 engrenagens → chave, triturador, forno de ligas, kits de reparo, cata-vento.",
            "• 3 Aço: 16 chapas de aço → acumulador, geotermia, caldeira, poste de aço, silvicultura, rancho.",
            "• 4 Eletrônica: 16 circuitos → solar, extratora Mk.II, torre Tesla, módulos, lâmpada de cultivo.",
            "• 5 Profundidade: um módulo instalado → Mk.III.",
            "O Quadro da fábrica mostra a fase, o que falta e os pedidos do dia. O laboratório de pesquisa abre Metalurgia, Eletrônica, Automação e Escala com pacotes grandes de peças.",
            "Os avanços (tecla L) liberam as receitas: veja a aba do Veio para o caminho completo.",
        ], [
            "The factory advances in phases, as in Satisfactory: within a phase each unlock asks for one part; moving on asks for a stack that only a working line delivers.",
            "• 1 Copper: one copper ingot opens the extractor, press, belts, mill and power.",
            "• 2 Workshop: 32 gears → wrench, crusher, alloy furnace, repair kits, wind turbine.",
            "• 3 Steel: 16 steel plates → battery, geothermal, steam engine, steel pole, forestry, ranch.",
            "• 4 Electronics: 16 circuits → solar, Mk.II extractor, Tesla tower, modules, grow lamp.",
            "• 5 Depth: one installed module → Mk.III.",
            "The Factory board shows the phase, what is missing and the daily orders. The research lab opens Metallurgy, Electronics, Automation and Scale with large part packages.",
            "Advancements (L key) unlock recipes: see the Veio tab for the whole path.",
        ]),
        "web": """
<p>A progressão anda em <b>fases</b>, como no Satisfactory e no Factorio. Dentro de uma fase, os desbloqueios pedem <b>uma peça</b>; passar para a fase seguinte pede uma <b>pilha</b>, que só uma linha funcionando entrega em um a três minutos.</p>
<table><tr><th>Fase</th><th>Pede</th><th>Libera</th><th>Gargalo típico</th></tr>
<tr><td>1. Cobre</td><td>1 lingote de cobre</td><td>extratora, prensa, esteiras, moinho, energia básica</td><td>—</td></tr>
<tr><td>2. Oficina</td><td>32 engrenagens</td><td>chave, triturador, forno de ligas, kits de reparo, cata-vento</td><td>energia: a linha de engrenagens puxa 40 FE/t</td></tr>
<tr><td>3. Aço</td><td>16 chapas de aço</td><td>acumuladores, geotermia, caldeira, poste de aço, lotes, silvicultura, rancho</td><td>carvão e energia</td></tr>
<tr><td>4. Eletrônica</td><td>16 circuitos</td><td>solar, Mk.II, torre Tesla, módulos, lâmpada de cultivo</td><td>silício</td></tr>
<tr><td>5. Profundidade</td><td>1 módulo instalado</td><td>Mk.III</td><td>transporte</td></tr></table>
<h3>Pesquisa</h3>
<table><tr><th>Pesquisa</th><th>Pacote</th><th>Libera</th></tr>
<tr><td>Metalurgia</td><td>128 aço + 128 engrenagens</td><td>acumulador, caldeira, túnel, monitor</td></tr>
<tr><td>Eletrônica</td><td>128 circuitos + 128 molduras</td><td>solar, Tesla, módulos, âncora de chunk</td></tr>
<tr><td>Automação</td><td>64 motores + 64 processadores</td><td>fabricadora, extratora Mk.III</td></tr>
<tr><td>Escala</td><td>32 núcleos + 256 aço</td><td>esteira expressa</td></tr></table>
<p>O <b>Quadro da fábrica</b> mostra a fase, licenças, entregas automatizáveis, pedidos diários e o gargalo atual. Os <b>avanços</b> (tecla L) entregam as receitas.</p>
""",
    },
    {
        "id": "modules", "icon": "veio:speed_module", "diagram": None,
        "title": ("Módulos e gargalos", "Modules and bottlenecks"),
        "game": ([
            "A fábrica cresce resolvendo um gargalo por vez: a extração limita a prensa, a energia limita as máquinas, o fio limita a energia que chega longe, a poluição limita o tamanho sem defesa.",
            "Módulos entram com o botão direito numa máquina de receita ou extratora (um por máquina). Agachar com a chave devolve.",
            "• Velocidade: ciclo 60%, energia 180%, poluição 150%",
            "• Eficiência: energia 60%, poluição 50%",
            "• Produtividade: +20% (um lote extra a cada 5 ciclos), ciclo 115%, energia 140%",
            "O painel de cada máquina mostra itens por minuto e o tempo ativo: use para achar onde a linha trava.",
        ], [
            "The factory grows by solving one bottleneck at a time: extraction limits the press, power limits machines, wire limits far power, pollution limits size without defense.",
            "Modules go in by right-clicking a recipe machine or extractor (one each). Sneak with the wrench to take it back.",
            "• Speed: 60% cycle, 180% power, 150% pollution",
            "• Efficiency: 60% power, 50% pollution",
            "• Productivity: +20% (one extra batch every 5 cycles), 115% cycle, 140% power",
            "Each machine panel shows items per minute and active time: use it to find where the line stalls.",
        ]),
        "web": """
<p>A fábrica cresce resolvendo <b>um gargalo de cada vez</b>: a extração limita a prensa, a energia limita as máquinas, o fio de madeira limita a energia que chega longe, a poluição limita o tamanho que a base aguenta sem defesa.</p>
<table><tr><th>Módulo</th><th>Ciclo</th><th>Energia/tick</th><th>Poluição</th><th>Efeito</th></tr>
<tr><td>Velocidade</td><td>60%</td><td>180%</td><td>150%</td><td>mais produção por máquina, 8% mais energia por produto</td></tr>
<tr><td>Eficiência</td><td>100%</td><td>60%</td><td>50%</td><td>menos energia e metade da poluição</td></tr>
<tr><td>Produtividade</td><td>115%</td><td>140%</td><td>120%</td><td>um lote extra a cada cinco ciclos (+20%)</td></tr></table>
<p>Instale com o botão direito (um por máquina); agachar com a chave devolve. O painel mostra itens por minuto, tempo ativo, módulo, poluição e integridade.</p>
""",
    },
    {
        "id": "transport", "icon": "veio:rail_terminal", "diagram": None,
        "title": ("Transporte entre zonas", "Transport between zones"),
        "game": ([
            "Para ligar uma mina distante à oficina: terminais ferroviários (vagões com baú e funis), portos de drones (até 16 itens a até 64 blocos) e terminais de caravanas (cavalos, burros, mulas, porcos).",
            "Clique com o mesmo corante nas duas pontas para ligar a rota. Agache com o produto que deve sair para definir o envio.",
            "No painel, os três espaços da esquerda são Enviar e os da direita, Recebido. Carregadores abastecem Enviar e retiram de Recebido.",
            "Estações de envio e recepção ligam postos distantes pelo canal (corante), no ritmo medido mesmo longe de você.",
        ], [
            "To link a far mine to the workshop: rail terminals (chest minecarts and hoppers), drone ports (up to 16 items up to 64 blocks) and caravan terminals (horses, donkeys, mules, pigs).",
            "Click both ends with the same dye to link the route. Sneak with the product that should leave to set the dispatch.",
            "In the panel the three left slots are Dispatch and the right ones Received. Haulers fill Dispatch and empty Received.",
            "Sender and receiver stations link distant posts by channel (dye), at the measured rate even far from you.",
        ]),
        "web": """
<p>Para ligar uma <b>mina distante</b> à oficina, o Veio tem três meios, configurados do mesmo jeito:</p>
<table><tr><th>Meio</th><th>Como anda</th><th>Libera com</th></tr>
<tr><td>Terminal ferroviário</td><td>trilhos e vagões com baú do Minecraft; funis carregam e descarregam</td><td>módulo mecânico</td></tr>
<tr><td>Porto de drones</td><td>até 16 itens a até 64 blocos, com energia</td><td>circuitos</td></tr>
<tr><td>Terminal de caravanas</td><td>cavalos, burros/mulas e porcos com ração, até 512 blocos</td><td>engrenagens</td></tr></table>
<ol><li>Clique com o <b>mesmo corante</b> nas duas pontas (o corante não é gasto).</li>
<li>Agache e clique com o <b>produto que deve sair</b>: vira o filtro de envio.</li>
<li>No painel: três espaços <b>Enviar</b> à esquerda, três <b>Recebido</b> à direita.</li>
<li>Carregadores e a fila do capataz abastecem o envio e distribuem o recebido.</li></ol>
<p>As <b>estações de envio e recepção</b> ligam postos distantes pelo canal (corante) e seguem no ritmo medido mesmo longe do jogador.</p>
""",
    },
    {
        "id": "workforce", "icon": "veio:farm_post", "diagram": "workforce",
        "title": ("Vila operária", "Worker village"),
        "game": ([
            "Aldeões contratados trabalham sem energia elétrica: gastam comida e uma esmeralda de contratação.",
            "Com uma esmeralda na mão, clique no posto: o aldeão livre mais próximo (a até 8 blocos) é contratado.",
            "• Lavrador: colhe e replanta uma lavoura de até 9x9  • Pastor: cuida, tosquia e recolhe dos animais",
            "• Carregador: abastece máquinas a até 6 blocos e recolhe os produtos  • Capataz: alimenta, recontrata e monta a fila de transferências",
            "• Quartel: milícia, guarda, arqueiro e mago defendem a vila à noite",
            "Comida no slot da esquerda: cada tarefa custa 2 pontos (pão vale 6, carne assada 8, trigo 1).",
            "Zonas de lavoura e cercado, celeiro e silo crescem por níveis. O centro de controle é a sede: metas, holding e comércio exterior.",
            "Agachar + chave dispensa o trabalhador. Trabalhadores civis dormem à noite.",
        ], [
            "Hired villagers work without electricity: they use food and one hiring emerald.",
            "With an emerald in hand, click the post: the nearest free villager (within 8 blocks) is hired.",
            "• Farmer: harvests and replants up to 9x9  • Shepherd: tends, shears and collects from animals",
            "• Hauler: feeds machines within 6 blocks and collects products  • Foreman: feeds, rehires and plans transfers",
            "• Barracks: militia, guard, archer and mage defend the village at night",
            "Food in the left slot: each task costs 2 points (bread is worth 6, cooked meat 8, wheat 1).",
            "Field and pen zones, barn and silo grow by levels. The control center is the headquarters: goals, holding and foreign trade.",
            "Sneak + wrench dismisses the worker. Civilian workers sleep at night.",
        ]),
        "web": """
<p>A vila operária é a alternativa <b>sem energia elétrica</b>: em vez de FE, os postos gastam <b>comida</b> e uma <b>esmeralda de contratação</b>, e são mais lentos, porque o aldeão precisa andar. As máquinas continuam funcionando sem trabalhadores; os dois caminhos convivem.</p>
<h3>Contratar</h3>
<p>Com uma esmeralda na mão, clique com o botão direito no posto. O aldeão adulto livre mais próximo, a até 8 blocos, é contratado (primeiro os sem profissão, depois comerciantes de nível 1). Agachar + chave dispensa.</p>
<table><tr><th>Posto</th><th>Faz</th></tr>
<tr><td>Posto do lavrador</td><td>ara, planta e colhe uma lavoura de até 9×9 (trigo, cenoura, batata, beterraba, fungo do Nether, frutas doces, cacau)</td></tr>
<tr><td>Posto do pastor</td><td>alimenta, tosquia e recolhe os produtos dos animais</td></tr>
<tr><td>Posto do carregador</td><td>abastece máquinas a até 6 blocos e recolhe os produtos; usa escadas entre andares</td></tr>
<tr><td>Mesa do capataz</td><td>alimenta e recontrata os postos e monta a fila de transferências dos carregadores</td></tr>
<tr><td>Quartel</td><td>treina milícia, guarda, arqueiro e mago (troca de classe: agachar com esmeralda)</td></tr>
<tr><td>Centro de controle</td><td>sede da companhia: metas, holding por empresa, obras, decretos, comércio exterior</td></tr></table>
<p><b>Comida:</b> cada tarefa custa 2 pontos. Trigo vale 1 (não paga uma colheita), cenoura e batata 1, pão 6 (paga 3), carne assada 8 (paga 4).</p>
<p><b>Zonas:</b> marcos de lavoura e de cercado demarcam onde trabalhar; celeiro e silo guardam as sobras. A vila cresce até 150 aldeões, sempre com cama livre.</p>
""",
    },
    {
        "id": "society", "icon": "veio:town_hall", "diagram": "society",
        "title": ("Sociedade da vila", "Village society"),
        "game": ([
            "Casa: até 4 aldeões a até 10 blocos viram uma família. Quem trabalha ganha salário; a família come da despensa e compra na loja mais próxima.",
            "Loja: contrate um comerciante e abasteça o estoque. Os preços sobem com a procura e caem com estoque parado; cada vila tem o seu mercado.",
            "Prefeitura: contrate o prefeito. Ele mede população, emprego, PIB, IDH e Gini num raio de 128 blocos e libera as fases da vila.",
            "• Povoado → Vila (12 moradores, IDH 0,40) → Cidade (36, IDH 0,55, PIB 400/dia) → Metrópole (80, IDH 0,70, 3 bairros)",
            "Da fase 2, casa nova pede poço a até 24 blocos; da 3, também poste; da 4, também loja. IDH baixo demais suspende novos níveis de zona.",
            "Casas muito juntas (mais de 6 vizinhas a 16 blocos) ficam apertadas, e poluição perto piora a moradia: bairros espalhados vivem melhor.",
            "Livro na prefeitura: relatório com bairros, problemas e a meta da próxima fase.",
        ], [
            "House: up to 4 villagers within 10 blocks become a family. Workers earn wages; the family eats from the pantry and buys at the nearest shop.",
            "Shop: hire a merchant and stock it. Prices rise with demand and fall with idle stock; each village has its own market.",
            "Town hall: hire the mayor. They measure population, jobs, GDP, HDI and Gini within 128 blocks and unlock the village phases.",
            "• Hamlet → Village (12 residents, HDI 0.40) → City (36, HDI 0.55, GDP 400/day) → Metropolis (80, HDI 0.70, 3 districts)",
            "From phase 2 a new house needs a well within 24 blocks; from 3 also a pole; from 4 also a shop. Too low an HDI suspends new zone levels.",
            "Houses packed together (more than 6 neighbours within 16 blocks) are crowded, and nearby pollution hurts housing: spread districts live better.",
            "Book at the town hall: report with districts, problems and the next phase goal.",
        ]),
        "web": """
<p>A vila deixa de ser só mão de obra: ela tem <b>famílias</b>, <b>comércio</b> e um <b>Estado</b> (a prefeitura) com indicadores e barreiras de progressão.</p>
<h3>Casas e famílias</h3>
<p>Até 4 aldeões a até 10 blocos de uma casa formam uma família. A cada minuto, quem trabalha recebe <b>4 moedas</b> de salário (um salário sustenta duas pessoas); a família come 1 ponto de comida por pessoa da despensa e, de dia, compra comida e bens (lã, couro, livros, velas…) na loja mais próxima. Coloque as casas onde quiser: mais de 6 vizinhas a 16 blocos deixam a casa apertada, e poluição perto piora a moradia.</p>
<h3>Lojas</h3>
<p>Contrate um comerciante e abasteça o estoque (à mão, funil ou carregador). Os preços são <b>locais</b>: sobem quando a procura passa o estoque (até o dobro) e caem com estoque parado (até 70%). O comerciante compra no <b>atacado</b> (75% do preço) o que você entrega e paga com as vendas. Clique nele para comprar com esmeraldas.</p>
<h3>Prefeitura e fases</h3>
<table><tr><th>Fase</th><th>Pede</th><th>Libera</th><th>Casa nova pede</th></tr>
<tr><td>Povoado</td><td>—</td><td>zonas nível 2, 30 aldeões, 4 casas</td><td>—</td></tr>
<tr><td>Vila</td><td>12 moradores, 3 famílias, IDH 0,40</td><td>nível 3, 60 aldeões, 10 casas</td><td>poço a 24 blocos</td></tr>
<tr><td>Cidade</td><td>36, 9 famílias, IDH 0,55, PIB 400/dia, emprego 60%</td><td>nível 4, 100 aldeões, 24 casas</td><td>+ poste a 12</td></tr>
<tr><td>Metrópole</td><td>80, 20 famílias, IDH 0,70, Gini ≤ 0,45, 3 bairros</td><td>150 aldeões, 48 casas</td><td>+ loja a 40</td></tr></table>
<p><b>IDH</b> de cada família: comida, moradia, serviços (saneamento, energia, comércio e lazer) e renda, multiplicados pelo ânimo da vila. Se o IDH cai muito, o prefeito suspende novos níveis de zona e nascimentos.</p>
""",
    },
    {
        "id": "economy", "icon": "minecraft:emerald", "diagram": "money",
        "title": ("Economia e moeda", "Economy and money"),
        "game": ([
            "A esmeralda é a moeda: 1 esmeralda = 60 moedas. O dinheiro só circula; nada aparece ou some sozinho.",
            "Companhia (você, no cofre do centro de controle) → salários → famílias → lojas (varejo) → atacado → companhia.",
            "Em cada venda, o imposto é retido na fonte e vai para a prefeitura (4%, 8%, 12% e 16% pelas fases). A prefeitura paga o prefeito, os poços e a assistência aos pobres.",
            "Dinheiro só entra ou sai em esmeraldas de verdade: depósito no cofre, saque pelo painel, suas compras e o comércio exterior.",
            "O centro de controle mostra cada empresa da holding (mineração, manufatura, energia, logística, agro...) com receita, gastos e lucro.",
            "O preço de referência é um só dentro e fora da vila (o pão vale 9). O preço local nunca passa do custo de importar.",
        ], [
            "The emerald is the currency: 1 emerald = 60 coins. Money only circulates; nothing appears or vanishes on its own.",
            "Company (you, in the control center vault) → wages → families → shops (retail) → wholesale → company.",
            "Each sale withholds tax at source for the town hall (4%, 8%, 12% and 16% by phase). The town hall pays the mayor, wells and aid to the poor.",
            "Money only enters or leaves as real emeralds: vault deposits, withdrawals from the panel, your purchases and foreign trade.",
            "The control center shows each holding company (mining, manufacturing, power, logistics, agro...) with revenue, costs and profit.",
            "There is one reference price inside and outside the village (bread is 9). The local price never exceeds the import cost.",
        ]),
        "web": """
<p>A economia é <b>fechada</b>: a esmeralda é a moeda (1 esmeralda = <b>60 moedas</b>) e o dinheiro só <b>circula</b>. A identidade contábil fecha sempre: o relatório da prefeitura mostra a moeda em circulação por conta.</p>
<h3>As contas</h3>
<table><tr><th>Conta</th><th>Quem é</th><th>Recebe</th><th>Paga</th></tr>
<tr><td>Companhia</td><td>você, no cofre do centro de controle</td><td>atacado das lojas, exportações</td><td>salários dos seus trabalhadores</td></tr>
<tr><td>Famílias</td><td>cada casa</td><td>salários, assistência</td><td>varejo nas lojas</td></tr>
<tr><td>Lojas</td><td>comerciantes independentes</td><td>varejo (famílias e você)</td><td>atacado à companhia (75%)</td></tr>
<tr><td>Prefeitura</td><td>o Estado</td><td>imposto retido na fonte em cada venda</td><td>prefeito, poços, assistência, obras cívicas, festas</td></tr></table>
<h3>Números de equilíbrio</h3>
<ul><li>Custo de vida 40 moedas/dia; salário de 4 moedas por turno sustenta duas pessoas.</li>
<li>Atacado a 75% do preço local; reserva de caixa das lojas; poupança das famílias volta ao consumo.</li>
<li>Imposto: 4%, 8%, 12% e 16% pelas fases da vila.</li>
<li>Preço único de referência dentro e fora (o pão vale 9 nos dois); o preço local nunca passa do custo de importar com margem e imposto.</li></ul>
<p>O <b>centro de controle</b> é a sede da <b>holding</b>: abas para mineração, manufatura, energia, logística, agro, extrativismo, urbanização, comércio e gestão e segurança, cada uma com unidades, trabalhadores, folha, produção, receita, gastos e lucro.</p>
""",
    },
    {
        "id": "trade", "icon": "veio:export_dock", "diagram": "trade",
        "title": ("Comércio exterior", "Foreign trade"),
        "game": ([
            "Porto de carga (no mar ou rio largo, navio a cada 2 min) e estação de carga (na fronteira, trem a cada minuto) vendem à bolsa mundial.",
            "Cotação = valor × complexidade (1x a 4x) × índice do dia. Inundar um item derruba a cotação dele: diversifique.",
            "Frete por pilha: pesa mais na matéria-prima. Exportar produtos de maior valor agregado compensa.",
            "Contratos com prazo pagam 50% a mais e sobem a reputação, que abre mercados de componentes, eletrônicos e módulos.",
            "Quando falta comida, as lojas importam pelas conexões; a tarifa fica na prefeitura.",
            "Aba Comércio exterior no centro de controle: bolsa, contratos, reputação e o que está parado.",
        ], [
            "The cargo port (on the sea or a wide river, a ship every 2 min) and the cargo station (at the border, a train every minute) sell to the world market.",
            "Quote = value × complexity (1x to 4x) × daily index. Flooding an item drops its quote: diversify.",
            "Freight per stack weighs most on raw materials. Exporting higher value-added goods pays off.",
            "Timed contracts pay 50% more and raise reputation, which opens component, electronics and module markets.",
            "When food runs short, shops import through the connections; the tariff goes to the town hall.",
            "Foreign trade tab at the control center: market, contracts, reputation and what is stuck.",
        ]),
        "web": """
<p>No estilo das <b>conexões externas</b> do Cities: Skylines, a vila vende ao mundo por pontos físicos:</p>
<table><tr><th>Conexão</th><th>Onde</th><th>Partidas</th><th>Capacidade</th></tr>
<tr><td>Porto de carga</td><td>no mar ou rio largo</td><td>navio a cada 2 min</td><td>9 pilhas</td></tr>
<tr><td>Estação de carga</td><td>na fronteira, a 96 blocos da sede</td><td>trem a cada minuto</td><td>6 pilhas</td></tr></table>
<h3>Bolsa mundial</h3>
<p><b>Cotação = valor × complexidade (1× a 4×) × índice do dia.</b> Quanto mais você vende um item, mais a cotação dele cai (saturação), e ela volta aos poucos. O frete é por pilha e pesa mais na matéria-prima: exportar produtos de maior valor agregado compensa.</p>
<h3>Contratos e reputação</h3>
<p>Contratos com prazo pagam o preço mundial <b>+50%</b> e sobem a reputação, que abre camadas de mercado: componentes, eletrônicos e módulos. Mercados abertos não fecham mais.</p>
<h3>Importação</h3>
<p>Quando falta comida, as lojas importam pelas conexões (a moeda sai da economia; a tarifa fica na prefeitura). Isso põe um <b>teto</b> no preço local.</p>
""",
    },
    {
        "id": "agenda", "icon": "veio:park", "diagram": "calendar",
        "title": ("Agenda, feriados e obras", "Calendar, holidays and civic works"),
        "game": ([
            "O ano tem 4 estações de 6 dias (24 dias). O 6º dia de cada estação é domingo.",
            "Feriados fixos: Fundação (primavera 1), Flores (primavera 4), São João (verão 3), Colheita (outono 4), Luzes (inverno 3), Virada do Ano (inverno 6). No feriado a vila para de trabalhar e comemora na praça.",
            "Dois dias antes, um visitante chega à prefeitura. Clique nele: ele conta o pedido. Entregue os materiais no 3º espaço da prefeitura (Agenda).",
            "Pedido pronto: a prefeitura reembolsa a companhia e, no dia, o ânimo sobe (+20 a +25). Feriado sem preparo: ânimo −10.",
            "Domingos sem feriado têm feira, baile ou torneio de pesca (+8 de ânimo se preparado).",
            "Obras cívicas: a cada 4 dias o prefeito pede um parque (240), praça (420) ou monumento (720) para o bairro sem lazer. Construa a até 32 blocos em 6 dias.",
            "Ânimo (0 a 100) multiplica o IDH de 0,9x a 1,1x; lazer conta como serviço do IDH.",
        ], [
            "The year has 4 seasons of 6 days (24 days). The 6th day of each season is Sunday.",
            "Fixed holidays: Founding (spring 1), Flowers (spring 4), Saint John (summer 3), Harvest (autumn 4), Lights (winter 3), New Year (winter 6). On a holiday the village stops working and celebrates at the plaza.",
            "Two days before, a visitor arrives at the town hall. Click them: they tell you the request. Deliver the materials in the town hall's 3rd slot (Agenda).",
            "Request ready: the town hall reimburses the company and morale rises on the day (+20 to +25). Unprepared holiday: morale −10.",
            "Sundays without a holiday have a fair, a dance or a fishing contest (+8 morale if prepared).",
            "Civic works: every 4 days the mayor asks for a park (240), plaza (420) or monument (720) for the district without leisure. Build it within 32 blocks in 6 days.",
            "Morale (0 to 100) multiplies HDI by 0.9x to 1.1x; leisure counts as an HDI service.",
        ]),
        "web": """
<p>O tempo passa com <b>estações</b>, <b>domingos</b> e <b>feriados</b>, como no calendário de Stardew Valley. Personagens chegam à prefeitura pedindo festas e obras, e o prefeito transforma cada pedido numa <b>missão secundária</b>.</p>
<table><tr><th>Data</th><th>Feriado</th><th>Quem pede</th><th>O que pede</th></tr>
<tr><td>Primavera, 1</td><td>Dia da Fundação</td><td>Seu Anselmo</td><td>2 bolos, 32 pães, 4 bandeiras brancas</td></tr>
<tr><td>Primavera, 4</td><td>Festa das Flores</td><td>Dona Margarida</td><td>16 papoulas, 16 dentes-de-leão, 4 garrafas de mel</td></tr>
<tr><td>Verão, 3</td><td>Festa de São João</td><td>Seu Benedito</td><td>24 batatas assadas, 2 fogueiras, 16 biscoitos</td></tr>
<tr><td>Outono, 4</td><td>Festa da Colheita</td><td>Tia Ceres</td><td>48 trigos, 8 tortas de abóbora, 16 pães</td></tr>
<tr><td>Inverno, 3</td><td>Festa das Luzes</td><td>Mestre Lumen</td><td>16 lanternas, 8 velas, 16 vidros</td></tr>
<tr><td>Inverno, 6</td><td>Virada do Ano</td><td>Pequena Aurora</td><td>16 fogos, 1 bolo, 16 biscoitos</td></tr></table>
<h3>Como funciona</h3>
<ol><li>Dois dias antes, o visitante chega à prefeitura (o chat avisa).</li>
<li>Clique nele: ele cumprimenta e diz o que falta.</li>
<li>Entregue no espaço <b>Agenda</b> da prefeitura (3º espaço).</li>
<li>Pronto: a prefeitura reembolsa a companhia com a conta pública; no dia, o ânimo sobe.</li></ol>
<p><b>Obras cívicas:</b> a cada 4 dias chega um pedido de <b>parque</b> (240 moedas), <b>praça</b> (420) ou <b>monumento</b> (720) para o bairro com mais casas sem lazer. Construa o bloco a até 32 blocos dele em 6 dias; a prefeitura paga a obra.</p>
<table><tr><th>O quê</th><th>Efeito no ânimo</th></tr><tr><td>Feriado preparado</td><td>+20 a +25</td></tr><tr><td>Feriado sem preparo</td><td>−10 (a vila para do mesmo jeito)</td></tr>
<tr><td>Domingo preparado</td><td>+8</td></tr><tr><td>Obra cívica pronta</td><td>+10 (monumento +12) e lazer permanente</td></tr><tr><td>Obra atrasada</td><td>−5</td></tr><tr><td>Todo dia</td><td>volta 3 pontos em direção a 50</td></tr></table>
""",
    },
    {
        "id": "defense", "icon": "veio:tesla", "diagram": "pollution",
        "title": ("Poluição e defesa", "Pollution and defense"),
        "game": ([
            "Máquinas poluem o chunk: gerador 6/s, forno de ligas e Mk.II 4, extratora, triturador e geotérmico 2, demais 1. A poluição se espalha e 2% some a cada passo.",
            "Acima de 400, o chat avisa. Um dia depois, à noite, com o chunk mais poluído acima de 1.200, vêm Saqueadores (4 na primeira noite, até 12).",
            "Eles atacam quem está na fábrica, depois os defensores do quartel, os aldeões e por fim as máquinas.",
            "• Torre Tesla: abate o hostil mais próximo a 10 blocos (2.400 FE por disparo)",
            "• Quartel: milícia, guarda, arqueiro e mago  • Muros vanilla barram o caminho",
            "• Lavouras e módulos de eficiência reduzem a poluição  • Kit de reparo recupera 50% de uma máquina",
            "No modo pacífico não há ataques.",
        ], [
            "Machines pollute their chunk: generator 6/s, alloy furnace and Mk.II 4, extractor, crusher and geothermal 2, others 1. Pollution spreads and 2% fades each step.",
            "Above 400, chat warns you. A day later, at night, with the most polluted chunk above 1,200, Raiders come (4 the first night, up to 12).",
            "They attack whoever is at the factory, then the barracks defenders, the villagers and finally the machines.",
            "• Tesla tower: kills the nearest hostile within 10 blocks (2,400 FE per shot)",
            "• Barracks: militia, guard, archer and mage  • Vanilla walls block the way",
            "• Crops and efficiency modules reduce pollution  • A repair kit restores 50% of a machine",
            "There are no attacks in peaceful mode.",
        ]),
        "web": """
<p>Máquinas trabalhando poluem o chunk onde estão; a poluição se espalha aos vizinhos a cada 3,2 s e 2% se dissipa.</p>
<table><tr><th>Máquina</th><th>Poluição por segundo</th></tr><tr><td>Gerador</td><td>6</td></tr><tr><td>Forno de ligas, extratora Mk.II</td><td>4</td></tr>
<tr><td>Extratora, triturador, geotérmico</td><td>2</td></tr><tr><td>Demais máquinas</td><td>1</td></tr><tr><td>Colheita da colheitadeira</td><td>−3 por colheita</td></tr></table>
<h3>As ondas</h3>
<ol><li>Chunk acima de <b>400</b>: o chat avisa e explica o que fazer.</li>
<li>Ataques só um dia inteiro depois, <b>só à noite</b>, e só se o chunk mais poluído passar de <b>1.200</b>.</li>
<li>A onda surge perto de quem está a até 64 blocos da fábrica: 4 <b>Saqueadores</b> na primeira noite, crescendo até 12; o <b>Bruto</b> aparece nas noites seguintes.</li>
<li>Eles correm ao jogador; sem luta, vão aos defensores do quartel, depois aos aldeões, e por fim às máquinas.</li></ol>
<h3>Defesa</h3>
<ul><li><b>Torre Tesla</b>: abate o hostil mais próximo num raio de 10 blocos, 2.400 FE por disparo, a cada 2 s; guarda 20.000 FE.</li>
<li><b>Quartel</b>: milícia, guarda (se cura), arqueiro e mago; troca de classe agachando com uma esmeralda.</li>
<li>Muros vanilla, lavouras (absorvem poluição) e módulos de eficiência.</li>
<li><b>Kit de reparo</b>: recupera 50% da integridade de uma máquina; máquina zerada para sem perder estoque.</li></ul>
""",
    },
    {
        "id": "climate", "icon": "veio:solar", "diagram": None,
        "title": ("Clima e estações", "Climate and seasons"),
        "game": ([
            "O ano tem 4 estações de 6 dias. A temperatura muda com a estação (15, 25, 13 e 1 °C de média), a hora, a altitude e a chuva.",
            "• Solar: rende mais no verão (×1,18) e menos no inverno (×0,62)",
            "• Vento: ×0,85 a ×1,2  • Rios: enchem na primavera  • Roda d'água congela abaixo de −3 °C",
            "• A geada para a lavoura abaixo da temperatura mínima de cada cultura; com telhado (estufa) vale 20 °C",
            "No inverno neva e os lagos congelam; na primavera derretem. O painel das máquinas de energia mostra estação e temperatura.",
        ], [
            "The year has 4 seasons of 6 days. Temperature changes with the season (15, 25, 13 and 1 °C on average), time, altitude and rain.",
            "• Solar: more in summer (×1.18), less in winter (×0.62)",
            "• Wind: ×0.85 to ×1.2  • Rivers: swell in spring  • Water wheels freeze below −3 °C",
            "• Frost stops crops below each crop's minimum temperature; under a roof (greenhouse) it counts as 20 °C",
            "In winter it snows and lakes freeze; they thaw in spring. Power machine panels show season and temperature.",
        ]),
        "web": """
<p>O ano tem <b>4 estações de 6 dias</b> (24 dias, cerca de 8 horas de jogo). A temperatura sai da média da estação (primavera 15 °C, verão 25, outono 13, inverno 1), ±5,5 °C ao longo do dia, −1 °C a cada 10 blocos acima de y 64 e −3 °C na chuva.</p>
<table><tr><th>Sistema</th><th>Efeito do clima</th></tr>
<tr><td>Solar</td><td>×1,18 no verão a ×0,62 no inverno; −0,4%/°C acima de 25 °C</td></tr>
<tr><td>Cata-vento</td><td>×0,85 a ×1,2</td></tr>
<tr><td>Turbina de rio</td><td>×1,35 na primavera, ×0,75 no verão</td></tr>
<tr><td>Roda d'água</td><td>congela abaixo de −3 °C</td></tr>
<tr><td>Lavoura (aspersor)</td><td>para abaixo da mínima da cultura (trigo 3 °C … melão 15 °C); estufa com telhado vale 20 °C</td></tr></table>
<p>No inverno neva no chão e a água parada congela; na primavera derrete. Folhas, grama e água ganham a cor de cada estação.</p>
""",
    },
    {
        "id": "megafactory", "icon": "veio:blueprint_designer", "diagram": None,
        "title": ("Megafábricas", "Megafactories"),
        "game": ([
            "Copiadora: monte a linha no quadrado acima dela e clique com papel; ela vira uma planta.",
            "Marco de obra: com a planta, a mão vazia mostra o orçamento de materiais; agachado com a chave, constrói pagando os materiais.",
            "Plantas preservam orientação e ajustes (filtros, prioridades), sem copiar inventários.",
            "Andares: carregadores sobem por escadas de mão; o elevador de carga leva itens entre andares.",
        ], [
            "Copier: build the line on the square above it and click with paper; it becomes a blueprint.",
            "Build marker: with the blueprint, an empty hand lists the materials; sneak with the wrench to build, paying the materials.",
            "Blueprints keep orientation and settings (filters, priorities), without copying inventories.",
            "Floors: haulers climb ladders; the cargo lift carries items between floors.",
        ]),
        "web": """
<ul><li><b>Copiadora</b>: monte a linha no quadrado acima dela e clique com papel: ela vira uma planta.</li>
<li><b>Marco de obra</b>: com a planta, mão vazia mostra o orçamento; agachado com a chave, constrói pagando os materiais.</li>
<li>Plantas preservam orientação, filtros e prioridades, sem copiar inventários.</li>
<li><b>Fábricas verticais</b>: carregadores sobem por escadas de mão até o piso de cima; o <b>elevador de carga</b> leva itens entre andares.</li></ul>
""",
    },
    {
        "id": "help", "icon": "veio:wrench", "diagram": None,
        "title": ("Problemas comuns", "Troubleshooting"),
        "game": ([
            "Passe o mouse no estado do painel da máquina: ele diz o motivo de estar parada.",
            "• Sem energia: combustível no gerador, cabos encostados, alcance dos postes",
            "• Saída cheia: leve a produção embora pela moldura vermelha",
            "• Sem material: confira a entrada (faixas verdes) e a direção das esteiras",
            "• Pausada: um sinal de redstone está chegando nela",
            "• Esteira laranja: parada de verdade (o destino não aceita). Esteira que entra na face vermelha não é aceita.",
            "• Trabalhador ausente: o aldeão está longe, num chunk descarregado ou morreu; contrate de novo",
            "Comandos: /veiotutorial ocultar · mostrar · reiniciar. Wiki: tecla J.",
        ], [
            "Hover over the machine panel status: it tells you why it stopped.",
            "• No power: generator fuel, touching cables, pole reach",
            "• Output full: carry products away from the red frame",
            "• No material: check the input (green strips) and belt direction",
            "• Paused: a redstone signal is reaching it",
            "• Orange belt: truly stuck (the target refuses). A belt entering the red face is not accepted.",
            "• Worker missing: the villager is far, in an unloaded chunk or dead; hire again",
            "Commands: /veiotutorial ocultar · mostrar · reiniciar. Wiki: J key.",
        ]),
        "web": """
<p>Primeiro passo, sempre: <b>passe o mouse no estado do painel</b> da máquina. Ele diz o motivo.</p>
<table><tr><th>Sintoma</th><th>Causa provável</th><th>O que fazer</th></tr>
<tr><td>Sem energia</td><td>gerador sem combustível, cabo sem encostar, poste longe</td><td>abasteça; encoste os cabos; aproxime um poste (raio 2/3)</td></tr>
<tr><td>Saída cheia</td><td>nada leva a produção embora</td><td>esteira, braço ou baú na moldura vermelha</td></tr>
<tr><td>Sem material</td><td>entrada vazia ou esteira na face errada</td><td>entre pelas faixas verdes; confira a direção</td></tr>
<tr><td>Pausada</td><td>sinal de redstone</td><td>retire a alavanca ou o comparador</td></tr>
<tr><td>Esteira laranja</td><td>o destino recusa (face vermelha, cheio)</td><td>mude o destino ou esvazie</td></tr>
<tr><td>Trabalhador ausente</td><td>aldeão longe, chunk descarregado ou morto</td><td>contrate de novo com uma esmeralda</td></tr>
<tr><td>Vila não cresce</td><td>sem cama livre, IDH baixo, falta poço/poste/loja</td><td>veja o relatório da prefeitura</td></tr></table>
<h3>Comandos e teclas</h3>
<table><tr><th>Comando / tecla</th><th>Faz</th></tr>
<tr><td><kbd>J</kbd></td><td>abre e fecha a wiki dentro do jogo</td></tr>
<tr><td><kbd>L</kbd></td><td>avanços: o caminho das receitas</td></tr>
<tr><td><code>/veiotutorial ocultar</code></td><td>esconde o objetivo (a fábrica continua)</td></tr>
<tr><td><code>/veiotutorial mostrar</code></td><td>mostra de novo</td></tr>
<tr><td><code>/veiotutorial reiniciar</code></td><td>refaz os exercícios</td></tr>
<tr><td>Chave + botão direito</td><td>gira a máquina; agachado, devolve módulo ou dispensa trabalhador</td></tr></table>
""",
    },
]
