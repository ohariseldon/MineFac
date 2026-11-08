# SIS-PER: Persistência e save

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. Inventário do OPUS A (A1 §5; A6 [R-08](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43) a [R-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)). Mapa completo das chaves em [04 §6 e §10.6–10.7](../04_ARQUITETURA_E_DEPENDENCIAS.md).

## A. Identidade

O que sobrevive a fechar e abrir o mundo. Não é um sistema de jogo; é a base de todos os outros. Existem três lugares: o NBT das block entities, arquivos em `<mundo>/veio/` e estado em memória que se perde.

## B. Funcionamento

### B.1 NBT das block entities

Cada máquina e posto guarda o seu estado no próprio NBT: energia, `Water`, `Fluid`, `Damage`, `Module`, `Tier`, status, fila do capataz, obras do centro (`Built`, `ObraI_J`). Chaves de livro: `Company`, `Public`, `Wallet`, `Prices`. O comércio usa `VERSION 3` (`trade.rs:389`), o único formato versionado. O pr47 acrescenta `Water`, `Home` e `HomeBeds`.

### B.2 Arquivos em `<mundo>/veio/` (só no pr47)

| Arquivo | Sistema | Gravação | Versão de formato |
|---|---|---|---|
| `tutorial/<UUID>.txt` | [SIS-TUT](SIS-TUT.md) | A cada 100 ticks, por arquivo temporário | não |
| `rios.txt` | [SIS-MUN](SIS-MUN.md) | A cada 1.200 ticks, por arquivo temporário | não |
| `estacoes.txt` | [SIS-TRA](SIS-TRA.md) | Direta | não |
| `ancoras.txt` | [SIS-TRA](SIS-TRA.md) | Direta | não |
| `pesquisa.txt` | [SIS-PES](SIS-PES.md) | A cada 600 ticks e ao concluir; direta | não |

Três arquivos são gravados sem arquivo temporário: uma queda no meio corrompe o estado ([R-08](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).

### B.3 Estado só em memória (perde-se ao reiniciar)

Poluição ([SIS-DEF](SIS-DEF.md), [DEC-059](../06_HISTORICO_E_DECISOES.md#dec-059)), contadores da holding e `BORDERS` ([SIS-ECO](SIS-ECO.md)), conexões externas, registro de portos de drone, censo e `paid_day` do pedido do dia ([SIS-PRG](SIS-PRG.md)), projeto da copiadora ([SIS-MEG](SIS-MEG.md)), bônus de refeição ([SIS-LAR](SIS-LAR.md)), relógio do mod (`CLOCK`), monitor da fábrica, `PERKS` das obras e registros de postos, casas e zonas. Os registros de postos e casas se refazem sozinhos em segundos. Os demais recomeçam do zero ([R-09](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).

### B.4 Troca de mundo

`server_about_to_start` (`lib.rs:89`, pr47) zera só casa, censo e copiadora. Pesquisa e rios detectam a troca por `world_dir` a cada 100 ticks. Os demais estáticos podem vazar de um mundo para outro na mesma sessão ([R-10](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).

### B.5 Compatibilidade de saves

- **Save da main 0.25 aberto no pr47:** os 55 blocos e 16 itens da main existem no pr47. As chaves NBT novas começam vazias ([R-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
- **Receitas:** quem já tinha o avanço `steel` na main mantém as receitas antigas. No pr47 elas passam a vir da pesquisa, e mundos existentes ficam com as duas ([R-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
- **Saves 0.16.1 e Realismo 1.1:** o bloco `veio:freight_station` era o terminal ferroviário (6 slots, saídas 0..3) e agora é a estação de exportação (9 slots de entrada). O terminal antigo vira estação de exportação e **exporta a carga** ([CNF-014](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

## C. Experiência do jogador

Fechar e abrir o mundo deveria ser invisível. Hoje a poluição some, o pedido do dia pode ser pago de novo, a holding zera e o bônus de refeição acaba.

## D. Implementação

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-PER-01 NBT das block entities | Por bloco; `Trade VERSION 3` | Integrado · Integrado · — | E2 |
| FUN-PER-02 Arquivos `<mundo>/veio/` | 5 arquivos sem versão | — · B.2 · — | E2 |
| FUN-PER-03 Reset ao trocar de mundo | Parcial: casa, censo, copiadora; `world_dir` | — · `lib.rs:89` · — | E2 |
| FUN-PER-04 Gravação atômica | Só rios e tutorial | — · `rivers.rs:769`; `onboarding.rs:403` · — | E2 |

## E. Histórico

O NBT é o modo de persistência desde a 0.6. Os arquivos em `<mundo>/veio/` entraram com o tutorial do Codex (0.15.x) e o #40 (rios, estações, âncora, pesquisa). Nenhuma frente definiu um formato comum.

## F. Pendências

1. Criar um cabeçalho de versão e uma gravação atômica comuns para os arquivos de `<mundo>/veio/` ([R-08](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
2. Decidir o que deve persistir: poluição, holding, `BORDERS`, `paid_day`, bônus de refeição, relógio ([R-09](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
3. Zerar todos os estáticos em `server_about_to_start` ([R-10](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
4. Escrever uma nota de migração para saves 0.16.1 e Realismo 1.1 ([CNF-014](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)) e para saves da main ([R-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
5. Teste em jogo: abrir um save da main no pr47.

### Tarefas no backlog

- [TAR-007](../08_BACKLOG_E_PRIORIDADES.md#tar-007): Proteger saves 0.16.1 e Realismo 1.1 da troca de sentido de `veio:freight_station`
- [TAR-018](../08_BACKLOG_E_PRIORIDADES.md#tar-018): Abrir um save 0.25 no pr47
- [TAR-029](../08_BACKLOG_E_PRIORIDADES.md#tar-029): Persistir o que some ao reiniciar
- [TAR-032](../08_BACKLOG_E_PRIORIDADES.md#tar-032): Formato versionado e gravação atômica em `<mundo>/veio/`
- [TAR-033](../08_BACKLOG_E_PRIORIDADES.md#tar-033): Reset central de estado entre mundos

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01) · pr47 **Integrado** (01–04), com lacunas |
| Confiança | Média: inventário E2; nenhum save real aberto |
| Prioridade (sugestão) | Alta: antes do merge e antes de qualquer jogador guardar um mundo longo |
| Criticidade (sugestão) | Alta: corrupção de save é a pior falha possível |
| Responsável conhecido | Nenhum: cada frente cuida do seu arquivo |
| DEC | [DEC-059](../06_HISTORICO_E_DECISOES.md#dec-059) |
| Q | [Q-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-11-condições-para-fundir-o-47), [Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan) |
| CNF | [CNF-014](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (`freight_station` sem migração) |
| Referências | A1 §5; A6 [R-08](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43) a [R-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43); [04 §6](../04_ARQUITETURA_E_DEPENDENCIAS.md) |
