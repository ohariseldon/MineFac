# 01 — Estado geral do projeto (síntese executiva)

> Retrato de 2026-11-08, ~21h30 UTC. Referências: `main@53267af` (Veio 0.25.0), `pr47@80980db` (candidata), `#51@a0b0d72`, `#52@0b827f6`.
> Evidência: E1 executado · E2 lido no código · E3 documento · E4 chat · ND não determinado · **J teste em jogo: nenhum até hoje**.

## 1. O que é o jogo, em um parágrafo

**Veio** é um jogo de automação industrial no estilo Factorio/Satisfactory que roda *dentro* do mundo procedural do Minecraft. Ele é escrito em Rust como mod nativo do **Horizon** (Minecraft em Rust) e vive no repositório **MineFac**, o canônico. Projeto-Moba e Wasému não fazem parte deste escopo.

O que o jogador faz:
- extrai das jazidas do mundo, gera e distribui energia por postes, processa em cadeias de máquinas e move itens por esteiras, braços, trens, drones e caravanas;
- contrata **aldeões** para operar postos, alimentar máquinas e cultivar;
- faz crescer uma **vila** com famílias, lojas, prefeito e obras, sustentada por uma economia de esmeraldas com contas separadas da companhia e da prefeitura;
- exporta para o mundo pelo porto e pela estação de fronteira (`veio:freight_station`), que não deve ser confundida com as estações de carga e descarga da fábrica distante.

A meta declarada pelo dono é que fábricas distantes continuem produzindo e que tudo rode leve num PC de 8 GB. Visão completa em [02](02_VISAO_E_CONCEPCAO_DO_JOGO.md).

## 2. As três camadas: não confundir

| Camada | O que é | Estado honesto | Verificação |
|---|---|---|---|
| **Build lançada**: `main` 0.25.0 | Fábrica, energia, logística, aldeões trabalhadores, sociedade da vila, economia fechada, comércio exterior, defesa, tutorial em livros | **Integrado**, não validado. Nada da 0.11 à 0.25 foi testado em partida (E3) | E1: `check_project` ok; `check_rust` 176 testes ok; `check_performance` 7 cenários ok |
| **Candidata consolidada**: PR #47 (`claude/project-thread-l94frq`) | Main 0.25 + obras e plantas, casa do fundador com o Lar, centro executivo, sons e formas das máquinas, caravanas, drones e ferrovias, grande indústria, água encanada, rios com nascente, estações de carga com produção virtual, laboratório, tutorial na tela, quadro da fábrica, megafábricas, lotes do Grok, rascunho desligado do mapa finito. São 79 commits, 575 arquivos e +93 mil linhas | **Integrado no código, nunca compilado contra o Horizon real, nunca jogado, mais pesado que a main e sem pacote próprio** | E1: `check_project` ok (aviso: 29 páginas longas (contadas por idioma: 17 em pt_br e 12 em en_us)); `check_rust` 325 ok; `check_performance` ok; **benchmarks: +36% por tick, 329 alocações por quadro (main: 0), `bench_large_village` em pânico** |
| **Desenvolvimento disperso** | #51 (Realismo 2.1, clima e estações do ano, 0.26.0) e #52 (desempenho: sons só perto do jogador, mural ocioso, medição de escala), ambos sobre o #47 e em andamento nas threads de realismo e de mapa. #49 e #50 (sons) já estão dentro do #47 | **Em desenvolvimento** | E1: nenhum dos dois resolve as regressões (#52 +35%, #51 +29%; 329 alocações) |

**Auditoria de perda de trabalho.** Nenhum código de branch, PR ou zip conhecido no GitHub ficou fora do #47 (E1/E2: `merge-base`, `cherry`, patch-id; detalhe em [10](10_EVIDENCIAS_E_RASTREABILIDADE.md)). Ficaram de fora só zips antigos (0.15.x). Commits locais que nunca foram enviados ao GitHub não são visíveis.

## 3. Progresso comprovado

- **Velocidade.** A main foi da 0.6 à 0.25 em cerca de 23 horas (7–8/11), em PRs de três frentes: Claude ("Cloud"), Codex e Grok. Cronologia em [06](06_HISTORICO_E_DECISOES.md).
- **Verificações offline.** Compilação contra um substituto da API do Horizon e testes de regra: 176 testes na main e 325 no pr47 passam (E1). Isso confere tipos e regras, **não** o comportamento no jogo.
- **Decisões do dono.** As decisões do chat de 08/11 que se traduzem em código foram conferidas e estão aplicadas no pr47 (E2). Exemplos:
  - recompensas pagas pela prefeitura (DEC-035);
  - `rail_terminal` (DEC-044);
  - casa do fundador com o Lar (DEC-046);
  - tempos de receita da 0.25 (DEC-045);
  - estações de carga e descarga para a fábrica distante (DEC-041), com uma ressalva: elas existem, mas a produção virtual não tem custo, e por isso o código só cumpre a decisão em parte (CNF-009).

  Registro completo em [06](06_HISTORICO_E_DECISOES.md).
- **Atlas.** 25 sistemas e 158 funcionalidades catalogados com estado por camada ([03](03_ATLAS_DE_SISTEMAS.md), [05](05_ESTADO_REAL_DA_BUILD.md)). Nenhuma funcionalidade está "Validada": falta o teste em jogo.

## 4. O que bloqueia

| # | Bloqueio | Por que importa | Onde |
|---|---|---|---|
| 1 | O #47 nunca foi compilado contra o Horizon real. Rios usam APIs (`read_columns`, `chunk_load`) que entraram depois da última compilação conhecida (0.16.1) | A candidata pode nem carregar | [R-01](09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43), [TAR-001](08_BACKLOG_E_PRIORIDADES.md) |
| 2 | Nenhum teste em partida desde a 0.11 | "Integrado" é o teto de tudo | R-41, TAR-003 |
| 3 | Regressão de desempenho no #47 | Contraria a meta de leveza (DEC-056) | [SIS-DES](03_atlas/SIS-DES.md), TAR-008/009/010 |
| 4 | O gate de receitas pode ser só visual (falta `doLimitedCrafting`), e as licenças do Quadro prometem liberações que vêm do laboratório | A progressão do jogo de fábrica | [SIS-PRG](03_atlas/SIS-PRG.md), CNF-002/003 |
| 5 | O pacote e o README do #47 apontam para a 0.16.1; o alvo muda (#51/#52 empilhados) | Instalação errada; integração sem fim | CNF-005, R-42 |

Também há problemas de economia e balanço que não impedem jogar, mas distorcem o jogo:
- esmeraldas criadas fora dos livros (CNF-001);
- produção virtual sem custo (CNF-009);
- imposto cobrado duas vezes (CNF-015).

Ver [09](09_CONFLITOS_LACUNAS_E_RISCOS.md).

## 5. Decisões que dependem do dono

As cinco primeiras da fila completa em [09 §5](09_CONFLITOS_LACUNAS_E_RISCOS.md#5-decisões-pendentes-do-dono). Todas têm alternativas e uma recomendação de agente:

1. **[Q-11](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-11-condições-para-fundir-o-47).** Em que condições o #47 entra na main? Recomendação: compilar, recuperar o desempenho, pacote próprio, teste curto e congelar o #47.
2. **[Q-03](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão).** Qual é a espinha da progressão: laboratório ou fases do Quadro? Recomendação: laboratório.
3. **[Q-08](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera).** Qual versão, e quem numera? Recomendação: só o integrador numera, e o #47 sai como 0.26.0.
4. **[Q-01](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros).** O que fazer com as fontes de esmeralda fora dos livros? Recomendação: híbrido.
5. **[Q-05](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo).** A produção virtual deve ter custo? Recomendação: sim.

## 6. Sequência recomendada de próximas ações

A lista justificada, com critérios de aceitação, está no topo de [08](08_BACKLOG_E_PRIORIDADES.md#0-próximas-10-ações-recomendadas). Em resumo, o ciclo atual é **tornar a candidata real antes de acrescentar conteúdo**:

1. congelar o #47 (TAR-011);
2. fixar o Horizon e compilar (TAR-002, TAR-001);
3. recuperar o desempenho (TAR-010, TAR-009, TAR-008);
4. consertar as telas e os textos que o teste exercita (TAR-006, TAR-020, TAR-015);
5. definir a versão e o pacote (TAR-040, TAR-004);
6. fazer o teste em partida com roteiro (TAR-003).

Os ciclos seguintes estão em [11](11_PLANO_DE_CONTINUIDADE.md#6-próximos-ciclos).

## 7. Quem está trabalhando agora

| Frente | Onde | Estado |
|---|---|---|
| Realismo sem pesar o PC | thread do projeto → PR #51 | Em desenvolvimento |
| Limite do mapa e otimização | thread do projeto → PR #52; rascunho `work-in-progress/finite-world/` | Em desenvolvimento |
| Consolidação mestra (este conjunto) | branch `claude/project-thread-zzxzah` | Entregue para revisão |
| Integração do #47 | thread do #47 (agora "Testar a build combinada no PC") | Em execução: corrigiu a primeira compilação real (`3f7cb28`); o merge aguarda a decisão do dono (Q-11) |

O mapa completo de agentes, branches e o fluxo proposto estão em [07](07_AGENTES_E_WORKFLOW.md).

## 8. O que mudou depois do corte (21:00–21:41 UTC)

O projeto continuou andando enquanto esta consolidação era escrita. Estes fatos foram registrados, mas **não auditados**:

| Fato | Evidência | Efeito |
|---|---|---|
| Houve uma primeira compilação real do #47 contra o Horizon 66f3981. Ela falhou, e o commit `3f7cb28` (21:40) corrigiu visibilidade (`pub(super)`→`pub(crate)`) e `Mob::on_climbable`, em que o substituto offline divergia | E2 `3f7cb28` | Confirma o R-01. Se a recompilação passou é ND ([TAR-001](08_BACKLOG_E_PRIORIDADES.md#tar-001)) |
| PR #53 (obras: traduções, menu do canteiro com 3 slots, ruínas, território), sobre o #47 | PR #53 `5c9ffdc` (E3) | Pode fechar TAR-006 e TAR-020 ([TAR-064](08_BACKLOG_E_PRIORIDADES.md#tar-064)) |
| PR #54 (0.27.0, agenda da vila: feriados, domingos, visitantes, obras cívicas, ânimo no IDH), sobre o #51 | PR #54 `2c49d41` (E3) | Nova colisão de versão; sobreposições CNF-021/022 ([TAR-065](08_BACKLOG_E_PRIORIDADES.md#tar-065)) |
| Um zip de teste "0.26.0 combinado" (#47+#51+#52+#53) foi montado para o PC do dono | [06 §3.6](06_HISTORICO_E_DECISOES.md#36-depois-dos-commits-de-referência-registro-preliminar-20382141) (E4) | Na prática antecipa a alternativa C da Q-08, sem decisão registrada |
| #51 e #52 avançaram (#51 agora "Realismo 3.0", `c57bc66`; #52 `639a16f`, esteiras inativas e energia em lotes) | git (E2) | As medições de desempenho da §2 valem para `a0b0d72`/`0b827f6`; os heads novos não foram medidos |
