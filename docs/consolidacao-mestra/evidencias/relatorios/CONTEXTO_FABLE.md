# Contexto da missão (preparado pelo supervisor "Fable", Fase I)

Data: 2026-11-08 (~20:15 UTC). Idioma de todos os entregáveis: **português do Brasil**.

## 1. Repositório canônico e escopo

- Repositório canônico: `ohariseldon/MineFac` (GitHub). Jogo chamado **Veio** no jogo; mod nativo em Rust para o **Horizon** (Minecraft em Rust, versão 26.3, fonte github.com/horizon-rust/horizon).
- **Não** é o Projeto-Moba nem o Wasému/pesquisa-macro. Nenhum desses repositórios está acessível nesta sessão; registrar como "fora de escopo / inacessível", sem inventar.
- "audiões" no prompt do usuário = provavelmente **aldeões** (villagers). Verifique e registre.
- Clones locais:
  - `/home/user/MineFac` = `origin/main` (Veio 0.25.0, commit 53267af). NÃO editar código ali; só o supervisor escreve docs nessa árvore.
  - `SP/pr47` = worktree de `origin/claude/project-thread-l94frq` (head 80980db), a branch do **PR #47** (aberto), que é a candidata a build consolidada: main 0.25 + obras + sons + formatos de máquina + caravanas/drones/ferrovias + grande indústria/água/rios (#40) + consolidação Codex 50995e6 + PRs do Grok + rascunho do mapa finito (desligado, em `work-in-progress/finite-world/`). 79 commits à frente da main, 575 arquivos, +93k/−4.5k linhas.
  - `SP` = `/tmp/claude-0/-home-user-MineFac/2438a94b-e52e-53f5-9ff0-c6cae7a87c22/scratchpad`
- Todas as 42 branches remotas foram buscadas (`git branch -r` no clone principal; use `git -C /home/user/MineFac ...`).
- Ferramentas: `cargo` 1.97 e `python3` disponíveis. Verificações: `python tools/check_project.py`, `python tools/check_rust.py` (testes Rust offline contra stub do Horizon em `native/offline-check/horizon-client`), `python tools/check_performance.py`. Não há CI (`.github` não existe). Não existe compilação contra o Horizon real nem teste em jogo nesta sessão.
- Upload do usuário: `/mnt/project-files/uploads/hearth/bfe368aa-...` (zip, 724 KB) = a "build parcial de obras" de outro agente, já integrada no PR #47 (commit f1452f5). Somente leitura; não precisa reabrir salvo para verificar algo pontual.

## 2. Pull requests (estado em 2026-11-08 ~20:12)

Abertos: #50 (sons ambiente/movimento, base codex/machine-audio), #49 (áudio posicional das máquinas, base codex/machine-identity), #47 (consolidação, base main), #40 (grande indústria, água, rios — já fundido dentro do #47), #31 (formatos das máquinas, base codex/interzone-logistics), #30 (quartel, base cursor/grok-noite — conteúdo já na main segundo outro agente), #28 (caravanas/drones/ferrovias), #25 (LAN Grok), #23 (pesquisa jazidas conectadas — só docs), #21 (vila só cresce com cama), #20 (balanço progressão/missões), #19 (duas prensas), #7 (Grok lote 2).
Fundidos: #1–#6, #8–#18, #22, #24, #26, #27, #29, #32–#39, #41–#46, #48 (main chegou a 0.25.0 por PRs #39, #41–#46, #48 do agente "Cloud"/Claude em `claude/trusting-ritchie-sc9ty8`, autor GitHub jefterpereira).
Autores GitHub: `ohariseldon` (Luiz, dono) e `jefterpereira` (conta usada por agentes).

## 3. Agentes e frentes (declarado em docs e no chat; verificar)

- "Cloud" = Claude (trabalhadores/aldeões, vila, sociedade, economia; branch `claude/trusting-ritchie-sc9ty8`).
- "Codex" = OpenAI Codex (expansão, integração, logística, braços, transportes, megafábricas, tutorial, sons; branches `codex/*`).
- "Grok" = xAI Grok via Cursor (desempenho e balanceamento; branches `cursor/grok-*`, `grok/*`).
- Threads Claude neste projeto (claude.ai Projects), 2026-11-08:
  - "Criar agricultura real, água e energia" (concluída): entregou PR #40 (água, rios, grande indústria, estações de carga/descarga, âncora de chunk, laboratório de pesquisa, monitor), e um artefato "Veio: lacunas em relação a jogos de fábrica".
  - "Integrar build parcial ao repositório" (concluída): montou o PR #47 e fez auditoria dizendo que nenhum código de branch/PR do GitHub ficou fora do #47; só zips 0.15.x antigos (branches `codex/arquivos-historicos`, `codex/arquivos-builds-anteriores`, `codex/consolidated-0161`). Commits locais nunca enviados não são visíveis.
  - "Aumentar o realismo sem pesar o PC" (em andamento agora, Opus 5.5 extra).
  - "Definir limite do mapa e otimizar" (em andamento agora: limite do mapa, buscas em lotes, teto global de ticks).
  - "Consolidar a documentação mestra do projeto" (esta missão).

## 4. Decisões do dono (Luiz) registradas no chat em 2026-11-08 — fonte primária de "decisão aprovada"

1. 17:18 — Recompensas de pedidos da vila e rendimento da obra Prefeitura são **pagos pelo caixa público (prefeitura)**; sem prefeitura ou sem saldo, só prestígio. (Implementado como `center::reward` no PR #47.)
2. 16:38 — Fábrica longe do jogador: escolheu **âncora de chunk com custo de energia** (opção recomendada). 17:01 — esclareceu que quer **comportamento tipo Satisfactory: fábricas distantes continuam produzindo e entregando sem carregar tudo, mantendo leveza**. Resultado: estações de carga/descarga com produção virtual (0.23.0 no #40, `native/api/src/stations.rs`); âncora continua opcional.
3. 16:38 — "Razões visíveis": usar mecanismos de Satisfactory/Factorio (itens por minuto no guia e no painel).
4. 16:32 — "Conserte essas lacunas" (aprovou corrigir as lacunas do artefato de análise de lacunas: progressão por volume/laboratório, cadeia mais funda, monitor, projeto final em fases, esteira expressa/túnel/prancheta, lava nos canos).
5. 15:55 — Pediu agricultura real (irrigação, necessidades), água/energia (bombas, canos, usinas reais, hidrelétrica) e rios com nascentes nas montanhas, afluentes, quedas e foz no mar; explicar no tutorial; não remodelar do zero o sistema do Minecraft.
6. 17:41 — Conflito de nome `veio:freight_station`: **o terminal ferroviário do Codex vira `veio:rail_terminal`**; a estação de carga do comércio exterior da main mantém `freight_station`.
7. 17:51 — Sons + caravanas + formatos de máquinas entram **no PR #47** junto com as obras (um PR só).
8. 19:16 — Casa do jogador: **casa do fundador (Codex) com o Lar (`veio:home`, obras) dentro**; refeição/bônus de casa só pelo Lar; bônus de comer fora do Codex permanece.
9. 19:16 — Tempos de receita: **manter os da 0.25** (não a progressão por fases do Codex).
10. 19:14 — "Junte tudo" (integrar todas as branches no #47); 19:01 — integrar commits novos.
11. 20:09 — Prioridades atuais: realismo maior sem pesar o PC (benchmark: PC com 8 GB de RAM; "é Rust, dá para pôr muito mais"), e limite de tamanho do mapa + otimização para rodar no computador do jogador.
12. 16:27 — Afirmou: "não tem build consolidada ainda"; muito trabalho vindo em aldeões, vilas, sistemas e visualização. Ideia central: jogo de fábrica tipo Factorio/Satisfactory dentro do Minecraft.

Memória do projeto (escrita por agentes anteriores; tratar como fonte secundária):
- Centro de controle após obras+0.21: slots 0 comida, 1-3 metas, 4 cofre, 5 saque, 6 obra, 7 decreto. Status de canteiro 36-39; 32-35 conexões externas; frete/caravana 40-43; descanso 28.
- Guia principal limitado a 100 páginas (check_project); terminais de transporte só no livro Transportes; Guia da Vila com 29 páginas acima de 14 linhas (aviso).
- Após merges, regenerar `data.rs` com `tools/build_production.py` (auto-merge pegou tempos do Codex por engano uma vez).
- Horizon não lê worldgen por datapack; rios são traçados em tempo de execução sobre chunks novos (`chunk_load` new=true, `read_columns`, `edit_blocks`), estado em `<world>/veio/rios.txt`. APIs novas (chunk_load, world_dir, read_columns) **não verificadas** contra o Horizon real.
- Água é NBT própria "Water" (mB), não tanques do Horizon.
- `launcher.exe` Windows não pode ser compilado na nuvem.
- Versões colidem frequentemente entre frentes; o #40 foi renumerado 0.18→0.19→0.20→0.22→0.23 e no #47 tudo fica como 0.25.0.

## 5. IDs estáveis de sistemas (preliminares; os agentes podem propor fusão/divisão, justificando)

| ID | Sistema |
|---|---|
| SIS-PLT | Plataforma, build e ferramentas (Horizon, launcher, geradores Python, conteúdo, empacotamento, verificações) |
| SIS-MUN | Mundo: geração, jazidas/veios conectados, rios com nascente, limite/mapa finito, restrição do mapa (MAPA.md) |
| SIS-EXT | Extração e processamento de minério (mineradoras, jazidas, britador, fundição) |
| SIS-PRO | Produção: máquinas, receitas, combustíveis, módulos/melhorias, Mk.II, grande indústria |
| SIS-ENE | Energia: geradores, postes/fios, rede, renováveis, hidrelétrica, Tesla |
| SIS-AGU | Água encanada: bomba, cano, aspersor, lava nos canos |
| SIS-AGR | Agro e extrativismo: lavoura irrigada, colheitadeira, ranch/pastoreio, silvicultura, zonas de lavoura/cercado |
| SIS-LOG | Logística local: esteiras (expressa, túnel), braços/inserters, filtros, divisor, controles de fluxo |
| SIS-TRA | Transporte entre zonas: ferrovia (rail_terminal), drones, caravanas, estações de carga/descarga (produção virtual), elevador de carga |
| SIS-TRB | Aldeões trabalhadores: postos (lavrador, pastor, carregador), capataz e fila de tarefas, contratação, spawn, crescimento da vila, IA de caminhada |
| SIS-VIL | Sociedade da vila: casas/bairros, famílias, lojas e comerciantes, prefeitura/prefeito, poço/saneamento, fases da vila, IDH/Gini/PIB |
| SIS-ECO | Economia e contabilidade: moeda (esmeralda), contas companhia/pública, impostos na fonte, salários, holding e empresas, preços locais, identidade contábil |
| SIS-MER | Mercado e comércio exterior: lojas/preços locais (market.rs), porto e estação de carga (freight_station), bolsa mundial, contratos com prazo, reputação, importação de comida |
| SIS-OBR | Obras e plantas: canteiro, mestre de obras, prédios desenhados, prestígio, pedidos diários, decretos, boletim |
| SIS-LAR | Casa do fundador / Lar do jogador (lareira, despensa, refeições, bônus, mural) |
| SIS-CTL | Centro de controle executivo: painéis Resumo/Atenção/Obras, abas da holding, foco/bússola, cofre/saque |
| SIS-PRG | Progressão: avanços, fases e licenças, quadro da fábrica, metas, laboratório de pesquisa, central de contratos/projetos, projeto final |
| SIS-TUT | Tutorial na tela e guias (livros: principal, Vila Operária, Transportes, Grande Indústria) |
| SIS-DEF | Ameaças e defesa: poluição, saqueadores, ondas noturnas, mobs que caçam aldeões, quartel, Tesla |
| SIS-MEG | Megafábricas: copiadora/blueprint, marco de obra, prancheta, escadas entre andares |
| SIS-APR | Apresentação: painéis, texturas, silhuetas/formatos de máquinas, animação, sons, realismo leve/shaders |
| SIS-DES | Desempenho: orçamento global de ticks, buscas incrementais, censo, alocações, benchmarks |
| SIS-MP  | Multijogador LAN |

## 6. Critérios de evidência (obrigatórios)

Níveis de confiança de cada afirmação:
- **E1 – Verificado executando**: você rodou um teste/verificação nesta sessão e viu passar (cite o comando e o resultado).
- **E2 – Verificado no código**: você leu o código e ele faz isso (cite `arquivo:linha` e o ramo: `main@53267af` ou `pr47@80980db` ou outra branch@sha).
- **E3 – Declarado em documento/commit/PR**: só o texto afirma (cite arquivo ou PR).
- **E4 – Declarado no chat/memória**: só um agente/usuário disse.
- **ND – Não determinado**.

Nunca promova E3/E4 a "implementado". Teste offline passando ≠ validado no jogo: nada foi testado em partida real nem compilado contra o Horizon real (usar o estado "Implementado isoladamente" ou "Integrado" conforme o caso; "Validado" só com evidência explícita, e mesmo assim qualifique "validado offline").

Estados (usar exatamente): Concebido · Aprovado · Em desenvolvimento · Implementado isoladamente · Integrado · Validado · Bloqueado · Suspenso · Substituído · Descartado · Indeterminado.
Para cada item, dê o estado em **três camadas**: `main` (build lançada 0.25.0), `pr47` (candidata consolidada) e `disperso` (branches/PRs/zip fora das duas).

## 7. Regras

- Não alterar código nem branches. Não fazer push, merge, comentários no GitHub. Só escrever arquivos de rascunho em `SP/a/` (Opus A) ou `SP/b/` (Opus B).
- Não inventar decisões. Uma proposta em documento não é decisão aprovada; só o item 4 acima (e textos explícitos do Luiz) contam como decisão do dono. Decisões de agentes: rotular "decisão de agente (não confirmada pelo dono)".
- Sem texto genérico que caberia a qualquer jogo. Densidade alta, rastreável.
- Quando houver números de regra (custos, tempos, preços, limites), cite-os com a fonte; são o que torna a ficha útil.
