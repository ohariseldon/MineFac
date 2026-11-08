# SIS-VIL: Sociedade da vila

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. O código de `workforce/society/*` é idêntico nas duas.
- **Linhas citadas:** salvo indicação, valem para `native/api/src/workforce/society/`.
- **Escopo:** este sistema cobre casas, famílias, lojas como ponto de consumo, prefeitura, indicadores e fases. O dinheiro está em [SIS-ECO](SIS-ECO.md). Preço e comércio estão em [SIS-MER](SIS-MER.md).

## A. Identidade

A vila deixa de ser só mão de obra. Famílias moram em casas, recebem salário, comem, compram conforto e são medidas por um prefeito, que libera fases. As fases destravam zonas maiores, mais população e mais alvarás.

- **Origem:** decisão de agente, 0.18.0 em 08/11 16:06 ([DEC-029](../06_HISTORICO_E_DECISOES.md#dec-029)). Não houve pedido explícito do dono. O peso da sociedade frente à fábrica está em aberto ([Q-12](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-12-peso-da-sociedade-frente-à-fábrica)).
- **Depende de:** [SIS-TRB](SIS-TRB.md) (aldeões e empregos), [SIS-ECO](SIS-ECO.md) (salários e impostos), [SIS-MER](SIS-MER.md) (lojas), [SIS-AGR](SIS-AGR.md) (comida), [SIS-ENE](SIS-ENE.md) (postes para o padrão da fase 3) e [SIS-DEF](SIS-DEF.md) (poluição piora a moradia).
- **Alimenta:** [SIS-TRB](SIS-TRB.md) (limite de população e de zona), [SIS-ECO](SIS-ECO.md) (consumo e impostos) e [SIS-CTL](SIS-CTL.md) (relatório).

## B. Funcionamento

**Ritmo:** casas e prefeituras trabalham uma vez por turno de 1.200 ticks (1 min); o dia tem 20 turnos (`mod.rs:36-38`).

### B.1 Casa e família

| Regra | Valor | Fonte |
|---|---|---|
| Moradores por casa | Até 4, aldeões a até 10 blocos; cada aldeão numa casa só | `CAPACITY`, `HOME_RADIUS` |
| Despensa | 3 slots; 1 ponto de comida por morador por turno (20 por dia, pouco mais de 3 pães) | `PANTRY_SLOTS`, `FOOD_PER_TURN` |
| Conforto | Gasta 1 por morador por turno; teto de 80 por morador | `COMFORT_PER_TURN`, `COMFORT_CAP` |
| Custo de vida | 40 moedas por morador por dia (30 em pão + 10 em bens) | `LIVING_COST` |
| Salário | 4 moedas por turno por empregado (2 × 40 ÷ 20): um salário sustenta duas pessoas | `WAGE_PER_TURN` |
| Poupança | A família guarda 2 dias de custo de vida antes de gastar em bens | `SAVINGS_DAYS` |
| Compras | O morador sem emprego vai à loja mais próxima (até 40 blocos) por 400 ticks | `SHOP_RADIUS`, `ERRAND` |
| Aglomeração | Mais de 6 casas a 16 blocos deixam a casa "apertada" | `CROWD_RADIUS`, `CROWD_LIMIT` |
| Bairro | Casas a até 24 blocos umas das outras formam um bairro | `DISTRICT_RADIUS` |

Fonte: `mod.rs:36-75`.

**Bens de conforto** (`GOODS`, `mod.rs:77-83`; preço base em moedas / pontos de conforto):

| Lã branca | Couro | Papel | Livro | Vela | Tocha | Frasco | Vaso | Mel | Quadro | Lanterna | Pena |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 4 / 8 | 6 / 12 | 2 / 4 | 10 / 20 | 4 / 8 | 1 / 2 | 2 / 4 | 3 / 6 | 5 / 10 | 8 / 16 | 6 / 12 | 1 / 2 |

Todos rendem 2 pontos de conforto por moeda. Escolher entre eles é questão de produção, não de eficiência.

### B.2 Alvará e padrão da fase

Uma casa só tem moradores se tiver **alvará** (`mod.rs:514`):
1. o número de casas licenciadas antes dela é menor que o limite de alvarás da fase;
2. a casa cumpre o padrão da fase: **poço** a até 24 blocos a partir da fase 2, **poste** a até 12 a partir da fase 3, **loja** a até 40 na fase 4 (`meets_standard`, `mod.rs:315-317`).

| Fase | Nome | Nível máx. de zona | População máx. | Alvarás | Imposto |
|---|---|---|---|---|---|
| 1 | Povoado (sem prefeito) | 2 | 30 | 4 | 4% |
| 2 | Vila | 3 | 60 | 10 | 8% |
| 3 | Cidade | 4 | 100 | 24 | 12% |
| 4 | Metrópole | 4 | 150 | 48 | 16% |

Fonte: `zone_limit`, `population_limit`, `permits` (`mod.rs:302-312`); `tax_rate` (`mayor.rs:13-15`).

### B.3 Prefeitura e fases

A prefeitura (`TOWN_HALL`) governa um raio de 128 blocos. Com um prefeito contratado, mede a vila a cada turno, decide a fase e aponta o pior problema (`mayor.rs:1-8`).

**Exigências para subir** (`PHASES`, `mayor.rs:17-22`). A vila sobe uma fase por vez (`next_phase`, `mayor.rs:130-135`):

| Para | Moradores com casa | Famílias | IDH | PIB/dia | Emprego | Gini máx. | Bairros |
|---|---|---|---|---|---|---|---|
| 2 Vila | 12 | 3 | 0,40 | — | — | — | — |
| 3 Cidade | 36 | 9 | 0,55 | 400 | 60% | — | — |
| 4 Metrópole | 80 | 20 | 0,70 | — | — | 0,45 | 3 |

- **Só promove com prefeito** (`staffed`, `mayor.rs:190`). Sem prefeito, a fase fica parada.
- **Suspensão:** se o IDH cair mais de 5 pontos abaixo do exigido pela fase atual, a prefeitura suspende novas licenças de zona e os nascimentos (`mayor.rs:196-198`; `mod.rs:322,332`).
- **Pior problema**, nesta ordem (`request`, `mayor.rs:142-153`): salários atrasados, cofre sem salário, fome, sem-teto, sem saneamento, sem comércio, sem energia (fase ≥ 2), desemprego, poluição, aglomeração. Sem problema, mostra a meta da próxima fase.
- **Nascimento com prefeito:** só onde houver vaga em casa com alvará (`birth_allowed`, `mod.rs:328-341`). Sem prefeitura, vale o limite do Povoado (30).

### B.4 Indicadores

| Indicador | Cálculo no código | Fonte |
|---|---|---|
| População | Moradores com casa, crianças, famílias; sem-teto contados à parte num raio de 128 | `mayor.rs:115-121`, `:155-162` |
| Emprego | % dos adultos com posto | `mayor.rs` |
| PIB | **Varejo do dia + exportações** (`DSold + DExport`) | `mayor.rs:187-189` |
| IDH da família | Média geométrica de 4 dimensões, piso 0,05 cada | `mod.rs:157-162` |
| IDH da vila | Média ponderada por pessoa; sem-teto entram com 0,05 e renda zero | `mayor.rs:116-121` |
| Gini | Renda por pessoa, ponderada; O(n log n) | `mayor.rs:48` |
| Bairros | Grupos de casas a 24 blocos | `mayor.rs:67`, `:115` |

**As 4 dimensões do IDH** (`mod.rs:603-613`):

| Dimensão | Fórmula |
|---|---|
| Comida | Turnos alimentados no dia × (70 + 10 × variedade, até 3) ÷ 100 |
| Moradia | 100 com alvará, 0 sem; × 0,7 se apertada; × 0,6 com poluição local > 400, ou × 0,3 acima de 1.200 |
| Serviços | (saneamento + energia + comércio) × 100 ÷ 3 |
| Renda | (renda ÷ (pessoas × 40)) × (50 + conforto/2) ÷ 100 |

- **O guia está errado sobre o PIB.** A página `veio.village.page.33` (lang pt_br `:1720` no pr47; na main é `page.24`, `:1033`) diz "tarefas feitas e vendas". O código soma varejo e exportações ([CNF-007](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
- **As obras não entram em serviços.** `SOCIEDADE.md` previa "serviços entram no IDH; prefeitura = obra". Nada em `society/` referencia `projects` (A7). Estado: Concebido.

### B.5 Loja e comerciante

A loja vende comida e bens às famílias e ao jogador. O comerciante é um aldeão contratado. Acima de 600 moedas de capital de giro (`SHOP_RESERVE`), o lucro vai para a família dele. Preço, atacado e importação estão em [SIS-MER](SIS-MER.md).

## C. Experiência do jogador

1. A pessoa põe casas espalhadas, para evitar o "apertado", e um poço perto.
2. Contrata um prefeito e abre o livro da prefeitura: bairros, pior problema e meta da próxima fase.
3. A vila chega a 12 moradores em 3 famílias com IDH 0,40 e vira Vila. Libera zona nível 3, 60 aldeões, 10 alvarás, e o imposto sobe para 8%.
4. Para chegar a Cidade, precisa de postes perto das casas, 60% de emprego e 400 moedas de PIB por dia.
5. Se a fábrica polui demais, a moradia cai, o IDH cai e a prefeitura suspende nascimentos e zonas.

Nada disso foi visto em jogo.

## D. Implementação

**Arquivos**
- `mod.rs`: casa, poço, turno da família, índices, limites da fase, `birth_allowed`.
- `mayor.rs`: fases, indicadores, Gini, bairros, relatório, suspensão.
- `market.rs` e `trade.rs`: ver [SIS-MER](SIS-MER.md). `ledger.rs`: ver [SIS-ECO](SIS-ECO.md).
- Registros em memória: `HOMES` e prefeituras. Recriados a cada turno; não persistem.
- Status 29 `HOUSED`, 30 `NO_PERMIT`, 31 `EMPTY_HOME` (`mod.rs:391-393`).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-VIL-01 Casas, famílias, carteira | 4 moradores; 10 blocos; despensa 3; salário 4/turno | `mod.rs:36-75` · idem · — | E2 |
| FUN-VIL-02 Lojas e comerciantes | Raio 40; reserva 600; lucro para a família do comerciante | `mod.rs`, `market.rs` · idem · — | E2 |
| FUN-VIL-03 Prefeitura, prefeito, fases | 3 transições em `PHASES`; promove só com prefeito; suspensão por IDH | `mayor.rs:17-22,130-135,196-198` · idem · — | E2 |
| FUN-VIL-04 Poço, energia e alvará | Poço 24 (fase 2), poste 12 (fase 3), loja 40 (fase 4); alvarás 4/10/24/48 | `mod.rs:302-317,514` · idem · — | E2 |
| FUN-VIL-05 IDH, Gini, bairros | 4 dimensões; geométrica; piso 0,05; bairro 24 | `mod.rs:157-162,603-613`; `mayor.rs:48,67` · idem · — | E2 |
| FUN-VIL-06 12 bens de conforto | 2 pontos por moeda | `mod.rs:77-83` · idem · — | E2 |

Estados: **Integrado** nas duas camadas. Ver [05](../05_ESTADO_REAL_DA_BUILD.md).

## E. Histórico

| Versão | Commit | Mudança |
|---|---|---|
| 0.18.0 | `0aa3fbb` | Sociedade: casas, lojas, prefeito, fases ([DEC-029](../06_HISTORICO_E_DECISOES.md#dec-029)) |
| 0.19.0 | `a48e14c` | Contas e impostos ([SIS-ECO](SIS-ECO.md)) |
| 0.21.0 | `9331dee` | Economia fechada; comerciantes independentes |
| 0.25.0 | `87ee998` | Salário pelo custo de vida; poupança que vira consumo |
| Obras | — | Integração "serviços no IDH" prevista e não feita |

A ideia anterior "Vila que vira cidade" ([DEC-038](../06_HISTORICO_E_DECISOES.md#dec-038), 07/11 23:03; casas variadas, praça, vida entre cama, sino e banca) ficou como Concebido. Parte dela foi superada pela sociedade da 0.18.

## F. Pendências

1. Corrigir a definição do PIB no guia (pr47 `page.33`; main `page.24`) ([CNF-007](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
2. Decidir se as obras entram no IDH como serviços. Hoje não entram.
3. Resolver os nomes cruzados: "Vila" e "Cidade" são fases da prefeitura e níveis de prestígio; "Prefeitura" é bloco e obra ([CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem)).
4. Decidir o peso da sociedade frente à fábrica antes de ampliar o sistema ([Q-12](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-12-peso-da-sociedade-frente-à-fábrica)).
5. Reduzir as 29 páginas do Guia da Vila acima de 14 linhas (contadas por idioma: 17 em pt_br e 12 em en_us) e corrigir as duas páginas "8." ([CNF-012](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
6. Teste em jogo: nenhum.

**Depois do corte:** o PR #54 (`2c49d41`, 0.27.0, sobre o #51) diz trazer a agenda da vila: feriados, domingos, visitantes, obras cívicas e ânimo no IDH. Sobrepõe-se a este sistema nos visitantes ([CNF-021](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)) e no IDH ([CNF-022](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)). Nada foi auditado ([TAR-065](../08_BACKLOG_E_PRIORIDADES.md#tar-065)).

### Tarefas no backlog

- [TAR-017](../08_BACKLOG_E_PRIORIDADES.md#tar-017): Serviços das obras no IDH
- [TAR-047](../08_BACKLOG_E_PRIORIDADES.md#tar-047): Nomes que colidem
- [TAR-048](../08_BACKLOG_E_PRIORIDADES.md#tar-048): Textos do guia desatualizados (PIB, Comércio)
- [TAR-060](../08_BACKLOG_E_PRIORIDADES.md#tar-060): Vila-cidade (casas variadas, praça, vida)

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** · pr47 **Integrado** · disperso — |
| Confiança | Média: E2 completo; sem E1 específico além do simulador econômico; J nenhum |
| Prioridade (sugestão) | Média: aguarda [Q-12](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-12-peso-da-sociedade-frente-à-fábrica) |
| Criticidade (sugestão) | Média: limita população e zonas, então trava a fábrica se falhar |
| Responsável conhecido | Claude nuvem ("Cloud"), versões 0.18–0.25 |
| DEC | [DEC-029](../06_HISTORICO_E_DECISOES.md#dec-029), [DEC-026](../06_HISTORICO_E_DECISOES.md#dec-026), [DEC-038](../06_HISTORICO_E_DECISOES.md#dec-038) |
| Q | [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem), [Q-06](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-06-teto-de-população-e-bônus-das-obras), [Q-12](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-12-peso-da-sociedade-frente-à-fábrica) |
| CNF | [CNF-021](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (visitantes do #54 × funcionários), [CNF-022](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (IDH do #54 × obras no IDH, ligado a [Q-06](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-06-teto-de-população-e-bônus-das-obras)), [CNF-007](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (PIB), [CNF-008](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (bônus × teto), [CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (nomes), [CNF-012](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (guia) |
| Referências | `frentes/SOCIEDADE.md`, `frentes/ECONOMIA.md`, `frentes/VILA-CIDADE.md`; Guia da Vila (`veio.village.page.*`); A2 VIL; A7 (PIB, serviços) |
