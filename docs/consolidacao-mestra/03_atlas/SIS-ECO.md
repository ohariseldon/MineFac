# SIS-ECO — Economia e contabilidade

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md) · bases `main@53267af`, `pr47@80980db`. Os arquivos `workforce/society/*` são idênticos nos dois ramos. Os caminhos citados são relativos a `native/api/src/`.

## A. Identidade

O dinheiro do Veio circula entre livros. Assim, a indústria tem consumidores, e não só "missões". O desenho é do agente Cloud (0.19–0.25). O dono decidiu um único fluxo: as recompensas de pedidos e da obra Prefeitura saem do caixa público ([DEC-035](../06_HISTORICO_E_DECISOES.md#dec-035)).

O README afirma que "o dinheiro só entra ou sai em esmeraldas" ([DEC-030](../06_HISTORICO_E_DECISOES.md#dec-030)). **O código não sustenta isso.** Há bordas sem esmeralda e emissões fora do livro (ver B.4 e B.5).

- Depende de [SIS-VIL](SIS-VIL.md) (famílias, prefeito), [SIS-MER](SIS-MER.md) (preços, comércio) e [SIS-OBR](SIS-OBR.md) (pagamentos).
- É exibido pelo [SIS-CTL](SIS-CTL.md).

## B. Funcionamento

### B.1 Unidades e ritmo

| Item | Valor |
|---|---|
| Moeda | 1 esmeralda = 60 moedas (`COINS_PER_EMERALD`) |
| Turno | 1.200 ticks; 20 turnos por dia de 24.000 ticks |
| Custo de vida | 40 por pessoa por dia: 20 pontos de comida (cerca de 3,3 pães × 9 = 30) + 20 de conforto (cerca de 10 em bens) |
| Salário | `WAGE_PER_TURN = 2 × 40 / 20` = **4 por turno (80 por dia)**. Sustenta duas pessoas: família de 4 com 2 trabalhadores fecha a conta; com 1, depende de assistência |

### B.2 Os livros

| Livro | Onde vive | Quem paga e recebe |
|---|---|---|
| **Company** | Cofre do centro de controle, procurado em qualquer ponto da dimensão. Slot 4 deposita, slot 5 saca até 16 esmeraldas | Paga os salários dos trabalhadores. Recebe o atacado das lojas, a exportação e as recompensas públicas. É também o **"caixa da vila" das obras** |
| **Public** | Prefeitura que governa a posição, num raio de 128 | Recebe o imposto e a tarifa de importação. Paga o prefeito e os servidores, os poços e a assistência. Paga a recompensa pública |
| **Shop** | Caixa de cada loja | Recebe o varejo. Paga o atacado, o salário do comerciante e a importação |
| **Carteira** | NBT "Wallet" de cada casa | Recebe salários e assistência. Paga as compras |

Os saldos vivem em `static Mutex` e são copiados para o NBT do bloco dono a cada ciclo. Por isso sobrevivem ao reinício.

Os contadores por divisão da holding e o `BORDERS` ficam **só em memória** e zeram ao reiniciar ([SIS-PER](SIS-PER.md)).

### B.3 Circuito interno

```
Companhia --salário 4/turno, por divisão--> Carteiras
Carteiras --compra a preço local (imposto retido)--> Loja
Loja --acerto do atacado 75% (imposto retido de novo)--> Companhia
Loja --comerciante: 4 + max(0, caixa − 4 − 600)/10 por turno--> Carteira do comerciante
Prefeitura --servidor público; assistência 1/pessoa/turno se riqueza < 40 e caixa público > 200--> Carteiras
Prefeitura --reward: pedidos da vila, juros da obra Prefeitura--> Companhia      (pr47)
Carteira com > 2 dias de custo de vida --1/4 do excedente por turno--> bens de conforto na loja
```

- Os salários não pagos acumulam em `Owed` e `OwedPublic`, com limite de 1 dia.
- O imposto da fase (4%, 8%, 12%, 16%) é retido em **cada venda**, tanto no varejo quanto no atacado. Portanto, o mesmo bem é tributado duas vezes ([Q-13](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-13-imposto-uma-ou-duas-vezes)).

### B.4 Bordas: por onde a moeda entra e sai

Esta lista vem de `ledger.rs:159-167`.

| Borda | Sentido |
|---|---|
| Depósito de esmeraldas no cofre | + |
| Saque de esmeraldas | − |
| Exportação e contratos | + (cria moeda) |
| Compras do jogador na loja | + |
| Importação e frete | − (destrói moeda) |
| Obras e melhoria de zona pagas com o caixa (`add_coins` negativo, categoria REDEEM) | − (destrói moeda) |

O `BORDERS` conta essas bordas por dimensão, **mas só em memória**.

### B.5 Fontes de moeda fora do livro

Estas fontes não passam por `border` nem pelo `sim_economy.py`. São esmeraldas **como item**:

| Fonte | Valor | Onde |
|---|---|---|
| Central: EXPORT perpétuo | 4 esmeraldas por 4 circuitos + 4 aço, **sem limite, também na main** | [SIS-PRG](SIS-PRG.md) |
| Contrato "village" (Povoado) | 8 esmeraldas | [SIS-PRG](SIS-PRG.md) |
| Grande projeto | 32 / 48 / 64 esmeraldas | [SIS-PRG](SIS-PRG.md) |
| Pedido do dia do Quadro | 1 a 6 esmeraldas no slot 1, uma vez por dia por dimensão, em memória | [SIS-PRG](SIS-PRG.md) |
| Armazém do povoado | 1 esmeralda a cada 60 pontos de comida; +50% com 3 aldeões a 48 blocos | [SIS-AGR](SIS-AGR.md) |
| Mk.II em jazida de esmeralda | 1 a cada 20 s (normal) ou 10 s (pura); 4.800 FE | [SIS-EXT](SIS-EXT.md) |
| Trocas vanilla | — | — |

A esmeralda dessas fontes só entra no livro quando o jogador a deposita no cofre.

### B.6 Holding

São 11 abas: divisões 1–9 mais o exterior (`FOREIGN 10`). As empresas são mineração, manufatura, energia, logística, agro, extrativismo, urbanização, comércio e gestão e segurança.

Cada empresa mostra receita, gastos, lucro, folha estimada, produção do dia e o que está parado. A amostra é de 100 (`SAMPLE`). O relatório da prefeitura exibe a moeda em circulação por conta.

### B.7 Equilíbrio

Há 7 regras de equilíbrio. Elas são testes `balance_*` e também `sim_economy.py --check`:

1. um salário paga duas pessoas;
2. o teto cobre importação mais imposto;
3. no teto, o atacado rende como exportar;
4. o frete pesa pelo menos 5 vezes mais no minério que na engrenagem;
5. o pão vale pelo menos 4 vezes os 2 trigos;
6. exportar a absorção rende pelo menos metade do cheio, nunca abaixo da sucata;
7. o contrato paga mais e não derruba a cotação.

O simulador **não modela** as fontes do B.5 nem a dupla tributação.

## C. Experiência do jogador

- O cofre do centro é a conta da empresa do jogador.
- Ele vê salários saindo, o atacado entrando e o lucro por empresa nas abas da holding.
- A esmeralda física é a ponte com o mundo vanilla: ele deposita para pagar folha e obras, e saca para comprar.
- Em jogo, o atalho mais visível é a central de contratos, que dá esmeralda sem passar pela economia.

## D. Implementação

`ledger.rs` (livros, bordas, `withhold`), `mod.rs` (constantes, salários, assistência), `mayor.rs` (`tax_rate`), `holding.rs`, `workforce/center.rs` (cofre, caixa, `reward`).

Categorias de fluxo: MADE 0, SOLD 1, WAGES 2, TAXES 3, RECEIVED 4, ASSIST 5, REDEEM 6, EXPORTS 7.

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-ECO-01 Moeda e contas | 60 moedas por esmeralda; livros Company, Public, Shop e carteiras | `mod.rs:40`; `ledger.rs:48`, `:112` (`ANYWHERE`), `:211-214` (`COMPANY_VAULT 4`, `COMPANY_CASHOUT 5`, `CASH_OUT 102`) · idem · — | E2 (A2) |
| FUN-ECO-02 Imposto na fonte por fase | 4/8/12/16%, retido no varejo e no atacado | `mayor.rs:13` (`tax_rate`); `ledger.rs:129` (`withhold`) · idem · — | E2 |
| FUN-ECO-03 Salário por empregador | 4/turno; comerciante com lucro distribuído; servidor pela prefeitura; `Owed` até 1 dia | `mod.rs:447-487` (`wages`) · idem · — | E2; E1 regra 1 (modelo) |
| FUN-ECO-04 Assistência pública | 1 por pessoa por turno se riqueza < 40 e caixa > 200 (`ASSIST_RESERVE`) | `mod.rs:547-549` · idem · — | E2 |
| FUN-ECO-05 Holding: lucro por divisão | 11 abas; só em memória | `holding.rs` · idem · — | E2 |
| FUN-ECO-06 Bordas de moeda (`BORDERS`) | Conta as bordas do B.4 por dimensão | `ledger.rs:48`, `:159-167`, `:199`, `:243`; `trade.rs:336-337`, `:365`, `:598`; `market.rs:464` · idem · — | E2 (A7) |
| FUN-ECO-07 Recompensas pagas pelo caixa público | Public → Company com rótulo RECEIVED; sem prefeitura, devolve 0 e dá só prestígio ([DEC-035](../06_HISTORICO_E_DECISOES.md#dec-035)) | ausente · `workforce/center.rs:609-620` · zip das obras (contido) | E2 |
| FUN-ECO-08 Rendimento da obra Prefeitura | Juros = (caixa Company ÷ 16), no máximo 8 por dia, pagos pelo Public via `reward` | ausente · `projects.rs:475-476`; `center.rs:592-601` · — | E2 (A7) |
| FUN-ECO-09 Fontes externas de moeda (fora do livro) | Tabela do B.5. **Defeito de desenho, não funcionalidade** | `contracts.rs:23`, `:35-49`, `:63-66`; `production/pantry.rs:1-21` · idem + `milestones/mod.rs:147-150` · — | E2 (A7 §1 #3) |

## E. Histórico

| Versão | Commit | Mudança |
|---|---|---|
| 0.19.0 | `a48e14c` | Contas companhia e pública; impostos |
| 0.20.0 | `95a1f7c` | Holding |
| 0.21.0 | `9331dee` | "Economia fechada": imposto na fonte, comerciantes independentes, atacado a 70% |
| 0.25.0 | `87ee998` | Salário pelo custo de vida, atacado a 75%, repasse do comerciante, poupança que vira consumo, simulador |
| pr47 | — | `reward` ([DEC-035](../06_HISTORICO_E_DECISOES.md#dec-035)) |

As fontes de esmeralda do B.5 vêm de antes (0.5, 0.9) ou de outras frentes (Quadro, #40) e nunca foram reconciliadas com o livro.

## F. Pendências

1. Decidir o destino das fontes externas ([Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros)): registrar, converter ou remover.
2. Decidir sobre a dupla tributação ([Q-13](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-13-imposto-uma-ou-duas-vezes)). Recalibrar o simulador.
3. Persistir os contadores da holding e o `BORDERS`, que hoje zeram ao reiniciar. É tarefa técnica de [SIS-PER](SIS-PER.md).
4. Fazer o `sim_economy.py` ler as constantes do Rust em vez de copiá-las à mão.
5. Corrigir o rótulo `WAGES` no acerto do atacado (`market.rs:236`).
6. Reescrever o README e a [DEC-030](../06_HISTORICO_E_DECISOES.md#dec-030) para descrever as bordas reais.
7. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-029](../08_BACKLOG_E_PRIORIDADES.md#tar-029): Persistir o que some ao reiniciar
- [TAR-035](../08_BACKLOG_E_PRIORIDADES.md#tar-035): Contabilizar a moeda criada fora do livro
- [TAR-036](../08_BACKLOG_E_PRIORIDADES.md#tar-036): Simulador econômico ligado ao código
- [TAR-042](../08_BACKLOG_E_PRIORIDADES.md#tar-042): Imposto uma ou duas vezes

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** · pr47 **Integrado** · disperso — |
| Confiança | Média: E2 completo; E1 só no modelo, que não cobre as emissões |
| Prioridade (sugestão) | Alta |
| Criticidade (sugestão) | Alta para o desenho (o loop médio); baixa para o save |
| Responsável conhecido | Agente Cloud; `reward` pelo agente das obras e pelo integrador do #47 |
| DEC | [030](../06_HISTORICO_E_DECISOES.md#dec-030), [032](../06_HISTORICO_E_DECISOES.md#dec-032), [035](../06_HISTORICO_E_DECISOES.md#dec-035), [022](../06_HISTORICO_E_DECISOES.md#dec-022) |
| Q | [Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros), [Q-02](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-02-dois-pedidos-diários), [Q-13](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-13-imposto-uma-ou-duas-vezes) |
| CNF | [CNF-001](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (emissões), [CNF-015](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (dupla tributação) |
| Referências | `frentes/ECONOMIA.md`, `frentes/SOCIEDADE.md`, `tools/sim_economy.py` |
