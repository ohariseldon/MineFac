# SIS-EXT: Extração e processamento de minério

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. Tempos em `tools/production.json` (gerado em `production/data.rs`). Linhas em `native/api/src/production/` no pr47.

## A. Identidade

É a primeira etapa de toda cadeia: extratoras sobre jazidas vanilla que nunca se esgotam, e as máquinas que transformam o bruto em chapa, lingote, silício e aço. A pureza da jazida vem de [SIS-MUN](SIS-MUN.md). As receitas seguintes estão em [SIS-PRO](SIS-PRO.md).

## B. Funcionamento

### B.1 Extratoras

| Nível | Tempo base por unidade | Energia | Liberação no pr47 | Fonte |
|---|---|---|---|---|
| Mk.I (`drill`) | 40 ticks (2 s) | 6 FE/t (240 FE por unidade) | inicial | `extraction.rs:21` |
| Mk.II (`drill_mk2`) | 20 ticks | 12 FE/t | `research/electronics` | `:22` |
| Mk.III (`drill_mk3`) | 10 ticks | 30 FE/t | `research/automation` | `:23` |

**Tempo real = tempo base × lentidão da jazida ÷ pureza.** A pureza vai de metade da velocidade (impura) ao quádruplo (industrial, só no pr47).

| Jazida (23 no pr47) | Nível mínimo | Lentidão |
|---|---|---|
| Cobre, ferro, ouro, carvão, redstone, lápis, quartzo, pepita de ouro; pedregulho, ardósia, areia, areia vermelha, cascalho, argila, netherrack, areia das almas, pedra-negra, pedra do End | Mk.I | ×1 |
| Ametista | Mk.II | ×4 |
| Obsidiana | Mk.II | ×8 |
| Diamante, **esmeralda** | Mk.II | ×20 |
| Detritos ancestrais | Mk.III | ×40 |

Fonte: `nodes` em `tools/production.json` (E2).

**Esmeralda minerada:** com Mk.II, 20 × 20 = 400 ticks por unidade (20 s numa jazida normal, 10 s numa pura), ou 4.800 FE. Isso cria moeda fora dos livros ([SIS-ECO](SIS-ECO.md) B.5, [CNF-001](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

### B.2 Processamento do minério

| Máquina | Receitas principais | Tempo / energia |
|---|---|---|
| Prensa | bruto de cobre ou ferro → chapa; bruto de ouro → lingote; lingote → chapa | 30 t / 8 FE (bruto); 15 t (lingote); ouro 40 t / 10 FE |
| Fornalha elétrica | 23 receitas de fornalha vanilla (minério, vidro, pedra, comida, carvão vegetal); detritos → sucata de netherita | 80 t / 10 FE (2,5 vezes a fornalha vanilla); sucata 400 t / 24 FE |
| Britador | quartzo → silício; osso → 4 farinhas de osso; pedregulho → cascalho → areia; argila → 4 bolas | 30–50 t / 10 FE |
| Forno de ligas | 2 chapas de ferro + carvão → aço; 4 areias + carvão → silício | 120 t / 16 FE; 160 t / 16 FE |

Fonte: `recipes` em `tools/production.json`; `production/data.rs:5-6` (prensa 30 t / 8 FE) (E2, A7). As receitas das duas camadas são **idênticas** para estas máquinas.

### B.3 Ritmo de referência

- Uma prensa faz 40 chapas por minuto (20 × 60 ÷ 30).
- Uma Mk.I numa jazida normal faz 30 brutos por minuto. Então **uma extratora normal alimenta pouco menos de uma prensa**.
- O guia de indústria do pr47 usa a razão "3 prensas = 2 montadores" ([SIS-PRO](SIS-PRO.md) B.1).
- O README do pr47 ainda diz "a primeira linha pede duas prensas por veio" (l.24). Isso vale para a prensa de 80 ticks do Grok, substituída pelos tempos da 0.25 ([DEC-012](../06_HISTORICO_E_DECISOES.md#dec-012), [DEC-045](../06_HISTORICO_E_DECISOES.md#dec-045)).

## C. Experiência do jogador

1. O tutorial manda "extrair cobre de verdade" (passo 5).
2. A pessoa põe uma Mk.I sobre cobre, liga um gerador e leva o bruto à prensa por esteira.
3. Um veio puro rende o dobro: vale procurar.
4. A Mk.II traz uma escolha: minerar esmeralda devagar, gastando energia, em vez de vender produtos.

## D. Implementação

**Arquivos:** `production/extraction.rs` (níveis e pureza); `tools/production.json` → `production/data.rs` (receitas e jazidas). Testes: production 16 (main) e 39 (pr47) (E1).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-EXT-01 Brocas Mk.I/II/III | 40/20/10 t; 6/12/30 FE | `extraction.rs` · `extraction.rs:21-23` · — | E2 |
| FUN-EXT-02 Britador, forno de ligas, fornalha elétrica | Ver B.2 | `data.rs` · `data.rs` · — | E2 |
| FUN-EXT-03 "Veio normal pede duas prensas" ([DEC-012](../06_HISTORICO_E_DECISOES.md#dec-012)) | Prensa de 80 t | — · Substituído (A3 marca Integrado só porque os commits `96458cf`/`3eb4b3d` estão no histórico; o JSON tem 30 t) · #19 (contido) | E3; E2 do JSON |
| FUN-EXT-04 Tempos de receita da 0.25 ([DEC-045](../06_HISTORICO_E_DECISOES.md#dec-045)) | 53 receitas da main idênticas no pr47 | Integrado · Integrado · Substituído (tempos por fase do Codex, `0ce91a4`/`50995e6`) | E2 (`data.rs` regenerado conferido, A7) |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 18:24 (0.6) | `5eaea2c` | Extratora e prensa |
| 07/11 20:04 (0.9) | `5944d37` | Receitas por tabela; pureza |
| 07/11 ~23h | Grok #19 | Prensa em 80 t, "duas prensas por veio" ([DEC-012](../06_HISTORICO_E_DECISOES.md#dec-012)) |
| 08/11 05:35 | `claude/progressao-fases` | Prensa em 40 t ([DEC-014](../06_HISTORICO_E_DECISOES.md#dec-014)) |
| 08/11 19:16 | Dono | Tempos da 0.25: prensa 30 t ([DEC-045](../06_HISTORICO_E_DECISOES.md#dec-045)) |

## F. Pendências

1. Corrigir a A3 (FUN-EXT-03 deve ser Substituído no pr47) e o README do pr47 l.24.
2. Decidir se a esmeralda minerada continua criando moeda ([Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros)).
3. Subproduto (escória) foi sugerido no documento de lacunas e não foi feito.
4. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-059](../08_BACKLOG_E_PRIORIDADES.md#tar-059): Subproduto (escória) e fluido industrial

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01, 02, 04) · pr47 **Integrado** (01, 02, 04), **Substituído** (03) |
| Confiança | Alta (E2 + E1 dos testes de produção) |
| Prioridade (sugestão) | Baixa: estável |
| Criticidade (sugestão) | Alta: toda cadeia começa aqui |
| Responsável conhecido | Codex local e Claude local (0.6–0.9); Grok (#19, substituído) |
| DEC | [DEC-010](../06_HISTORICO_E_DECISOES.md#dec-010), [DEC-012](../06_HISTORICO_E_DECISOES.md#dec-012), [DEC-014](../06_HISTORICO_E_DECISOES.md#dec-014), [DEC-045](../06_HISTORICO_E_DECISOES.md#dec-045) |
| Q | [Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros) |
| CNF | [CNF-001](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (esmeralda minerada), [CNF-006](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (README do pr47) |
| Referências | README §Produção; `frentes/GROK-BALANCE.md`; `frentes/PROGRESSAO-FASES.md`; A2 EXT; A7 (`data.rs`) |
