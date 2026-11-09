# SIS-MER: Mercado e comércio exterior

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. O código de `workforce/society/*` é idêntico nas duas.
- **Linhas citadas:** salvo indicação, valem para `native/api/src/workforce/society/`.

## A. Identidade

O sistema dá destino à produção e é o motor que permite uma indústria maior que a vila. A ECONOMIA.md resume: "O comércio exterior é o que sustenta uma indústria maior" (E3; números do modelo em E1).

- **Duas partes:**
  1. o **mercado local**: lojas com comerciante, preço por oferta e procura, atacado;
  2. o **comércio exterior**: porto, estação de fronteira, bolsa mundial, contratos, reputação e importação.
- **Inspiração:** conexões externas de Cities: Skylines, declarada na 0.23 ([Agente]).
- **Depende de** [SIS-PRO](SIS-PRO.md) (produtos), [SIS-ECO](SIS-ECO.md) (livros, imposto) e [SIS-VIL](SIS-VIL.md) (famílias que compram).
- **Alimenta** [SIS-ECO](SIS-ECO.md) (receita da companhia) e [SIS-VIL](SIS-VIL.md) (PIB, comida).

## B. Funcionamento

**Unidades.** 1 esmeralda = 60 moedas. Os preços internos são guardados em centavos de moeda (`reference_cents`). O dia tem 20 turnos de 1.200 ticks. Ver [SIS-ECO](SIS-ECO.md#b-funcionamento).

### B.1 Preço de referência (a âncora)

O preço de referência de um produto é `valor-base × fator de camada`:

| Camada | Fator |
|---|---|
| Matéria-prima | 100% |
| Processado | 150% |
| Componente | 200% |
| Eletrônico | 300% |
| Módulo | 400% |

O mesmo valor vale dentro e fora da vila. O pão vale 9 na loja e 9 × índice na bolsa.

Antes da 0.25, o pão valia 6 na loja e 9 lá fora, então exportar sempre ganhava e a vila passava fome. Foi isso que motivou a âncora única (ECONOMIA.md, E3).

Fora da bolsa, a comida vale os pontos da despensa: ×1,5 se for preparada (assados, tortas, sopas), ×1 se for crua. Os bens de conforto seguem a tabela `GOODS` (12 itens: lã 4/8, couro 6/12…).

### B.2 Preço local (cada loja tem o seu)

O **fator da loja** varia de 50% a 200% do preço de referência. Preço local = referência × fator, com mínimo de 1. A cada turno:

1. **Alvo.**
   - Sem procura: o alvo é o fator atual − 5, com piso de 70 (estoque parado barateia até 70%).
   - Com procura: o alvo é `100 / √(estoque ÷ procura diária)`, com cobertura mínima de 0,25. Isso dá o fator máximo de 200 quando há 1/4 de dia de estoque.
   - Se faltou comida no turno, o alvo sobe 25.
2. **Passo.** O fator anda 1/4 da distância até o alvo.
3. **Teto de importação.** Com conexão externa, o fator não passa de `import_ceiling`: o custo de importar com margem de 10%, já descontado o imposto da fase. Resultado: a inflação para no preço de importar.
4. **Procura.** É a média móvel diária das unidades vendidas. A loja lembra até 12 itens.

### B.3 Atacado e liquidação

- **Entrega.** O que a companhia entrega no estoque da loja (à mão, por funil ou por carregador) vira uma **dívida da loja com a companhia**, chamada "Payable". O valor é preço local × **75%** × unidades novas, lançado por divisão da holding. A entrega registra também `MADE` (produção) na prefeitura.
- **Acerto** (`settle`). A cada turno a loja paga `min(dívida, caixa da loja)`, e só se existir o cofre da companhia na dimensão. Do valor pago, retém o **imposto da fase** para a prefeitura; o resto entra na companhia.
- **Varejo.** As famílias e o jogador compram a preço local. O imposto é retido e o resto fica no caixa da loja.
- **Consequência:** o mesmo bem é tributado **duas vezes**, no varejo e no atacado ([Q-13](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-13-imposto-uma-ou-duas-vezes), [CNF-015](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
- **Venda ao jogador.** O jogador compra pela tela de troca vanilla. O lote é o maior que cabe em 1 esmeralda, até 16 unidades. Um item acima de 60 moedas vai sozinho, com o preço arredondado para cima em esmeraldas.

### B.4 Comércio exterior: conexões

| Conexão | Bloco | Exigência | Capacidade e horário | Frete |
|---|---|---|---|---|
| Porto | `veio:export_dock` | ≥ 20 blocos de água (mar ou rio largo) | 9 pilhas a cada 2 turnos (navio a cada 2 min) | 4 + 6 por pilha: barato e lento |
| Estação de fronteira | `veio:freight_station` | Na fronteira, a 96 blocos da sede | 6 pilhas por turno (trem por minuto) | 2 + 12 por pilha: caro e frequente |

Status de painel: 32 sem companhia, 33 sem água, 34 fora da fronteira, 35 mercado fechado.

O frete é **por pilha**, então pesa onde o valor é baixo. Na estação, o frete é cerca de 10% do valor do minério de ferro e cerca de 1% da engrenagem (ECONOMIA.md). A carga de um mercado fechado fica esperando no porto.

### B.5 Bolsa mundial

- **Catálogo.** 34 produtos com valor-base e camada. Os itens da grande indústria (motor, processador, núcleo, kit Mk.II) **não estão no catálogo**.
- **Cotação do dia.** Cheio × índice diário. O índice é determinístico, entre 80% e 120%.
- **Saturação.** À vista = `cheio × absorção / (absorção + pressão)`. Com pressão igual à absorção, a cotação cai à metade.
  - Absorção diária por camada: 256 / 512 / 512 / 256 / 128. Matéria-prima e nicho de alta tecnologia absorvem menos.
  - Pressão = metade da média recente + o que foi vendido hoje. Com isso, o mercado se recupera em alguns dias.
- **Piso de sucata.** 25% do cheio: inundar o mercado nunca zera a receita.
- **Exemplo do modelo** (ECONOMIA.md, E1 no simulador):
  - 256 engrenagens por dia rendem 2.418; 256 minérios de ferro rendem 263;
  - 1.024 circuitos por dia rendem cerca de 2× o que rendem 256 por dia.
- **Escopo.** A bolsa e a reputação são **por dimensão**.

### B.6 Contratos e reputação

| Regra | Valor |
|---|---|
| Contratos simultâneos | Até 3 |
| Prazo | 3 a 4 dias |
| Repetição | Um produto não se repete entre contratos |
| Lote | `256 / (camada + 1)`, arredondado a 16 (mínimo 16), + (n mod 4) × 16. Cerca de 256 / 128 / 80 / 64 / 48 por camada |
| Pagamento | Preço cheio do dia **+50%**, **sem saturação**: o contrato é um comprador à parte e não derruba a cotação à vista. Prêmio mínimo para itens baratos |
| Reputação | Cada contrato cumprido dá +1. As camadas abrem com reputação **1, 3 e 6** (componentes, eletrônicos, módulos). **Camada aberta nunca fecha** |
| Persistência | NBT do centro, formato v3 |

### B.7 Importação de comida

- **Quando.** A loja fica sem a comida que mais vende (`STAPLE` = pão).
- **Quanto.** Até 16 unidades por evento.
- **Custo.** Cotação mundial + 20% de frete + 10% de tarifa. A moeda **sai** da economia, mas a tarifa fica na prefeitura.
- **Margem.** O comerciante põe 10% sobre o importado. Por isso, importar nunca dá prejuízo à loja, e o preço local nunca passa do custo de importar.

### B.8 Papel estratégico

O sistema empurra o jogador para cadeias longas:

1. o valor agregado manda;
2. o frete castiga o minério;
3. os contratos sobem a reputação, e a reputação abre eletrônicos e módulos.

No modelo, com porto, o pão local sobe até perto do teto (12,4 a 14,6) e a vila continua 100% alimentada. Sem conexão, a companhia empata com a vila, menos o imposto (−83 por dia). Ver [SIS-ECO](SIS-ECO.md#b-funcionamento).

## C. Experiência do jogador

- **No mercado local**, o jogador:
  - contrata o comerciante e abastece o estoque;
  - vê os preços subirem quando falta e caírem quando sobra;
  - compra na tela de troca vanilla.
- **No comércio exterior**, ele:
  - constrói um porto no mar ou uma estação na fronteira;
  - acompanha a "bolsa do dia" no relatório e a aba *Comércio exterior* do [centro de controle](SIS-CTL.md);
  - recebe avisos de contrato e de mercado aberto no chat;
  - vê as partidas com buzina e fumaça (0.24).

Nada disso foi jogado (J = nenhum).

## D. Implementação

**Arquivos**
- `market.rs`: preço local, atacado, importação, venda ao jogador.
- `trade.rs`: conexões, bolsa, contratos, persistência.
- O STAPLE (pão) está em `trade.rs:84`.

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-MER-01 Preço local dinâmico | Fator 50–200; ocioso ≥ 70; alvo 100/√cobertura; +25 se falta comida; passo 1/4; teto de importação ([DEC-032](../06_HISTORICO_E_DECISOES.md#dec-032)) | `market.rs:26-34`, `:90` (target), `:104` (update), `:145` (import_ceiling) · idem · — | E2 (A2); E1 `balance_*` via `check_project` e `sim_economy.py --check` = ok (modelo); J nenhum |
| FUN-MER-02 Atacado 75% e liquidação | Payable = local × 75%; acerto limitado ao caixa, com imposto retido | `market.rs:200` (receive_deliveries), `:221` (settle; a saída vai para a categoria `WAGES` da loja, rótulo discutível), `:246` (receive_sale) · idem · — | E2; a dupla tributação foi achada por leitura (A2) |
| FUN-MER-03 Importação de comida | Mundial + 20% + 10%; margem 10%; até 16 por evento | `market.rs` (importação), `trade.rs:28-43` · idem · — | E2 |
| FUN-MER-04 Porto `export_dock` | ≥ 20 de água; 9 pilhas / 2 turnos; frete 4 + 6/pilha | `trade.rs:504` (schedule), status `:37-40` · idem · — | E2 |
| FUN-MER-05 Estação `freight_station` (comércio) | Fronteira a 96; 6 pilhas/turno; frete 2 + 12/pilha ([DEC-044](../06_HISTORICO_E_DECISOES.md#dec-044)) | `trade.rs:27` (STATION) · idem · — | E2. **Sem migração** de mundos com o terminal antigo do Codex ([CNF-014](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)) |
| FUN-MER-06 Bolsa mundial (34 produtos, saturação, sucata) | Camadas, índice 80–120, absorção, pressão, sucata 25% | `trade.rs:46` (PRODUCTS), `:106` (index), `:128` (absorb), `:132-134` (pressure), `:141` (spot_cents) · idem · — | E2; E1 no modelo |
| FUN-MER-07 Contratos com prazo e reputação | ≤ 3; 3–4 dias; +50%; reputação 1/3/6; camada nunca fecha | `trade.rs` (contratos, `TIER_AT`), persistência `:389` (VERSION 3, chamada em `workforce/mod.rs:959`) · idem · — | E2; E1 regra 7 do simulador |
| FUN-MER-08 Venda ao jogador em lotes | Até 16 por esmeralda; item caro sozinho | `market.rs:310` (bundle_at) · idem · — | E2 |

## E. Histórico

| Versão | Mudança |
|---|---|
| 0.18.0 (`0aa3fbb`) | Lojas e comerciantes |
| 0.19.0 | Preços locais por oferta e procura |
| 0.22.0 (`cbc53a9`) | Cais de exportação |
| 0.23.0 (`e8ac991`) | Refeito como conexões externas |
| 0.24.0 | Avisos; carga esperando; bolsa e reputação salvas |
| 0.25.0 (`87ee998`) | Âncora única, teto de importação, atacado 75% (era 70% na 0.21), sucata, frete por pilha |

Tudo é do agente Cloud ([DEC-031](../06_HISTORICO_E_DECISOES.md#dec-031), [DEC-032](../06_HISTORICO_E_DECISOES.md#dec-032)). Em [DEC-044](../06_HISTORICO_E_DECISOES.md#dec-044) o dono decidiu que o ID `freight_station` fica com este sistema.

## F. Pendências

1. **Itens da grande indústria fora da bolsa.** Decidir se exportam ([Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros)/[Q-03](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão)).
2. **Dupla tributação.** Ver [Q-13](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-13-imposto-uma-ou-duas-vezes).
3. **Guia desatualizado.** A página "Comércio" do Guia principal ainda descreve a exportação pela central de contratos, não o porto e a estação.
4. **Escala.** A importação de 16 unidades por evento não foi medida com 150 aldeões (ND).
5. **Índice previsível.** O índice é determinístico e qualquer jogador que conheça a fórmula o prevê (decisão de produto não tomada).
6. **Rótulo contábil.** A saída do acerto aparece como `WAGES` no livro da loja.
7. **Mundos com o terminal antigo do Codex.** Mundos da 0.16.1 do Codex têm o bloco `freight_station` exportando a carga que era do terminal ferroviário. Falta nota de migração ou conversão ([CNF-014](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
8. **Simulador.** `sim_economy.py` copia as constantes do Rust à mão.
9. **Teste em jogo:** nenhum.

### Tarefas no backlog

- [TAR-007](../08_BACKLOG_E_PRIORIDADES.md#tar-007): Proteger saves 0.16.1 e Realismo 1.1 da troca de sentido de `veio:freight_station`
- [TAR-048](../08_BACKLOG_E_PRIORIDADES.md#tar-048): Textos do guia desatualizados (PIB, Comércio)
- [TAR-056](../08_BACKLOG_E_PRIORIDADES.md#tar-056): Grande indústria na bolsa
- [TAR-057](../08_BACKLOG_E_PRIORIDADES.md#tar-057): Bolsa: índice previsível e importação em escala

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** · pr47 **Integrado** · disperso — |
| Confiança | Média: E2 em todo o código, E1 só no modelo Python; J nenhum |
| Prioridade (sugestão) | Alta: é o motor do loop médio |
| Criticidade (sugestão) | Média: balanço, não save |
| Responsável conhecido | Agente Cloud (`claude/trusting-ritchie-sc9ty8`); sem frente ativa |
| DEC | [031](../06_HISTORICO_E_DECISOES.md#dec-031), [032](../06_HISTORICO_E_DECISOES.md#dec-032), [044](../06_HISTORICO_E_DECISOES.md#dec-044) |
| Q | [Q-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-01-esmeraldas-criadas-fora-dos-livros), [Q-03](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão), [Q-13](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-13-imposto-uma-ou-duas-vezes) |
| CNF | [CNF-011](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (grande indústria fora da bolsa), [CNF-015](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (dupla tributação), [CNF-014](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (`freight_station` sem migração) |
| Referências | `frentes/ECONOMIA.md`, `frentes/SOCIEDADE.md`, README 0.22–0.25 |
