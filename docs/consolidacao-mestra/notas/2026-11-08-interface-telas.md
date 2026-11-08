# 2026-11-08 · Interface · Telas no padrão vanilla

**Frente:** interface (SIS-UI e telas dos blocos de todos os sistemas)
**Branch:** `claude/project-thread-j8sy4o` (base `claude/project-thread-l94frq`)
**Pedido:** Luiz achou a UI feia e difícil de entender ("Entrada 1, Entrada 2, Saída", o porto com "0 itens, líquido, 0 moedas", o celeiro parecendo uma bancada). Referência dada por ele: a mesa de ferraria e a mesa de encantamento do Minecraft.

## O que mudou

- **Kit de telas novo** (`native/api/src/ui/`): moldura, cabeçalho com o item e o nome do bloco e, embaixo, em cinza, o que ele faz; linha de estado com lâmpada; o "palco" de cada tipo de bloco; uma fileira de números sem caixa; rodapé com o "como e por quê" (ou o conselho do problema, na cor do estado); inventário vanilla. Painel cinza liso, sem faixas coloridas.
- **Palco por tipo de bloco** (`ui/spec.rs` liga cada bloco a uma família e a um tipo): receitas com fantasmas das entradas e da saída (alternando as receitas, como a ferraria), seta com progresso e "vai para"; extratora com a jazida; geradores com chama e combustível fantasma; solar, vento e água com o céu; esteiras, divisor, braço; lavoura, rancho, celeiro e silo; postos com o trabalhador e a ração; marcadores com o mapa da área; porto e estação com o navio/trem, a partida e as pilhas no cais; terminais com a rota; casas, loja, poço; laboratório; Tesla; âncora; canteiro de obras com a planta subindo.
- **Central de contratos e quadro da fábrica** viraram telas do kit no padrão ferraria: o que o contrato/fase pede aparece apagado nos slots da esquerda, a recompensa apagada à direita; o quadro mostra os ícones do que a fase libera, o pedido do dia e as máquinas paradas.
- **Centro de controle, prefeitura, lar e monitor** ganharam a moldura e o cabeçalho do kit, uma linha por assunto com ícone, fantasmas nos slots (pão, esmeralda, papel, tijolos, lenha) e nomes que não se sobrepõem mais. As abas do centro viraram botões `<` e `>` (`holding::click` usa os mesmos retângulos de `workforce::center`).
- **Dados do menu:** 14 números por tela (o 14º é a máscara de slots vazios, `ui::screens::EMPTY`), para os fantasmas aparecerem só em slot vazio. Todos os `menu/*.json` foram para `data_slots: 14`.
- **Textos:** `tools/build_ui_text.py` gera `veio.ui.tag.*` (o que faz, uma linha), `veio.ui.how.*` (como e por quê, até duas linhas), nomes dos cartões e rótulos, em pt e en, e reprova texto que não cabe (largura medida pela fonte do jogo em `tools/ui_font.py`).
- **Atlas de ícones** `content/assets/veio/textures/gui/ui.png` (256x256, desenhado por `tools/build_ui_atlas.py`, 60 ícones próprios). `check_project.py` aceita esse atlas fora da regra de 16x16.
- **Prévia offline:** `python3 tools/preview_screens.py [--lang en_us] [--only a,b] [--out DIR]` desenha as 77 telas sem o jogo, pelo `veio-check`.
- **Correção:** no grande projeto da central, o progresso da segunda meta ia para o lugar da estrela do farol e aparecia errado; agora `presentation::hub_info` publica o certo em cada fase.

## Por quê

Cada tela deve se explicar sozinha, como as telas vanilla: o que entra (fantasma no slot), a seta, o que sai e uma prévia do resultado. Números secundários ficam nas dicas ao passar o mouse.

## Como verificar

- `python3 tools/check_rust.py` (326 testes; novo: `screen_slots_stay_inside_the_panel_above_the_inventory_and_never_overlap`).
- `python3 tools/check_project.py`.
- `python3 tools/preview_screens.py` e abrir o `index.html` gerado.
- Falta a compilação real contra o Horizon e o teste no Windows (thread do PC).

## Pendências

- Ligar `workforce::next_step` e `production::missing_line` (PR #56) no rodapé do kit quando o #56 entrar na base.
- A chave `veio.project.lab.name` não existe nos idiomas da base (aparece crua na página Obras do centro).
