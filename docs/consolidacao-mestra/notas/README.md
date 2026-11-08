# Notas de mudança

> Regra do dono ([DEC-067](../06_HISTORICO_E_DECISOES.md#dec-067), 08/11 22:38): toda alteração no projeto, de qualquer frente, deixa uma nota curta aqui. De tempos em tempos, um **compilado geral** leva o conteúdo das notas para os documentos 00–11. O objetivo é não perder progresso nem processo.

## Quando escrever

Escreva uma nota sempre que alguém, humano ou agente, local ou na nuvem:
- abrir, atualizar ou fazer merge de um PR;
- mudar código, conteúdo, versão, pacote ou launcher;
- receber uma decisão do dono;
- testar em jogo, mesmo que o teste falhe;
- descobrir um problema, um conflito ou um risco.

A nota vai **no mesmo PR** da mudança. Se não houver PR (teste local, decisão no chat), vai num PR só de notas ou no próximo PR da frente.

## Nome do arquivo

`AAAA-MM-DD-<frente>-<assunto>.md`, tudo em minúsculas, com hífens e sem acentos. Exemplos: `2026-11-09-claude-obras-traducoes.md`, `2026-11-09-dono-teste-0-26.md`.

Para `<frente>`, use quem fez (`dono`, `claude`, `cloud`, `codex`, `grok`, `integrador`; ver [07](../07_AGENTES_E_WORKFLOW.md)) ou a frente temática da thread (por exemplo, `visual`, `realismo`, `mapa`, `obras`). Duas notas no mesmo dia e assunto levam sufixo `-2`, `-3`.

## Modelo (cabe em meia tela)

```markdown
# <Assunto em uma linha>

- **Data / frente / autor:** 2026-11-09 · claude · thread "Fix Safari export"
- **Onde:** PR #NN, branch `x`, commit `abc1234` (ou "chat", "PC do dono")
- **Sistemas:** SIS-XXX, SIS-YYY · **FUN:** FUN-XXX-NN (ou "nova, sem ID")
- **Tipo:** código · conteúdo · decisão · teste · problema · docs

## O que mudou
2 a 5 linhas: o que passou a existir ou deixou de existir.

## Evidência
E1 (comando e resultado), E2 (`arquivo:linha@commit`), E3, E4 ou J (teste em jogo: versão, roteiro, resultado).

## Toca no compilado
- IDs afetados ou novos: DEC / TAR / CNF / Q / R
- Status, slot, bloco, NBT ou arquivo de mundo reservado (04 §10)
- O que fica pendente
```

Não copie o PR inteiro nem o diff: a nota aponta para eles. Se um campo não se aplica, apague-o.

## Compilado geral

Quem fizer o compilado:
1. lê as notas com data posterior ao último compilado (a lista abaixo);
2. atualiza as fichas (§D e §F), o 05, o 04 §10, o 06, o 08 e o 09, seguindo o [11 §4](../11_PLANO_DE_CONTINUIDADE.md#4-quando-atualizar-o-quê);
3. troca o commit de referência no topo dos arquivos que mudaram e no [01](../01_ESTADO_GERAL_DO_PROJETO.md);
4. acrescenta uma linha na tabela abaixo.

As notas ficam no lugar depois do compilado. Elas são o registro do processo e não se apagam.

| Compilado | Data | Notas cobertas | Referência | PR |
|---|---|---|---|---|
| Consolidação mestra (inicial) | 2026-11-08 | — (ponto de partida) | `main@53267af`, `pr47@80980db` | #55 |

## Notas

| Data | Frente | Assunto | Arquivo |
|---|---|---|---|
| 2026-11-08 | claude | Consolidação mestra e regra das notas | [2026-11-08-claude-consolidacao-mestra.md](2026-11-08-claude-consolidacao-mestra.md) |
