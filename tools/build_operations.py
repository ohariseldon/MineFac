"""Common operation advice; does not change recipes, worker AI or simulation."""

def add_labels(langs):
    sectors = {
        "extraction": ("Setor: extração", "Sector: extraction"),
        "industry": ("Setor: indústria", "Sector: industry"),
        "farming": ("Setor: agricultura", "Sector: farming"),
        "food": ("Setor: alimentos", "Sector: food"),
        "power": ("Setor: energia", "Sector: power"),
        "logistics": ("Setor: logística", "Sector: logistics"),
        "projects": ("Setor: projetos", "Sector: projects"),
        "defense": ("Setor: defesa", "Sector: defense"),
        "workforce": ("Setor: coordenação de trabalhadores", "Sector: worker coordination"),
    }
    advice = {
        "idle": ("Confira os insumos e a configuração da estação.", "Check station inputs and setup."),
        "working": ("Operando. A saída precisa continuar com espaço.", "Operating. Keep room available at the outlet."),
        "input": ("Confira os materiais e as quantidades exigidas na receita.", "Check the materials and amounts required by the recipe."),
        "blocked": ("Retire produtos ou libere espaço na próxima etapa da linha.", "Collect products or clear room in the next stage of the line."),
        "ore": ("Coloque a extratora diretamente sobre uma jazida compatível.", "Place the extractor directly on a compatible ore vein."),
        "power": ("Confira geração, carga e conexões dos postes ou cabos.", "Check generation, charge and pole or cable connections."),
        "fuel": ("Abasteça a entrada com um combustível aceito pelo gerador.", "Supply the input with a fuel accepted by this generator."),
        "sun": ("Confira o horário e deixe o painel sob céu aberto.", "Check the time of day and expose the panel to open sky."),
        "charged": ("O ciclo ou a carga está completo; confira as saídas.", "The cycle or charge is complete; check the outputs."),
        "redstone": ("A redstone pausou a operação. Confira o controle da fábrica.", "Redstone paused operation. Check the factory control circuit."),
        "crops": ("Confira a área da fazenda e aguarde plantas maduras.", "Check the farm area and wait for mature crops."),
        "watch": ("A torre aguarda um alvo hostil no alcance.", "The tower is waiting for a hostile target in range."),
        "tier": ("A jazida exige uma extratora de nível superior.", "This vein requires a higher-tier extractor."),
        "damaged": ("Repare a máquina com um kit antes de retomar.", "Repair the machine with a kit before resuming."),
        "water": ("Confira a água exigida pela máquina e sua posição.", "Check the water required by the machine and its placement."),
        "wind": ("Confira céu aberto, altitude e condições do vento.", "Check open sky, altitude and wind conditions."),
        "seeds": ("Abasteça as sementes ou mudas aceitas e confira o solo.", "Supply accepted seeds or saplings and check the soil."),
        "animals": ("Confira os animais compatíveis na área do rancho.", "Check compatible animals in the ranch area."),
        "no_worker": ("Segure uma esmeralda e clique no posto perto de um aldeão adulto.", "Hold an emerald and click the post near an adult villager."),
        "absent": ("Traga o trabalhador de volta ou dispense com agachar + chave.", "Bring the worker back or dismiss with sneak + wrench."),
        "no_task": ("Plante e espere amadurecer na área 9x9 do posto.", "Plant crops in the post's 9x9 area and let them ripen."),
        "no_food": ("Coloque comida no slot do trabalhador.", "Put food in the worker slot."),
        "path": ("Abra caminho entre o posto e as plantas.", "Clear a walkable path between the post and the crops."),
        "walking": ("O trabalhador está indo até a tarefa.", "The worker is walking to the task."),
        "idle_work": ("Nada a atender: abasteça o estoque ou aguarde produtos.", "Nothing to serve: stock the post or wait for products."),
        "no_wage": ("Coloque esmeraldas na mesa para recontratar trabalhadores.", "Put emeralds in the desk to rehire workers."),
        "zone": ("Postos perto da borda trabalham nesta zona.", "Posts near the border work in this zone."),
        "rest": ("Noite: o trabalhador dorme e volta de manhã.", "Night: the worker sleeps and returns in the morning."),
        "site_plan": ("Escolha a planta (chave) e pague a obra no centro de controle.", "Pick the blueprint (wrench) and pay for the project at the control center."),
        "site_blocked": ("Tire o bloco marcado no painel de dentro da planta.", "Remove the block shown in the panel from inside the blueprint."),
        "site_building": ("O mestre de obras está construindo. Mantenha comida no canteiro.", "The master builder is building. Keep food in the site."),
        "site_done": ("Prédio de pé: o efeito da obra vale para a vila.", "Building standing: the project's effect applies to the village."),
        "housed": ("A família mora aqui. Veja a dica do painel para o que falta.", "A family lives here. The panel hint shows what is missing."),
        "no_permit": ("Sem alvará: o prefeito exige saneamento, energia ou comércio, ou acabaram os alvarás da fase.", "No permit: the mayor requires sanitation, power or commerce, or the phase ran out of permits."),
        "empty_home": ("Casa vazia: aldeões a até 10 blocos viram moradores.", "Empty house: villagers within 10 blocks move in."),
        "storing": ("A fila do capataz guarda e retira itens daqui.", "The foreman's queue stores and takes items here."),
        "unknown": ("Confira a ajuda da máquina para identificar o impedimento.", "Check machine help to identify what is preventing operation."),
    }
    for prefix, entries in (("sector", sectors), ("advice", advice)):
        for name, (pt, en) in entries.items():
            langs["pt_br"][f"veio.operation.{prefix}.{name}"] = pt
            langs["en_us"][f"veio.operation.{prefix}.{name}"] = en
