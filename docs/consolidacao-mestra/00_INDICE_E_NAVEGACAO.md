# 00 — Índice e navegação da consolidação mestra

> **Ponto central de entrada** do projeto Veio (repositório MineFac). Retrato de 2026-11-08, verificado contra `main@53267af` (0.25.0) e `pr47@80980db` (candidata), com o que veio depois do corte registrado à parte.
> Produzido pela missão de consolidação: um supervisor e dois agentes de investigação (auditoria técnica e produto/governança) com revisão cruzada. **Nenhum código foi alterado.**

## Comece por aqui

| Se você quer… | Leia |
|---|---|
| Entender o projeto em 5 minutos | [01 Estado geral](01_ESTADO_GERAL_DO_PROJETO.md) |
| Saber o que o jogo pretende ser | [02 Visão e concepção](02_VISAO_E_CONCEPCAO_DO_JOGO.md) |
| Entender um sistema a fundo | [03 Atlas](03_ATLAS_DE_SISTEMAS.md) → ficha em `03_atlas/` |
| Saber como o código está organizado, ou reservar um número de status, slot ou bloco | [04 Arquitetura e Registro de IDs](04_ARQUITETURA_E_DEPENDENCIAS.md#10-registro-de-ids) |
| Saber se algo funciona de verdade | [05 Estado real da build](05_ESTADO_REAL_DA_BUILD.md) |
| Saber por que algo é como é | [06 Histórico e decisões](06_HISTORICO_E_DECISOES.md) |
| Saber quem faz o quê e como trabalhar | [07 Agentes e workflow](07_AGENTES_E_WORKFLOW.md) |
| Pegar a próxima tarefa | [08 Backlog](08_BACKLOG_E_PRIORIDADES.md#0-próximas-10-ações-recomendadas) |
| Ver os problemas conhecidos e o que depende do dono | [09 Conflitos, lacunas e riscos](09_CONFLITOS_LACUNAS_E_RISCOS.md) |
| Checar uma fonte, um commit, uma branch ou o status de um documento antigo | [10 Evidências](10_EVIDENCIAS_E_RASTREABILIDADE.md) |
| Retomar o trabalho ou manter esta pasta atualizada | [11 Continuidade](11_PLANO_DE_CONTINUIDADE.md) |

## Os documentos

| # | Documento | Conteúdo | Autor principal |
|---|---|---|---|
| 00 | Este índice | Navegação, perguntas do mandato, mapa por sistema e por tema | Supervisor |
| 01 | [Estado geral](01_ESTADO_GERAL_DO_PROJETO.md) | Síntese executiva, três camadas, bloqueios, decisões pendentes, próximas ações | Supervisor |
| 02 | [Visão e concepção](02_VISAO_E_CONCEPCAO_DO_JOGO.md) | Pilares, identidade, experiência do minuto 0 ao fim, loops, restrições, quem definiu cada parte | Opus B |
| 03 | [Atlas de sistemas](03_ATLAS_DE_SISTEMAS.md) + [`03_atlas/`](03_atlas/) | 25 fichas A–G; mapa de dependências; sistemas estruturantes e sobrepostos | Opus B, com a seção D do Opus A |
| 04 | [Arquitetura e dependências](04_ARQUITETURA_E_DEPENDENCIAS.md) | Fronteira com o Horizon, módulos, fluxo do tick, persistência, pipeline de conteúdo, launcher, **Registro de IDs** | Opus A |
| 05 | [Estado real da build](05_ESTADO_REAL_DA_BUILD.md) | Matriz de 158 FUN × 3 camadas com evidência; visão transversal; resultados das verificações e dos benchmarks | Opus A |
| 06 | [Histórico e decisões](06_HISTORICO_E_DECISOES.md) | Cronologia 0.6 → 0.25 e linhas paralelas; DEC-001…066; reconciliações; o que foi substituído ou abandonado | Opus B |
| 07 | [Agentes e workflow](07_AGENTES_E_WORKFLOW.md) | Quem é quem, o que roda agora, conflitos entre agentes, fluxo consolidado, DoD, handoff, onboarding | Opus B |
| 08 | [Backlog e prioridades](08_BACKLOG_E_PRIORIDADES.md) | TAR-001…065 em 6 categorias, com caminho crítico do #47 e as 10 próximas ações | Opus A |
| 09 | [Conflitos, lacunas e riscos](09_CONFLITOS_LACUNAS_E_RISCOS.md) | CNF-001…022, os riscos R-01…R-43, as lacunas e as questões Q-01…Q-15 ao dono | Supervisor |
| 10 | [Evidências e rastreabilidade](10_EVIDENCIAS_E_RASTREABILIDADE.md) | Índice de fontes, cobertura, confiabilidade, genealogia de 42+ branches e PRs, status dos documentos legados | Opus A |
| 11 | [Plano de continuidade](11_PLANO_DE_CONTINUIDADE.md) | Leituras de retomada, regras de conflito, IDs, gatilhos de atualização, próximos ciclos, limites | Supervisor |

## Respostas às perguntas do mandato

| Pergunta | Resposta curta | Onde está a resposta completa |
|---|---|---|
| O que é o jogo e qual a visão integral? | Automação industrial estilo Factorio/Satisfactory dentro do mundo do Minecraft, em Rust sobre o Horizon, com aldeões, vila e economia fechada; leve num PC de 8 GB | [02](02_VISAO_E_CONCEPCAO_DO_JOGO.md) |
| Quais sistemas existem e como funcionam? | 25 sistemas, 158 funcionalidades | [03](03_ATLAS_DE_SISTEMAS.md) |
| O que está implementado, parcial ou planejado? | Main 0.25 integrada; candidata #47 integrada no código, sem compilação real nem teste em jogo; nada validado | [05](05_ESTADO_REAL_DA_BUILD.md), [01 §2](01_ESTADO_GERAL_DO_PROJETO.md#2-as-três-camadas-não-confundir) |
| Como os sistemas interagem? | Mapa Mermaid e tabela de dependências; sistemas estruturantes | [03 §2](03_ATLAS_DE_SISTEMAS.md), [04 §4](04_ARQUITETURA_E_DEPENDENCIAS.md#4-dependências-entre-módulos) |
| Quais decisões foram tomadas, modificadas ou revogadas? | DEC-001…066, separando dono e agente | [06 §3](06_HISTORICO_E_DECISOES.md#3-registro-de-decisões), [06 §5](06_HISTORICO_E_DECISOES.md#5-funcionalidades-substituídas-e-abandonadas) |
| O que está em desenvolvimento e por quem? | Realismo (#51), mapa e desempenho (#52), obras (#53), agenda da vila (#54) | [07 §2](07_AGENTES_E_WORKFLOW.md#2-o-que-roda-agora-0811-2141-utc), [01 §7–8](01_ESTADO_GERAL_DO_PROJETO.md#7-quem-está-trabalhando-agora) |
| O que foi concluído e não integrado? | Nada de código ficou fora do #47; o próprio #47 não está na main | [10 §4.5](10_EVIDENCIAS_E_RASTREABILIDADE.md#45-verificação-nenhum-código-ficou-fora-do-47), [05 §2.6](05_ESTADO_REAL_DA_BUILD.md#26-trabalho-em-branch-fora-da-main) |
| Quais as lacunas, contradições e dependências? | 22 conflitos, 43 riscos, 15 questões ao dono | [09](09_CONFLITOS_LACUNAS_E_RISCOS.md) |
| Qual o estado real da build consolidada? | Não existe build consolidada validada; a candidata está mais pesada que a main e sem pacote próprio | [01 §4](01_ESTADO_GERAL_DO_PROJETO.md#4-o-que-bloqueia), [05 §3](05_ESTADO_REAL_DA_BUILD.md#3-resultados-e1) |
| Como organizar o desenvolvimento daqui em diante? | Tornar a candidata real (compilar, desempenho, pacote, teste) antes de conteúdo novo; só o integrador numera versões | [08](08_BACKLOG_E_PRIORIDADES.md), [07 §4](07_AGENTES_E_WORKFLOW.md#4-fluxo-consolidado-1-dono-vários-agentes), [11](11_PLANO_DE_CONTINUIDADE.md) |

## Mapa por sistema

Cada linha leva às seções da ficha: A conceito, B regras, D implementação, E histórico, F pendências, G estado. Também aponta a matriz da build e as tarefas do backlog.

| Sistema | Conceito | Regras | Implementação | Histórico | Pendências | Estado | Matriz | Tarefas |
|---|---|---|---|---|---|---|---|---|
| **SIS-PLT** Plataforma, build e ferramentas | [A](03_atlas/SIS-PLT.md#a-identidade) | [B](03_atlas/SIS-PLT.md#b-funcionamento) | [D](03_atlas/SIS-PLT.md#d-implementação) | [E](03_atlas/SIS-PLT.md#e-histórico) | [F](03_atlas/SIS-PLT.md#f-pendências) | [G](03_atlas/SIS-PLT.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#11-sis-plt-plataforma-build-e-ferramentas-11) | [001](08_BACKLOG_E_PRIORIDADES.md#tar-001), [002](08_BACKLOG_E_PRIORIDADES.md#tar-002), [004](08_BACKLOG_E_PRIORIDADES.md#tar-004), [010](08_BACKLOG_E_PRIORIDADES.md#tar-010), [015](08_BACKLOG_E_PRIORIDADES.md#tar-015), [019](08_BACKLOG_E_PRIORIDADES.md#tar-019), [036](08_BACKLOG_E_PRIORIDADES.md#tar-036), [040](08_BACKLOG_E_PRIORIDADES.md#tar-040), [050](08_BACKLOG_E_PRIORIDADES.md#tar-050), [051](08_BACKLOG_E_PRIORIDADES.md#tar-051), [054](08_BACKLOG_E_PRIORIDADES.md#tar-054) |
| **SIS-MUN** Mundo | [A](03_atlas/SIS-MUN.md#a-identidade) | [B](03_atlas/SIS-MUN.md#b-funcionamento) | [D](03_atlas/SIS-MUN.md#d-implementação) | [E](03_atlas/SIS-MUN.md#e-histórico) | [F](03_atlas/SIS-MUN.md#f-pendências) | [G](03_atlas/SIS-MUN.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#12-sis-mun-mundo-6) | [024](08_BACKLOG_E_PRIORIDADES.md#tar-024), [025](08_BACKLOG_E_PRIORIDADES.md#tar-025), [058](08_BACKLOG_E_PRIORIDADES.md#tar-058) |
| **SIS-EXT** Extração e processamento | [A](03_atlas/SIS-EXT.md#a-identidade) | [B](03_atlas/SIS-EXT.md#b-funcionamento) | [D](03_atlas/SIS-EXT.md#d-implementação) | [E](03_atlas/SIS-EXT.md#e-histórico) | [F](03_atlas/SIS-EXT.md#f-pendências) | [G](03_atlas/SIS-EXT.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#13-sis-ext-extração-e-processamento-4) | [059](08_BACKLOG_E_PRIORIDADES.md#tar-059) |
| **SIS-PRO** Produção | [A](03_atlas/SIS-PRO.md#a-identidade) | [B](03_atlas/SIS-PRO.md#b-funcionamento) | [D](03_atlas/SIS-PRO.md#d-implementação) | [E](03_atlas/SIS-PRO.md#e-histórico) | [F](03_atlas/SIS-PRO.md#f-pendências) | [G](03_atlas/SIS-PRO.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#14-sis-pro-produção-7) | [045](08_BACKLOG_E_PRIORIDADES.md#tar-045), [056](08_BACKLOG_E_PRIORIDADES.md#tar-056), [059](08_BACKLOG_E_PRIORIDADES.md#tar-059) |
| **SIS-ENE** Energia | [A](03_atlas/SIS-ENE.md#a-identidade) | [B](03_atlas/SIS-ENE.md#b-funcionamento) | [D](03_atlas/SIS-ENE.md#d-implementação) | [E](03_atlas/SIS-ENE.md#e-histórico) | [F](03_atlas/SIS-ENE.md#f-pendências) | [G](03_atlas/SIS-ENE.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#15-sis-ene-energia-5) | [049](08_BACKLOG_E_PRIORIDADES.md#tar-049) |
| **SIS-AGU** Água encanada | [A](03_atlas/SIS-AGU.md#a-identidade) | [B](03_atlas/SIS-AGU.md#b-funcionamento) | [D](03_atlas/SIS-AGU.md#d-implementação) | [E](03_atlas/SIS-AGU.md#e-histórico) | [F](03_atlas/SIS-AGU.md#f-pendências) | [G](03_atlas/SIS-AGU.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#16-sis-agu-água-encanada-4) | [026](08_BACKLOG_E_PRIORIDADES.md#tar-026), [031](08_BACKLOG_E_PRIORIDADES.md#tar-031) |
| **SIS-AGR** Agro e extrativismo | [A](03_atlas/SIS-AGR.md#a-identidade) | [B](03_atlas/SIS-AGR.md#b-funcionamento) | [D](03_atlas/SIS-AGR.md#d-implementação) | [E](03_atlas/SIS-AGR.md#e-histórico) | [F](03_atlas/SIS-AGR.md#f-pendências) | [G](03_atlas/SIS-AGR.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#17-sis-agr-agro-e-extrativismo-7) | [031](08_BACKLOG_E_PRIORIDADES.md#tar-031), [063](08_BACKLOG_E_PRIORIDADES.md#tar-063) |
| **SIS-LOG** Logística local | [A](03_atlas/SIS-LOG.md#a-identidade) | [B](03_atlas/SIS-LOG.md#b-funcionamento) | [D](03_atlas/SIS-LOG.md#d-implementação) | [E](03_atlas/SIS-LOG.md#e-histórico) | [F](03_atlas/SIS-LOG.md#f-pendências) | [G](03_atlas/SIS-LOG.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#18-sis-log-logística-local-7) | [062](08_BACKLOG_E_PRIORIDADES.md#tar-062) |
| **SIS-TRA** Transporte entre zonas e produção virtual | [A](03_atlas/SIS-TRA.md#a-identidade) | [B](03_atlas/SIS-TRA.md#b-funcionamento) | [D](03_atlas/SIS-TRA.md#d-implementação) | [E](03_atlas/SIS-TRA.md#e-histórico) | [F](03_atlas/SIS-TRA.md#f-pendências) | [G](03_atlas/SIS-TRA.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#19-sis-tra-transporte-entre-zonas-7) | [007](08_BACKLOG_E_PRIORIDADES.md#tar-007), [027](08_BACKLOG_E_PRIORIDADES.md#tar-027), [029](08_BACKLOG_E_PRIORIDADES.md#tar-029), [044](08_BACKLOG_E_PRIORIDADES.md#tar-044) |
| **SIS-TRB** Aldeões trabalhadores | [A](03_atlas/SIS-TRB.md#a-identidade) | [B](03_atlas/SIS-TRB.md#b-funcionamento) | [D](03_atlas/SIS-TRB.md#d-implementação) | [E](03_atlas/SIS-TRB.md#e-histórico) | [F](03_atlas/SIS-TRB.md#f-pendências) | [G](03_atlas/SIS-TRB.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#110-sis-trb-aldeões-trabalhadores-14) | [010](08_BACKLOG_E_PRIORIDADES.md#tar-010), [046](08_BACKLOG_E_PRIORIDADES.md#tar-046) |
| **SIS-VIL** Sociedade da vila | [A](03_atlas/SIS-VIL.md#a-identidade) | [B](03_atlas/SIS-VIL.md#b-funcionamento) | [D](03_atlas/SIS-VIL.md#d-implementação) | [E](03_atlas/SIS-VIL.md#e-histórico) | [F](03_atlas/SIS-VIL.md#f-pendências) | [G](03_atlas/SIS-VIL.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#111-sis-vil-sociedade-6) | [017](08_BACKLOG_E_PRIORIDADES.md#tar-017), [047](08_BACKLOG_E_PRIORIDADES.md#tar-047), [048](08_BACKLOG_E_PRIORIDADES.md#tar-048), [060](08_BACKLOG_E_PRIORIDADES.md#tar-060), [065](08_BACKLOG_E_PRIORIDADES.md#tar-065) |
| **SIS-ECO** Economia e contabilidade | [A](03_atlas/SIS-ECO.md#a-identidade) | [B](03_atlas/SIS-ECO.md#b-funcionamento) | [D](03_atlas/SIS-ECO.md#d-implementação) | [E](03_atlas/SIS-ECO.md#e-histórico) | [F](03_atlas/SIS-ECO.md#f-pendências) | [G](03_atlas/SIS-ECO.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#112-sis-eco-economia-e-contabilidade-9) | [029](08_BACKLOG_E_PRIORIDADES.md#tar-029), [035](08_BACKLOG_E_PRIORIDADES.md#tar-035), [036](08_BACKLOG_E_PRIORIDADES.md#tar-036), [042](08_BACKLOG_E_PRIORIDADES.md#tar-042) |
| **SIS-MER** Mercado e comércio exterior | [A](03_atlas/SIS-MER.md#a-identidade) | [B](03_atlas/SIS-MER.md#b-funcionamento) | [D](03_atlas/SIS-MER.md#d-implementação) | [E](03_atlas/SIS-MER.md#e-histórico) | [F](03_atlas/SIS-MER.md#f-pendências) | [G](03_atlas/SIS-MER.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#113-sis-mer-mercado-e-comércio-exterior-8) | [007](08_BACKLOG_E_PRIORIDADES.md#tar-007), [048](08_BACKLOG_E_PRIORIDADES.md#tar-048), [056](08_BACKLOG_E_PRIORIDADES.md#tar-056), [057](08_BACKLOG_E_PRIORIDADES.md#tar-057) |
| **SIS-OBR** Obras e plantas | [A](03_atlas/SIS-OBR.md#a-identidade) | [B](03_atlas/SIS-OBR.md#b-funcionamento) | [D](03_atlas/SIS-OBR.md#d-implementação) | [E](03_atlas/SIS-OBR.md#e-histórico) | [F](03_atlas/SIS-OBR.md#f-pendências) | [G](03_atlas/SIS-OBR.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#114-sis-obr-obras-e-plantas-10) | [006](08_BACKLOG_E_PRIORIDADES.md#tar-006), [017](08_BACKLOG_E_PRIORIDADES.md#tar-017), [020](08_BACKLOG_E_PRIORIDADES.md#tar-020), [021](08_BACKLOG_E_PRIORIDADES.md#tar-021), [022](08_BACKLOG_E_PRIORIDADES.md#tar-022), [028](08_BACKLOG_E_PRIORIDADES.md#tar-028), [046](08_BACKLOG_E_PRIORIDADES.md#tar-046), [047](08_BACKLOG_E_PRIORIDADES.md#tar-047), [064](08_BACKLOG_E_PRIORIDADES.md#tar-064), [065](08_BACKLOG_E_PRIORIDADES.md#tar-065) |
| **SIS-LAR** Casa do fundador e Lar | [A](03_atlas/SIS-LAR.md#a-identidade) | [B](03_atlas/SIS-LAR.md#b-funcionamento) | [D](03_atlas/SIS-LAR.md#d-implementação) | [E](03_atlas/SIS-LAR.md#e-histórico) | [F](03_atlas/SIS-LAR.md#f-pendências) | [G](03_atlas/SIS-LAR.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#115-sis-lar-casa-do-fundador-e-lar-4) | — |
| **SIS-CTL** Centro de controle | [A](03_atlas/SIS-CTL.md#a-identidade) | [B](03_atlas/SIS-CTL.md#b-funcionamento) | [D](03_atlas/SIS-CTL.md#d-implementação) | [E](03_atlas/SIS-CTL.md#e-histórico) | [F](03_atlas/SIS-CTL.md#f-pendências) | [G](03_atlas/SIS-CTL.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#116-sis-ctl-centro-de-controle-4) | [034](08_BACKLOG_E_PRIORIDADES.md#tar-034), [061](08_BACKLOG_E_PRIORIDADES.md#tar-061) |
| **SIS-PRG** Progressão | [A](03_atlas/SIS-PRG.md#a-identidade) | [B](03_atlas/SIS-PRG.md#b-funcionamento) | [D](03_atlas/SIS-PRG.md#d-implementação) | [E](03_atlas/SIS-PRG.md#e-histórico) | [F](03_atlas/SIS-PRG.md#f-pendências) | [G](03_atlas/SIS-PRG.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#117-sis-prg-progressão-8) | [005](08_BACKLOG_E_PRIORIDADES.md#tar-005), [016](08_BACKLOG_E_PRIORIDADES.md#tar-016), [028](08_BACKLOG_E_PRIORIDADES.md#tar-028), [043](08_BACKLOG_E_PRIORIDADES.md#tar-043) |
| **SIS-PES** Pesquisa | [A](03_atlas/SIS-PES.md#a-identidade) | [B](03_atlas/SIS-PES.md#b-funcionamento) | [D](03_atlas/SIS-PES.md#d-implementação) | [E](03_atlas/SIS-PES.md#e-histórico) | [F](03_atlas/SIS-PES.md#f-pendências) | [G](03_atlas/SIS-PES.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#118-sis-pes-pesquisa-2) | [016](08_BACKLOG_E_PRIORIDADES.md#tar-016), [020](08_BACKLOG_E_PRIORIDADES.md#tar-020), [047](08_BACKLOG_E_PRIORIDADES.md#tar-047) |
| **SIS-TUT** Tutorial e guias | [A](03_atlas/SIS-TUT.md#a-identidade) | [B](03_atlas/SIS-TUT.md#b-funcionamento) | [D](03_atlas/SIS-TUT.md#d-implementação) | [E](03_atlas/SIS-TUT.md#e-histórico) | [F](03_atlas/SIS-TUT.md#f-pendências) | [G](03_atlas/SIS-TUT.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#119-sis-tut-tutorial-e-guias-6) | [015](08_BACKLOG_E_PRIORIDADES.md#tar-015), [023](08_BACKLOG_E_PRIORIDADES.md#tar-023), [030](08_BACKLOG_E_PRIORIDADES.md#tar-030), [048](08_BACKLOG_E_PRIORIDADES.md#tar-048), [052](08_BACKLOG_E_PRIORIDADES.md#tar-052) |
| **SIS-DEF** Ameaças e defesa | [A](03_atlas/SIS-DEF.md#a-identidade) | [B](03_atlas/SIS-DEF.md#b-funcionamento) | [D](03_atlas/SIS-DEF.md#d-implementação) | [E](03_atlas/SIS-DEF.md#e-histórico) | [F](03_atlas/SIS-DEF.md#f-pendências) | [G](03_atlas/SIS-DEF.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#120-sis-def-ameaças-e-defesa-6) | [029](08_BACKLOG_E_PRIORIDADES.md#tar-029) |
| **SIS-MEG** Megafábricas | [A](03_atlas/SIS-MEG.md#a-identidade) | [B](03_atlas/SIS-MEG.md#b-funcionamento) | [D](03_atlas/SIS-MEG.md#d-implementação) | [E](03_atlas/SIS-MEG.md#e-histórico) | [F](03_atlas/SIS-MEG.md#f-pendências) | [G](03_atlas/SIS-MEG.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#121-sis-meg-megafábricas-3) | [029](08_BACKLOG_E_PRIORIDADES.md#tar-029), [047](08_BACKLOG_E_PRIORIDADES.md#tar-047) |
| **SIS-APR** Apresentação | [A](03_atlas/SIS-APR.md#a-identidade) | [B](03_atlas/SIS-APR.md#b-funcionamento) | [D](03_atlas/SIS-APR.md#d-implementação) | [E](03_atlas/SIS-APR.md#e-histórico) | [F](03_atlas/SIS-APR.md#f-pendências) | [G](03_atlas/SIS-APR.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#122-sis-apr-apresentação-9) | [009](08_BACKLOG_E_PRIORIDADES.md#tar-009), [013](08_BACKLOG_E_PRIORIDADES.md#tar-013), [014](08_BACKLOG_E_PRIORIDADES.md#tar-014), [034](08_BACKLOG_E_PRIORIDADES.md#tar-034), [047](08_BACKLOG_E_PRIORIDADES.md#tar-047), [055](08_BACKLOG_E_PRIORIDADES.md#tar-055), [063](08_BACKLOG_E_PRIORIDADES.md#tar-063) |
| **SIS-DES** Desempenho | [A](03_atlas/SIS-DES.md#a-identidade) | [B](03_atlas/SIS-DES.md#b-funcionamento) | [D](03_atlas/SIS-DES.md#d-implementação) | [E](03_atlas/SIS-DES.md#e-histórico) | [F](03_atlas/SIS-DES.md#f-pendências) | [G](03_atlas/SIS-DES.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#123-sis-des-desempenho-5) | [008](08_BACKLOG_E_PRIORIDADES.md#tar-008), [009](08_BACKLOG_E_PRIORIDADES.md#tar-009), [013](08_BACKLOG_E_PRIORIDADES.md#tar-013), [025](08_BACKLOG_E_PRIORIDADES.md#tar-025), [037](08_BACKLOG_E_PRIORIDADES.md#tar-037), [038](08_BACKLOG_E_PRIORIDADES.md#tar-038), [039](08_BACKLOG_E_PRIORIDADES.md#tar-039) |
| **SIS-MP** Multijogador LAN | [A](03_atlas/SIS-MP.md#a-identidade) | [B](03_atlas/SIS-MP.md#b-funcionamento) | [D](03_atlas/SIS-MP.md#d-implementação) | [E](03_atlas/SIS-MP.md#e-histórico) | [F](03_atlas/SIS-MP.md#f-pendências) | [G](03_atlas/SIS-MP.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#124-sis-mp-multijogador-lan-2) | [041](08_BACKLOG_E_PRIORIDADES.md#tar-041) |
| **SIS-PER** Persistência e ciclo de vida do mundo | [A](03_atlas/SIS-PER.md#a-identidade) | [B](03_atlas/SIS-PER.md#b-funcionamento) | [D](03_atlas/SIS-PER.md#d-implementação) | [E](03_atlas/SIS-PER.md#e-histórico) | [F](03_atlas/SIS-PER.md#f-pendências) | [G](03_atlas/SIS-PER.md#g-classificação) | [05](05_ESTADO_REAL_DA_BUILD.md#125-sis-per-persistência-4) | [007](08_BACKLOG_E_PRIORIDADES.md#tar-007), [018](08_BACKLOG_E_PRIORIDADES.md#tar-018), [029](08_BACKLOG_E_PRIORIDADES.md#tar-029), [032](08_BACKLOG_E_PRIORIDADES.md#tar-032), [033](08_BACKLOG_E_PRIORIDADES.md#tar-033) |

## Mapa por tema

Atalhos para os nomes que aparecem nas conversas.

| Tema | Sistema principal | Também |
|---|---|---|
| Prefeitura, prefeito, fases Povoado → Metrópole, IDH, Gini, PIB | [SIS-VIL](03_atlas/SIS-VIL.md) | [SIS-ECO](03_atlas/SIS-ECO.md) (caixa público, impostos), [SIS-OBR](03_atlas/SIS-OBR.md) (obra "Prefeitura"), Q-04 |
| Mercado, lojas, preços locais, atacado | [SIS-MER](03_atlas/SIS-MER.md) | [SIS-ECO](03_atlas/SIS-ECO.md), CNF-015 (imposto duplo) |
| Comércio exterior: porto, estação de fronteira (`freight_station`), bolsa, contratos, reputação, importação | [SIS-MER](03_atlas/SIS-MER.md) | [SIS-PRG](03_atlas/SIS-PRG.md) (central de contratos), CNF-001, CNF-011 |
| Economia fechada, esmeraldas, contas da companhia e pública, holding | [SIS-ECO](03_atlas/SIS-ECO.md) | [SIS-CTL](03_atlas/SIS-CTL.md) (abas da holding), Q-01 |
| Aldeões ("audiões"): postos, capataz, carregadores, contratação, crescimento | [SIS-TRB](03_atlas/SIS-TRB.md) | [SIS-VIL](03_atlas/SIS-VIL.md), [SIS-DES](03_atlas/SIS-DES.md) (censo, buscas incrementais) |
| Fábricas: máquinas, receitas, módulos, Mk.II, grande indústria | [SIS-PRO](03_atlas/SIS-PRO.md) | [SIS-EXT](03_atlas/SIS-EXT.md), [SIS-PES](03_atlas/SIS-PES.md), Q-14 |
| Fábrica distante, estações de carga e descarga, âncora | [SIS-TRA](03_atlas/SIS-TRA.md) | DEC-040/041, Q-05, CNF-009 |
| Trens, drones, caravanas, elevador | [SIS-TRA](03_atlas/SIS-TRA.md) | [SIS-MEG](03_atlas/SIS-MEG.md) |
| Esteiras, braços, filtros, divisor, túnel, prancheta | [SIS-LOG](03_atlas/SIS-LOG.md) | — |
| Energia, postes, hidrelétrica, Tesla | [SIS-ENE](03_atlas/SIS-ENE.md) | [SIS-AGU](03_atlas/SIS-AGU.md) |
| Água encanada, bomba, cano, aspersor, lava | [SIS-AGU](03_atlas/SIS-AGU.md) | [SIS-AGR](03_atlas/SIS-AGR.md) |
| Agricultura, irrigação, pastoreio, silvicultura, zonas | [SIS-AGR](03_atlas/SIS-AGR.md) | [SIS-TRB](03_atlas/SIS-TRB.md) |
| Rios com nascente, jazidas conectadas, mapa finito | [SIS-MUN](03_atlas/SIS-MUN.md) | Q-09, CNF-019 |
| Casa do fundador, Lar, refeições, banquete | [SIS-LAR](03_atlas/SIS-LAR.md) | DEC-046, CNF-017 |
| Obras, canteiro, plantas, prestígio, pedidos, decretos | [SIS-OBR](03_atlas/SIS-OBR.md) | [TAR-064](08_BACKLOG_E_PRIORIDADES.md#tar-064) (#53), CNF-004 |
| Centro de controle, foco, bússola, cofre | [SIS-CTL](03_atlas/SIS-CTL.md) | [04 §10.3](04_ARQUITETURA_E_DEPENDENCIAS.md#103-slots-do-centro-de-controle-veiocontrol_center) |
| Progressão, avanços, quadro da fábrica, licenças, grande projeto | [SIS-PRG](03_atlas/SIS-PRG.md) | [SIS-PES](03_atlas/SIS-PES.md), Q-03, CNF-002/003 |
| Laboratório e pesquisa da vila | [SIS-PES](03_atlas/SIS-PES.md) | CNF-018 |
| Tutorial na tela e livros-guia | [SIS-TUT](03_atlas/SIS-TUT.md) | CNF-012 |
| Poluição, saqueadores, noite, quartel | [SIS-DEF](03_atlas/SIS-DEF.md) | — |
| Copiadora, marco de obra, escadas entre andares | [SIS-MEG](03_atlas/SIS-MEG.md) | — |
| Visual, formas das máquinas, sons, realismo, clima e estações do ano | [SIS-APR](03_atlas/SIS-APR.md) | Q-07, #51 |
| Desempenho, orçamento de ticks, 8 GB, benchmarks | [SIS-DES](03_atlas/SIS-DES.md) | [05 §3.2](05_ESTADO_REAL_DA_BUILD.md#32-benchmarks-nativeoffline-check---release), #52 |
| Save, arquivos do mundo, troca de mundo | [SIS-PER](03_atlas/SIS-PER.md) | [04 §6](04_ARQUITETURA_E_DEPENDENCIAS.md#6-persistência) |
| Jogar na LAN | [SIS-MP](03_atlas/SIS-MP.md) | Q-15 |
| Launcher, pacote, Horizon, verificações, versões | [SIS-PLT](03_atlas/SIS-PLT.md) | Q-08, CNF-005 |
| Teste em jogo (roteiro, quem testa, o que conta como "Validado") | [TAR-003](08_BACKLOG_E_PRIORIDADES.md#tar-003) | R-41. O roteiro ainda não existe: escrevê-lo é o aceite da TAR-003 |
| Próxima tarefa, em qualquer sistema | [08 §0](08_BACKLOG_E_PRIORIDADES.md#0-próximas-10-ações-recomendadas) | A coluna "Tarefas" do [Mapa por sistema](#mapa-por-sistema) lista os IDs; estado e prioridade ficam no 08 |
| Arquivo de código → sistema (ex.: de quem é `stations.rs`) | [04 §3](04_ARQUITETURA_E_DEPENDENCIAS.md#3-módulos-pr47), coluna SIS | Linha "Arquivos" do §D de cada ficha |

**Três "estações" que não são a mesma coisa:**
- a **estação de fronteira** `veio:freight_station` é comércio exterior ([SIS-MER](03_atlas/SIS-MER.md));
- as **estações de carga e descarga** (`production/stations`) ligam a fábrica distante por produção virtual ([SIS-TRA](03_atlas/SIS-TRA.md), CNF-009);
- o **terminal ferroviário** `veio:rail_terminal` é a parada dos trens ([SIS-TRA](03_atlas/SIS-TRA.md), DEC-044).

Já as **estações do ano** são do clima do #51 ([SIS-APR](03_atlas/SIS-APR.md)).

## Panorama de cobertura

**Examinado, por agente:**
- todas as 42 branches remotas e os PRs #1–#54 (os #51–#54 só por inventário);
- o código Rust da main e do pr47 (~32 mil linhas no pr47);
- os geradores Python e o conteúdo;
- os ~50 documentos `.md`;
- as verificações e os benchmarks, executados nesta sessão;
- o chat e as threads do projeto de 08/11;
- a memória do projeto.

**Não examinado ou inacessível:**
- compilação contra o Horizon real e partidas jogadas;
- commits locais nunca enviados;
- a máquina do dono;
- as descrições completas dos PRs (foram usados os commits);
- os PRs #51–#54 em profundidade;
- os repositórios Projeto-Moba e Wasému, que não são este jogo.

Detalhe em [10 §2](10_EVIDENCIAS_E_RASTREABILIDADE.md#2-cobertura).

## Preparar o ambiente

O resumo abaixo vem do [CONTRIBUTING.md](../../CONTRIBUTING.md) e do [04 §8–§9](04_ARQUITETURA_E_DEPENDENCIAS.md#8-launcher-e-pacote). Em caso de divergência, vale o CONTRIBUTING.

| Para | O que precisa | Como |
|---|---|---|
| Conferir conteúdo (JSON, traduções, guias, texturas, receitas) | Python 3 (testado com 3.13 nesta sessão) | `python3 tools/check_project.py` |
| Compilar e testar as regras sem o jogo | Rust 1.90 ou superior (CONTRIBUTING); crate em edição 2024 (`native/api/Cargo.toml`). Funciona sem internet | `python3 tools/check_rust.py`. Compila `native/api/src` contra o substituto `native/offline-check/horizon-client`, **não** contra o Horizon |
| Medir desempenho | Igual ao anterior | `python3 tools/check_performance.py` (7 cenários); benchmarks `bench_*` com `--release -- --ignored` ([04 §9](04_ARQUITETURA_E_DEPENDENCIAS.md#9-testes-e-verificações-arquitetura)) |
| Compilar de verdade e jogar | Windows, Horizon Launcher instalado com conta e Prepare feitos, instância **Veio** | `launcher.exe` da raiz, ou `horizon-launcher.exe native-build … --instance Veio --trust-native-code --profile dev` (CONTRIBUTING). Compilação a frio: cerca de 20 min (E4) |

Três cuidados:
- **Horizon sem versão fixa.** `native/api/Cargo.toml` aponta para o Git do Horizon sem `rev`. A referência conhecida é `66f3981` (CONTRIBUTING); fixá-la é a [TAR-002](08_BACKLOG_E_PRIORIDADES.md#tar-002).
- **O substituto pode mentir.** Ele já divergiu do Horizon real (`Mob::on_climbable`, visibilidade; corrigido em `3f7cb28`, [01 §8](01_ESTADO_GERAL_DO_PROJETO.md#8-o-que-mudou-depois-do-corte-21002141-utc)). Ao usar uma API nova, copie a assinatura do fonte real do Horizon para o substituto.
- **Passar nos testes offline não é funcionar no jogo.** É evidência E1 de tipos e regras, nunca J.

## Glossário

| Termo | Significado |
|---|---|
| Tick | 1/20 de segundo de jogo. Um dia tem 24.000 ticks |
| Turno | 1.200 ticks; 20 turnos por dia. Salário e assistência são pagos por turno ([SIS-ECO](03_atlas/SIS-ECO.md)) |
| FE, FE/t | Unidade de energia do Veio e consumo por tick ([SIS-ENE](03_atlas/SIS-ENE.md)) |
| mB | Milibalde: 1.000 mB = 1 balde de água ([SIS-AGU](03_atlas/SIS-AGU.md)) |
| Moeda | 60 moedas = 1 esmeralda ([SIS-ECO](03_atlas/SIS-ECO.md), FUN-ECO-01) |
| Company, Public, Shop | Os três livros da economia fechada: o caixa da companhia (que é também o "caixa da vila" das obras), o caixa da prefeitura e o caixa de cada loja ([SIS-ECO](03_atlas/SIS-ECO.md)) |
| Divisão, aba da holding | Conta da companhia por ramo: divisões 1–9 mais `FOREIGN` 10, mostradas em 11 abas ([04 §3](04_ARQUITETURA_E_DEPENDENCIAS.md#3-módulos-pr47), [SIS-CTL](03_atlas/SIS-CTL.md)) |
| Povoado, Vila, Cidade… | Fases da vila; a primeira não tem prefeito ([SIS-VIL](03_atlas/SIS-VIL.md)) |
| Aldeão, "audião" | Morador da vila; quando contratado, trabalhador de um posto ([SIS-TRB](03_atlas/SIS-TRB.md)) |
| Prancheta | `veio:clipboard`, FUN-LOG-07 ([SIS-LOG](03_atlas/SIS-LOG.md)) |
| main, pr47, disperso | As três camadas do projeto ([01 §2](01_ESTADO_GERAL_DO_PROJETO.md#2-as-três-camadas-não-confundir)) |
| Substituto, stub | `native/offline-check/horizon-client`, a imitação da API do Horizon usada nos testes offline |
| "Cloud", Codex, Grok | As três frentes de agentes que escreveram o código; "Cloud" é como o chat chama o Claude na nuvem ([07](07_AGENTES_E_WORKFLOW.md)) |
| E1–E4, ND, J | Níveis de evidência ([Convenções](#convenções)) |

## Convenções

- **Estados:** Concebido · Aprovado · Em desenvolvimento · Implementado isoladamente · Integrado · Validado · Bloqueado · Suspenso · Substituído · Descartado · Indeterminado. Sempre por camada: `main`, `pr47` (candidata) e disperso.
- **Evidência:**
  - E1: executado nesta sessão.
  - E2: lido no código, com `arquivo:linha@commit`.
  - E3: documento, commit ou PR.
  - E4: chat ou memória.
  - ND: não determinado.
  - J: teste em jogo (hoje, nenhum).
- **IDs estáveis:** `SIS-`, `FUN-`, `DEC-`, `TAR-`, `CNF-`, `Q-`, `R-`. Regras em [11 §3](11_PLANO_DE_CONTINUIDADE.md#3-identificadores-estáveis).
- **Decisão do dono × decisão de agente:** sempre separadas. Recomendação de agente não é decisão.
