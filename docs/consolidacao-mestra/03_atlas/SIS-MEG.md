# SIS-MEG: Megafábricas

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `pr47@80980db`. O sistema não existe na main. Linhas em `native/api/src/` no pr47.
- **Origem:** Codex local, 08/11 13:16 (`import-megafactories`, contido no pr47). `frentes/MEGAFABRICAS.md` diz que foi pedido do usuário (E3, sem registro no chat).

## A. Identidade

Copiar e colar fábricas, como o Blueprint Designer do Satisfactory, e construir para cima. Serve para escalar sem montar bloco a bloco.

- **Depende de:** [SIS-LOG](SIS-LOG.md) (ajustes copiados), [SIS-PRG](SIS-PRG.md) (liberado pelo avanço `circuits` no pr47).
- **Alimenta:** [SIS-TRB](SIS-TRB.md) (escadas para aldeões), [SIS-TRA](SIS-TRA.md) (elevador, descrito lá).

## B. Funcionamento

### B.1 Copiadora (`veio:blueprint_designer`)

Fonte: `blueprint/mod.rs:1-33`.

1. O volume acima da copiadora é a área de cópia: um quadrado de 5, 9, 15 ou 21 blocos centrado nela, até 24 de altura, desenhado no mundo por uma moldura. O comentário do código chama essa área de "prancheta", o mesmo nome da ferramenta `veio:clipboard` ([SIS-LOG](SIS-LOG.md) B.4; [CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
2. **Copiar:** papel na mão e clique. Copia blocos, orientação (9 propriedades: `facing`, `axis`, `half`, `type`, `shape`, `hinge`, `face`, `part`, `rotation`) e ajustes (`RoutingPriority`, `Down`, `ManualDirection`, `LogisticsFilter`). Nunca copia inventários. Até 6.000 blocos.
3. Chave + agachado muda o tamanho; chave gira; mão vazia dá um resumo no chat.

### B.2 Marco de obra (`veio:build_marker`)

1. Fica onde a cópia vai nascer, no lugar equivalente ao da copiadora. Usa a copiadora mais próxima com projeto, até 256 blocos (`LINK`).
2. A diferença de orientação entre o marco e a copiadora gira a obra.
3. Mão vazia: orçamento no chat (materiais, tamanho, obstáculos).
4. Chave + agachado: constrói, pagando com o inventário de quem clicou. Falta material ou espaço: nada é cobrado e o chat diz o que falta.

### B.3 Fábricas verticais

- Escadas de mão, andares e elevadores são copiados como qualquer bloco.
- **Escadas para aldeões:** os carregadores sobem e descem colunas de escada de mão, até 6 andares, num alcance de 24 (`workforce/ladders.rs`; [SIS-TRB](SIS-TRB.md) B.6).
- **Elevador de carga:** ver [SIS-TRA](SIS-TRA.md) B.5.

### B.4 Onde fica o projeto

O projeto copiado fica **em memória**. Não passa para outro mundo e se perde ao reiniciar. `server_about_to_start` zera o estado ao trocar de mundo (`lib.rs:89`; [SIS-PER](SIS-PER.md)). Em LAN, o retorno vai a quem clicou (`CHECKPOINT`, E3).

## C. Experiência do jogador

1. A pessoa monta um módulo de fábrica de 15 × 15 sobre a copiadora e clica com papel.
2. Põe um marco de obra a 100 blocos dali, pede o orçamento e constrói com o próprio inventário.
3. Empilha andares e põe escadas: os carregadores passam a atender o andar de cima.

## D. Implementação

**Arquivos:** `blueprint/mod.rs`, `blueprint/tests.rs` (9 testes, E1); `workforce/ladders.rs`. Liberação: avanço `circuits` no pr47 ([SIS-PRG](SIS-PRG.md) B.1).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-MEG-01 Copiadora de plantas | 5/9/15/21 × 24; 6.000 blocos; 9 propriedades; 4 ajustes | — · `blueprint/mod.rs:20-33` · import-megafactories (contido) | E1 (9 testes) |
| FUN-MEG-02 Marco de obra | Até 256 blocos; paga com o inventário | — · `blueprint/mod.rs:10-15,21,25` · idem | E2 |
| FUN-MEG-04 Escadas entre andares | Até 6 andares; alcance 24 | — · `workforce/ladders.rs:1-9` · idem | E2 |

FUN-MEG-03 (prancheta) passou a FUN-LOG-07, porque é a ferramenta `veio:clipboard` de logística.

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 08/11 13:16 | Codex local | Copiadora, marco, elevador, escadas |
| 08/11 18:11 | `codex/consolidated-016` | Entra na consolidação do Codex |
| 08/11 19:14 | Dono | "Junte tudo" ([DEC-048](../06_HISTORICO_E_DECISOES.md#dec-048)) |

## F. Pendências

1. Persistir os projetos da copiadora, ou registrar que não persistem.
2. Medir o custo de colar 6.000 blocos (fora de `check_performance`).
3. Resolver o nome "prancheta" ([CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem)).
4. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-029](../08_BACKLOG_E_PRIORIDADES.md#tar-029): Persistir o que some ao reiniciar
- [TAR-047](../08_BACKLOG_E_PRIORIDADES.md#tar-047): Nomes que colidem

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main — · pr47 **Integrado** (01, 02, 04) |
| Confiança | Média: E1 nos testes da copiadora; `set_block_property` difere no Horizon real (09 §4.1) |
| Prioridade (sugestão) | Baixa |
| Criticidade (sugestão) | Média: colagem grande pode pesar e depende de API não verificada |
| Responsável conhecido | Codex local |
| DEC | [DEC-048](../06_HISTORICO_E_DECISOES.md#dec-048) |
| Q | [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem) |
| CNF | [CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (prancheta) |
| Referências | `frentes/MEGAFABRICAS.md`, `CHECKPOINT-INTEGRACAO.md`; A2 MEG |
