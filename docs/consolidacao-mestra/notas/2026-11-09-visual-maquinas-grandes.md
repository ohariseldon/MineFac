# Máquinas grandes: cada máquina com planta, tamanho e silhueta próprios

- **Data / frente / autor:** 2026-11-09 · visual · Claude, thread "Dar tamanho e personalidade às máquinas"
- **Onde:** branch `claude/project-thread-sx2015` (sobre `claude/teste-combinado`), PR aberto a partir dela
- **Sistemas:** máquinas de produção e energia, logística (escotilhas), tutorial (rastreio de entregas) · **FUN:** nova, sem ID
- **Tipo:** código · conteúdo

## O que mudou
- 19 máquinas viraram multiblocos: ao colocar, o bloco da máquina (núcleo) monta em volta a sua estrutura. Plantas de 2×2 a 4×4 e 3×5 de chão, 1 a 5 de altura (extratoras são torres, a cortadora é uma mesa baixa e comprida, a máquina a vapor é 3×5 com caldeira e volante, a fundição tem chaminé etc.). Prévia: `docs/visual/machine-frames.png`.
- Novos blocos sem item: `veio:structure` (peça, propriedades `piece` e `tier`), `veio:hatch_in` e `veio:hatch_out` (escotilhas com 1 slot). Planta e modelos em `tools/machine_frames.py`; `build_content.py` gera modelos, blockstates e `native/api/src/frames/data.rs`.
- Regras (`native/api/src/frames.rs`): só coloca com a planta livre e **chão firme e plano** embaixo de toda ela; se não couber, recusa e contorna de vermelho os blocos no caminho por 6 s. Quebrar qualquer parte desmonta a máquina e devolve o núcleo. Explosão não leva a estrutura; o núcleo repõe peças que faltem (a cada 200 ticks). Máquina montada não gira com a chave.
- Núcleo na fileira da frente, com a frente livre: painel, luz, entrada pela frente e energia (área do poste em volta do núcleo) seguem como antes. A saída sai pela escotilha de trás (16 itens por vez); escotilhas de entrada laterais na integradora e na fabricadora.
- Mundos antigos: máquinas já colocadas (sem NBT `Frame`) seguem compactas e funcionando.

## Evidência
- E1: `python3 tools/check_rust.py` → 383 testes ok (9 novos em `frames/tests.rs`); `check_project.py`, `check_machine_forms.py` e `check_performance.py` (checks idênticos) ok.
- Não testado em jogo. Pontos a ver no PC: estados com `[piece=…]` via `edit_blocks`, contorno vermelho (`NativeBlockRim`), `setblock … destroy` devolvendo o núcleo, escotilhas com esteira/braço/funil.

## Toca no compilado
- NBT novo: `Frame` (núcleo montado), `CoreSet`/`CoreX`/`CoreY`/`CoreZ` (escotilha).
- O tutorial e a wiki (outras threads) precisam mencionar a área plana e a escotilha de saída; o rastreio do tutorial já conta a escotilha como o núcleo (`frames::owner`).
