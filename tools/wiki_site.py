"""Site da Wiki do Veio (docs/wiki/index.html), gerado com os mesmos dados da wiki do jogo.

Chamado por build_wiki.build. Os artigos vêm de wiki_articles.py; o catálogo, a tabela de
máquinas e a árvore de produção saem das receitas; os ícones são as texturas do próprio Veio.
O arquivo segue o contrato das páginas publicadas (sem doctype/html/head/body: o publicador
embrulha), e também abre direto no navegador.
"""
import base64
import html
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "content/assets/veio"
OUT = ROOT / "docs/wiki/index.html"

TAB_NAMES = ["Indústria", "Energia", "Logística", "Mundo e vila"]
TAB_COLORS = ["#c8743a", "#d9a520", "#4f86c6", "#5a9e5a"]
MACHINE_ORDER = ["press", "assembler", "cutter", "integrator", "crusher", "alloy_furnace", "electronics", "fabricator",
                 "electric_furnace", "mill", "kitchen"]


def esc(text):
    return html.escape(str(text), quote=True)


class Site:
    def __init__(self, langs, known, recipes, tabs, unlocks, production, entries):
        self.pt = langs["pt_br"]
        self.known = known
        self.recipes = recipes
        self.tabs = tabs
        self.production = production
        self.entries = entries
        self.ids = {entry[0] for entry in entries}
        self.icons = {}
        groups = production.get("groups", {})
        self.machine_recipes = []
        for recipe in production["recipes"]:
            inputs = []
            for item, count in recipe["in"]:
                items = groups.get(item[1:], [item]) if item.startswith("@") else [item]
                inputs.append((items, count))
            self.machine_recipes.append({"machine": "veio:" + recipe["machine"], "in": inputs, "out": tuple(recipe["out"]),
                                         "ticks": recipe["ticks"], "power": recipe["power"], "alt": recipe.get("alt", False)})

    # ---------- nomes e ícones ----------
    def name(self, item):
        if item.startswith("#"):
            return {"#minecraft:planks": "Tábuas"}.get(item, item[1:])
        return self.known.get(item, {}).get("pt_br") or item.split(":")[1].replace("_", " ")

    def icon_uri(self, item):
        if item in self.icons:
            return self.icons[item]
        uri = None
        if item.startswith("veio:"):
            name = item[5:]
            path = ASSETS / f"textures/item/{name}.png"
            if not path.exists():
                model = ASSETS / f"models/block/{name}.json"
                if model.exists():
                    textures = json.loads(model.read_text(encoding="utf-8")).get("textures", {})
                    for key in ("front", "top", "particle", "body", "belt", "coil"):
                        value = textures.get(key, "")
                        if value.startswith("veio:"):
                            candidate = ASSETS / f"textures/{value[5:]}.png"
                            if candidate.exists():
                                path = candidate
                                break
            if path.exists():
                uri = "data:image/png;base64," + base64.b64encode(path.read_bytes()).decode()
        self.icons[item] = uri
        return uri

    def icon(self, item, size=32, link=True):
        """Ícone do item (textura do Veio) ou um selo com as iniciais (itens do Minecraft)."""
        title = esc(self.name(item))
        uri = self.icon_uri(item)
        if uri:
            inner = f'<img src="{uri}" alt="{title}" width="{size}" height="{size}">'
        else:
            letters = "".join(word[0] for word in self.name(item).split()[:2]).upper()
            inner = f'<span class="mc" style="width:{size}px;height:{size}px">{esc(letters)}</span>'
        anchor = f"item-{item.split(':')[1]}"
        if link and item in self.ids:
            return f'<a class="ico" href="#{anchor}" title="{title}">{inner}</a>'
        return f'<span class="ico" title="{title}">{inner}</span>'

    def slot(self, item=None, count=1):
        if not item:
            return '<span class="slot"></span>'
        badge = f'<b class="count">{count}</b>' if count > 1 else ""
        return f'<span class="slot">{self.icon(item, 28)}{badge}</span>'

    def text(self, key):
        return self.pt.get(key, "").replace("\n", " ")

    # ---------- seções geradas ----------
    def recipe_grid(self, item, craft, makes):
        cells = "".join(self.slot(craft[index] if index < len(craft) else None) for index in range(9))
        return (f'<div class="craft"><div class="grid9">{cells}</div><span class="arrow">→</span>'
                f'{self.slot(item, makes)}</div>')

    def process_row(self, recipe, machine=True):
        parts = []
        if machine:
            parts.append(f'<span class="machine">{self.icon(recipe["machine"], 24)}<span>{esc(self.name(recipe["machine"]))}</span></span>')
        for items, count in recipe["in"]:
            parts.append(self.slot(items[0], count))
        parts.append('<span class="arrow">→</span>')
        parts.append(self.slot(recipe["out"][0], recipe["out"][1]))
        seconds = f"{recipe['ticks'] / 20:.1f}".replace(".", ",")
        rate = 60 * 20 / recipe["ticks"] * recipe["out"][1]
        meta = f'{seconds} s · {recipe["power"]} FE/t · {rate:.0f}/min'.replace(".", ",")
        alt = ' <em class="alt">alternativa</em>' if recipe["alt"] else ""
        return f'<div class="proc">{"".join(parts)}<span class="meta">{meta}{alt}</span></div>'

    def catalog(self):
        out = []
        for tab_index, (name, _, _, _) in enumerate(self.tabs):
            cards = []
            for item, tab, craft, makes, text, unlock in self.entries:
                if tab != tab_index:
                    continue
                short = item.split(":")[1]
                description = self.text(text) if text else "Item do Minecraft que o Veio fabrica ou usa."
                body = [f'<p>{esc(description)}</p>']
                if unlock:
                    body.append(f'<p class="unlock">Liberado por: <b>{esc(self.pt.get(unlock, unlock))}</b></p>')
                if craft:
                    body.append('<h5>Fabricação (bancada)</h5>' + self.recipe_grid(item, craft, makes))
                made = [recipe for recipe in self.machine_recipes if recipe["out"][0] == item]
                if made:
                    body.append('<h5>Produzido em</h5>' + "".join(self.process_row(recipe) for recipe in made[:6]))
                processes = [recipe for recipe in self.machine_recipes if recipe["machine"] == item]
                if processes:
                    body.append(f'<h5>Processa ({len(processes)})</h5>' + "".join(self.process_row(recipe, False) for recipe in processes[:12]))
                used = []
                for other, _, other_craft, _, _, _ in self.entries:
                    if item in other_craft and other not in used:
                        used.append(other)
                for recipe in self.machine_recipes:
                    if any(item in items for items, _ in recipe["in"]) and recipe["out"][0] not in used:
                        used.append(recipe["out"][0])
                if used:
                    body.append('<h5>Usado em</h5><div class="used">' + "".join(self.icon(other, 28) for other in used) + "</div>")
                search = esc(f"{self.name(item)} {short}".lower())
                cards.append(f'<article class="card" id="item-{short}" data-search="{search}">'
                             f'<header>{self.icon(item, 40, link=False)}<div><h4>{esc(self.name(item))}</h4>'
                             f'<span class="tab" style="--tab:{TAB_COLORS[tab_index]}">{TAB_NAMES[tab_index]}</span></div></header>'
                             f'{"".join(body)}</article>')
            out.append(f'<section class="tabgroup" id="cat-{name}"><h3>{TAB_NAMES[tab_index]} '
                       f'<small>{len(cards)} itens</small></h3><div class="cards">{"".join(cards)}</div></section>')
        return "".join(out)

    def machines_table(self):
        rows = []
        order = {machine: index for index, machine in enumerate(MACHINE_ORDER)}
        for recipe in sorted(self.machine_recipes, key=lambda r: (order.get(r["machine"][5:], 99), r["out"][0])):
            inputs = " + ".join(f'{count}× {esc(self.name(items[0]))}' + (" (ou similar)" if len(items) > 1 else "")
                                for items, count in recipe["in"])
            seconds = f"{recipe['ticks'] / 20:.1f}".replace(".", ",")
            rate = f'{60 * 20 / recipe["ticks"] * recipe["out"][1]:.0f}'
            alt = " <em class='alt'>alt.</em>" if recipe["alt"] else ""
            rows.append(f'<tr><td>{self.icon(recipe["machine"], 20)} {esc(self.name(recipe["machine"]))}</td><td>{inputs}</td>'
                        f'<td>{recipe["out"][1]}× {esc(self.name(recipe["out"][0]))}{alt}</td><td class="num">{seconds} s</td>'
                        f'<td class="num">{recipe["power"]}</td><td class="num">{rate}</td></tr>')
        return ('<div class="scroll"><table class="machines"><thead><tr><th>Máquina</th><th>Entradas</th><th>Saída</th>'
                '<th>Tempo</th><th>FE/t</th><th>Saída/min</th></tr></thead><tbody>' + "".join(rows) + "</tbody></table></div>")

    # ---------- diagramas ----------
    def diagram(self, kind):
        return getattr(self, f"svg_{kind}", lambda: "")()

    @staticmethod
    def box(x, y, w, h, label, cls="node", sub=None):
        lines = [f'<rect class="{cls}" x="{x}" y="{y}" width="{w}" height="{h}" rx="6"/>']
        ty = y + h / 2 + (-3 if sub else 4)
        lines.append(f'<text class="lbl" x="{x + w / 2}" y="{ty}" text-anchor="middle">{esc(label)}</text>')
        if sub:
            lines.append(f'<text class="sub" x="{x + w / 2}" y="{ty + 13}" text-anchor="middle">{esc(sub)}</text>')
        return "".join(lines)

    @staticmethod
    def arrow(x1, y1, x2, y2, cls="edge", label=None, curve=0):
        mx, my = (x1 + x2) / 2, (y1 + y2) / 2 + curve
        path = f'<path class="{cls}" d="M{x1},{y1} Q{mx},{my} {x2},{y2}" marker-end="url(#tip)"/>'
        if label:
            path += f'<text class="elbl" x="{mx}" y="{my - 4}" text-anchor="middle">{esc(label)}</text>'
        return path

    def svg(self, width, height, body, label):
        return (f'<figure class="diagram"><div class="scroll"><svg viewBox="0 0 {width} {height}" '
                f'style="width:100%;max-width:{width}px;min-width:{min(width, 620)}px" '
                f'role="img" aria-label="{esc(label)}"><defs><marker id="tip" viewBox="0 0 10 10" refX="9" refY="5" '
                f'markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" class="tipfill"/></marker></defs>'
                f'{body}</svg></div><figcaption>{esc(label)}</figcaption></figure>')

    def svg_start(self):
        steps = ["Gerador", "Prensa ligada", "4 chapas", "Engrenagem", "Extratora", "Esteira → prensa", "Esteira → baú", "Encomenda"]
        body = []
        for index, step in enumerate(steps):
            col, row = index % 4, index // 4
            x, y = 20 + col * 175, 20 + row * 90
            body.append(self.box(x, y, 140, 46, step, "node step", f"etapa {index + 1}"))
            if col < 3:
                body.append(self.arrow(x + 140, y + 23, x + 172, y + 23))
        body.append(self.arrow(20 + 3 * 175 + 70, 66, 20 + 70, 110, curve=40))
        return self.svg(720, 160, "".join(body), "As 8 etapas do tutorial, da energia à primeira encomenda.")

    def svg_power(self):
        body = [self.box(20, 30, 150, 44, "Gerador", "node src", "40 FE/t · carvão"),
                self.box(20, 90, 150, 44, "Solar / vento / água", "node src", "sem poluir"),
                self.box(20, 150, 150, 44, "Geotérmico / vapor", "node src", "80 a 120 FE/t"),
                self.box(250, 60, 150, 44, "Poste de madeira", "node mid", "8 blocos · 120 FE/t"),
                self.box(250, 130, 150, 44, "Cabo encostado", "node mid", "ligação curta"),
                self.box(480, 30, 150, 44, "Máquinas", "node dst", "buffer de 2.000 FE"),
                self.box(480, 100, 150, 44, "Acumulador", "node dst", "30.000 FE"),
                self.box(480, 170, 150, 44, "Torre Tesla", "node dst", "2.400 FE/disparo")]
        for y in (52, 112, 172):
            body.append(self.arrow(170, y, 248, 82 if y < 140 else 152))
        body += [self.arrow(400, 82, 478, 52), self.arrow(400, 82, 478, 122), self.arrow(400, 152, 478, 192),
                 self.arrow(555, 144, 555, 170, label=None)]
        return self.svg(660, 230, "".join(body), "De onde vem e para onde vai a energia.")

    def svg_io(self):
        body = ['<text class="h" x="20" y="22">Certo</text>',
                '<rect class="belt" x="20" y="70" width="110" height="26" rx="3"/>',
                '<text class="sub" x="75" y="62" text-anchor="middle">esteira entrando</text>',
                self.arrow(40, 83, 128, 83, "edge flow"),
                '<rect class="machine" x="132" y="40" width="86" height="86" rx="8"/>',
                '<rect class="good" x="132" y="44" width="5" height="78"/>', '<rect class="good" x="136" y="40" width="78" height="5"/>',
                '<rect class="good" x="136" y="121" width="78" height="5"/>', '<rect class="bad" x="213" y="62" width="6" height="42"/>',
                '<text class="lbl" x="175" y="87" text-anchor="middle">Prensa</text>',
                '<rect class="belt" x="222" y="70" width="110" height="26" rx="3"/>',
                self.arrow(224, 83, 330, 83, "edge flow"), '<text class="sub" x="277" y="62" text-anchor="middle">produção saindo</text>',
                '<text class="good-t" x="175" y="150" text-anchor="middle">verde: recebe</text>',
                '<text class="bad-t" x="275" y="150" text-anchor="middle">vermelho: solta</text>',
                '<text class="h" x="380" y="22">Não aceito</text>',
                '<rect class="machine" x="380" y="40" width="86" height="86" rx="8"/>',
                '<rect class="bad" x="461" y="62" width="6" height="42"/>',
                '<text class="lbl" x="423" y="87" text-anchor="middle">Prensa</text>',
                '<rect class="belt" x="470" y="70" width="110" height="26" rx="3"/>',
                self.arrow(578, 83, 472, 83, "edge flow bad-e"),
                '<text class="sub" x="525" y="62" text-anchor="middle">entrando pela saída</text>',
                '<text class="bad-t" x="480" y="150" text-anchor="middle">a máquina recusa; a esteira não aceita pela frente</text>']
        return self.svg(640, 170, "".join(body), "Entrada pelas faixas verdes, saída pela moldura vermelha.")

    def svg_purity(self):
        classes = [("Impura", "1–2", 0.5), ("Normal", "3–6", 1), ("Pura", "7–15", 2), ("Industrial", "16+", 4)]
        body = []
        base, scale = 190, 38
        for index, (label, blocks, speed) in enumerate(classes):
            x = 80 + index * 130
            h = speed * scale
            body.append(f'<rect class="bar" x="{x}" y="{base - h}" width="70" height="{h}" rx="3"/>')
            body.append(f'<text class="lbl" x="{x + 35}" y="{base - h - 8}" text-anchor="middle">{speed:g}×</text>'.replace(".", ","))
            body.append(f'<text class="lbl" x="{x + 35}" y="{base + 18}" text-anchor="middle">{label}</text>')
            body.append(f'<text class="sub" x="{x + 35}" y="{base + 32}" text-anchor="middle">{blocks} blocos</text>')
        for speed in (1, 2, 4):
            y = base - speed * scale
            body.append(f'<line class="grid" x1="60" y1="{y}" x2="600" y2="{y}"/><text class="sub" x="52" y="{y + 4}" text-anchor="end">{speed}×</text>')
        body.append(f'<line class="axis" x1="60" y1="{base}" x2="600" y2="{base}"/>')
        return self.svg(620, 240, "".join(body), "Velocidade da extratora pela pureza do veio conectado.")

    def svg_chains(self):
        machines = {"press", "assembler", "cutter", "integrator", "alloy_furnace", "electronics", "fabricator", "crusher"}
        edges, outputs = [], {}
        for recipe in self.machine_recipes:
            machine = recipe["machine"][5:]
            if machine not in machines or recipe["alt"]:
                continue
            out = recipe["out"][0]
            if machine == "crusher" and out != "veio:silicon":
                continue
            outputs.setdefault(out, machine)
            for items, _ in recipe["in"]:
                edges.append((items[0], out))
        nodes = sorted({a for a, _ in edges} | {b for _, b in edges})
        depth = {}

        def level(node, seen=()):
            if node in depth:
                return depth[node]
            parents = [a for a, b in edges if b == node and a not in seen]
            depth[node] = 0 if not parents else 1 + max(level(parent, seen + (node,)) for parent in parents)
            return depth[node]
        for node in nodes:
            level(node)
        columns = {}
        for node in nodes:
            columns.setdefault(depth[node], []).append(node)
        width, height = 170, 46
        positions, body = {}, []
        tallest = max(len(column) for column in columns.values())
        for col, items in sorted(columns.items()):
            items.sort(key=lambda node: (outputs.get(node, ""), self.name(node)))
            offset = (tallest - len(items)) * (height + 18) / 2
            for row, node in enumerate(items):
                positions[node] = (20 + col * (width + 60), 20 + offset + row * (height + 18))
        for a, b in edges:
            (x1, y1), (x2, y2) = positions[a], positions[b]
            body.append(self.arrow(x1 + width, y1 + height / 2, x2 - 2, y2 + height / 2, "edge chain"))
        for node, (x, y) in positions.items():
            machine = outputs.get(node)
            sub = self.name("veio:" + machine).lower() if machine else "matéria-prima"
            body.append(self.box(x, y, width, height, self.name(node), "node raw" if not machine else "node", sub))
        total_w = 40 + (max(columns) + 1) * (width + 60) - 60
        total_h = 40 + tallest * (height + 18)
        return self.svg(total_w, total_h, "".join(body), "Árvore de produção industrial (gerada das receitas). Embaixo de cada peça, a máquina que a faz.")

    def svg_phases(self):
        phases = [("Cobre", "1 lingote de cobre"), ("Oficina", "32 engrenagens"), ("Aço", "16 chapas de aço"),
                  ("Eletrônica", "16 circuitos"), ("Profundidade", "1 módulo instalado")]
        body = ['<line class="axis" x1="40" y1="70" x2="880" y2="70"/>']
        for index, (name, need) in enumerate(phases):
            x = 60 + index * 200
            body.append(f'<circle class="dot" cx="{x}" cy="70" r="11"/><text class="lbl" x="{x}" y="74" text-anchor="middle">{index + 1}</text>')
            body.append(f'<text class="h" x="{x}" y="40" text-anchor="middle">{esc(name)}</text>')
            body.append(f'<text class="sub" x="{x}" y="104" text-anchor="middle">{esc(need)}</text>')
        return self.svg(900, 130, "".join(body), "As cinco fases da fábrica e a pilha que cada uma pede.")

    def svg_workforce(self):
        body = [self.box(280, 20, 180, 46, "Centro de controle", "node top", "sede da companhia"),
                self.box(60, 110, 160, 46, "Mesa do capataz", "node", "alimenta e recontrata"),
                self.box(290, 110, 160, 46, "Quartel", "node", "defesa da vila"),
                self.box(520, 110, 160, 46, "Zonas", "node", "lavoura e cercado"),
                self.box(20, 200, 140, 46, "Carregadores", "node leaf", "abastecem máquinas"),
                self.box(170, 200, 120, 46, "Lavradores", "node leaf", "9×9"),
                self.box(300, 200, 140, 46, "Milícia / guarda", "node leaf", "arqueiro, mago"),
                self.box(520, 200, 160, 46, "Pastores", "node leaf", "animais")]
        body += [self.arrow(370, 66, 140, 108), self.arrow(370, 66, 370, 108), self.arrow(370, 66, 600, 108),
                 self.arrow(140, 156, 90, 198), self.arrow(140, 156, 230, 198), self.arrow(370, 156, 370, 198),
                 self.arrow(600, 156, 600, 198)]
        return self.svg(700, 260, "".join(body), "Organograma da vila operária.")

    def svg_society(self):
        body = [self.box(270, 15, 200, 50, "Prefeitura", "node top", "prefeito · fases · indicadores"),
                self.box(20, 110, 160, 50, "Impostos", "node", "4% a 16% na fonte"),
                self.box(200, 110, 160, 50, "Serviços", "node", "poço, assistência"),
                self.box(380, 110, 160, 50, "Barreiras", "node", "alvará das casas"),
                self.box(560, 110, 160, 50, "Agenda", "node", "feriados e obras"),
                self.box(110, 210, 160, 50, "Lojas", "node leaf", "comerciantes"),
                self.box(290, 210, 160, 50, "Famílias", "node leaf", "casas, salário, IDH"),
                self.box(470, 210, 160, 50, "Bairros", "node leaf", "lazer, saneamento")]
        body += [self.arrow(370, 65, 100, 108), self.arrow(370, 65, 280, 108), self.arrow(370, 65, 460, 108),
                 self.arrow(370, 65, 640, 108), self.arrow(100, 160, 190, 208), self.arrow(280, 160, 370, 208),
                 self.arrow(460, 160, 550, 208), self.arrow(640, 160, 560, 208)]
        return self.svg(740, 280, "".join(body), "Organograma do Estado da vila.")

    def svg_money(self):
        body = [self.box(285, 20, 170, 50, "Companhia", "node top", "você · cofre"),
                self.box(560, 150, 170, 50, "Famílias", "node", "casas"),
                self.box(285, 280, 170, 50, "Lojas", "node", "comerciantes"),
                self.box(10, 150, 170, 50, "Prefeitura", "node gov", "conta pública"),
                self.arrow(455, 45, 640, 148, label="salários", curve=-30),
                self.arrow(645, 200, 455, 305, label="varejo", curve=30),
                self.arrow(370, 278, 370, 72),
                '<text class="elbl" x="378" y="180">atacado 75%</text>',
                self.arrow(285, 310, 100, 202, "edge tax", label="imposto na fonte", curve=30),
                self.arrow(95, 150, 283, 50, "edge tax", label="obras e festas", curve=-30),
                self.arrow(180, 182, 558, 182, "edge tax", curve=-150),
                '<text class="elbl" x="252" y="118" text-anchor="middle">assistência, salário do secretário</text>']
        return self.svg(740, 345, "".join(body), "O dinheiro só circula: nada aparece, nada some. Esmeraldas entram e saem só pelo cofre, pelas suas compras e pelo comércio exterior.")

    def svg_trade(self):
        body = [self.box(20, 80, 150, 50, "Fábrica", "node", "produtos"),
                self.box(230, 30, 160, 50, "Porto de carga", "node mid", "navio a cada 2 min"),
                self.box(230, 130, 160, 50, "Estação de carga", "node mid", "trem a cada 1 min"),
                self.box(450, 80, 160, 50, "Bolsa mundial", "node top", "valor × complexidade"),
                self.box(670, 30, 160, 50, "Companhia", "node dst", "moedas da venda"),
                self.box(670, 130, 160, 50, "Reputação", "node dst", "abre novos mercados"),
                self.arrow(170, 100, 228, 58), self.arrow(170, 110, 228, 152),
                self.arrow(390, 55, 448, 100), self.arrow(390, 155, 448, 110),
                self.arrow(610, 100, 668, 58, label="cotação − frete"), self.arrow(610, 112, 668, 152, label="contratos +50%"),
                self.arrow(450, 115, 170, 118, "edge tax", label="importação de comida", curve=60)]
        return self.svg(850, 230, "".join(body), "Comércio exterior: conexões físicas e a bolsa mundial.")

    def svg_calendar(self):
        import math
        seasons = [("Primavera", "#7fb069"), ("Verão", "#e8b33a"), ("Outono", "#c8743a"), ("Inverno", "#6fa3d8")]
        holidays = {0: "Fundação", 3: "Flores", 8: "São João", 15: "Colheita", 20: "Luzes", 23: "Virada"}
        cx, cy, r = 220, 210, 150
        body = []
        for day in range(24):
            a0 = (day / 24) * 2 * math.pi - math.pi / 2
            a1 = ((day + 1) / 24) * 2 * math.pi - math.pi / 2
            x0, y0, x1, y1 = cx + r * math.cos(a0), cy + r * math.sin(a0), cx + r * math.cos(a1), cy + r * math.sin(a1)
            xi0, yi0, xi1, yi1 = cx + 95 * math.cos(a0), cy + 95 * math.sin(a0), cx + 95 * math.cos(a1), cy + 95 * math.sin(a1)
            color = seasons[day // 6][1]
            cls = "day hol" if day in holidays else ("day sun" if day % 6 == 5 else "day")
            body.append(f'<path class="{cls}" style="--season:{color}" d="M{x0:.1f},{y0:.1f} A{r},{r} 0 0 1 {x1:.1f},{y1:.1f} '
                        f'L{xi1:.1f},{yi1:.1f} A95,95 0 0 0 {xi0:.1f},{yi0:.1f} z"/>')
            am = (day + 0.5) / 24 * 2 * math.pi - math.pi / 2
            body.append(f'<text class="sub" x="{cx + 122 * math.cos(am):.1f}" y="{cy + 122 * math.sin(am) + 4:.1f}" text-anchor="middle">{day % 6 + 1}</text>')
            if day in holidays:
                lx, ly = cx + 185 * math.cos(am), cy + 185 * math.sin(am)
                anchor = "start" if math.cos(am) > 0.04 else ("end" if math.cos(am) < -0.04 else "middle")
                body.append(f'<text class="lbl" x="{lx:.1f}" y="{ly + 4:.1f}" text-anchor="{anchor}">{holidays[day]}</text>')
        for index, (name, color) in enumerate(seasons):
            body.append(f'<rect x="470" y="{110 + index * 26}" width="14" height="14" rx="3" style="fill:{color}"/>'
                        f'<text class="lbl" x="492" y="{122 + index * 26}">{name}</text>')
        body += ['<rect class="day hol" x="470" y="230" width="14" height="14" rx="3" style="--season:#999"/><text class="lbl" x="492" y="242">feriado (a vila para)</text>',
                 '<rect class="day sun" x="470" y="256" width="14" height="14" rx="3" style="--season:#999"/><text class="lbl" x="492" y="268">domingo (evento menor)</text>',
                 f'<text class="h" x="{cx}" y="{cy - 4}" text-anchor="middle">1 ano</text>',
                 f'<text class="sub" x="{cx}" y="{cy + 14}" text-anchor="middle">24 dias</text>']
        return self.svg(700, 420, "".join(body), "O ano da vila: 4 estações de 6 dias, domingos e os 6 feriados.")

    def svg_pollution(self):
        body = []
        base, top = 200, 30
        def y(value):
            return base - (base - top) * value / 1600
        points = " ".join(f"{40 + t * 6:.0f},{y(min(1600, 20 * t + 0.08 * t * t)):.1f}" for t in range(0, 101, 4))
        body.append(f'<polyline class="curve" points="{points}"/>')
        for value, label, cls in ((400, "aviso no chat", "warn"), (1200, "ondas à noite", "bad-l")):
            body.append(f'<line class="{cls}" x1="40" y1="{y(value):.1f}" x2="660" y2="{y(value):.1f}"/>'
                        f'<text class="lbl" x="664" y="{y(value) + 4:.1f}">{value} · {label}</text>')
        body.append(f'<line class="axis" x1="40" y1="{base}" x2="660" y2="{base}"/><line class="axis" x1="40" y1="{top}" x2="40" y2="{base}"/>')
        body.append(f'<text class="sub" x="350" y="{base + 20}" text-anchor="middle">tempo com a fábrica crescendo, sem defesa nem eficiência</text>')
        body.append(f'<text class="sub" x="34" y="{top + 4}" text-anchor="end">1600</text><text class="sub" x="34" y="{base + 4}" text-anchor="end">0</text>')
        return self.svg(820, 230, "".join(body), "Poluição do chunk mais poluído e os dois limites (ilustrativo).")

    # ---------- página ----------
    def page(self):
        from wiki_articles import ARTICLES
        nav, sections = [], []
        groups = [("Começo", ["start"]), ("Fábrica", ["power", "io", "extraction", "chains", "modules", "progression", "megafactory"]),
                  ("Logística", ["transport"]), ("Vila e sociedade", ["workforce", "society", "economy", "trade", "agenda"]),
                  ("Mundo", ["defense", "climate"]), ("Ajuda", ["help"])]
        by_id = {article["id"]: article for article in ARTICLES}
        for group, ids in groups:
            links = "".join(f'<a href="#{i}">{esc(by_id[i]["title"][0])}</a>' for i in ids)
            nav.append(f'<div class="navgroup"><span>{group}</span>{links}</div>')
        nav.append('<div class="navgroup"><span>Referência</span><a href="#machines">Máquinas e receitas</a>'
                   + "".join(f'<a href="#cat-{name}">{TAB_NAMES[index]}</a>' for index, (name, *_rest) in enumerate(self.tabs)) + "</div>")
        for group, ids in groups:
            for article_id in ids:
                article = by_id[article_id]
                figure = self.diagram(article["diagram"]) if article.get("diagram") else ""
                sections.append(f'<section class="article" id="{article_id}"><p class="eyebrow">{group}</p>'
                                f'<h2>{self.icon(article["icon"], 32, link=False)}{esc(article["title"][0])}</h2>'
                                f'{figure}{article["web"].replace("<table>", '<div class="scroll"><table>').replace("</table>", "</table></div>")}</section>')
        machines = (f'<section class="article" id="machines"><p class="eyebrow">Referência</p><h2>Máquinas e receitas</h2>'
                    f'<p>Todas as receitas de máquina do jogo, com tempo por ciclo, potência e produção por minuto de uma máquina sem módulo.</p>'
                    f'{self.machines_table()}</section>')
        catalog = (f'<section class="article" id="catalog"><p class="eyebrow">Referência</p><h2>Catálogo de itens</h2>'
                   f'<p>Cada bloco e item do Veio, com a receita de bancada, a máquina que produz, o que processa e onde é usado. '
                   f'Clique num ícone para ir à ficha dele. Itens do Minecraft aparecem com as iniciais.</p>{self.catalog()}</section>')
        count = len(self.entries)
        return TEMPLATE.replace("{{NAV}}", "".join(nav)).replace("{{BODY}}", "".join(sections) + machines + catalog) \
            .replace("{{COUNT}}", str(count)).replace("{{MACHINES}}", str(len(self.machine_recipes)))


def build(langs, known, recipes, tabs, unlocks, production, entries):
    site = Site(langs, known, recipes, tabs, unlocks, production, entries)
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(site.page(), encoding="utf-8")


TEMPLATE = """<title>Wiki do Veio</title>
<meta name="claude:viewport-fit-sides" content="cover">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Barlow+Condensed:wght@500;600;700&family=IBM+Plex+Sans:ital,wght@0,400;0,500;0,600;1,400&family=IBM+Plex+Mono:wght@500&display=swap">
<style>
/* Layout: wiki de consulta, menu lateral fixo + coluna de leitura; fichas do catálogo em grade. */
:root {
  --bg: #eef0f3; --panel: #ffffff; --ink: #1d2128; --soft: #5d6471; --line: #d5d9e0;
  --copper: #b4602c; --copper-soft: #f3e2d6; --slot: #8b8b8b; --slot-dark: #373737; --slot-light: #ffffff;
  --good: #2f8f46; --bad: #c8342a; --warn: #c58a12; --node: #f6f7f9; --raw: #efe6dc;
  --display: "Barlow Condensed", "Arial Narrow", sans-serif; --body: "IBM Plex Sans", "Segoe UI", system-ui, sans-serif;
  --mono: "IBM Plex Mono", ui-monospace, Consolas, monospace;
}
@media (prefers-color-scheme: dark) { :root:not([data-theme="light"]) {
  --bg: #14171c; --panel: #1c2027; --ink: #e7e9ee; --soft: #9aa2b1; --line: #2e343e;
  --copper: #e08a52; --copper-soft: #3a2619; --node: #232832; --raw: #2f2721; color-scheme: dark } }
:root[data-theme="dark"] {
  --bg: #14171c; --panel: #1c2027; --ink: #e7e9ee; --soft: #9aa2b1; --line: #2e343e;
  --copper: #e08a52; --copper-soft: #3a2619; --node: #232832; --raw: #2f2721; color-scheme: dark }
* { box-sizing: border-box; }
body { background: var(--bg); color: var(--ink); font: 15px/1.6 var(--body); }
a { color: var(--copper); }
.wrap { display: grid; grid-template-columns: 250px minmax(0, 1fr); gap: 32px; max-width: 1280px; margin: 0 auto;
  padding-left: max(16px, env(safe-area-inset-left, 0px)); padding-right: max(16px, env(safe-area-inset-right, 0px)); padding-block: 24px 64px; }
nav { position: sticky; top: env(safe-area-inset-top, 0px); align-self: start; max-height: 100vh; overflow-y: auto; padding-block: 8px 24px; }
.brand { font: 700 34px/1 var(--display); letter-spacing: .02em; display: flex; align-items: center; gap: 10px; }
.brand i { width: 14px; height: 14px; background: var(--copper); transform: rotate(45deg); display: inline-block; }
.tag { color: var(--soft); font-size: 13px; margin: 6px 0 16px; }
#q { width: 100%; padding: 9px 12px; border: 1px solid var(--line); border-radius: 8px; background: var(--panel); color: var(--ink); font: inherit; }
#q:focus { outline: 2px solid var(--copper); outline-offset: 1px; }
.navgroup { display: flex; flex-direction: column; margin-top: 18px; }
.navgroup span { font: 600 12px/1 var(--body); text-transform: uppercase; letter-spacing: .08em; color: var(--soft); margin-bottom: 6px; }
.navgroup a { color: var(--ink); text-decoration: none; padding: 3px 8px; border-left: 2px solid transparent; }
.navgroup a:hover, .navgroup a:focus-visible { border-color: var(--copper); color: var(--copper); }
main { min-width: 0; }
.hero { background: var(--panel); border: 1px solid var(--line); border-radius: 12px; padding: 24px 28px; display: grid; gap: 10px; }
.hero h1 { font: 700 44px/1.05 var(--display); margin: 0; text-wrap: balance; }
.hero p { margin: 0; max-width: 68ch; color: var(--soft); }
.stats { display: flex; flex-wrap: wrap; gap: 8px 22px; font: 500 13px var(--mono); color: var(--soft); }
.stats b { color: var(--copper); font-weight: 500; }
.article { padding-block: 34px 8px; border-bottom: 1px solid var(--line); }
.article > p, .article > ul, .article > ol { max-width: 72ch; }
.eyebrow { font: 600 12px/1 var(--body); text-transform: uppercase; letter-spacing: .1em; color: var(--copper); margin: 0 0 6px; }
h2 { font: 700 34px/1.1 var(--display); margin: 0 0 14px; display: flex; align-items: center; gap: 12px; text-wrap: balance; }
h3 { font: 600 22px/1.2 var(--display); margin: 24px 0 8px; letter-spacing: .01em; }
h3 small { font: 500 13px var(--mono); color: var(--soft); margin-left: 8px; }
table { border-collapse: collapse; width: 100%; margin: 12px 0; font-size: 14px; background: var(--panel); }
th, td { border-bottom: 1px solid var(--line); padding: 7px 10px; text-align: left; vertical-align: top; }
th { font: 600 12px var(--body); text-transform: uppercase; letter-spacing: .06em; color: var(--soft); }
td.num { font-variant-numeric: tabular-nums; white-space: nowrap; font-family: var(--mono); }
.scroll { overflow-x: auto; }
code, kbd { font: 500 13px var(--mono); background: var(--copper-soft); padding: 1px 6px; border-radius: 4px; }
kbd { border: 1px solid var(--line); }
.tip { background: var(--copper-soft); border-left: 3px solid var(--copper); padding: 10px 14px; border-radius: 0 8px 8px 0; }
.chip { font: 600 12px var(--mono); padding: 1px 6px; border-radius: 4px; color: #fff; }
.chip.green { background: var(--good); } .chip.red { background: var(--bad); }
em.alt { font-size: 11px; color: var(--warn); font-style: normal; text-transform: uppercase; letter-spacing: .06em; }
/* Diagramas */
.diagram { margin: 16px 0 18px; background: var(--panel); border: 1px solid var(--line); border-radius: 12px; padding: 14px; }
.diagram svg { display: block; max-width: none; height: auto; font-family: var(--body); }
.diagram figcaption { font-size: 13px; color: var(--soft); margin-top: 8px; }
.node { fill: var(--node); stroke: var(--line); stroke-width: 1.5; }
.node.top { fill: var(--copper-soft); stroke: var(--copper); }
.node.raw { fill: var(--raw); }
.node.src { stroke: var(--warn); } .node.dst { stroke: var(--good); } .node.gov { stroke: var(--soft); stroke-dasharray: 4 3; }
.node.step { stroke: var(--copper); }
.lbl { fill: var(--ink); font-size: 13px; font-weight: 600; }
.sub { fill: var(--soft); font-size: 11px; }
.h { fill: var(--ink); font: 700 18px var(--display); }
.elbl { fill: var(--copper); font-size: 11px; font-weight: 600; }
.edge { fill: none; stroke: var(--soft); stroke-width: 1.6; }
.edge.chain { stroke: var(--copper); stroke-opacity: .55; }
.edge.tax { stroke-dasharray: 5 4; }
.edge.flow { stroke: var(--ink); stroke-width: 2; }
.edge.bad-e { stroke: var(--bad); }
.tipfill { fill: var(--soft); }
.machine { fill: var(--node); stroke: var(--soft); stroke-width: 1.5; }
.belt { fill: #3a3a3a; } .good { fill: var(--good); } .bad { fill: var(--bad); }
.good-t { fill: var(--good); font-size: 12px; font-weight: 600; } .bad-t { fill: var(--bad); font-size: 12px; font-weight: 600; }
.bar { fill: var(--copper); }
.grid { stroke: var(--line); stroke-dasharray: 3 3; } .axis { stroke: var(--soft); stroke-width: 1.5; }
.dot { fill: var(--copper-soft); stroke: var(--copper); stroke-width: 2; }
.curve { fill: none; stroke: var(--copper); stroke-width: 2.5; }
.warn { stroke: var(--warn); stroke-width: 1.5; stroke-dasharray: 6 4; } .bad-l { stroke: var(--bad); stroke-width: 1.5; stroke-dasharray: 6 4; }
.day { fill: var(--season); fill-opacity: .55; stroke: var(--panel); stroke-width: 2; }
.day.sun { fill-opacity: .85; } .day.hol { fill-opacity: 1; stroke: var(--ink); stroke-width: 1.5; }
/* Catálogo */
.cards { display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 14px; }
.card { background: var(--panel); border: 1px solid var(--line); border-radius: 10px; padding: 14px 16px; min-width: 0; scroll-margin-top: 16px; }
.card:target { outline: 2px solid var(--copper); }
.card header { display: flex; gap: 12px; align-items: center; }
.card h4 { font: 600 20px/1.1 var(--display); margin: 0; }
.card p { margin: 8px 0; font-size: 14px; }
.card h5 { font: 600 12px var(--body); text-transform: uppercase; letter-spacing: .07em; color: var(--soft); margin: 12px 0 6px; }
.tab { font: 600 11px var(--body); text-transform: uppercase; letter-spacing: .08em; color: var(--tab); }
.unlock { color: var(--good); }
.ico { display: inline-flex; line-height: 0; }
.ico img { image-rendering: pixelated; }
.mc { display: inline-flex; align-items: center; justify-content: center; background: #6d6d6d; color: #fff;
  font: 600 11px var(--mono); border-radius: 3px; }
.slot { position: relative; display: inline-flex; width: 36px; height: 36px; align-items: center; justify-content: center;
  background: var(--slot); box-shadow: inset 2px 2px 0 var(--slot-dark), inset -2px -2px 0 var(--slot-light); }
.slot .count { position: absolute; right: 2px; bottom: 0; font: 700 12px var(--mono); color: #fff; text-shadow: 1px 1px 0 #3f3f3f; }
.grid9 { display: grid; grid-template-columns: repeat(3, 36px); gap: 0; }
.craft { display: flex; align-items: center; gap: 10px; }
.arrow { font-size: 20px; color: var(--soft); padding: 0 4px; }
.proc { display: flex; flex-wrap: wrap; align-items: center; gap: 4px; margin: 4px 0; }
.proc .machine { display: inline-flex; align-items: center; gap: 6px; font-size: 13px; font-weight: 600; margin-right: 6px; min-width: 120px; }
.proc .meta { font: 500 12px var(--mono); color: var(--soft); margin-left: 6px; }
.used { display: flex; flex-wrap: wrap; gap: 4px; }
.used .ico { background: var(--slot); padding: 3px; box-shadow: inset 2px 2px 0 var(--slot-dark), inset -2px -2px 0 var(--slot-light); }
.machines td:first-child { white-space: nowrap; }
.empty { color: var(--soft); display: none; }
@media (max-width: 860px) {
  .wrap { grid-template-columns: minmax(0, 1fr); }
  nav { position: static; max-height: none; }
  .navgroup { flex-direction: row; flex-wrap: wrap; gap: 2px 10px; align-items: baseline; }
  .navgroup span { width: 100%; }
  .hero h1 { font-size: 36px; }
}
@media (prefers-reduced-motion: no-preference) { html { scroll-behavior: smooth; } }
</style>
<div class="wrap">
<nav aria-label="Seções">
  <div class="brand"><i></i>Veio</div>
  <p class="tag">Wiki do jogo · tudo que a fábrica e a vila fazem</p>
  <label for="q" class="tag" style="display:block;margin:0 0 4px">Buscar item</label>
  <input id="q" type="search" placeholder="prensa, engrenagem, poste…" autocomplete="off">
  {{NAV}}
</nav>
<main>
  <header class="hero">
    <p class="eyebrow">Wiki do Veio</p>
    <h1>Automação industrial dentro do Minecraft</h1>
    <p>Extraia das jazidas do mundo, transforme em peças, ligue tudo com energia e esteiras e faça a vila crescer em volta da fábrica. Esta wiki explica cada sistema e traz a ficha de cada item. No jogo, aperte <kbd>J</kbd> para a mesma consulta.</p>
    <div class="stats"><span><b>{{COUNT}}</b> itens e blocos</span><span><b>{{MACHINES}}</b> receitas de máquina</span><span><b>17</b> sistemas</span></div>
  </header>
  {{BODY}}
  <p class="empty" id="none">Nenhum item com esse nome.</p>
</main>
</div>
<script>
(function () {
  var q = document.getElementById('q');
  var cards = Array.prototype.slice.call(document.querySelectorAll('.card'));
  var groups = Array.prototype.slice.call(document.querySelectorAll('.tabgroup'));
  var none = document.getElementById('none');
  q.addEventListener('input', function () {
    var term = q.value.trim().toLowerCase();
    var shown = 0;
    cards.forEach(function (card) {
      var hit = !term || card.getAttribute('data-search').indexOf(term) >= 0;
      card.hidden = !hit;
      if (hit) shown++;
    });
    groups.forEach(function (group) { group.hidden = !group.querySelector('.card:not([hidden])'); });
    none.style.display = shown ? 'none' : 'block';
    if (term && shown) { var first = cards.find(function (c) { return !c.hidden; }); if (first) first.scrollIntoView({ block: 'start' }); }
  });
})();
</script>
"""
