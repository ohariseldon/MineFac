# SIS-ENE: Energia

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. Linhas em `native/api/src/` no pr47. A unidade é FE (Forge Energy); FE/t é por tick.

## A. Identidade

Energia é o segundo gargalo da progressão, depois da extração. O sistema liga combustíveis vanilla, renováveis presos ao terreno e, no pr47, a hidrelétrica dos rios que o dono pediu ([DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039): "usinas reais, hidrelétrica").

- **Depende de:** [SIS-AGR](SIS-AGR.md) (biomassa), [SIS-AGU](SIS-AGU.md) (água da caldeira, lava da geotérmica), [SIS-MUN](SIS-MUN.md) (quedas dos rios).
- **Alimenta:** todas as máquinas ([SIS-EXT](SIS-EXT.md), [SIS-PRO](SIS-PRO.md)), [SIS-TRA](SIS-TRA.md) (estações, drones, âncora), [SIS-PES](SIS-PES.md) e [SIS-DEF](SIS-DEF.md) (Tesla).

## B. Funcionamento

### B.1 A escada de energia

Fonte: `production/generation.rs:1-9`.

| Etapa | Fonte | Potência | Custo e limite |
|---|---|---|---|
| 1 | Roda d'água | 8 FE/t por face com água corrente, até 32 | Grátis, limpa, presa ao terreno |
| 1 | Cata-vento | 8 FE/t no nível do mar; sobe com a altitude e com chuva, até 32 | Idem |
| 2 | Gerador a combustível | 40 FE/t | Combustível; polui |
| 2 | Gerador a biomassa | 40 FE/t | Biomassa da lavoura; polui menos |
| 3 | Caldeira a vapor | 120 FE/t, o dobro de energia por combustível | Precisa de água ao lado (pr47: cano) |
| 4 | Geotérmico | 80 FE/t | Lava |
| 4 | Solar | 16 FE/t ao meio-dia, pela altura do sol | Escala em quantidade; não funciona à noite |
| 5 | Hidrelétrica (pr47) | até 320 FE/t | Represa num rio ([SIS-MUN](SIS-MUN.md)) |

Os sensores de roda, vento e sol leem o mundo a cada 40 ticks (`SENSE_EVERY`).

### B.2 Combustíveis

O valor do combustível é medido em **ticks de fornalha vanilla** (`fuel_ticks`). Cada queimador converte um tick de queima em FE (`burner`, `generation.rs:14-20`):

| Queimador | FE/t | FE por tick de queima |
|---|---|---|
| Gerador | 40 | 10 |
| Gerador a biomassa | 40 | 10 |
| Caldeira | 120 | 20 |

| Combustível (gerador) | Ticks | FE no gerador | FE na caldeira |
|---|---|---|---|
| Carvão ou carvão vegetal | 1.600 | 16.000 | 32.000 |
| Bloco de carvão | 16.000 | 160.000 | 320.000 |
| Vara de blaze | 2.400 | 24.000 | 48.000 |
| Bloco de alga seca | 4.000 | 40.000 | 80.000 |
| Tronco ou bloco de bambu | 300 | 3.000 | 6.000 |
| Graveto | 100 | 1.000 | 2.000 |

Gerador a biomassa: biomassa 600 ticks; plantas 75; trigo e cana 100; bloco de alga e fardo de feno 900 (`fuels` em `tools/production.json`).

**Caldeira no pr47:** alimentada por cano, consome 5 mB de água por tick de queima (`STEAM_WATER`, `production/water.rs:30`) e guarda 8.000 mB.

### B.3 Rede

Os postes se ligam sozinhos aos vizinhos por fios aéreos e alimentam as máquinas da sua área, como no Factorio (`power.rs:1-8`).

| Poste | Alcance do fio | Raio da área | Vazão | Buffer |
|---|---|---|---|---|
| Madeira | 8 | 2 | 120 FE/t | 1.000 |
| Aço | 16 | 3 | 400 FE/t | 4.000 |

- Cada poste escolhe até 6 vizinhos (`MAX_LINKS`). A cada 40 ticks pede a reconstrução dos fios e a varredura da área, em etapas sob cotas globais ([SIS-DES](SIS-DES.md)). A transferência roda todo tick (`power.rs:26-35`).
- Acumulador (bateria): 30.000 FE. Cada máquina guarda 2.000 FE (README, E3).

### B.4 Hidrelétrica

A turbina não guarda água. Mede a represa atrás dela e gera pela queda, como P = ρ·g·Q·h (`production/water.rs:6-14`):

`potência = 6 × min(queda, 32) × min(espelho, 40) ÷ 40 × (1 + min(vazão, 12) ÷ 4)`, até 320 FE/t (`hydro_power`, `:320-328`).

- Exige queda ≥ 3 e espelho d'água ≥ 12. Senão, 0.
- Com espelho ≥ 40 e vazão ≥ 12, rende 24 × queda. O teto de 320 exige queda de 14 ou mais.
- Uma queda de 10 rende 240 FE/t: seis geradores a carvão, ou duas caldeiras, sem combustível.

## C. Experiência do jogador

1. A pessoa começa com roda d'água e gerador a carvão.
2. Passa à caldeira, que rende o dobro por carvão, e encana água até ela.
3. Usa o solar com acumuladores para atravessar a noite.
4. No pr47, acha uma cachoeira, represa e põe uma turbina: energia grande e limpa.

## D. Implementação

**Arquivos:** `power.rs` e `power/` (postes, rede); `production/generation.rs` (queimadores, renováveis); `production/water.rs` (turbina, caldeira por cano). Testes: power 9 (E1). Desempenho (E1): `256_poles` 2,29 → 1,53 ms; `dense_pole_links` 0,87 → 0,61 ms.

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-ENE-01 Postes e rede | Madeira 8/2/120/1.000; aço 16/3/400/4.000; 6 vizinhos | `power.rs` · `power.rs:28-35` · — | E1 |
| FUN-ENE-02 Gerador e combustíveis (inclui a ex-PRO-02) | 40 FE/t; ×10 por tick de queima; tabela B.2 | `generation.rs`; `production.json` · `generation.rs:14-30` · — | E2 |
| FUN-ENE-03 Renováveis e geotérmica | Roda e vento até 32; solar 16; geotérmica 80 | `generation.rs` · `generation.rs:1-9,32-36` · — | E2 |
| FUN-ENE-04 Vapor e bateria | Caldeira 120 FE/t, ×20; bateria 30.000 | Integrado (avanço `steel`) · Integrado (pesquisa Metalurgia; água por cano) · — | E2 |
| FUN-ENE-05 Hidrelétrica | Fórmula B.4; teto 320 | — · `water.rs:320-328` · #40 (contido) | E2; nunca vista em rio real |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 18:24 (0.6) | `5eaea2c` | Gerador, solar, acumulador, cabos |
| 07/11 19:37 (0.8) | `7644aab` | Postes com fios aéreos |
| 07/11 20:04 (0.9) | `5944d37` | Renováveis |
| 07/11 noite | Grok lote 1 | Renováveis reforçados |
| 08/11 15:55 | Dono | Usinas reais e hidrelétrica ([DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039)) |
| 08/11 ~17h | #40 | Hidrelétrica; caldeira por cano; liberação pela pesquisa |

## F. Pendências

1. Medir o balanço da hidrelétrica contra a caldeira em rios reais. Hoje uma queda de 10 rende seis geradores sem custo.
2. Confirmar no jogo os sensores de roda e vento.
3. A meta energética "atravessar uma noite" está Concebida ([DEC-023](../06_HISTORICO_E_DECISOES.md#dec-023)).
4. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-049](../08_BACKLOG_E_PRIORIDADES.md#tar-049): Calibrar hidrelétrica × carvão × vapor

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01–04) · pr47 **Integrado** (01–05) |
| Confiança | Alta para a rede (E1); média para geração (E2) |
| Prioridade (sugestão) | Baixa |
| Criticidade (sugestão) | Alta: tudo depende de energia |
| Responsável conhecido | Codex e Claude local (0.6–0.9); Grok (lote 1); Claude do #40 (hidrelétrica) |
| DEC | [DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039), [DEC-050](../06_HISTORICO_E_DECISOES.md#dec-050), [DEC-023](../06_HISTORICO_E_DECISOES.md#dec-023) |
| Q | [Q-05](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo) (energia da produção virtual) |
| CNF | — |
| Referências | README §Energia; `frentes/AGUA-E-RIOS.md`; A2 ENE; A7 (combustíveis em ticks) |
