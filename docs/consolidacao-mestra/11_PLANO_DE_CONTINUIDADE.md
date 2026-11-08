# 11 — Plano de continuidade e governança documental

> Este arquivo diz **como retomar o trabalho e manter esta documentação viva**. Os papéis e o fluxo dos agentes estão em [07](07_AGENTES_E_WORKFLOW.md); o backlog ordenado está em [08](08_BACKLOG_E_PRIORIDADES.md). Aqui não se repete nada disso: só o procedimento.
> Tudo o que está aqui é **proposta** do supervisor e dos agentes. Vira regra quando o dono aprovar (DEC em [06](06_HISTORICO_E_DECISOES.md)).

## 1. Retomar o projeto em cinco leituras

Um agente ou desenvolvedor novo lê, nesta ordem, e para quando tiver o que precisa:

| # | Ler | Para saber | Tempo |
|---|---|---|---|
| 1 | [01 Estado geral](01_ESTADO_GERAL_DO_PROJETO.md) | O que é o jogo, as três camadas (main, candidata, disperso), o que bloqueia | 5 min |
| 2 | A ficha do seu sistema em [03_atlas/](03_ATLAS_DE_SISTEMAS.md) | Regras, implementação com `arquivo:linha`, pendências, decisões | 10 min |
| 3 | As linhas FUN do seu sistema em [05](05_ESTADO_REAL_DA_BUILD.md) | Estado real e evidência por camada | 3 min |
| 4 | As TAR do seu sistema em [08](08_BACKLOG_E_PRIORIDADES.md) e as Q/CNF em [09](09_CONFLITOS_LACUNAS_E_RISCOS.md) | O que fazer e o que não fazer sem perguntar ao dono | 5 min |
| 5 | O registro de IDs em [04](04_ARQUITETURA_E_DEPENDENCIAS.md) | Números de status, slots, blocos, NBT e arquivos já ocupados | 2 min |

Não é preciso ler o histórico de chats, os ~40 documentos em `frentes/` nem o `TRABALHADORES.md` inteiro. O status de cada documento legado (vigente, histórico ou substituído) está em [10](10_EVIDENCIAS_E_RASTREABILIDADE.md).

## 2. Regras de leitura quando as fontes divergem

1. **Para descrever o jogo, vale o código da camada** (`main` ou candidata), citado como `arquivo:linha@commit`.
2. **Para a direção, vale a última decisão do dono** registrada em [06](06_HISTORICO_E_DECISOES.md), com data e hora.
3. Um documento que contradiz o código ganha a marca "divergente" em [05](05_ESTADO_REAL_DA_BUILD.md) e abre uma CNF em [09](09_CONFLITOS_LACUNAS_E_RISCOS.md). Ele não é corrigido em silêncio.
4. O documento mais recente não vence só por ser recente. Também não vence o maior, nem o da main.
5. Decisão de agente fica sempre rotulada como tal, até o dono confirmar.
6. Sem teste em jogo (evidência J), nenhuma funcionalidade passa de **Integrado**.

## 3. Identificadores estáveis

| Prefixo | Exemplo | Onde mora | Regra |
|---|---|---|---|
| `SIS-XXX` | `SIS-MER` | [03](03_ATLAS_DE_SISTEMAS.md) | Lista fechada de 25. Fundir ou dividir só com DEC |
| `FUN-XXX-NN` | `FUN-MER-05` | fichas em `03_atlas/` e matriz em [05](05_ESTADO_REAL_DA_BUILD.md) | Um comportamento verificável. O número nunca é reaproveitado. Uma FUN substituída fica marcada "Substituída por …" |
| `DEC-NNN` | `DEC-045` | [06](06_HISTORICO_E_DECISOES.md) | Só acrescenta. Campos: data/hora, quem, tipo, texto, fonte, FUN afetadas, estado do código, substitui/substituída por |
| `TAR-NNN` | `TAR-001` | [08](08_BACKLOG_E_PRIORIDADES.md) | Fecha com evidência (commit, PR, comando) |
| `CNF-NNN` | `CNF-013` | [09](09_CONFLITOS_LACUNAS_E_RISCOS.md) | Fecha com uma DEC ou com uma TAR concluída |
| `Q-NN` | `Q-03` | [09 §5](09_CONFLITOS_LACUNAS_E_RISCOS.md#5-decisões-pendentes-do-dono) | Pergunta ao dono. A resposta vira DEC |
| `R-NN` | `R-14` | [09 Apêndice A](09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43) | Risco técnico com severidade |

Em commits e PRs, cite os IDs num rodapé, por exemplo `Refs: FUN-OBR-04, DEC-035, TAR-021`. Assim qualquer agente acha, pelo `git log --grep`, o que mudou numa funcionalidade.

## 4. Quando atualizar o quê

A tabela é a regra mínima. Quem faz a mudança faz a atualização, no mesmo PR.

| Gatilho | Atualizar | Quem |
|---|---|---|
| O dono responde uma Q ou decide algo no chat | Criar a DEC em [06](06_HISTORICO_E_DECISOES.md), com o texto literal e a hora; fechar a Q e a CNF em [09](09_CONFLITOS_LACUNAS_E_RISCOS.md); liberar as TAR "aguarda Q-NN" em [08](08_BACKLOG_E_PRIORIDADES.md); ajustar a seção B da ficha | Quem recebeu a resposta |
| Um PR de frente muda código | Na ficha do sistema: seção D (`arquivo:linha@commit`) e F (pendências). Em [05](05_ESTADO_REAL_DA_BUILD.md): estado da FUN na camada "disperso". Em [04](04_ARQUITETURA_E_DEPENDENCIAS.md): Registro de IDs, se reservou status, slot, bloco, NBT ou arquivo de mundo | A frente |
| Merge na branch de integração ou na main | Mudar as camadas em [05](05_ESTADO_REAL_DA_BUILD.md); fechar as TAR em [08](08_BACKLOG_E_PRIORIDADES.md); atualizar o quadro do [01](01_ESTADO_GERAL_DO_PROJETO.md) e o commit de referência no topo dos arquivos | Integrador |
| Teste em jogo | Coluna J em [05](05_ESTADO_REAL_DA_BUILD.md), com versão, quem, roteiro e resultado. Só aqui uma FUN pode virar **Validado** | Quem testou (normalmente o dono) |
| Uma frente nova começa | Linha em [07](07_AGENTES_E_WORKFLOW.md) (agente, branch, SIS, TAR); responsável na TAR em [08](08_BACKLOG_E_PRIORIDADES.md) | Quem delegou |
| Um documento novo em `frentes/` ou na raiz | Linha de status em [10](10_EVIDENCIAS_E_RASTREABILIDADE.md). O conteúdo normativo vai para a ficha do sistema, não para um documento solto | Autor |

Uma TAR só fecha quando estas atualizações estiverem feitas. É o "critério de conclusão documental" do [07](07_AGENTES_E_WORKFLOW.md).

## 5. Procedimento de integração (resumo operacional)

O detalhe está em [07](07_AGENTES_E_WORKFLOW.md). Aqui fica só a lista de conferência do integrador antes de cada merge:

1. `git fetch` de todas as branches; conferir se a base da frente é a candidata atual.
2. Rodar `python tools/check_project.py`, `python tools/check_rust.py` e `python tools/check_performance.py`, e colar a saída no PR (E1).
3. Rodar os benchmarks (`bench_factory`, `bench_frame`, `bench_large_village`) e comparar com o último registro em [05](05_ESTADO_REAL_DA_BUILD.md). Uma regressão acima do limite aprovado bloqueia o merge.
4. Regenerar os arquivos gerados: `tools/build_production.py` (`data.rs`), `build_content.py`, `build_tutorial.py` e `build_textures.py`. Merges automáticos já trouxeram tempos de receita errados uma vez.
5. Conferir o Registro de IDs em [04](04_ARQUITETURA_E_DEPENDENCIAS.md): nenhum status, slot ou bloco reaproveitado com outro sentido.
6. Versão: só o integrador numera, se a [Q-08](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera) for aprovada assim.
7. Atualizar as camadas em [05](05_ESTADO_REAL_DA_BUILD.md) e o quadro do [01](01_ESTADO_GERAL_DO_PROJETO.md).

## 6. Próximos ciclos

A ordem detalhada e justificada das tarefas está no topo de [08](08_BACKLOG_E_PRIORIDADES.md). Em três ciclos:

**Ciclo 1. Tornar a candidata real.** Sem código novo de conteúdo.
- Compilar o pr47 pelo launcher, com o Horizon fixado.
- Recuperar o desempenho da main (bisseção) e consertar `bench_large_village`.
- Pacote 0.26.0 próprio e README corrigido.
- Roteiro curto de teste em jogo e registro J.
- Congelar o #47 até o merge.
- Depende do dono para [Q-11](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-11-condições-para-fundir-o-47) e [Q-08](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera), e de uma máquina Windows com Horizon para compilar e jogar.

**Ciclo 2. Consertar o núcleo do jogo de fábrica.**
- Confirmar e ligar o gate de receitas (CNF-003).
- Aplicar a resposta da [Q-03](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão) (espinha da progressão).
- Fechar as torneiras de moeda conforme a [Q-01](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros).
- Dar custo à produção virtual conforme a [Q-05](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo).
- Persistência com versão e gravação atômica (SIS-PER).
- CI rodando as três verificações em cada PR.

**Ciclo 3. Retomar as frentes.**
- Realismo (#51) e mapa e desempenho (#52) voltam como frentes sobre a main nova, cada uma com seu SIS e suas TAR.
- Pendências das obras (`HANDOFF_OBRAS.md`).
- Fim de jogo ([Q-10](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-10-fim-de-jogo)) e peso da sociedade ([Q-12](09_CONFLITOS_LACUNAS_E_RISCOS.md#q-12-peso-da-sociedade-frente-à-fábrica)), decididos com o resultado do primeiro teste em jogo.

## 7. Manutenção desta pasta

- **Fonte única.** Esta pasta é o estado canônico. O README do jogo continua sendo a porta de entrada do jogador e deve apontar para cá. Os documentos em `frentes/` passam a ser registros de lote ou histórico (status em [10](10_EVIDENCIAS_E_RASTREABILIDADE.md)), e nenhum deles é apagado.
- **Commit de referência.** Todo arquivo desta pasta diz no topo contra qual commit foi verificado. Quando a main andar, quem atualizar troca o commit e revisa só as linhas afetadas.
- **Changelog.** O histórico de versões em [06](06_HISTORICO_E_DECISOES.md) só cresce. O primeiro parágrafo do README do jogo, que hoje acumula cerca de 3 KB de histórico de 0.16 a 0.25, pode ser enxugado para apontar para cá (TAR de documentação).
- **Automação sugerida (TAR, não decisão).** Estender `tools/check_project.py` para conferir três coisas:
  - todo `FUN-/DEC-/TAR-/CNF-/Q-` citado em commit ou documento existe;
  - nenhum código de `STATUS_KEYS` ou slot se repete com outro sentido;
  - todo `.md` do repositório tem status em [10](10_EVIDENCIAS_E_RASTREABILIDADE.md).
- **Proporcionalidade.** O projeto tem um dono e vários agentes de IA. O mínimo obrigatório é DEC para decisões, atualização da ficha e do [05](05_ESTADO_REAL_DA_BUILD.md) por quem muda o código, e o integrador conferindo a §5. O resto é ajuda, não burocracia.

## 8. Limites desta consolidação

Esta consolidação foi feita em 2026-11-08 por um supervisor e dois agentes de auditoria, só com o que é visível no GitHub e nesta sessão.

**Não foram vistos:**
- commits locais não enviados;
- a máquina do dono;
- o Horizon real;
- partidas jogadas;
- as descrições completas dos PRs no GitHub (foram usados os commits);
- os repositórios Projeto-Moba e Wasému, fora do escopo.

**Ficaram fora do retrato:**
- os PRs #51 e #52 só foram inventariados;
- trabalho que as threads de realismo e de mapa produzirem depois de 2026-11-08 ~21h30 UTC.

A cobertura completa está em [10](10_EVIDENCIAS_E_RASTREABILIDADE.md).
