# Continuação: obras, funcionários, ruínas e proteção

Branch `cloud/obras-planta` (base: `origin/main` 542b0a0). Testes: `python tools/check_rust.py` (178 ok).

## Feito
- Plantas redesenhadas com interiores (~25% maiores), 9 prédios + Centro de pesquisa (`lab`).
- `workforce/staff.rs`: funcionário no canteiro concluído (cozinheira, professor, ferreiro, bibliotecário,
  carteiro, estalajadeiro, cientista); visitantes nos assentos no horário (alunos crescem na escola);
  pesquisas (Agronomia, Logística, Nutrição, Pedagogia, Engenharia: cientista instala módulos).
- Canteiro com 3 slots (comida, entrada, produto). Efeito da obra só vale **funcionando**
  (`site::near().operating`); centro mostra "de pé · parado" (`veio.center.project.idle`, bit 26).
- Projeto `lab` (bit 512, Vila), pesquisas nos bits 20+ dos efeitos (`projects::research_bit`).

## Falta
1. **Traduções** novas: `veio.site.no_staff`, `veio.site.next.<role>` (cook, teacher, smith, librarian,
   postman, innkeeper, scientist, market, council), `veio.center.project.idle`,
   `veio.research.done`, `veio.research.<key>.name/effect`, `veio.project.lab.name/effect`;
   menu do canteiro com 3 slots em `tools/build_content.py`; páginas do guia (lab, funcionários).
2. **Ruínas** (`workforce/ruins.rs`): na chegada (`spawn::tick`, quando `CENTER` era None) escolher lugares
   num anel de 48–80 blocos com `surface_heights` (MotionBlockingNoLeaves == OceanFloor, desnível ≤ 3,
   sem sobrepor + margem 3), `setblock ... veio:site[facing=..] keep`, mapa PLANNED em memória; no
   1º tick o canteiro pega a planta, marca `Fixed=1` (`cycle` recusa trocar) e carimba ruína parcial
   (camadas baixas, subconjunto determinístico, sem blocos com PROPS/protegidos) via `edit_blocks`.
3. **Proteção** (lib.rs + stub `native/offline-check/horizon-client`): ganchos gerados do Horizon:
   `destroy_block(&self, host: &mut dyn Host, event: &mut DestroyBlockEvent) -> Flow` {pos, state};
   `left_click_block(..., &mut LeftClickBlockEvent) -> Flow` {pos, face, state};
   `use_item_on(..., &mut UseItemOnEvent<'_>) -> Flow` {stack: &ItemStack, pos, face} (cancelar também
   bloqueia `use` do item — liberar comida etc.); `explosion_detonate(..., &mut ExplosionDetonateEvent<'_>)`
   {dimension: &str, center: [f64;3], blocks: &mut Vec<[i32;3]>} (tirar posições protegidas).
   `host.acting_player()` dá o jogador (criativo pode passar). Território = bounds da planta + 3, para
   canteiros `Fixed` e em BUILDING/DONE; liberar blocos naturais (grama, terra, flores, folhas, areia,
   cascalho, neve). Publicar a área no registro `BUILDINGS` (campo novo) e só contar em `near()` os que valem.
4. Docs (TRABALHADORES.md/README), checks (`check_project`, `check_performance`), zip consolidado.

## Integração com a main 0.22.0 (feita)
- Centro de controle com 8 espaços: 0 comida, 1-3 metas, 4 cofre, 5 saque (da 0.21), **6 obra**, **7 decreto**
  (`projects::SITE`/`DECREE` mudaram de 5/6 para 6/7).
- Estados do canteiro mudaram para 33-36 (`site::SITE_*`); o 32 é `no_company` do porto de exportação.
- Resumo do centro: linha 4 é a da companhia (caixa e exportação do dia, clique saca 1 esmeralda);
  a comida foi para a linha de aldeões. `DA5` traz (fila, ocupação+1) e `DA6` a exportação do dia (`center::refresh`).
- Economia fechada: `center::add_coins` é borda (esmeraldas de verdade); recompensas de pedidos e o
  rendimento da Prefeitura saem do caixa público (`center::reward`). Sem prefeitura ou sem saldo, só prestígio.
- Ainda pendente: menu do canteiro com 3 espaços em `tools/build_content.py` (o layout já tem 3).
