# 09 — Conflitos, lacunas, riscos e decisões pendentes

> Referência: `main@53267af` (0.25.0), `pr47@80980db` (candidata), `#51@a0b0d72`, `#52@0b827f6`. Data: 2026-11-08.
> Este arquivo lista **problemas conhecidos**, não soluções aprovadas. As mitigações e recomendações são de agentes. Nenhuma delas é decisão do dono até virar DEC em [06](06_HISTORICO_E_DECISOES.md).
> Níveis de evidência: E1 (executado), E2 (lido no código), E3 (documento), E4 (chat/memória), ND (não determinado).

## 1. Resumo em cinco frases

1. **A candidata (PR #47) nunca foi compilada contra o Horizon real nem jogada.** Também está mais pesada que a main: +36% por tick e 329 alocações por quadro, contra 0 na main. Isso contraria a meta de leveza em PC de 8 GB.
2. **A progressão pode não estar trancando nada.** Nenhum mundo liga `doLimitedCrafting`, e as licenças do Quadro anunciam liberações que, no pr47, vêm do laboratório.
3. **A "economia fechada" tem torneiras.** Contrato de exportação sem limite, contrato Povoado, grande projeto, pedido do dia do Quadro e armazém criam esmeraldas fora do livro. O simulador não as modela.
4. **A produção virtual das estações não custa nada.** Ela cria itens sem insumo nem energia, continua depois que o posto é desmontado e usa canais globais entre dimensões.
5. **O alvo de integração continua mudando.** Os PRs #51 (0.26.0) e #52 foram empilhados sobre o #47 antes da revisão, e versões colidem entre frentes desde a 0.16.

## 2. Conflitos (CNF)

Cada CNF fecha quando houver uma DEC (produto) ou uma TAR concluída (técnico). As TAR estão em [08](08_BACKLOG_E_PRIORIDADES.md).

| CNF | Conflito | Fontes em choque | Tipo | Canônico provisório (para **descrever** o jogo vale o código; para **direção** vale a última DEC do dono) | Encaminhamento |
|---|---|---|---|---|---|
| CNF-001 | Esmeraldas criadas fora dos livros × "economia fechada" | README 0.21 / DEC-030 (agente) × `contracts.rs:63-66`, `milestones/mod.rs:147-150`, `production/pantry.rs`, grande projeto (E2) | Produto | O código cria moeda nessas fontes. A DEC-035 (dono, 17:18) indica preferência pelo circuito fechado, mas cobre só os pedidos da vila e os juros | [Q-01](#q-01-esmeraldas-criadas-fora-dos-livros) |
| CNF-002 | Duas espinhas de progressão: fases e licenças do Quadro × laboratório de pesquisa | `milestones/mod.rs:37-52,246`, `frentes/PROGRESSAO-FASES.md` × `research.rs`, `research/*.json`, `frentes/GRANDE-INDUSTRIA.md` (E2) | Produto | No pr47 as liberações vêm de `research/*.json`. O texto "Libera…" do Quadro está errado | [Q-03](#q-03-espinha-da-progressão) |
| CNF-003 | Liberar receita por avanço sem `doLimitedCrafting` | grep vazio nos dois ramos (E2); comportamento do Horizon ND | Técnico, com efeito de produto | ND: pode ser só visual (some do livro de receitas) | TAR de teste no Horizon; depois Q-03 |
| CNF-004 | Dois "pedidos diários": encomenda do Quadro (cria 1–6 esmeraldas; em memória, repete ao reabrir) × pedidos da vila (pagos pelo caixa público, DEC-035) | `milestones/` × `workforce/requests.rs` (E2) | Produto | Os dois coexistem | [Q-02](#q-02-dois-pedidos-diários) |
| CNF-005 | Versões: duas 0.16.0; #40 renumerado 5 vezes; main e pr47 declaram 0.25.0 com conteúdo diferente; o zip 0.25.0 do pr47 é **o mesmo blob da main** (`e01ef740`); launcher do pr47 é 0.16.1; #51 já se declara 0.26.0 | git log (E2), `git rev-parse` dos zips (E1) | Técnico e produto | O pr47 **não tem pacote próprio** | [Q-08](#q-08-número-de-versão-e-quem-numera) |
| CNF-006 | README do pr47 contradiz o código | l.21 "rendem 15% mais" × sem bônus de casa (`home/mod.rs:101`, DEC-046); l.86 manda usar `Veio-0.16.1.zip` (inexistente); §Poluição "64/18 blocos" × `ARRIVAL 26.0` (`pollution.rs:47`); "duas prensas" × tempos da 0.25 (DEC-045); progressão por fases × pesquisa | Técnico | Vale o código | TAR de documentação |
| CNF-007 | PIB: guia diz "tarefas feitas e vendas" × código `DSold + DExport` | lang pt_br: `veio.village.page.33` `:1720` no pr47 (na main, `page.24` `:1033`) × `mayor.rs:187-188` (E2) | Técnico | PIB = varejo + exportação | TAR de texto |
| CNF-008 | Bônus de população das obras (Escola +15, Estalagem +20, prestígio +5/nível) × teto absoluto de 150 e limite por fase | `village.rs:121-129`, `projects.rs:440-458` (E2) | Produto | Numa vila madura o bônus pode não ter efeito | [Q-06](#q-06-teto-de-população-e-bônus-das-obras) |
| CNF-009 | "Fábrica distante continua produzindo" (dono, 17:01) × produção virtual gratuita; trem, drones e caravanas param fora da área simulada | DEC-041 × `stations.rs:117-140` (E2) | Produto | Só as estações cumprem a decisão, e sem custo | [Q-05](#q-05-produção-virtual-e-transporte-longo) |
| CNF-010 | Nomes cruzados: "Vila"/"Cidade" em duas escadas (fases da prefeitura × prestígio); "Prefeitura" é bloco e obra; "estações" (carga) × "estações do ano" (#51); "Prancheta" (`veio:clipboard`) × Copiadora | lang, `projects.rs`, #51 (E2) | Produto | — | [Q-04](#q-04-nomes-que-colidem) |
| CNF-011 | Itens da grande indústria (motor, processador etc.) fora da bolsa mundial (`trade.rs` PRODUCTS = 34) | E2 | Produto | Não são exportáveis | Q-01 / Q-03 |
| CNF-012 | Guia da Vila com 29 páginas acima de 14 linhas (contadas por idioma: 17 em pt_br e 12 em en_us) e duas páginas "8."; guia principal no teto de 100 páginas | `check_project` (E1, aviso) | Técnico | Aviso, não falha | TAR de texto |
| CNF-013 | Códigos de status 40/41 com dois sentidos (`center.rs:39-40` zona/armazém × `freight.rs:15-16` rota/animal) | E2 | Técnico | **Sem efeito hoje**: `waiting()` filtra. Risco latente | TAR: registro central de status ([04 §Registro de IDs](04_ARQUITETURA_E_DEPENDENCIAS.md)) |
| CNF-014 | `veio:freight_station`: no Codex 0.16.1 era o terminal ferroviário; na main e no pr47 é a estação de exportação (DEC-044, dono 17:41). Sem migração | `a79f326`, `trade.rs:27` (E2) | Técnico | Em mundos 0.16.1 e Realismo 1.1, o terminal vira estação de exportação e **exporta a carga** | TAR de nota de migração; a decisão do dono já cobriu o nome |
| CNF-015 | Imposto da fase cobrado duas vezes sobre o mesmo bem (varejo e atacado) × `ECONOMIA.md`, que calcula o teto "com o imposto" uma vez | `market.rs` `receive_sale`/`settle` (E2) × E3 | Produto | O código cobra duas vezes | [Q-13](#q-13-imposto-uma-ou-duas-vezes) |
| CNF-016 | Produtividade: 1 extra a cada 10 ciclos (main) × a cada 5 (pr47, c5db4f6, frente de balanço), sem decisão do dono | `upgrades.rs:35` (E2) | Produto | pr47 = 20% | [Q-14](#q-14-produtividade-10-ou-20) |
| CNF-017 | Bônus de comer fora de casa (×0,95/×0,91 no ritmo) é **global**; pesquisa e grande projeto concedem a `@a`; cofre único por dimensão | `home/food.rs:22-29,59-69`, `research.rs` (E2) | Produto | Em LAN, um jogador muda o estado de todos | [Q-15](#q-15-o-que-é-de-cada-jogador-em-lan) |
| CNF-018 | Pesquisa da vila (staff) × laboratório: chave `veio.research.done` com aridade diferente; 10 traduções de pesquisa da vila faltam (o jogador vê a chave crua) | `staff.rs:390-392`, lang (E2) | Técnico | — | TAR de texto |
| CNF-019 | `MAPA.md` registra como "decisão de produto autorizada pelo usuário" um Overworld de 15.000² com recursos garantidos; não há registro disso no chat de 08/11; garantia de recursos depende de API ausente no Horizon 66f3981 | E3 × E4 | Produto | Atribuída ao dono só pelo próprio documento (E3), sem registro do dono no chat; não confirmada até a Q-09 (mesma redação da DEC-006) | [Q-09](#q-09-mapa-finito) |
| CNF-020 | `frentes/DESEMPENHO.md` ainda traz, na seção da 0.17.1, "0 alocações por quadro". A seção foi herdada pelo pr47, #51 e #52, que medem 329. O #52 só acrescenta uma seção de callgrind (−7,7%) e não corrige a antiga | E1 (`bench_interleaved.log`) | Técnico | Vale a medição | TAR de bisseção de desempenho |
| CNF-021 | Visitantes `veio:visitor` e obras cívicas do #54 × funcionários das obras (FUN-OBR-11, #47/#53) | PR #54 (E3) × `workforce/staff.rs` (E2) | Produto | Ainda não determinado; o #54 não foi auditado | [TAR-065](08_BACKLOG_E_PRIORIDADES.md#tar-065) |
| CNF-022 | Ânimo e lazer no IDH (#54) × serviços das obras no IDH (TAR-017, Concebido) | PR #54 (E3) × `society/mod.rs:613` (E2) | Produto | Duas propostas para o mesmo indicador | [TAR-065](08_BACKLOG_E_PRIORIDADES.md#tar-065), [TAR-017](08_BACKLOG_E_PRIORIDADES.md#tar-017) |

Conflitos já **reconciliados** (com DEC do dono e código conferido) estão em [06 §4 Reconciliações](06_HISTORICO_E_DECISOES.md#4-reconciliações): casa do fundador × Lar (DEC-046), tempos 0.25 × fases do Codex (DEC-045), `rail_terminal` (DEC-044), âncora × estações (DEC-040/041), recompensas pagas pela prefeitura (DEC-035), sons e obras no mesmo PR (DEC-047).

## 3. Riscos técnicos priorizados

Severidade: **Crítica** (impede jogar ou lançar, ou corrompe save) · **Alta** (quebra sistema visível ou invalida a candidata) · **Média** · **Baixa**. A lista completa, com 43 riscos (R-01…R-43), está no [Apêndice A](#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43). Aqui estão os dez que mudam a ordem do trabalho.

| # | Risco | Evidência | Sev. | Sistema | Mitigação sugerida (agente) |
|---|---|---|---|---|---|
| 1 | R-01: pr47 nunca compilado contra o Horizon real até o corte. `read_columns` e `chunk_load` (rios) entraram depois da última compilação alegada (0.16.1, 4ffb605). **Após o corte:** a primeira tentativa real falhou e foi corrigida por `3f7cb28` (21:40); o risco se confirmou, e a recompilação é ND | E2 git log -S; E2 `3f7cb28` | **Crítica** | [SIS-PLT](03_atlas/SIS-PLT.md) | `horizon-launcher native-build` numa máquina com Horizon antes do merge |
| 2 | R-41: nada da 0.11 à 0.25 foi testado em partida | E3 README | **Crítica** | todos | Roteiro curto de teste em jogo (ver [08](08_BACKLOG_E_PRIORIDADES.md)) |
| 3 | R-21/CNF-003: gate de receitas possivelmente inócuo | E2 | Alta | [SIS-PRG](03_atlas/SIS-PRG.md) | Testar no Horizon; ligar `doLimitedCrafting` nos mundos do Veio se confirmado |
| 4 | R-20/CNF-002: licenças do Quadro não liberam o que prometem no pr47 | E2 | Alta | [SIS-PRG](03_atlas/SIS-PRG.md), [SIS-PES](03_atlas/SIS-PES.md) | Depende da Q-03 |
| 5 | R-14/15/16: regressões de desempenho (+36% por tick; 329 alocações por quadro; `bench_large_village` em pânico por `world_dir` no harness). #51 e #52 não resolvem | E1, 5 rodadas `--release` | Alta | [SIS-DES](03_atlas/SIS-DES.md) | Bisseção main↔pr47 (suspeitos: `appearance`, `motion`, `soundscape`, cadeia `tick_factory`); `world_dir` no harness |
| 6 | R-23/24: README e pacote apontam para a 0.16.1; launcher do pr47 é 0.16.1 | E2 | Alta | [SIS-PLT](03_atlas/SIS-PLT.md) | Pacote próprio e README corrigido antes do merge |
| 7 | R-30: menu do canteiro com 1 slot × código com 3; cerca de 25 traduções das obras faltando | E2; `HANDOFF_OBRAS.md` | Alta | [SIS-OBR](03_atlas/SIS-OBR.md) | Concluir pendências do handoff |
| 8 | R-08/09: arquivos em `<mundo>/veio/` sem versão; 3 deles gravados sem arquivo temporário; poluição, holding e relógio do mod se perdem ao reiniciar | E2 | Alta | [SIS-PER](03_atlas/SIS-PER.md) | Cabeçalho de versão + gravação atômica comum |
| 9 | R-02 + R-31: Horizon sem `rev` fixo; nenhuma CI | E2 | Alta | [SIS-PLT](03_atlas/SIS-PLT.md) | Fixar `rev`; rodar as três verificações em cada PR |
| 10 | R-42: trabalho novo empilhado sobre a candidata antes da revisão: #51 (0.26.0), #52, e após o corte #53 (obras), #54 (0.27.0, sobre o #51) e um zip de teste "0.26.0 combinado" (#47+#51+#52+#53) | E2 | Alta | processo | Congelar o #47; #51/#52 entram depois como frentes |

## 4. Lacunas

### 4.1 Lacunas de verificação (o que ninguém sabe)

| Lacuna | Por que importa | Como fechar |
|---|---|---|
| Nenhum teste em jogo (coluna J vazia para todas as 158 FUN) | "Integrado" é o teto honesto de qualquer funcionalidade hoje | Roteiro de teste em partida com registro J em [05](05_ESTADO_REAL_DA_BUILD.md) |
| Assinaturas e semântica do Horizon (`set_block_property` difere; `read_columns` em chunk recém-gerado; `edit_blocks` dentro de `chunk_load`; `forceload` via comando; `advancement grant @a`) | Rios, âncora, progressão e copiadora dependem disso | Compilação real e teste dirigido |
| Desempenho no jogo real (FPS, memória, 8 GB) | Meta explícita do dono (20:09) | Medição no Horizon com fábrica de referência |
| Sistemas novos fora de `check_performance` (rios, água, estações, som, obras, copiadora com até 6.000 blocos) | O orçamento global não os cobre | Novos cenários de benchmark |
| `sim_economy.py` replica constantes do Rust à mão e ignora as fontes da CNF-001 | O "ok" do simulador não cobre o jogo real | Simulador ler as constantes geradas; modelar as bordas |
| Commits locais nunca enviados (máquina do Luiz em `C:\Users\User\Desktop\clauduxu`, outros agentes) | Trabalho pode existir fora do GitHub | Cada agente roda `git status` e `git log --branches --not --remotes` antes de apagar pastas |

### 4.2 Lacunas de produto (o que ninguém decidiu)

- **Fim de jogo:** não existe ([Q-10](#q-10-fim-de-jogo)). O grande projeto em 4 fases termina com 2 estrelas do Nether e 2 faróis. A meta sustentada está só no README.
- **Peso da sociedade frente à fábrica** ([Q-12](#q-12-peso-da-sociedade-frente-à-fábrica)). Sociedade, contas, holding, economia fechada e comércio exterior foram construídos por agentes em cerca de 1h30, sem pedido explícito do dono.
- **Clima e estações do ano (#51):** efeito de jogo ou só visual? ([Q-07](#q-07-realismo-clima-e-estações-do-ano)).
- **Serviços das obras no IDH:** previsto em `SOCIEDADE.md`, mas em estado Concebido (`society/mod.rs:613` não referencia `projects`).
- **Pendências do `HANDOFF_OBRAS.md`:** ruínas e proteção de território continuam sem implementação.

## 5. Decisões pendentes do dono

Cada questão traz contexto, alternativas e a recomendação de agente. **Nenhuma é decisão.** Quando o dono responder, a resposta vira DEC em [06](06_HISTORICO_E_DECISOES.md), a CNF correspondente fecha e as TAR marcadas "aguarda Q-NN" em [08](08_BACKLOG_E_PRIORIDADES.md) são liberadas.

| Ordem sugerida | Q | Pergunta | Bloqueia | Recomendação (agente) |
|---|---|---|---|---|
| 1 | [Q-11](#q-11-condições-para-fundir-o-47) | Em que condições o #47 entra na main, e o que fazer com #51/#52? | Tudo | Compilar, recuperar o desempenho, pacote próprio, teste curto em jogo, congelar o #47 |
| 2 | [Q-03](#q-03-espinha-da-progressão) | Qual é a espinha da progressão: laboratório ou fases do Quadro? | O jogo de fábrica em si | Laboratório como espinha; fases do Quadro como marcos que abrem níveis de pesquisa |
| 3 | [Q-08](#q-08-número-de-versão-e-quem-numera) | Qual número de versão, e quem numera? | Pacote, launcher, saves | Só o integrador numera; #47 = 0.26.0; o #51 é renumerado ao entrar |
| 4 | [Q-01](#q-01-esmeraldas-criadas-fora-dos-livros) | O que fazer com as esmeraldas criadas fora dos livros? | Balanço, comércio | Híbrido: entrada externa registrada no início de jogo; circuito no meio e no fim |
| 5 | [Q-05](#q-05-produção-virtual-e-transporte-longo) | A produção virtual deve ter custo? | DEC-041, economia | Sim, com insumo e energia; passo intermediário com teto e invalidação ao quebrar |
| 6 | [Q-13](#q-13-imposto-uma-ou-duas-vezes) | Imposto uma ou duas vezes sobre o mesmo bem? | Balanço 0.25 | Só no varejo, recalibrando pelo simulador |
| 7 | [Q-02](#q-02-dois-pedidos-diários) | Manter dois pedidos diários? | Q-01, Q-03 | Fundir no pedido da vila, com persistência em disco |
| 8 | [Q-09](#q-09-mapa-finito) | Mapa finito: tamanho e garantias | Thread do mapa | Primeiro só o limite; confirmar o número 15.000 |
| 9 | [Q-07](#q-07-realismo-clima-e-estações-do-ano) | Realismo opcional ou padrão; clima com efeito? | Thread do realismo | Opcional, com critério numérico de FPS e memória; clima só visual na primeira entrega |
| 10 | [Q-15](#q-15-o-que-é-de-cada-jogador-em-lan) | O que é de cada jogador em LAN? | Multijogador | Misto: tecnologia e cofre compartilhados; bônus local |
| 11 | [Q-14](#q-14-produtividade-10-ou-20) | Produtividade 10% ou 20%? | Balanço | 20% (com 10% o módulo produz menos por minuto) |
| 12 | [Q-06](#q-06-teto-de-população-e-bônus-das-obras) | O que fazer com o teto de população e os bônus das obras? | Vila | Trocar o bônus de teto por efeito de serviço ou IDH |
| 13 | [Q-04](#q-04-nomes-que-colidem) | Como resolver os nomes que colidem? | Clareza | Renomear prestígio e obra "Prefeitura"; dizer sempre "estações do ano" |
| 14 | [Q-10](#q-10-fim-de-jogo) | Qual é o fim de jogo? | Conteúdo | Metas sustentadas, apoiadas no grande projeto |
| 15 | [Q-12](#q-12-peso-da-sociedade-frente-à-fábrica) | Qual o peso da sociedade frente à fábrica? | Direção | Congelar até o primeiro teste em jogo; decidir depois |

### Q-01. Esmeraldas criadas fora dos livros
**Fontes que criam moeda sem passar pelos livros** (E2):

| Fonte | Valor | Onde |
|---|---|---|
| Contrato EXPORT da central | 4 por remessa, sem limite, também na main | `contracts.rs:63-66` |
| Contrato Povoado | 8 | `contracts.rs:35-49` |
| Grande projeto | 32 / 48 / 64 por fase | E2 |
| Pedido do dia do Quadro | 1–6 | `milestones/mod.rs:147-150` |
| Armazém do povoado | 1 a cada 60 pontos de comida (+50% com 3 aldeões a 48 blocos) | `production/pantry.rs` |
| Mk.II sobre jazida de esmeralda | até 1 a cada 10 s (jazida pura) | `production.json:90` |

**Observação:** a esmeralda só entra no livro quando é depositada no cofre (`ledger.rs:192-200`).

**Alternativas:**
- **A.** Manter.
- **B.** Registrar tudo como entrada externa num livro "Exterior", com teto.
- **C.** Converter tudo ao circuito.
- **D.** Híbrido.

**Recomendação: D.** Dois motivos:
- A escolha das 17:18 indica preferência pelo circuito.
- O início de jogo precisa de uma porta de moeda: contratar custa 1 e as zonas custam 8–32.

### Q-02. Dois pedidos diários
**Alternativas:**
- **A.** Renomear os dois.
- **B.** Fundir no pedido da vila.
- **C.** Remover o do Quadro.

**Recomendação: B, com persistência.** O pedido da vila já segue a DEC-035. Hoje o do Quadro paga de novo quando o mundo é reaberto.

### Q-03. Espinha da progressão
**Sete mecanismos convivem no pr47:**
1. avanços;
2. fases e licenças do Quadro;
3. laboratório;
4. central de contratos;
5. grande projeto;
6. fases da prefeitura;
7. prestígio, reputação e níveis de zona.

Os mecanismos 2 e 3 foram feitos em paralelo por agentes diferentes para a mesma lacuna. O dono aprovou corrigir as lacunas (16:32) e manter os tempos da 0.25 (19:16), mas não escolheu entre eles.

**Alternativas:**
- **A.** Laboratório como espinha; fases do Quadro abrem níveis de pesquisa.
- **B.** Fases do Quadro como espinha; o laboratório vira ramo opcional.
- **C.** Manter os dois.

**Recomendação: A.** Nenhuma alternativa funciona sem resolver antes a CNF-003 (gate real).

### Q-04. Nomes que colidem
- "Vila" e "Cidade" aparecem nas fases da prefeitura e no prestígio.
- "Prefeitura" é bloco e obra.
- "estações" de carga colidem com "estações do ano".

**Recomendação de baixo custo:**
- renomear o prestígio, por exemplo para "Fama";
- renomear a obra, por exemplo para "Paço Municipal";
- usar sempre "estações do ano".

### Q-05. Produção virtual e transporte longo
**Situação atual:**
- A estação descarregada cria itens no ritmo medido, sem insumo nem energia.
- Isso continua mesmo depois que o posto é desmontado.
- Trem, drones e caravanas param fora da área simulada.

**Alternativas:**
- **A.** Com custo.
- **B.** Sem custo, mas com teto, decaimento e invalidação ao quebrar.
- **C.** Manter.

**Recomendação: A**, com B como passo intermediário. Caravanas e drones ficam documentados como de curta distância. A chave (dimensão, canal) é correção técnica, não questão de produto.

### Q-06. Teto de população e bônus das obras
**Regra atual:** `min(desenvolvimento + extra das obras, 150)`, mais o limite da fase (30/60/100/150) e a cama livre.

**Alternativas:**
- **A.** O bônus fura o limite da fase.
- **B.** O teto absoluto sobe acima de 150.
- **C.** Trocar o bônus por efeito de serviço ou IDH.

**Recomendação: C.** O 150 é um limite de desempenho medido.

### Q-07. Realismo, clima e estações do ano
**Situação:** o visual base é o estilo Minecraft. O Realismo leve 1.1 é opcional. O #51 traz Realismo 2.1, clima e estações do ano, e não foi auditado.

**Recomendação:**
- realismo opcional, promovido a padrão só com critério numérico (FPS e memória no PC de 8 GB);
- clima e estações do ano só visuais na primeira entrega.

### Q-08. Número de versão e quem numera
**Alternativas:**
- **A.** Só o integrador numera; #47 = 0.26.0; o #51 é renumerado.
- **B.** O #51 fica com 0.26.0 e o #47 vira 0.25.1.
- **C.** Fundir #47+#51+#52 numa 0.26.0.
- **D.** Cada frente sobe a versão.

**Recomendação: A.** É a única alternativa que impede a próxima colisão.

### Q-09. Mapa finito
**Alternativas:**
- **A.** 15.000² com recursos garantidos.
- **B.** Só o limite, com muralha e Nether ajustado, sem garantia de recursos.
- **C.** Outro tamanho.

**Recomendação: B primeiro**, e o dono confirma o número. A garantia de recursos depende de uma API (`native_chunk_generate`) que o Horizon 66f3981 não expõe. O rascunho do mapa usa mixins de motor (R-06) e está desligado.

### Q-10. Fim de jogo
**Alternativas:**
- **A.** O grande projeto como fim, com cena final.
- **B.** Metas sustentadas (por exemplo, 20 circuitos/min por 3 min).
- **C.** Sandbox.

**Recomendação: B, apoiado em A.**

### Q-11. Condições para fundir o #47
**Situação:** o #47 tem 79 commits e +93k linhas. Nunca foi compilado nem jogado, está mais lento que a main e não tem pacote próprio.

**Alternativas:**
- **A.** Fundir já.
- **B.** Fundir depois de uma lista de condições.
- **C.** Deixar aberto como integração contínua.

**Recomendação: B, nesta ordem:**
1. compilação real com o Horizon fixado;
2. desempenho perto da main (proposta: `bench_factory` até +10%, 0 alocações por quadro, `bench_large_village` rodando);
3. pacote 0.26.0 e README corrigido;
4. sessão curta em jogo;
5. congelar o #47 até o merge;
6. depois disso, fechar os PRs já absorvidos (#7, #19, #20, #21, #23, #25, #28, #30, #31, #40, #49, #50) com referência ao #47.

Também definir quem pode fazer merge na main.

### Q-12. Peso da sociedade frente à fábrica
**Alternativas:**
- **A.** Pilar no mesmo nível da fábrica.
- **B.** Camada de suporte simplificada.
- **C.** Congelar até o primeiro teste em jogo.

**Recomendação: C**, e depois decidir entre A e B com base no teste.

### Q-13. Imposto uma ou duas vezes
**Situação:** o imposto da fase (4–16%) é retido no varejo e de novo no atacado.

**Alternativas:**
- **A.** Só no varejo.
- **B.** Só no atacado.
- **C.** Manter os dois e recalibrar.

**Recomendação: A**, com o simulador lendo as constantes do Rust.

### Q-14. Produtividade 10% ou 20%
**Situação:** o módulo deixa o ciclo 15% mais lento e gasta 40% mais energia. Com 10%, a máquina produz cerca de 4% **menos** por minuto (1,10/1,15). Com 20%, produz cerca de 4% mais.

**Recomendação: 20%.**

### Q-15. O que é de cada jogador em LAN
**Situação:**
- O bônus de comer fora de casa vale para o mundo inteiro.
- Pesquisa e grande projeto valem para todos (`@a`).
- Lar e tutorial são por jogador.
- Há um cofre por dimensão.

**Alternativas:**
- **A.** Tudo compartilhado.
- **B.** Por jogador.
- **C.** Misto.

**Recomendação: C.** Tecnologia e cofre compartilhados; o bônus vale só perto de quem comeu.

## Apêndice A. Registro completo de riscos técnicos (R-01…R-43)

Fonte: auditoria do Opus A, revisada na Fase III. A severidade segue a escala da §3. As mitigações são sugestões de agente.

### A.1 APIs do Horizon e compilação

| ID | Risco | Evidência | Sev. | Mitigação sugerida |
|---|---|---|---|---|
| R-01 | Nenhuma compilação do pr47 contra o Horizon real; `read_columns` e `chunk_load` (rios) entraram depois da única compilação alegada (4ffb605, 0.16.1) | E2 git log -S (bcfb9d5); E3 CONSOLIDACAO; `check_native_api.py` sem rmeta | **Crítica** | `horizon-launcher native-build` numa máquina com Horizon antes de fundir o #47 |
| R-02 | Dependência `horizon-client` por `git` sem `rev` e sem Cargo.lock: cada build pega um Horizon diferente (66f3981 do stub × 5f7cadf atual) | E2 `native/api/Cargo.toml` | Alta | Fixar `rev` e registrar o hash testado no README |
| R-03 | Assinatura de `set_block_property` difere (real tem `&mut Ticks`, `flags: i32`) | E2 comparação stub × `native_*.rs` | Média (ND se o trait gerado adapta) | Conferir com `check_native_api.py` |
| R-04 | Semântica não verificada: `read_columns` em chunk recém-gerado, `edit_blocks` dentro de `chunk_load`, `run_command("forceload …")`, `advancement grant @a`, `place_block` da copiadora | E2 (uso) + ND (comportamento) | Alta | Teste em partida com roteiro curto |
| R-05 | Âncora usa comando `forceload` em vez da API de tickets do Horizon (`native_chunk_tickets.rs`) | E2 `industry.rs:285`; Horizon | Média | Avaliar migração para tickets (sem comando, sem permissão de operador) |
| R-06 | Mapa finito depende de mixins de motor (22/24 injeções), fora da API de mods; quebra a cada mudança do Horizon | E3 CONTINUAR.md | Alta (para a frente do mapa) | Manter desligado até haver API; preferir limite por borda do mundo |
| R-07 | Shader `api = "0.1"` aceito pelo manifesto do Horizon 5f7cadf, mas WGSL não validado; PR #51 já reescreve o pacote | E2 `manifest.rs:611-626` | Média | Rodar `horizon-shader check` (ou `tools/check_shaders.py` do #51) |

### A.2 Save e compatibilidade

| ID | Risco | Evidência | Sev. | Mitigação |
|---|---|---|---|---|
| R-08 | Arquivos `<mundo>/veio/*.txt` sem número de versão; `estacoes.txt`, `ancoras.txt`, `pesquisa.txt` gravados sem arquivo temporário (queda no meio = arquivo truncado) | E2 (`stations.rs:207`, `industry.rs:260`, `research.rs:45`; rios e tutorial usam temporário) | Alta | Cabeçalho de versão + gravação atômica comum (SIS-PER) |
| R-09 | Estado em memória perdido ao reiniciar: poluição, contas da holding, `BORDERS`, CONNECTIONS, PORTS dos drones, bônus de refeição, relógio do mod (`defense::CLOCK` começa em 0 a cada processo) | E2 `defense.rs:21-45`, `ledger.rs:48`, `home/food.rs:28-29` | Média | Persistir o que é indicador visível (holding, poluição) |
| R-10 | Reset entre mundos parcial: `server_about_to_start` (pr47) limpa só home, census, blueprint; os demais dependem de detectar `world_dir`; na main não há reset | E2 `lib.rs:89` (pr47) | Média | Um reset central de todos os `static` |
| R-11 | Save da main 0.25 aberto no pr47: todos os 55 blocos e 16 itens da main existem no pr47 (E2, conjunto contido); NBT novo ("Water", "Home") nasce vazio; mudança de liberação de receitas por avanço (steel → research) altera progresso de mundos existentes | E2 | Média | Nota de migração; teste de abrir um mundo 0.25 |
| R-12 | Rios só em chunks novos: mundo antigo nunca recebe rios perto da base | E2 `rivers.rs:758` (`new`) | Baixa | Documentar |
| R-13 | Canais de estação globais entre dimensões; produção virtual cria itens sem insumo/energia e continua mesmo se o posto foi desmontado enquanto descarregado | E2 `stations.rs:117-140` | Média (balanço/exploração) | Chave (dimensão, canal); cobrar energia virtual; invalidar ao quebrar |

### A.3 Desempenho e limites

| ID | Risco | Evidência | Sev. | Mitigação |
|---|---|---|---|---|
| R-14 | `bench_factory` +36 % por tick no pr47 (mediana 268,7 → 364,8 µs, 5 rodadas `--release` intercaladas) | E1 `bench_interleaved.log` | Alta (contraria a DEC-056: leveza em PC de 8 GB) | Perfilar `tick_factory` (cadeia de `if/else` de 13 sistemas por bloco) |
| R-15 | `bench_frame` 0 → 329 alocações por quadro, 41–49 → 59–82 µs | E1 X6; `frentes/DESEMPENHO.md` (0 alocações) | Alta | Bisseção entre main e pr47 (suspeitos: `appearance`, `motion`, `soundscape`) — o PR #52 mexe em som/mural |
| R-16 | `bench_large_village` quebrado no pr47 (panic `world_dir`): a escala de 150 aldeões deixou de ser medida | E1 X7 | Alta | Implementar `world_dir` no harness da vila |
| R-17 | Sistemas novos fora de `check_performance`: rios (48 passos/t, 2 edições de chunk/t), água, estações, som, obras, copiadora (6000 blocos) | E2 | Média | Novos cenários |
| R-18 | Guia principal no teto de 100 páginas (`check_project` falha na 101ª); `industry_guide` sem `check_page_fit`; 29 páginas (contadas por idioma: 17 em pt_br e 12 em en_us) do guia da vila acima de 14 linhas | E1 X1 | Média | Dividir livros; incluir todos no `check_page_fit` |
| R-19 | Cotas de `budget` fixas (`LIMITS`), sem teto global de tempo por tick; mapa sem limite (frente em andamento) | E2 `budget.rs:8`; E4 | Média | Orçamento por tempo medido |

### A.4 Inconsistências código × documento/decisão

| ID | Inconsistência | Evidência | Sev. |
|---|---|---|---|
| R-20 | Quadro anuncia que licenças liberam steam_engine/battery/geothermal, drill_mk2/solar/tesla, drill_mk3; no pr47 isso vem de `research/*.json` | E2 `milestones/mod.rs:37-52,246`; advancements | Alta (jogador recebe licença e nada libera) |
| R-21 | Bloqueio de receitas depende de avanços, mas nenhum `doLimitedCrafting` é definido → no vanilla as receitas continuam fabricáveis | E2 (grep vazio nos dois ramos); comportamento do Horizon ND | Alta (progressão inteira) |
| R-22 | README pr47 l.21: casa do fundador "rendem 15% mais" — código não dá bônus de casa (DEC-046) | E2 `home/mod.rs:101` | Média |
| R-23 | README pr47 l.86 manda usar `release/Veio-0.16.1.zip` (inexistente); `release/Veio-Realismo-1.1.zip` contém Veio 0.16.1 | E2 | Alta (jogador instala código antigo) |
| R-24 | `launcher.exe` do pr47 é o binário 0.16.1 do Codex; `check_project` não confere versão do launcher | E2 | Média |
| R-25 | `TRABALHADORES.md:234` estados do canteiro 32–35 (código 36–39); `:589` "26 estados" e `:131` "30 entradas" (código 44) | E2 | Baixa |
| R-26 | `CONSOLIDACAO.md` números da 0.16.1 (558 JSON, 68 texturas, 248 testes) e `frentes/PROGRESSAO-FASES.md` (tempos por fase) apresentados como estado atual | E2 × E1 | Baixa |
| R-27 | Produtividade 10 % → 20 % no pr47 (Grok/Jefter c5db4f6) sem decisão explícita do dono | E2 `upgrades.rs:35` | Média (balanço) |
| R-28 | Economia "fechada" com emissões fora do ledger: EXPORT ilimitado (main e pr47), contrato village, grande projeto, pedido do dia; `sim_economy.py` não as modela | E2 `contracts.rs:63-66`, `milestones/mod.rs:147-150` | Média |
| R-29 | Duas pesquisas (laboratório × vila) com chave `veio.research.done` de aridade diferente | E2 `staff.rs:391`, `research.rs` | Média (texto errado na tela) |
| R-30 | Menu do canteiro `slots: 1` × código 3 slots; ~25 traduções faltando das obras | E2 | Alta (obra sem insumo visível) |

### A.5 Dívidas técnicas

| ID | Dívida | Evidência | Sev. |
|---|---|---|---|
| R-31 | Sem CI: nenhuma das verificações roda automaticamente em PR | E2 | Alta |
| R-32 | `tick_factory` como cadeia de `if/else` por id de bloco (13 destinos no pr47) | E2 `lib.rs:1093` | Média |
| R-33 | Acoplamento cíclico `workforce ↔ presentation` (39/23) e `workforce ↔ holding` (20/25); `defense` é dono do relógio global | E2 contagem de referências | Média |
| R-34 | Códigos de status numéricos sem registro central (40/41 colidem entre `freight` e `center`) | E2 `freight.rs:15-16`, `center.rs:39-40` | Baixa |
| R-35 | Código morto (`sites_near`, `research_percent`, `distance`; import `Mutex` não usado na main) | E1 avisos | Baixa |
| R-36 | Testes de onboarding dependem de execução serial (`--test-threads=1`) | E1 X8 | Baixa |
| R-37 | Importações de patch triplicadas no histórico do pr47 (0ce91a4/0be045c/823d626 etc.) dificultam `bisect` e `blame` | E2 `git log` | Baixa |
| R-38 | `sim_economy.py` duplica constantes do Rust à mão | E2 | Média |
| R-39 | Versões colidem entre frentes (0.16.0, 0.16.1, 0.18–0.23, agora 0.26.0 no #51) | E2/REST | Média (rastreabilidade) |
| R-40 | `launcher.exe` versionado no git apesar de `package.py` dizer que é gerado | E2 | Baixa |

### A.6 Processo e escopo

| ID | Risco | Evidência | Sev. |
|---|---|---|---|
| R-41 | Nada da 0.11.0 à 0.25.0 foi testado em partida (README pr47 l.31) | E3 | **Crítica** para "build consolidada" |
| R-42 | Trabalho novo empilhado sobre o pr47 antes da revisão (#51 com 53 arquivos e versão 0.26.0; #52) | REST | Alta (o alvo do #47 continua a mudar) |
| R-43 | Bônus/pesquisa/grande projeto globais (`@a`, atomics) em LAN: um jogador muda o estado de todos | E2 | Baixa (coop) |
