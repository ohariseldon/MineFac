# B4: Agentes, frentes e workflow

Autor: OPUS B. Data: 2026-11-08.

Fontes:
- git log de todas as branches (autor, e-mail, fuso);
- COORDENACAO, CLOUD, CODEX, GROK, CHECKPOINT-INTEGRACAO e HANDOFF_OBRAS (E3);
- `list_threads` do projeto e chat de 08/11 (E4);
- CONTEXTO_FABLE.

Identidades que não consegui confirmar estão marcadas como ND.

---

## Parte 1. Mapa de agentes

### 1.1 Quem é quem

| Agente | Modelo / produto | Como aparece no git | Ambiente | Branches | Frente declarada | Situação hoje |
|---|---|---|---|---|---|---|
| **Luiz (dono)** | humano | GitHub `ohariseldon`. Merges assinados por "bizi" e "jefterpereira": **ND quem é quem** | Windows; `E:\GPTmodMine` (`.launcher`, `.game`, `.build`); Horizon Launcher; claude.ai Projects | — | Decide, testa (DEC-005) e faz merge | Ativo |
| **Fable (supervisor)** | Claude, modelo ND | — | claude.ai (esta missão) | — | Consolidação documental | Em execução |
| **OPUS A** | Claude Opus 5.5 | — | Container nuvem; `SP/a/` | — | Auditoria técnica do código | Em execução |
| **OPUS B** | Claude Opus 5.5 | — | Container nuvem; `SP/b/` | — | Produto, sistemas, governança | Em execução |
| **Claude "Cloud" (nuvem)** | Claude, modelo ND | Autor "Claude", +0000; sessão `01APXd4y…` | Container nuvem (não compila `.exe`) | `claude/trusting-ritchie-sc9ty8` (PRs #3–#48) | Trabalhadores, vila, sociedade, economia, desempenho 0.17 | **Histórica**: entregou 0.10–0.25 na main |
| **Claude local** | Claude, modelo ND | Autor "Claude", -0300; sessão `01KjyTj4…` | Máquina do dono | `main` antiga (0.7–0.9); `claude/ajustes-vila-mobs` (0.16.0 alternativa) | Visual, Tesla, postes, poluição, produção 0.9; casa do fundador; quartel | Histórica |
| **"Claude (MineFac)"** | Claude, modelo ND | Autor "Claude (MineFac)", outro e-mail de commit; sessão `01KjyTj6…` | ND (provável local) | `claude/progressao-fases` | Progressão por fases, Quadro da fábrica (EXPERIENCIA-JOGADOR, PROGRESSAO-FASES) | Histórica; os tempos foram revogados (DEC-045) |
| **Agente das obras (Cloud)** | Claude, modelo ND | Nenhum commit no GitHub; `cloud/obras-planta 4a096e9` só em zip | Nuvem; ficou sem tokens | `cloud/obras-planta` (local, nunca enviada) | Obras, plantas, Lar, prestígio, centro executivo | **Histórica com pendências** (HANDOFF_OBRAS) |
| **Thread "Criar agricultura real, água e energia"** | Claude, modelo ND | "Claude", +0000; sessão `01HkMwb4aD…` | Nuvem | `claude/project-thread-6tjsil` (PR #40) | Água, rios, grande indústria, estações, âncora, laboratório, monitor | Concluída |
| **Thread "Integrar build parcial"** | Claude, modelo ND | "Claude", +0000; sessão `01HkMwb4qX…` | Nuvem | `claude/project-thread-l94frq` (PR #47) | Integrador de fato desde 17:13 | Concluída; o #47 segue aberto |
| **Thread "Aumentar o realismo sem pesar o PC"** | Claude Opus 5.5 (extra) | — | Nuvem | ND | Realismo leve (DEC-056) | **Em execução** |
| **Thread "Definir limite do mapa e otimizar"** | Claude, modelo ND | — | Nuvem | ND | Limite do mapa, buscas em lotes, teto global de ticks | **Em execução** |
| **Codex local** | OpenAI Codex, modelo ND | Autor "Codex", codex@users.noreply, -0300 | Máquina do dono (`E:`); compila no Horizon (0.16.1 em 7 min 14 s) | `codex/expansion`, `codex/inserters`, `optimization`, `scaling`, `global-budget`, `incremental-*`, `consolidated-016` → `checkpoint-consolidacao`; mapa finito (`c8e94dc`) | Expansão, logística, integração, desempenho, tutorial na tela, megafábricas, mapa finito | **Interrompido a pedido do usuário** (CHECKPOINT) |
| **Codex nuvem** | OpenAI Codex, modelo ND | Autor "Jefter Pereira" / conta `jefterpereira` | Nuvem | `codex/interzone-logistics` (#28), `machine-identity` (#31), `machine-audio` (#49), ambiente (#50), jazidas (#23); docs de balanço da 0.15 | Transportes, formas, sons, pesquisa de jazidas | Histórica; PRs abertos com conteúdo já no #47 |
| **Grok** | xAI Grok via Cursor | Autor "Cursor Agent", +0000 | Cursor nuvem | `cursor/grok-*`, `grok/*` (lotes 1–8, noite, #19, #21, #25, #7, #30) | Desempenho e balanço | Histórica; PRs #7, #19, #20, #21, #25, #30 abertos |

**Observações** (todas ND, nenhuma inventada):
- O e-mail de commit de "bizi" sugere o dono. O de "Claude (MineFac)" é outra conta, com titular desconhecido.
- A conta `jefterpereira` aparece tanto como autora de commits do Codex nuvem quanto como quem fez os merges dos PRs do Cloud. O CONTEXTO a descreve como "conta usada por agentes".
- O balanço das missões e da economia da 0.15 (BALANCEAMENTO-MISSOES, BALANCEAMENTO-ECONOMIA) diz "continuação da frente Grok", mas os commits são de "Jefter Pereira" (c5db4f6, 3b3008d). A frente declarada e a autoria git não batem.

### 1.2 O que foi atribuído, entregue e o que ficou pendente

| Frente | Atribuído (fonte) | Entregue (camada) | Pendente |
|---|---|---|---|
| Cloud | Aldeões trabalhadores (CLOUD.md, "atribuída pelo usuário"); vila → cidade (VILA-CIDADE) | Lavrador → sociedade → economia 0.25 (main); plantas e obras (pr47 via zip) | Teste em jogo desde a 0.11; VILA-CIDADE (casas variadas, praça); serviços no IDH |
| Codex local | Expansão e integração; logística; restrição do mapa (CODEX.md) | Braços e filtros (main); otimizações #22–#38 (main); consolidação, tutorial, megafábricas (pr47) | Mapa finito (rascunho desligado; Nether falha); compilação final da candidata; launcher 0.16.1 |
| Grok | Desempenho e balanceamento (GROK.md) | Lotes 1 e 3–8 (main); balanço, cama livre, LAN (pr47) | Duas prensas: substituída; PRs abertos |
| Codex nuvem | ND (sem ordem escrita) | Transportes, formas, sons (pr47) | Rotas longas fora da simulação; PRs abertos |
| Claude (MineFac) | ND | Fases e Quadro (pr47) | Tempos revogados; docs desatualizados |
| Agente das obras | Obras e Lar (README "frente Cloud") | Pr47 (`f1452f5`) | Traduções, ruínas, proteção de território, menu do canteiro (HANDOFF) |
| Thread #40 | Agricultura, água, rios; lacunas (dono 15:55, 16:32) | Pr47 | Itens fora da bolsa; teste das APIs de chunk |
| Thread #47 | Integrar obras e todas as branches (dono 17:13, 17:51, 19:14) | Pr47 | Merge; versão; docs |
| Thread realismo | Realismo sem pesar (dono 20:09) | Em curso | — |
| Thread mapa | Limite e otimização (dono 20:09) | Em curso | — |

### 1.3 Duplicações e conflitos entre agentes

| # | O que colidiu | Agentes | Efeito | Resolução |
|---|---|---|---|---|
| 1 | Progressão contra o "um item" | Claude (MineFac): fases e licenças × Thread #40: laboratório | Dois sistemas de gate; liberações deslocadas | **Aberta** (B6-Q3) |
| 2 | Casa do jogador | Claude local: casa do fundador × Agente das obras: Lar | Dois bônus de casa | Dono 19:16 (DEC-046) |
| 3 | ID `veio:freight_station` | Codex nuvem: terminal × Cloud: comércio | Colisão de bloco | Dono 17:41 (DEC-044) |
| 4 | Tempos de receita | Cloud 30t × Claude (MineFac) 40t × Grok 80t | Auto-merge pegou tempos errados uma vez (memória) | Dono 19:16 (DEC-045) |
| 5 | Pedido diário | Quadro (Codex/MineFac) × Obras | Duas recompensas | **Aberta** (B6-Q2) |
| 6 | Pesquisa | Laboratório (#40) × pesquisas da vila (obras) | Mesmo nome, regras diferentes | **Aberta** |
| 7 | Integrador | Codex local (COORDENACAO) × Cloud ("build consolidada" 0.15) × Claude local (0.16.0 alternativa) × Thread #47 | Três consolidações paralelas; duas 0.16.0 | De fato: Thread #47. Codex interrompido pelo usuário (DEC-049) |
| 8 | Versões | Todos | 0.16.0 ×2; #40 renumerado 5 vezes; 0.25.0 ×2; Codex 0.15.1/0.15.3 | **Aberta** (B6-Q8) |
| 9 | Códigos de status e slots | Obras × Cloud × Codex nuvem | Canteiro 32→33→36; descanso 28/29; 40/41 com dois sentidos | Parcial (DEC-052); 40/41 a confirmar |
| 10 | Emissão de moeda | Cloud (economia fechada) × Codex/Claude (Quadro, central) × #40 (grande projeto) | Princípio violado | **Aberta** (B6-Q1) |
| 11 | Documentação de coordenação | COORDENACAO parada na 0.9.1 | Agentes novos leem regras velhas | Proposta em B5 |

### 1.4 Em execução, só atribuída ou histórica

| Situação | Agentes / frentes |
|---|---|
| **Em execução agora** | Fable, OPUS A, OPUS B; thread realismo; thread mapa |
| **Só atribuída (sem entrega)** | VILA-CIDADE (casas variadas, praça, vida); meta de produção sustentada (README "Direção de jogo"); subproduto e fluido do Nether (doc de lacunas); jazidas em terreno novo (PR #23) |
| **Interrompida** | Codex local (consolidação e mapa finito); agente das obras (sem tokens) |
| **Histórica (entregue)** | Cloud (0.10–0.25); Claude local (0.7–0.9, casa); Claude (MineFac); Codex nuvem; Grok; threads #40 e #47 |

---

## Parte 2. Workflow consolidado (1 dono, vários agentes)

Princípio: **menos papéis e um único caminho até a main**. O dono continua decidindo pelo chat (cartões de decisão). Tudo o mais é rastreável por ID (B5).

### 2.1 Papéis

| Papel | Quem | Faz | Não faz |
|---|---|---|---|
| **Dono** | Luiz | Aprova DEC; responde a B6; **é o único que faz merge na main** (recomendação, B6-Q11); faz ou delega o teste em jogo | Resolver conflito de código |
| **Integrador único** | Uma thread por vez, com nome fixo no REGISTRO | Mantém a branch `integracao/<versão>`; resolve conflitos técnicos; regenera `data.rs`; **sobe a versão**; atualiza o REGISTRO de IDs e o CHANGELOG; abre o PR para a main | Criar funcionalidade; decidir produto |
| **Frente** | Um agente por conjunto de SIS | Trabalha em `frente/<sis>-<tema>`; entrega com DoD "Implementado isoladamente"; reserva IDs antes de codar | Subir versão; mexer em SIS de outra frente sem TAR combinada |
| **Auditor** | Opcional, uma thread | Revisão cruzada E1/E2; confirma afirmações das fichas | Editar código |

### 2.2 Ciclo de vida e Definition of Done por estado

| Estado | Quem promove | Evidência mínima (DoD) |
|---|---|---|
| Concebido | Qualquer um | Uma FUN escrita na ficha SIS, com origem |
| **Aprovado** | **Só o dono** | DEC com texto do dono (E4 com hora) ou cartão respondido |
| Em desenvolvimento | Frente | Branch criada; TAR aberta com FUN e DEC |
| **Implementado isoladamente** | Frente | **E2** (arquivo:linha) + **E1** na branch (`check_project`, `check_rust`, `check_performance` passando, com o comando e a saída colados no PR) + ficha FUN atualizada + IDs reservados no REGISTRO |
| **Integrado** | Integrador | E1 na branch de integração, depois de regenerar `data.rs` e o conteúdo; nenhuma colisão no REGISTRO; guia dentro dos limites (100 páginas, 14 linhas); docs sem afirmação contrária ao código (lista de checagem do B5). **[B7] Para ir à main, exige também:** `native-build` contra o Horizon numa revisão fixa (registrada); benchmarks sem regressão acima do limite combinado; pacote próprio com a mesma versão no launcher e no conteúdo |
| **Validado** | Dono ou testador designado | **Teste em partida registrado**: versão, compilado pelo launcher, roteiro, resultado e prints. "Validado offline" **não existe**: teste offline nunca promove a Validado |
| Bloqueado / Suspenso | Integrador ou dono | Motivo e TAR que desbloqueia |
| Substituído / Descartado | Dono, ou integrador com DEC | DEC que substitui; nunca apagar o doc (B5) |

**E3 e E4 nunca promovem.** Um "feito" em doc ou chat sem E1/E2 mantém o estado anterior.

### 2.3 Registro de slots, status e IDs (antes de codar)

Arquivo único: `docs/consolidacao-mestra/REGISTRO_IDS.md`. Esquema detalhado em B5. A frente **reserva** faixas antes de implementar e o integrador **confirma** na integração.

| Recurso | Fonte da verdade no código | Regra |
|---|---|---|
| Códigos de status do painel | `presentation.rs:19` (`STATUS_KEYS`, 44 entradas) | Faixas por SIS: 0–17 máquinas; 18–25 postos; 26–28 zonas e descanso; 29–31 casas; 32–35 conexões externas; 36–39 canteiro; 40–43 frete. **Códigos de pendência do centro (40/41) precisam de faixa própria ou de outro tipo** |
| Slots de blocos com painel | Por bloco (ex.: centro 0–7) | Tabela por bloco; mudar exige DEC de agente registrada |
| IDs de bloco e item `veio:*` | `content/` | Nenhum ID reaproveitado com outro sentido (lição do `freight_station`) |
| Chaves NBT e formatos | Por módulo (ex.: `Trade` v3) | Versão do formato no REGISTRO |
| Arquivos em `<mundo>/veio/` | `rios.txt`, `pesquisa.txt`, `estacoes.txt`, `ancoras.txt`, `tutorial/<UUID>.txt` | Nome e dono (SIS) registrados |
| Avanços e licenças | `content/.../advancements` | Gate e fonte (avanço, pesquisa, licença) |
| Prefixos de tradução | `veio.<sis>.*` | Um prefixo por SIS |

### 2.4 Política de versão

Recomendação para o dono decidir (B6-Q8):
1. **Só o integrador sobe a versão**, uma vez por integração que vai para a main.
2. As frentes **não** mudam `version` nos manifestos nem no launcher. O pacote de teste de uma frente usa o sufixo `-<frente>.<n>` (ex.: `0.26.0-obras.3`), que nunca vai para `release/`.
3. Mudou `native/`, sobe o MINOR. Só conteúdo ou docs, sobe o PATCH.
4. O launcher e o conteúdo **sempre** com o mesmo número (hoje o pr47 tem 0.16.1 × 0.25.0).
5. `release/Veio-X.zip` só é gerado pelo integrador e nunca é reescrito com outro conteúdo.
6. Primeira aplicação: o #47 vira **0.26.0**.

### 2.5 Handoff (modelo obrigatório ao terminar ou interromper)

Arquivo `HANDOFF-<frente>.md` na branch, no máximo 60 linhas:

```
Frente / agente / sessão:
Branch e commit final (enviado? sim/não; se não, onde está o zip/bundle):
Base (commit da main ou integração):
FUN entregues (ID, estado, evidência E1/E2):
DEC tomadas pelo agente (texto, motivo) — aguardam dono? sim/não:
IDs reservados/usados (status, slots, blocos, NBT, arquivos):
Verificações rodadas (comando + resultado):
Não feito / pendente (TAR):
Riscos e conflitos conhecidos com outras frentes:
O que NÃO foi testado (sempre: jogo real, Horizon real):
```

Lições que este modelo cobre:
- a branch das obras nunca foi enviada;
- a COORDENACAO parou na 0.9.1;
- o Codex foi interrompido com um checkpoint bom (CHECKPOINT-INTEGRACAO serve de modelo).

### 2.6 Onboarding de um agente novo

Ordem de leitura, no máximo 30 min:
1. `docs/consolidacao-mestra/README.md` (índice e estado);
2. Visão (B1);
3. A ficha SIS da frente (B2);
4. DECs vigentes da SIS (B3);
5. `REGISTRO_IDS.md`;
6. `CONTRIBUTING.md`;
7. O handoff anterior da frente.

Regras de entrada:
- Partir da branch de integração vigente, não da main antiga (a `main` local estava em 0.17.2 quando a remota era 0.25).
- Rodar as três verificações antes de mexer.
- Não ler os docs com status "histórico" como regra (B5).
- Perguntar ao dono, por cartão, só o que está no B6 ou for decisão de produto. Conflito técnico vai para o integrador.

### 2.7 Cadência sugerida

| Evento | Quando | Quem |
|---|---|---|
| Integração | Ao fim de cada entrega de frente ou, no máximo, 1 por dia | Integrador |
| Teste em jogo curto (roteiro de 30 min) | Antes de cada merge na main | Dono / testador |
| Revisão do registro de decisões | A cada integração | Integrador propõe, dono confirma |
| Revisão do B6 | Quando a fila tiver ≥ 3 itens | Dono |
