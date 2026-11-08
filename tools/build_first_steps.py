"""Guided commissioning exercises for Minecraft players new to automation.

Kept apart from the technical reference. Inventory milestones never claim to
verify placement or continuous flow; each exercise asks for an observable test.
"""

import math
import unicodedata


def wrapped_lines(text, limit=108):
    # Conservative vanilla book width; accented Latin letters use their base glyph.
    def width(word):
        widths={' ':4,'i':2,'l':3,'I':4,'t':4,'f':5,'k':5,'.':2,',':2,':':2,';':2,'!':2,"'":3,'>':5}
        return sum(widths.get(unicodedata.normalize('NFD',c)[0],6) for c in word)
    lines=[]
    for paragraph in text.split('\n'):
        line=''
        for word in paragraph.split():
            candidate=(line+' '+word).lstrip()
            if line and width(candidate)>limit:
                lines.append(line);line=word
            else: line=candidate
        lines.append(line)
    return lines


def build(label, read, write, assets, data):
    lessons = [
        ("Oficina da vila", "Village workshop", [
            ("\"Nossa oficina parou. Precisamos de chapas, engrenagens e de alguém que mantenha a produção.\"\n\nSeu primeiro pedido: 4 chapas de cobre. Vamos montar e testar cada trecho.",
             '"Our workshop stopped. We need plates, gears and someone to keep production running."\n\nFirst request: 4 copper plates. We will build and test one section at a time.'),
            ("Você já conhece Minecraft. Aqui, pense em FLUXO:\n\nentrada > máquina > saída\n\nA máquina transforma materiais. Energia faz trabalhar. Transporte leva itens. Você vai cuidar dessas três partes.",
             'You know Minecraft. Here, think in FLOWS:\n\ninput > machine > output\n\nMachines transform materials. Power runs them. Transport moves items. You will manage all three.')]),
        ("1. Prepare a bancada", "1. Prepare the workbench", [
            ("Meta: montar só o primeiro teste.\n\nSepare 3 lingotes de cobre, 2 de ferro, 1 pistão, 1 fornalha, 1 pedra lisa, 1 graveto, combustível e 4 cobres brutos.\n\nDeixe espaço para ampliar.",
             'Goal: build only the first test.\n\nGather 3 copper ingots, 2 iron ingots, 1 piston, 1 furnace, 1 smooth stone, 1 stick, fuel and 4 raw copper.\n\nLeave room to expand.'),
            ("Lingotes constroem as máquinas; cobre BRUTO vira chapa na prensa. Separe os dois.\n\nAs receitas estão no fim deste caderno. Faça gerador, prensa e 16 cabos. Não precisa de extratora ainda.",
             'Ingots build machines; RAW copper becomes plates in the press. Keep them separate.\n\nRecipes are at the back of this workbook. Make a generator, press and 16 cables. No extractor needed yet.')]),
        ("2. Faça chegar energia", "2. Supply power", [
            ("Coloque em linha, encostados:\n\ngerador > cabo > prensa\n\nAbra o gerador e ponha carvão. Abra a prensa: a energia deve subir.\n\nCabos levam energia; não levam cobre.",
             'Place in a touching line:\n\ngenerator > cable > press\n\nPut coal in the generator. Open the press: its energy should rise.\n\nCables carry power; they do not carry copper.'),
            ("Prensa carregada, mas parada? Isso é esperado: ainda falta material.\n\nSem energia? Veja se há combustível e se o cabo encosta nos dois blocos.\n\nConfirme a energia antes de alimentar a prensa.",
             'Press charged but stopped? Expected: it still needs material.\n\nNo power? Check fuel and whether the cable touches both blocks.\n\nConfirm power before feeding the press.')]),
        ("3. Seu primeiro ciclo", "3. Your first cycle", [
            ("Abra a prensa com a mão vazia. Ponha 4 cobres brutos na entrada à esquerda.\n\nObserve a seta avançar; recolha 4 chapas da saída à direita.\n\nCada ciclo usa 1 cobre e energia para fazer 1 chapa.",
             'Open the press with an empty hand. Put 4 raw copper in its left input.\n\nWatch progress advance; collect 4 plates from the right output.\n\nEach cycle uses 1 copper and power to make 1 plate.'),
            ("Teste: retire o material da entrada. Ela deve esperar; ponha-o de volta e veja o ciclo continuar.\n\nEsperar por material não é defeito. Leia o estado do painel antes de mudar a construção.",
             'Test: remove the input material. The press should wait; put it back and watch it resume.\n\nWaiting for material is not a fault. Read the panel status before changing the build.')]),
        ("4. Uma peça usa outra", "4. One part uses another", [
            ("Use 2 das chapas, 1 pistão e 1 bancada para fazer o montador. Ligue-o por um ramal de cabos.\n\nLeve as outras 2 chapas à entrada dele. Recolha 1 engrenagem.\n\nVocê ligou duas receitas.",
             'Use 2 plates, 1 piston and 1 crafting table to make an assembler. Power it with a cable branch.\n\nPut the other 2 plates in its input. Collect 1 gear.\n\nYou have connected two recipes.'),
            ("Cada engrenagem consome 2 chapas. O montador não trabalha se receber só uma.\n\nVocê ainda leva tudo à mão. Funciona, mas cada viagem prende você à produção. Vamos eliminar essas viagens.",
             'Each gear consumes 2 plates. The assembler cannot work with just one.\n\nYou still carry everything by hand. It works, but each trip ties you to production. Let us remove those trips.')]),
        ("5. Aceite a encomenda", "5. Take the order", [
            ("\"O teste funcionou! Agora a oficina precisa de 24 chapas e 8 engrenagens.\"\n\nUse sua engrenagem para fabricar a central de projetos. Abra-a e confira os dois materiais pedidos.",
             '"The test worked! Now the workshop needs 24 plates and 8 gears."\n\nUse your gear to craft a project hub. Open it and check the two requested materials.'),
            ("8 engrenagens usam 16 chapas. Somadas às 24 da entrega: produza mais 40 chapas. Reserve peças de construção à parte.\n\nEntregar paga 64 esteiras e 32 cabos para ampliar a fábrica.",
             '8 gears use 16 plates. Add the 24 for delivery: produce 40 more plates. Keep construction parts separate.\n\nDelivery pays 64 belts and 32 cables to expand your factory.')]),
        ("6. Automatize a coleta", "6. Automate extraction", [
            ("Faça uma extratora. Coloque-a SOBRE minério de cobre e ligue um cabo energizado. Preserve o minério abaixo.\n\nAbra o painel. Sem colocar cobre nela, veja cobre bruto aparecer na saída.",
             'Make an extractor. Place it ON copper ore and connect a powered cable. Keep the ore underneath.\n\nOpen its panel. Without putting copper in, watch raw copper appear in its output.'),
            ("Teste: deixe-a trabalhar enquanto organiza a oficina. Volte e confira a saída.\n\nEla repetiu a coleta por você. Ainda é preciso levar o cobre à prensa. Esse será o próximo trecho.",
             'Test: leave it working while you organize the workshop. Return and check its output.\n\nIt repeated extraction for you. You still need to carry copper to the press. That is the next section.')]),
        ("7. Automatize a viagem", "7. Automate the trip", [
            ("Uma opção: faça 16 esteiras e monte na mesma altura:\n\nextratora > esteira > prensa\n\nO bocal da extratora aponta à esteira; a esteira aponta à prensa. Se preciso, recoloque o bloco olhando para o destino.",
             'One option: make 16 belts and build at one height:\n\nextractor > belt > press\n\nThe extractor port faces the belt; the belt faces the press. If needed, replace the block while facing its destination.'),
            ("Ligue cabos à extratora E à prensa. A esteira não precisa de energia elétrica.\n\nTeste: esvazie a entrada da prensa e espere. Cobre deve chegar sozinho e virar chapa.\n\nVocê também pode usar braços, funis ou carregadores. O objetivo reconhece as entregas reais.",
             'Power BOTH extractor and press with cables. Belts need no electricity.\n\nTest: empty the press input and wait. Copper should arrive on its own and become plates.\n\nArms, hoppers and haulers also work. The objective recognizes actual deliveries.')]),
        ("8. Guarde sem recolher", "8. Store without collecting", [
            ("Na saída da prensa, coloque outra esteira e depois um baú:\n\nprensa > esteira > baú\n\nDeixe os bocais voltados ao baú. Você pode guardar também em barril, funil, celeiro, silo ou saída do posto do carregador.",
             'After the press output, place another belt and then a chest:\n\npress > belt > chest\n\nFace the ports toward the chest. Barrels, hoppers, barns, silos and hauler post outputs also count as storage.'),
            ("Teste: recolha as chapas do baú. Afaste-se sem abrir máquinas. Volte e veja se há novas chapas.\n\nSe houver, a linha trabalhou sem sua ajuda. Se não, confira energia, cobre e direção, nessa ordem.",
             'Test: take the plates from the chest. Step away without opening machines. Return and look for new plates.\n\nIf present, the line worked without you. Otherwise check power, copper and direction, in that order.')]),
        ("9. Divida sua produção", "9. Allocate your output", [
            ("Separe 24 chapas no inventário para a central. Leve outras 16 ao montador para fazer 8 engrenagens.\n\nNão precisa automatizar tudo agora. A coleta e a prensagem já continuam enquanto você monta peças.",
             'Reserve 24 plates in inventory for the hub. Carry another 16 to the assembler for 8 gears.\n\nYou need not automate everything now. Extraction and pressing keep running while you assemble parts.'),
            ("Quer automatizar as engrenagens? Ligue:\n\nprensa > esteira > montador > esteira > baú\n\nReserve primeiro as 24 chapas. Se mandar todas ao montador, elas virarão engrenagens e faltarão na entrega.",
             'Want automated gears? Connect:\n\npress > belt > assembler > belt > chest\n\nReserve the 24 plates first. Sending all plates to the assembler turns them into gears and leaves none for delivery.')]),
        ("10. Entregue à oficina", "10. Supply the workshop", [
            ("Abra a central. Ponha 24 chapas na entrada 1 e 8 engrenagens na entrada 2. Deixe as saídas livres.\n\nA central consome a encomenda. Retire 64 esteiras e 32 cabos. Você financiou a próxima ampliação.",
             'Open the hub. Put 24 plates in input 1 and 8 gears in input 2. Keep outputs clear.\n\nThe hub consumes the order. Collect 64 belts and 32 cables. You funded the next expansion.'),
            ("A central agora pede outros materiais. Retire as sobras antes de abastecer o novo pedido.\n\nO marco Reserva da oficina só reconhece itens no inventário. Quem recebe a entrega e paga é a central.",
             'The hub now asks for different materials. Clear leftovers before feeding the new order.\n\nThe Workshop stock milestone only recognizes inventory items. The hub receives delivery and pays you.')]),
        ("11. Encontre o gargalo", "11. Find the bottleneck", [
            ("Gargalo é o trecho que limita o restante da linha.\n\nEntrada vazia: observe quem deveria abastecer.\nSaída cheia: observe quem deveria receber.\nSem energia: confira combustível e rede.",
             'A bottleneck is the section limiting the rest of the line.\n\nEmpty input: check the supplier.\nFull output: check the receiver.\nNo power: check fuel and the power network.'),
            ("Abra o painel e observe itens/min e tempo ativo. Mude UMA coisa; espere uma nova leitura e compare.\n\nDesafio: aumente as chapas que chegam ao baú. Uma esteira rápida só ajuda se o transporte for o limite.",
             'Open the panel and watch items/min and active time. Change ONE thing; wait for a new reading and compare.\n\nChallenge: increase plates reaching the chest. A faster belt only helps if transport is the limit.')]),
        ("12. Cresça com a vila", "12. Grow with the village", [
            ("Continue no Quadro da fábrica: entregue 16 chapas. A próxima fase pede 32 engrenagens para abrir a oficina.\n\nUma extratora normal alimenta uma prensa. Duas linhas sustentam um montador. Confira a energia ao ampliar.",
             'Continue at the Factory board: deliver 16 plates. The next phase needs 32 gears to open the workshop.\n\nOne normal extractor feeds one press. Two lines feed an assembler. Check power when expanding.'),
            ("Quer ajuda na lavoura? Venda comida no armazém para obter esmeraldas. Faça um posto do lavrador; o guia da vila explica a contratação.\n\nVocê escolhe o próximo problema a resolver. A fábrica pode continuar trabalhando.",
             'Want farm help? Sell food at the pantry for emeralds. Make a farmhand post; the village guide explains hiring.\n\nChoose the next problem to solve. Your factory can keep working.')]),
    ]
    label('veio.first.title', 'Sua primeira oficina', 'Your first workshop')
    label('veio.first.index', 'Exercícios', 'Exercises')
    label('veio.first.continue', 'Próxima >', 'Next >')
    label('veio.first.check', 'Conferir o teste >', 'Check the test >')
    label('veio.first.recipes', 'Receitas da oficina', 'Workshop recipes')
    label('veio.first.reference', 'Manual completo: livro + papel. Este caderno: livro + pedregulho.', 'Full reference: book + paper. This workbook: book + cobblestone.')
    pages=[None,None]  # Two short index pages, no overflowing table of contents.
    targets=[]
    shorts=[('Missão','Mission'),('1. Materiais','1. Materials'),('2. Energia','2. Power'),
            ('3. Ciclo','3. Cycle'),('4. Engrenagens','4. Gears'),('5. Encomenda','5. Order'),
            ('6. Extração','6. Extraction'),('7. Transporte','7. Transport'),
            ('8. Estoque','8. Storage'),('9. Distribuição','9. Allocation'),
            ('10. Entrega','10. Delivery'),('11. Gargalos','11. Bottlenecks'),('12. Próximo passo','12. Next step')]
    for i,(pt_title,en_title,parts) in enumerate(lessons):
        targets.append(len(pages)+1)
        label(f'veio.first.lesson.{i}',pt_title,en_title)
        label(f'veio.first.short.{i}',*shorts[i])
        # Both languages use the same page targets. Hard-wrap and paginate prose
        # before adding navigation so the test instructions cannot be cut off.
        lines=[wrapped_lines('\n\n'.join(part[code] for part in parts)) for code in (0,1)]
        count=math.ceil(max(map(len,lines))/9)
        chunks=[]
        for language_lines in lines:
            size=math.ceil(len(language_lines)/count)
            chunks.append(['\n'.join(language_lines[j*size:(j+1)*size]).strip() for j in range(count)])
        for j in range(count):
            key=f'veio.first.lesson.{i}.{j}'
            label(key,chunks[0][j],chunks[1][j])
            extra=[{'translate':f'veio.first.lesson.{i}','bold':True}, {'text':'\n\n'},
                   {'translate':key}, {'text':'\n'},
                   {'translate':'veio.first.index','color':'dark_blue',
                    'click_event':{'action':'change_page','page':1 if i<7 else 2}},
                   {'text':' | '}, {'translate':'veio.first.continue','color':'dark_blue',
                    'click_event':{'action':'change_page','page':len(pages)+2}}]
            pages.append({'text':'','extra':extra})
    recipes_target=len(pages)+1
    for name in ('generator','cable','press','assembler','contract_hub','drill','belt'):
        pages.append({'text':'','extra':[
            {'translate':f'block.veio.{name}','bold':True},{'text':'\n\n'},
            {'translate':f'veio.guide.make.{name}'},{'text':'\n\n'},
            {'translate':'veio.first.index','color':'dark_blue','click_event':{'action':'change_page','page':1}}]})
    for index in (0,1):
        extra=[{'translate':'veio.first.title','bold':True},{'text':'\n'}]
        for i in range(index*7,min(index*7+7,len(lessons))):
            extra.extend([{'translate':f'veio.first.short.{i}','color':'dark_blue',
                           'click_event':{'action':'change_page','page':targets[i]}},{'text':'\n'}])
        extra.extend([{'translate':'veio.first.recipes','color':'dark_blue',
                       'click_event':{'action':'change_page','page':recipes_target}}, {'text':'\n'},
                      {'translate':'veio.first.index','color':'dark_blue',
                       'click_event':{'action':'change_page','page':2 if index==0 else 1}}])
        pages[index]={'text':'','extra':extra}
    # Recovery and reference instructions have their own short page.
    pages.append({'text':'','extra':[{'translate':'veio.first.reference'}, {'text':'\n\n'},
        {'translate':'veio.first.index','color':'dark_blue','click_event':{'action':'change_page','page':1}}]})
    # Last exercise leads to recipes. All links are checked by check_project.
    book={'minecraft:written_book_content':{'title':{'raw':'Veio - Oficina'}, 'author':'Veio',
          'generation':0,'resolved':True,'pages':[{'raw':p} for p in pages]},
          'minecraft:custom_name':{'translate':'veio.first.title','italic':False}}
    write(f'{data}/recipe/first_steps.json',{'type':'minecraft:crafting_shapeless','category':'misc',
          'ingredients':['minecraft:book','minecraft:cobblestone'],
          'result':{'id':'minecraft:written_book','count':1,'components':book}})
    write(f'{data}/loot_table/tutorial/first_steps.json',{'type':'minecraft:generic','pools':[{
        'rolls':1,'entries':[{'type':'minecraft:item','name':'minecraft:written_book',
        'functions':[{'function':'minecraft:set_components','components':book}]}]}]})
    print('Workshop workbook pages:',len(pages))
