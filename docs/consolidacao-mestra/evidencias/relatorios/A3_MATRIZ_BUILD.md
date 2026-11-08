# A3 — Matriz da build (funcionalidade × camada)

Autor: OPUS A. Data: 2026-11-08. Camadas: **main** = `main@53267af` (0.25.0 lançada); **pr47** = `pr47@80980db` (candidata); **disperso** = branches/PRs/zips fora das duas (ou a origem ainda aberta de algo que entrou no pr47).
Estados: Concebido · Aprovado · Em desenvolvimento · Implementado isoladamente · Integrado · Validado · Bloqueado · Suspenso · Substituído · Descartado · Indeterminado. "—" = não existe na camada.
Regra aplicada: "Integrado" = código ligado aos ganchos de `lib.rs` e coberto pelas verificações offline (E1); **nenhuma linha é "Validado"** (sem compilação contra o Horizon real e sem partida). Na coluna disperso, "(#N, contido)" = a branch de origem é ancestral do pr47 (E2 por `merge-base --is-ancestor`), o PR continua aberto.
Evidência: E1 executado · E2 código · E3 documento/commit · E4 chat/memória · ND.

| ID | Sistema | Funcionalidade | main | pr47 | disperso | Evid. | Observação |
|---|---|---|---|---|---|---|---|
| FUN-PLT-01 | PLT | Validação de conteúdo `check_project` | Integrado | Integrado | — | E1 | main 529 JSON/93 pág./65 tex.; pr47 742/100/89 + aviso 29 pág. > 14 linhas |
| FUN-PLT-02 | PLT | Testes Rust offline contra stub | Integrado | Integrado | — | E1 | 176/0/5 × 325/0/9; avisos 4 × 3 linhas |
| FUN-PLT-03 | PLT | Cenários `check_performance` | Integrado | Integrado | — | E1 | 7 cenários; checks idênticos |
| FUN-PLT-04 | PLT | Compilação contra o Horizon real | Indeterminado | Indeterminado | Indeterminado (E3: 0.16.1 compilada em 4ffb605) | E3 | `read_columns`/`chunk_load` nunca compilados |
| FUN-PLT-05 | PLT | `check_native_api.py` (rmeta real) | Implementado isoladamente | Implementado isoladamente | — | E2 | Não roda sem `.rmeta` do Horizon |
| FUN-PLT-06 | PLT | Pacote jogável `release/Veio-<v>.zip` | Integrado | — | Substituído (zips 0.15.1/0.15.2) | E2 | pr47 só tem o zip da main + "Realismo 1.1" = 0.16.1 |
| FUN-PLT-07 | PLT | Launcher instala e compila (`native-build`) | Integrado | Integrado | — | E1 | Testes 2 × 6; Cargo 0.10.0 × 0.16.1; pr47 com perfil nativo `performance` (main `dev`) |
| FUN-PLT-08 | PLT | Geradores Python reprodutíveis | Integrado | Integrado | — | E1 | Só PNG (zlib) e `.ogg` diferem em bytes |
| FUN-PLT-09 | PLT | `sim_economy.py` (7 invariantes) | Integrado | Integrado | — | E1 | Não modela emissões fora do ledger |
| FUN-PLT-10 | PLT | CI automática | — | — | — | E2 | `.github` inexistente |
| FUN-PLT-11 | PLT | Dependência do Horizon fixada (`rev`/lock) | — | — | — | E2 | `git = …` sem `rev`, sem Cargo.lock |
| FUN-MUN-01 | MUN | Jazida com pureza (área 3×3) | Integrado | Substituído | — | E2 | Substituída por veio conectado |
| FUN-MUN-02 | MUN | Veio conectado até 16 blocos + pureza industrial | — | Integrado | Implementado isoladamente (#23 protótipo; import-progressao, contidos) | E2 | `extraction.rs:34-57` |
| FUN-MUN-03 | MUN | Rios com nascente, afluentes, quedas e foz | — | Integrado | — | E1+E2 | 9 testes; só chunks novos; estado em `rios.txt` (gravação via temporário) |
| FUN-MUN-04 | MUN | Mapa finito (mixins, 15 000×15 000) | — | Em desenvolvimento | Em desenvolvimento (c8e94dc = 80980db por patch-id) | E2+E3 | Desligado, fora do crate |
| FUN-MUN-05 | MUN | Segurança de portais/Nether no mapa finito | — | Em desenvolvimento | Em desenvolvimento | E3 | Auditoria do Nether falhou (CONTINUAR.md) |
| FUN-EXT-01 | EXT | Brocas Mk1/Mk2/Mk3 | Integrado | Integrado | — | E2 | 40/20/10 ticks; 6/12/30 FE |
| FUN-EXT-02 | EXT | Britador, forno de liga, fundição | Integrado | Integrado | — | E2 | Via `data.rs` |
| FUN-EXT-03 | EXT | Balanço "veio normal pede duas prensas" | — | Integrado | Implementado isoladamente (#19, contido) | E3 | Commits 96458cf/3eb4b3d |
| FUN-EXT-04 | EXT | Tempos de receita da 0.25 (decisão 9) | Integrado | Integrado | Substituído (tempos por fase do Codex, 0ce91a4/50995e6) | E2 | 53 receitas idênticas |
| FUN-PRO-01 | PRO | Máquinas por tabela (`data.rs`) | Integrado | Integrado | — | E1+E2 | 57 receitas no pr47 |
| FUN-PRO-02 | PRO | Combustíveis gerador/biomassa | Integrado | Integrado | — | E2 | Carvão 1600 … biomassa 600 |
| FUN-PRO-03 | PRO | Módulos velocidade/eficiência/produtividade | Integrado | Integrado | — | E2 | Produtividade 10 % → 20 % |
| FUN-PRO-04 | PRO | Kit Mk.II | — | Integrado | — | E2 | ×0,5 tempo, ×2 energia |
| FUN-PRO-05 | PRO | Cadeia funda: motor, processador, núcleo, fabricador | — | Integrado | Implementado isoladamente (#40, contido) | E2 | 4 receitas novas |
| FUN-PRO-06 | PRO | Kit de reparo/integridade | Integrado | Integrado | — | E2 | `REPAIR=50` |
| FUN-PRO-07 | PRO | Monitor de itens/min (decisão 3) | — | Integrado | Implementado isoladamente (#40, contido) | E2 | `industry.rs`, até 4095/min, 9 linhas |
| FUN-PRO-08 | PRO | Bônus de refeição sobre o tempo de máquina | — | Integrado | — | E2 | Global (atomics), não por jogador |
| FUN-ENE-01 | ENE | Postes madeira/aço e rede | Integrado | Integrado | — | E1 | 256_poles 2,29 → 1,53 ms |
| FUN-ENE-02 | ENE | Gerador a carvão e biomassa | Integrado | Integrado | — | E2 | 40 FE/t |
| FUN-ENE-03 | ENE | Roda d'água, eólica, solar, geotérmica | Integrado | Integrado | — | E2 | `generation.rs` |
| FUN-ENE-04 | ENE | Máquina a vapor e bateria | Integrado | Integrado | — | E2 | pr47: liberadas pela pesquisa; pr47: vapor consome 5 mB de água por operação |
| FUN-ENE-05 | ENE | Turbina hidrelétrica | — | Integrado | Implementado isoladamente (#40, contido) | E2 | `HYDRO_MAX 320` |
| FUN-AGU-01 | AGU | Bomba | — | Integrado | #40 (contido) | E2 | 25 mB × 4 fontes, 6 FE/t |
| FUN-AGU-02 | AGU | Cano com vazão por face | — | Integrado | #40 (contido) | E2 | 200 mB/face, NBT "Water"; lava nos canos (5 mB/fonte, balde 1000) — comportamento ND |
| FUN-AGU-03 | AGU | Aspersor e fertilidade do solo | — | Integrado | #40 (contido) | E2 | 1/10, 1/30, máx. 400 |
| FUN-AGR-01 | AGR | Lavoura automática | Integrado | Integrado | — | E2 | Raio 4, 3 camadas; marcadores `field_marker`/`pen_marker` |
| FUN-AGR-02 | AGR | Plantadeira, colheitadeira, lâmpada de cultivo | Integrado | Integrado | — | E2 | `crops.rs` |
| FUN-AGR-03 | AGR | Rancho e posto de pastor | Integrado | Integrado | — | E2 | Cap 10 animais |
| FUN-AGR-04 | AGR | Silvicultura | Integrado | Integrado | — | E2 | 8 pontos, tronco ≥ 4 |
| FUN-AGR-05 | AGR | Moinho, cozinha, despensa | Integrado | Integrado | — | E2 | Preço despensa 60 |
| FUN-AGR-06 | AGR | Grok lote 2 (lavrador mais barato, trigo) | — | Substituído | Substituído (#7 aberto) | E2+E3 | 008a980 registra sem reaplicar |
| FUN-AGR-07 | AGR | Lavoura irrigada | — | Integrado | #40 (contido) | E2 | Via aspersor |
| FUN-LOG-01 | LOG | Esteira e esteira rápida | Integrado | Integrado | — | E1 | Testes logistics 16/17 |
| FUN-LOG-02 | LOG | Esteira expressa | — | Integrado | #40 (contido) | E2 | |
| FUN-LOG-03 | LOG | Túnel de esteira | — | Integrado | #40 (contido) | E2 | Alcance 6 |
| FUN-LOG-04 | LOG | Divisor, fusor, filtros, prioridade | Integrado | Integrado | — | E1 | logistics_controls 2/3 |
| FUN-LOG-05 | LOG | Braços normal/rápido/longo | Integrado | Integrado | — | E1 | inserters 9/9 |
| FUN-LOG-06 | LOG | Recusa de entrega (`container_accepts`) | Integrado | Integrado | — | E1 | operations 8/8 |
| FUN-TRA-01 | TRA | Terminal ferroviário `veio:rail_terminal` | — | Integrado | Implementado isoladamente (#28 como `freight_station`, contido) | E2 | Decisão 6; carga movida por carrinho/funil vanilla |
| FUN-TRA-02 | TRA | Drones e porto de drone | — | Integrado | #28 (contido) | E1+E2 | 64 bl., 16 itens; exige chunks carregados; cartão de rota `veio:route_card` (`freight.rs:11`) |
| FUN-TRA-03 | TRA | Caravanas de animais | — | Integrado | #28 (contido) | E1+E2 | 512 bl.; cavalo/burro/porco |
| FUN-TRA-04 | TRA | Estações de carga/descarga | — | Integrado | #40 (contido) | E1+E2 | 16 canais × 4096 |
| FUN-TRA-05 | TRA | Produção virtual com chunk descarregado (decisão 2) | — | Integrado | #40 (contido) | E1+E2 | Sem insumo nem energia; 194 µs/1000 estações |
| FUN-TRA-06 | TRA | Âncora de chunk com custo de energia | — | Integrado | #40 (contido) | E2 | `forceload`, 32 FE/t, `ancoras.txt` |
| FUN-TRA-07 | TRA | Elevador de carga | — | Integrado | import-megafactories (contido) | E1 | 2 testes |
| FUN-TRB-01 | TRB | Posto de carregador | Integrado | Integrado | — | E1 | Raio 6, lote 16 |
| FUN-TRB-02 | TRB | Mesa do capataz e fila de tarefas | Integrado | Integrado | — | E1 | Fila 8; foreman_95 0,75 → 0,63 ms |
| FUN-TRB-03 | TRB | Planejamento incremental do capataz | Integrado | Integrado | — | E2 | Codex 13758bd |
| FUN-TRB-04 | TRB | Busca incremental do carregador | Integrado | Integrado | — | E2 | Codex 604369d |
| FUN-TRB-05 | TRB | Lavrador e pastor | Integrado | Integrado | — | E1 | |
| FUN-TRB-06 | TRB | Zonas de trabalho em 3 níveis | Integrado | Integrado | — | E1 | Testes `zones::unlocked` |
| FUN-TRB-07 | TRB | Crescimento da vila 30 → 150 | Integrado | Integrado | — | E1 | |
| FUN-TRB-08 | TRB | Nascimento exige cama livre | — | Integrado | #21 (contido) | E3 | 19a311e/662b9a3 |
| FUN-TRB-09 | TRB | Spawn no centro da vila | Integrado | Integrado | — | E2 | `SEARCH 96` |
| FUN-TRB-10 | TRB | Cérebro/navegação e descanso noturno | Integrado | Integrado | — | E3 | 0.16.0; nunca visto em jogo |
| FUN-TRB-11 | TRB | Carga visível nas mãos | Integrado | Integrado | — | E3 | 0.16.1 |
| FUN-TRB-12 | TRB | Vila grande (150 + 300 animais) medida | Integrado | Bloqueado | — | E1 | pr47 entra em pânico `world_dir` no harness |
| FUN-VIL-01 | VIL | Casas, famílias, carteira | Integrado | Integrado | — | E2 | 4 moradores/casa |
| FUN-VIL-02 | VIL | Lojas e comerciantes | Integrado | Integrado | — | E2 | |
| FUN-VIL-03 | VIL | Prefeitura, prefeito, fases | Integrado | Integrado | — | E2 | 3 fases em `mayor.rs:18` |
| FUN-VIL-04 | VIL | Poço/saneamento e energia na casa | Integrado | Integrado | — | E2 | Raios 24/12 |
| FUN-VIL-05 | VIL | IDH, Gini, bairros | Integrado | Integrado | — | E2 | |
| FUN-VIL-06 | VIL | 12 bens de conforto | Integrado | Integrado | — | E2 | `GOODS` |
| FUN-ECO-01 | ECO | Contas companhia/pública/loja | Integrado | Integrado | — | E2 | 0.19.0 |
| FUN-ECO-02 | ECO | Imposto na fonte por fase | Integrado | Integrado | — | E2 | 4/8/12/16 % |
| FUN-ECO-03 | ECO | Salário por empregador | Integrado | Integrado | — | E2 | 4/turno |
| FUN-ECO-04 | ECO | Assistência pública | Integrado | Integrado | — | E2 | 1/pessoa/turno |
| FUN-ECO-05 | ECO | Holding: lucro por divisão (11 abas) | Integrado | Integrado | — | E2 | Só em memória |
| FUN-ECO-06 | ECO | Fronteiras de moeda (`BORDERS`) | Integrado | Integrado | — | E2 | Só em memória |
| FUN-ECO-07 | ECO | Recompensas pagas pelo caixa público (decisão 1) | — | Integrado | Implementado isoladamente (zip obras = f1452f5, contido) | E2 | `center.rs:609` |
| FUN-ECO-08 | ECO | Rendimento da obra Prefeitura | — | Integrado | — | E2 | 1/16 do caixa até 8/dia (cabeçalho `projects.rs`) |
| FUN-ECO-09 | ECO | Emissão de esmeraldas fora do ledger | Integrado | Integrado | — | E2 | EXPORT, village, grande projeto, pedido do dia |
| FUN-MER-01 | MER | Preço local dinâmico | Integrado | Integrado | — | E2 | Fator 50–200 % |
| FUN-MER-02 | MER | Atacado 75 % e liquidação | Integrado | Integrado | — | E2 | Só com cofre da companhia |
| FUN-MER-03 | MER | Importação de comida | Integrado | Integrado | — | E2 | Até 16 unidades |
| FUN-MER-04 | MER | Porto `export_dock` | Integrado | Integrado | — | E2 | 9 pilhas/2 turnos |
| FUN-MER-05 | MER | Estação `freight_station` (comércio) | Integrado | Integrado | — | E2 | 6 pilhas/turno |
| FUN-MER-06 | MER | Bolsa mundial de 34 produtos | Integrado | Integrado | — | E2 | Índice 80–120 |
| FUN-MER-07 | MER | Contratos com prazo e reputação | Integrado | Integrado | — | E2 | +50 %, máx. 3 |
| FUN-MER-08 | MER | Venda ao jogador em lotes por esmeralda | Integrado | Integrado | — | E2 | Até 16 por esmeralda |
| FUN-OBR-01 | OBR | Canteiro de obras | — | Integrado | Implementado isoladamente (zip obras, contido) | E2 | Menu 1 slot × código 3 |
| FUN-OBR-02 | OBR | 10 obras com efeitos | — | Integrado | idem | E2 | Bits 1…512 |
| FUN-OBR-03 | OBR | Prestígio da vila | — | Integrado | idem | E2 | 0/60/200/500 |
| FUN-OBR-04 | OBR | Pedidos diários da vila | — | Integrado | idem | E2 | 27 itens, 4 slots |
| FUN-OBR-05 | OBR | Decretos (CALL/RATION/PLENTY) | — | Integrado | idem | E2 | Slot 7 do centro |
| FUN-OBR-06 | OBR | 9 plantas prontas | — | Integrado | idem | E1 | 7542 blocos |
| FUN-OBR-07 | OBR | Centro de pesquisa da vila (5 pesquisas) | — | Integrado | idem | E2 | Chaves de tradução faltando |
| FUN-OBR-08 | OBR | Boletim da manhã | — | Integrado | idem | E2 | `center.rs:15` |
| FUN-OBR-09 | OBR | Ruínas | — | Concebido | Concebido (HANDOFF_OBRAS) | E3 | `ruins.rs` inexistente |
| FUN-OBR-10 | OBR | Proteção de obras (ganchos) | — | Concebido | Concebido | E3 | 4 ganchos ausentes |
| FUN-LAR-01 | LAR | Casa do fundador (estrutura, mural) | — | Integrado | Implementado isoladamente (d1e2f30, contido) | E1+E2 | 251 comandos |
| FUN-LAR-02 | LAR | Lar `veio:home` (lareira, despensa, conforto, banquete) | — | Integrado | zip obras (contido) | E2 | Nível máx. 4 |
| FUN-LAR-03 | LAR | Bônus só pelo Lar; comer fora mantém bônus (decisão 8) | — | Integrado | — | E2 | `home/mod.rs:101` |
| FUN-LAR-04 | LAR | Cama restrita à casa (`bed_allowed`) | — | Integrado | — | E2 | `home/mod.rs:306` |
| FUN-CTL-01 | CTL | Centro com metas, cofre e saque | Integrado | Integrado | — | E2 | Saque até 16 |
| FUN-CTL-02 | CTL | Abas da holding | Integrado | Integrado | — | E2 | 0.20.0 |
| FUN-CTL-03 | CTL | Páginas Resumo/Atenção/Obras | — | Integrado | zip obras (contido) | E2 | `PAGES 4` |
| FUN-CTL-04 | CTL | Foco/bússola | — | Integrado | idem | E2 | `focus.rs` |
| FUN-PRG-01 | PRG | Avanços liberando receitas | Integrado | Integrado | — | E2 | Sem `doLimitedCrafting`: bloqueio real ND |
| FUN-PRG-02 | PRG | Quadro da fábrica, fases e licenças | — | Integrado | Implementado isoladamente (import-factory-board, contido) | E1+E2 | Texto "Libera" divergente |
| FUN-PRG-03 | PRG | Pedido do dia do quadro | — | Integrado | idem | E2 | 1–6 esmeraldas fora do ledger |
| FUN-PRG-04 | PRG | Alertas de gargalo (censo) | — | Integrado | idem | E2 | `census.rs` |
| FUN-PRG-05 | PRG | Laboratório de pesquisa (4 tópicos) | — | Integrado | #40 (contido) | E1+E2 | Estado global `pesquisa.txt` |
| FUN-PRG-06 | PRG | Central de contratos | Integrado | Integrado | — | E2 | Valores maiores no pr47 |
| FUN-PRG-07 | PRG | Grande projeto em 4 fases | — | Integrado | #40 (contido) | E2 | Avanço `grand_project` |
| FUN-PRG-08 | PRG | Progressão por fases do Codex (30/min) | — | Substituído | Substituído (0ce91a4) | E2 | Decisão 9 |
| FUN-TUT-01 | TUT | Dicas contextuais | Integrado | Integrado | — | E2 | Chaves dinâmicas |
| FUN-TUT-02 | TUT | Guia principal | Integrado | Integrado | — | E1 | 93 → 100 páginas (teto) |
| FUN-TUT-03 | TUT | Guia da Vila Operária | Integrado | Integrado | — | E1 | pr47: 29 páginas > 14 linhas |
| FUN-TUT-04 | TUT | Guia Transportes | — | Integrado | — | E1 | |
| FUN-TUT-05 | TUT | Guia Grande Indústria | — | Integrado | #40 (contido) | E1 | 26 páginas; fora do `check_page_fit` |
| FUN-TUT-06 | TUT | Tutorial interativo por jogador | — | Integrado | Substituído (zip 0.15.1, `TRACE_TTL 200`) | E1 | 19 + 3 testes |
| FUN-DEF-01 | DEF | Poluição e ataques | Integrado | Integrado | — | E2 | Só em memória |
| FUN-DEF-02 | DEF | Ondas noturnas com arqueiros e brutos | Integrado | Integrado | — | E2 | 3ª/5ª noite |
| FUN-DEF-03 | DEF | Quartel (4 classes) | Integrado | Integrado | Integrado (#30 aberto, conteúdo na main) | E2 | |
| FUN-DEF-04 | DEF | Bobina de Tesla | Integrado | Integrado | — | E2 | 2400 FE/tiro |
| FUN-DEF-05 | DEF | Defensores patrulham à noite | — | Integrado | — | E3 | fa96ceb |
| FUN-MEG-01 | MEG | Copiadora de plantas | — | Integrado | import-megafactories (contido) | E1 | 9 testes |
| FUN-MEG-02 | MEG | Marco de obra | — | Integrado | idem | E2 | |
| FUN-MEG-03 | MEG | Prancheta | — | Integrado | idem | E2 | |
| FUN-MEG-04 | MEG | Escadas entre andares | — | Integrado | idem | E2 | Até 6 andares |
| FUN-APR-01 | APR | Painéis e telas | Integrado | Integrado | — | E2 | 44 status |
| FUN-APR-02 | APR | Texturas 16×16 | Integrado | Integrado | — | E1 | 65 → 89 |
| FUN-APR-03 | APR | Silhuetas e colisão por máquina | — | Integrado | #31 (contido) | E1 | 34 formas, 27 silhuetas |
| FUN-APR-04 | APR | Animação de peças | — | Integrado | #50 (contido) | E1 | motion 2 testes |
| FUN-APR-05 | APR | Som posicional das máquinas | — | Integrado | #49 (contido) | E1 | 30 `.ogg` |
| FUN-APR-06 | APR | Ambiente sonoro limitado | — | Integrado | #50 (contido) | E1 | soundscape 7 |
| FUN-APR-07 | APR | Shader "realismo leve" | — | Integrado | Implementado isoladamente (zip Realismo 1.1 = 0.16.1) | E2 | api 0.1 aceita pelo Horizon; WGSL ND |
| FUN-APR-08 | APR | Quadro desenhado sem alocação (0.17.1) | Integrado | — | — | E1 | pr47: 329 alocações/quadro |
| FUN-DES-01 | DES | Orçamento global por faixa | Integrado | Integrado | — | E1 | 3 testes |
| FUN-DES-02 | DES | Topologia/busca incremental | Integrado | Integrado | — | E1 | |
| FUN-DES-03 | DES | Bench fábrica 480 blocos | Integrado | Integrado | — | E1 | ~272 → ~385 µs/tick |
| FUN-DES-04 | DES | Teto global de ticks/limite do mapa | — | Em desenvolvimento | Em desenvolvimento (thread "Definir limite do mapa") | E4 | |
| FUN-MP-01 | MP | Convidado LAN vai à vila do anfitrião | — | Integrado | #25 (contido) | E2 | `spawn.rs:151` |
| FUN-MP-02 | MP | Estado por jogador (tutorial por UUID) | — | Integrado | — | E2 | Pesquisa e bônus são globais |
| FUN-PER-01 | PER | NBT de block entities | Integrado | Integrado | — | E2 | |
| FUN-PER-02 | PER | Arquivos `<mundo>/veio/` | — | Integrado | — | E2 | 5 arquivos, sem versão |
| FUN-PER-03 | PER | Reset de estado ao trocar de mundo | — | Integrado | — | E2 | Parcial: home/census/blueprint + detecção por `world_dir` |
| FUN-PER-04 | PER | Gravação atômica | — | Integrado | — | E2 | Só rios e tutorial |

Total: 150 linhas.

---

## Visão transversal

### 1. Desconectado (existe mas não está ligado ou não chega ao jogador)
- `site.rs:142 sites_near`, `staff.rs:124 research_percent`, `foreman.rs:149 distance` — funções nunca chamadas (avisos E1).
- Menu do canteiro com 1 slot (`content/data/veio/horizon/menu/site.json`) contra 3 slots no código (`workforce/mod.rs:150`): o 2º insumo e a saída não aparecem na tela (E2; efeito em jogo ND).
- Texto "Libera" do quadro (`milestones/mod.rs:246`) aponta para receitas que no pr47 são concedidas por `research/*.json`, não pelas licenças.
- `release/Veio-Realismo-1.1.zip` (pr47) entrega Veio 0.16.1: desconectado do código da candidata.
- `tools/check_native_api.py`: existe, mas não pode rodar sem o `.rmeta` real.
- `native_chunk_tickets.rs` do Horizon (API de tickets) não usado; a âncora usa o comando `forceload`.

### 2. Depende de sistema inexistente ou não verificado
- Rios dependem de `read_columns` + `chunk_load(new=true)` + `edit_blocks` durante geração — nunca compilados nem vistos.
- Bloqueio de receitas por avanço depende de `doLimitedCrafting` (não definido em lugar nenhum) ou de comportamento próprio do Horizon (ND).
- Proteção das obras depende de ganchos `destroy_block`/`left_click_block`/`use_item_on`/`explosion_detonate` não implementados.
- Ruínas dependem de `workforce/ruins.rs` inexistente.
- Mapa finito depende de mixins de motor (22/24 injeções) fora da API de mods.
- `bench_large_village` depende de `world_dir` que o harness não implementa.

### 3. Duplicado ou divergente
| Par | Situação (E2) |
|---|---|
| Casa do fundador × Lar | Coexistem conforme a decisão 8: `home/` (Codex, estrutura + bônus de comer fora) e `workforce/home.rs` (Lar, refeição/conforto). `home::time_factor` não dá bônus de casa. README pr47 l.21 ainda promete 15 %. |
| `freight_station` × `rail_terminal` | Resolvido (decisão 6): comércio exterior = `veio:freight_station` (`trade.rs`), terminal ferroviário = `veio:rail_terminal` (a79f326). |
| Tempos 0.25 × fases do Codex | Resolvido no código (decisão 9): `production.json` da main idêntico no pr47; a progressão por fases do Codex (0ce91a4) foi sobrescrita. Memória registra que o auto-merge já trouxe tempos do Codex por engano uma vez. |
| Contratos × pedidos × projetos | Três sistemas que pagam por entregas: central de contratos (`contracts.rs`, esmeraldas como item), pedidos da vila (`requests.rs`, pagos pelo caixa público via `center::reward`), pedido do dia do quadro (`milestones`, esmeraldas no slot 1) + grande projeto. Só o segundo respeita a economia fechada. |
| Centro de controle × quadro da fábrica | Ambos mostram metas, alertas e pedidos: centro (`center.rs`, metas da vila e holding) e quadro (`milestones/`, fases da fábrica, gargalos). Raios iguais (64) e censos separados (`census.rs` × `center` STALE 200). |
| Âncora × estações | Duas respostas à decisão 2: âncora (`industry.rs`, carrega de verdade com `forceload`, 32 FE/t) e estação (produção virtual sem carregar). Âncora "continua opcional" (contexto §4). |
| Pesquisa do laboratório × pesquisa da vila | `research.rs` (4 tópicos, avanços) e `staff.rs` (5 pesquisas da vila); chave `veio.research.done` com aridades diferentes. |
| Dois detectores de servidor | `host.window()` e `industry::on_server` (`industry.rs:106`). |
| Códigos de status 40/41 | `freight::NO_ROUTE/NO_ANIMAL` e `center::ZONE_READY/STORE_FULL`. |

### 4. Apresentado como concluído sem validação
- README pr47 l.31 admite: "As entregas da 0.11.0 à 0.25.0 aguardam teste no jogo" (E3); nenhuma tem teste em partida.
- `CONSOLIDACAO.md`/commit 4ffb605 "after full Horizon native compilation" (E3) vale para a 0.16.1 do Codex, não para o pr47.
- 0.17.1 "nenhuma alocação ao desenhar um quadro" — verdade na main (E1), falsa no pr47 (E1).
- Thread do #40 declarou rios/estações prontos; `read_columns`/`chunk_load` sem compilação (E4 + E2).
- "Nenhum código ficou fora do #47" — confirmado por esta auditoria (A4 §4), com a ressalva de commits locais invisíveis.

### 5. Essencial não abordado
- CI e compilação reprodutível contra um Horizon fixado.
- Teste em partida de qualquer versão ≥ 0.11.0.
- Versão de formato dos arquivos `<mundo>/veio/` e migração de saves entre 0.25 (main) e pr47 (novos NBT, novos blocos).
- Persistência de poluição, holding e `BORDERS`.
- Benchmarks para rios, água, estações em escala real, som e obras.
- Contabilização das emissões de esmeralda fora do ledger.

### 6. Trabalho em branch fora da main
Até 80980db, todo o código fora da main está no pr47 (79 commits); fora do pr47 só zips 0.15.1/0.15.2 e um commit com o mesmo patch-id do head (A4). **Exceção nova:** PRs #51 (Veio 0.26.0: Realismo 2.1, clima e estações; 53 arquivos, +2683/−189, novos `ambience.rs`/`climate.rs`) e #52 (sons só perto do jogador, mural ocioso, medição de escala; 6 arquivos) foram abertos às 20:34–20:35Z com base no pr47 e não foram auditados em código. PRs abertos cujo conteúdo já está no pr47: #50, #49, #40, #31, #28, #25, #23, #21, #20, #19, #7; #30 já está na main.
