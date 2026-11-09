# Validação independente da consolidação mestra

- **Objeto:** `docs/consolidacao-mestra/` (00–11 e 25 fichas `03_atlas/SIS-*.md`), lido em 2026-11-08 depois da última alteração do 00.
- **Código:** `main@53267af` em `/home/user/MineFac` e `pr47@80980db` em `.../scratchpad/pr47`.
- **Restrições respeitadas:** nenhum arquivo editado e nenhum git que mude estado. As ferramentas auxiliares ficaram em `scratchpad/tools/`.
- **Contagem de saltos:** conta cada link seguido a partir do `00_INDICE_E_NAVEGACAO.md`. Rolar dentro do mesmo arquivo não conta.

---

## 1. Consultas representativas

Legenda:
- **Achada:** S (sim), P (parcial), N (não).
- **Coerente:** S, P ou N, conforme os arquivos concordam.
- **Evidência:** S quando o código confere, P quando confere com erros de linha, N quando não há evidência.

### 1.1 SIS-VIL: prefeitura e sociedade da vila

| # | Pergunta | Resposta (só dos docs) | Onde | Saltos | Achada | Coerente | Evidência |
|---|---|---|---|---|---|---|---|
| 1 | Como funciona? | Turno de 1.200 t. Casas com até 4 moradores e alvará pelo padrão da fase. O prefeito mede IDH, Gini, PIB e bairros, sobe uma fase por vez (12/3/0,40 → 36/9/0,55/400/60% → 80/20/0,70/Gini 0,45/3 bairros) e suspende licenças se IDH + 5 < exigido | SIS-VIL §B.1–B.4 | 1 | S | S, com 05 §1.11 | S. `mod.rs:36-83,302-341,514,603-613` e `mayor.rs:13-22,130-153,187-198` conferem. Única imprecisão: `staffed` está em `mayor.rs:170/189`, e a ficha diz `:190` (o 05 diz `:189`) |
| 2 | Onde está a implementação? | `workforce/society/mod.rs` e `mayor.rs`. Status 29–31. Código idêntico nas duas camadas | SIS-VIL §D; 05 §1.11; 04 §10.1 | 1 | S | S | S (`diff -rq` confirma que são idênticos) |
| 3 | O que já foi concluído? | 6/6 FUN Integrado em main e pr47; nada Validado; J vazio | SIS-VIL §G; 05 §1.11 | 1 | S | S | S para a existência. "Concluído" só no sentido offline |
| 4 | O que está pendente? | PIB no guia (CNF-007), obras no IDH, nomes cruzados, peso da sociedade, guia longo, teste em jogo | SIS-VIL §F | 1 | S | **P**, ver abaixo | P |
| 5 | Quem depende dele? | TRB, ECO e CTL ("Alimenta") | SIS-VIL §A; 03 §2–3 | 1–2 | S | **P**: a tabela 03 §3 diz "TRB (limite), CTL"; o Mermaid inclui AGR; SIS-MER diz depender de VIL | — |
| 6 | Que agente? | Claude "Cloud", 0.18–0.25 | SIS-VIL §G; 07 §1.1 | 1–2 | S | P: o #54 (Cloud, agenda da vila e ânimo no IDH) mexe no sistema e não aparece em §D/§F nem no CNF da ficha | S (DEC-029) |
| 7 | Próxima tarefa? | Não está dito. Por inferência: TAR-048 (P2, sem dependência). TAR-065 é P1, mas está bloqueada; TAR-017 e TAR-047 também estão bloqueadas | 00 coluna Tarefas → 08 (5 TAR) | 1 por TAR (5 aberturas) | **P** | — | — |
| 8 | Decisão do dono? | Q-12 (peso), Q-04 (nomes), Q-06 (teto e bônus) | SIS-VIL §G → 09 §5 | 2 | S | **N** para "obras no IDH", ver abaixo | — |

**Dificuldades em SIS-VIL**
- A decisão "obras entram no IDH" aponta para quatro destinos diferentes:
  - a ficha §F.2 não cita Q;
  - a TAR-017 diz "aguarda Q-06";
  - o 03 §5 diz Q-12 e "Sem CNF";
  - o 09 tem a CNF-022 exatamente para isso, e o 09 §4.2 trata o tema como lacuna sem Q.
- A ficha não cita a CNF-022 nem a CNF-021.
- O "29 páginas acima de 14 linhas" (§F.5) conta página × idioma. O log tem 17 entradas pt_br e 12 em en_us.

### 1.2 SIS-MER: mercado e comércio exterior

| # | Pergunta | Resposta | Onde | Saltos | Achada | Coerente | Evidência |
|---|---|---|---|---|---|---|---|
| 1 | Como funciona? | Âncora = base × fator de camada (100–400%). Fator da loja 50–200, com alvo 100/√cobertura, passo 1/4 e teto de importação. Atacado 75% com acerto limitado ao caixa. Porto 9 pilhas a cada 2 turnos, frete 4+6. Estação 6 pilhas por turno, frete 2+12. Bolsa de 34 produtos com índice 80–120, absorção 256/512/512/256/128 e sucata 25%. Contratos ≤3, +50%, reputação 1/3/6 | SIS-MER §B | 1 | S | S | S: `market.rs:26-34,90-150,200-255,310` e `trade.rs:27-43,46-84,106-141,263,390,504-505` conferem |
| 2 | Onde está a implementação? | `market.rs` e `trade.rs`; persistência no NBT do centro, v3 | SIS-MER §D | 1 | S | S, com 05 §1.13 | S |
| 3 | O que já foi concluído? | 8/8 FUN Integrado nas duas camadas | §G; 05 §1.13 | 1 | S | S | S |
| 4 | O que está pendente? | 9 itens: grande indústria fora da bolsa, imposto duplo, guia "Comércio", importação em escala, índice previsível, rótulo `WAGES`, migração `freight_station`, simulador, J | §F | 1 | S | P: §F.1 cita "Q-01/Q3" ("Q3" não existe; o certo é Q-03). §F.5 é decisão de produto sem Q | S: `veio.guide.page.11` ainda fala da central |
| 5 | Quem depende dele? | ECO e VIL | §A; 03 §3 | 1–2 | S | P: o 03 §3 diz só ECO, e o CTL depende de MER na mesma tabela | — |
| 6 | Que agente? | Cloud (`claude/trusting-ritchie-sc9ty8`) | §E e §G; DEC-031/032 | 1–2 | S | S | S |
| 7 | Próxima tarefa? | Não está dito. Por inferência: TAR-007 (P1, sem bloqueio), que é sobretudo de TRA/PER. Depois TAR-048 | 00 → 08 (4 TAR) | 1 por TAR | P | — | — |
| 8 | Decisão do dono? | Q-01, Q-03, Q-13; a DEC-044 já foi decidida pelo dono | §G → 09 §5 | 2 | S | P: falta Q para o índice previsível (TAR-057 "decisão sobre a semente") | — |

**Dificuldades em SIS-MER**
- O 00 "Mapa por tema" chama de "estação de carga" a estação de exportação (SIS-MER) e também a `cargo_station` (SIS-TRA). O 01 §1 repete "exporta… pela estação de carga". É a colisão da Q-04 dentro do próprio índice.

### 1.3 SIS-TRA: fábrica distante e estações

| # | Pergunta | Resposta | Onde | Saltos | Achada | Coerente | Evidência |
|---|---|---|---|---|---|---|---|
| 1 | Como funciona? | Canal por corante (16), 8 itens a cada 8 t, 10 FE por item, 4.096 no canal, janela de 1.200 t. Após 100 t sem rodar, `simulate` soma ritmo × tempo sem insumo nem energia, com canal global. Também cobre drones, caravanas, terminal, elevador e âncora | SIS-TRA §B | 1 | S | S, com DEC-043, CNF-009 e R-13 | **P**: valores certos, mas várias linhas erradas (§2 #8) |
| 2 | Onde está a implementação? | `stations.rs` (384 linhas), `freight.rs` (355), `freight/caravan.rs` (248), `lift.rs` (150), `industry.rs` (399); só no pr47 | §D | 1 | S | S | P: os tamanhos estão exatos; os testes estão imprecisos (§2 #9) |
| 3 | O que já foi concluído? | 7/7 Integrado no pr47; ausente na main | §G; 05 §1.9 | 1 | S | **N** na confiança: o 03 §1 diz "Alta", a ficha §G diz "Média" | S |
| 4 | O que está pendente? | 10 itens (Q-05, dimensão, meios locais, status 40–43, migração, `forceload`, gravação atômica, benchmark, "80 µs", J) | §F | 1 | S | P: §F.2 trata canal por dimensão como decisão ("ou registrar a decisão de deixá-los globais"), mas a Q-05 e a TAR-027 dizem que é correção técnica | S: `frentes/GRANDE-INDUSTRIA.md:78` diz 80 µs |
| 5 | Quem depende dele? | PRO, TRB e MEG | §A | 1 | S | P: o 03 §3 diz só "PRO" | — |
| 6 | Que agente? | Codex nuvem (terminal, drones, caravanas), thread #40 (estações e âncora), Codex local (elevador), integrador (renomeação) | §G; 07 §1.1 | 1–2 | S | S | S (sessões no 07) |
| 7 | Próxima tarefa? | TAR-027: "a parte técnica não depende do dono e pode entrar já" | 00 → 08#tar-027 | 1 | S | S | — |
| 8 | Decisão do dono? | Q-05 (custo da produção virtual e meios locais) e Q-04 (nome "estações") | §G → 09 §5 | 2 | S | S, com DEC-041 e CNF-009 | — |

### 1.4 Síntese das consultas

- 24 perguntas: 21 achadas, 3 parciais (todas sobre "próxima tarefa" ou falta de Q).
- Média de 1,3 salto.
- **Ponto fraco comum:**
  - nenhuma ficha diz qual é a próxima tarefa do sistema;
  - o 00 lista as TAR sem estado nem prioridade, e só abrindo cada uma se descobre que a maioria está bloqueada;
  - as dependências divergem entre a ficha §A, a tabela 03 §3 e o Mermaid.

---

## 2. Checagem por amostragem contra o código

| # | Afirmação | Doc | Camada | Resultado | Evidência |
|---|---|---|---|---|---|
| 1 | Exigências das fases (12/3/40 · 36/9/55/400/60 · 80/20/70/Gini 45/3) em `mayor.rs:17-22`; imposto 4/8/12/16 em `:13-15` | SIS-VIL §B.2–B.3 | main = pr47 | **Correta** | `PHASES` nas l.18–22 e `tax_rate` nas l.13–15 |
| 2 | "Só promove com prefeito (`staffed`, `mayor.rs:190`)" | SIS-VIL §B.3 | ambas | **Imprecisa** | `staffed` está na l.170 e o uso na l.189. O 05 cita `:189` |
| 3 | PIB errado na página `veio.village.page.33` (lang `:1720`) | SIS-VIL §B.4; CNF-007; TAR-048 | ambas (a ficha se diz das duas) | **Imprecisa** | Vale para o pr47. Na main é a `page.24`, na l.1033. O aceite da TAR-048 ("pág. 33") só vale para o pr47 |
| 4 | Bolsa: `PRODUCTS` `trade.rs:46` (34), `index :106`, `absorb :128`, `pressure :132-134`, `spot_cents :141`, `STAPLE :84` | SIS-MER §D; 05 §1.13 | ambas | **Correta** | Linhas exatas; 34 entradas; absorção 256/512/512/256/128 |
| 5 | Lote do contrato `256/(camada+1)` arredondado a 16, mais (n mod 4) × 16; ≈ 256/128/80/64/48 | SIS-MER §B.6 | ambas | **Correta** | `trade.rs:263` |
| 6 | Atacado 75%; `receive_deliveries :200`, `settle :221` (saída em `WAGES`), `receive_sale :246`; acerto só com cofre da companhia | SIS-MER §B.3/§D; 05 | ambas | **Correta** | `market.rs:26,200-252`; `ledger::take(..., ledger::WAGES, ...)`; `company_open` |
| 7 | `simulate` em `stations.rs:117-140`; canal global; `forget_idle` só apaga cargas sem ritmo | SIS-TRA §B.1; 05; R-13 | pr47 | **Correta** | l.117–140; `pools.entry(channel)` sem dimensão; l.143–145 |
| 8 | Drone: custo `800 + 32×d` em `freight.rs:100-102`; anti-ciclo A→B→A em `freight.rs:95`; FUN-TRA-02 `:95-107` | SIS-TRA §B.3 e §D | pr47 | **Errada** (linhas) | O custo está na l.236 e o anti-ciclo na l.230. As l.95–107 são de `right_click`. Os valores (800, 32, 16 itens, 20 + 2×d em l.247, buffer 8.000 em `lib.rs:528`) estão certos |
| 9 | "Testes: `freight/tests.rs` 21, stations 2, lift 2" | SIS-TRA §D | pr47 | **Imprecisa** | `freight/tests.rs` tem 14 `#[test]`. Os outros 7 são `freight::caravan::tests`. Stations tem 2 testes ok e mais o `stations_cost` ignorado |
| 10 | `STATUS_KEYS` com 36 entradas (main) e 44 (pr47); 36–39 em `site.rs:42-45`; 40–43 em `freight.rs:15-18`; `waiting()` em `center.rs:111-113` | 04 §10.1–10.2 | ambas | **Correta** | `[&str; 36]` e `[&str; 44]`; constantes nas linhas citadas |
| 11 | Main com 55 blocos e 16 itens; pr47 com +21 blocos e +9 itens (lista nominal); 1.834 e 1.050 chaves de tradução; `veio.village` 131; 52 avanços | 04 §10.5 e §10.8 | ambas | **Correta** | Diferença nominal idêntica nas chaves `block.veio.*` e `item.veio.*`; contagens exatas |
| 12 | `bench_factory` com mediana 268,7 → 364,8 µs (+36%) | 05 §3.2; R-14 | ambas | **Correta** | `bench_interleaved.log`: as 5 rodadas dão essas medianas |
| 13 | Produtividade 10 → 20 em `upgrades.rs:35` | 05; CNF-016; R-27 | pr47 (main l.31) | **Correta** | `bonus: 10` na main e `bonus: 20` no pr47 |
| 14 | Juros da obra Prefeitura `(caixa/16).min(8)` em `projects.rs:475-476`; `reward` Public → Company em `center.rs:609-620` | 05 FUN-ECO-07/08; DEC-035 | pr47 | **Correta** | Linhas exatas |
| 15 | Custo das estações "194,4 µs" com fonte `SP/a/logs/pr47_stations_cost.log:26` | SIS-TRA §B.1 | pr47 | **Imprecisa** | O valor está na l.25 do log. O log está num `/tmp` efêmero |

**Resultado:** 10 corretas, 4 imprecisas e 1 errada. Os números de regra e de contagem são confiáveis. As referências `arquivo:linha` de `freight.rs` na ficha SIS-TRA parecem vir de outra versão do arquivo.

---

## 3. Contradições entre documentos

### 3.1 Estado e confiança

1. **Confiança do 03 §1 × fichas §G em 5 sistemas:**
   - SIS-TRA: Alta × Média;
   - SIS-TRB: Alta × Média;
   - SIS-APR: Alta × Média;
   - SIS-PER: Alta × Média;
   - SIS-MP: Média × Baixa.

   MUN e PRG também baixam na ficha.
2. **SIS-MP na main:** o 03 §1 diz "Indeterminado"; a ficha §G e o 05 §1.24 dizem "—" (ausente). A §G da ficha MP começa com o texto truncado "Por jogador main —".
3. **FUN-VIL-04:** a ficha usa o nome "Poço, energia e alvará" e o 05 usa "Poço, saneamento e energia na casa". A ficha cita loja 40 e alvarás; o 05 só cita os raios 24/12. FUN-VIL-05 inclui "PIB" no 05 e não na ficha. O estado coincide.
4. **#47:** o 01 §7 diz "Integração do #47: Concluída; aguarda Q-11". O 07 §1.1 diz "Thread do #47: Em execução" (compilação real e `3f7cb28`). O diagrama 07 §2.2 usa `#47@3f7cb28`, e o resto usa `80980db`.
5. **DEC-041:** o 01 §3 a dá como exemplo de "decisão do dono aplicada no pr47". O 05 (FUN-TRA-05), a ficha SIS-TRA e a CNF-009 dizem que o código **diverge da intenção**.

### 3.2 Versões e testes

6. As versões estão coerentes:
   - main e pr47 declaram 0.25.0;
   - o launcher é 0.10.0 na main e 0.16.1 no pr47;
   - o #51 é 0.26.0 e o #54 é 0.27.0.
7. As contagens 176/0/5 e 325/0/9 batem com os logs em 01, 05, 10 e SIS-PLT. Os números 331 e 351 (06, 07) estão rotulados E4. Sem contradição.

### 3.3 IDs

8. **"Q3"** em SIS-MER §F.1 não é ID; o certo é Q-03.
9. **IDs órfãos da auditoria intermediária em 09:** "prioridade 11 do dono" (R-14) e "decisão 8" (R-22) não existem neste conjunto.
10. **DEC-067** aparece no 07 §7 como "o próximo número". Está correto, mas o 11 §3 usa `DEC-045` como exemplo e não diz qual é o próximo.
11. **03 §5 × 09:** o 03 §5 diz "VIL (IDH) × OBR × #54 … **Sem CNF**", mas a CNF-022 existe no 09 para esse conflito.
12. **TAR-017** está "Bloqueado (aguarda Q-06)". Isso não bate com a ficha SIS-VIL §F.2 (sem Q) nem com o 03 §5 (Q-12).

### 3.4 Decisões do dono

13. **DEC-006 (mapa de 15.000²):** o 06 a classifica como "Dono, só em documento" e "contrato em vigor". A CNF-019 diz "**Decisão de agente até confirmação**". As duas classificações são incompatíveis.
14. **DEC-005 ("validação em jogo fica com o jogador"):** é "Dono, só em documento", mas o 07 §1.1 a usa como papel firmado do dono ("pela DEC-005, valida em jogo"), sem a ressalva.
15. **DEC-004 e DEC-049:** também são "Dono, só em documento". O tipo está rotulado de forma transparente no 06 §1; não é erro, mas o 01 e o 07 não carregam a ressalva quando citam o efeito.

---

## 4. Lacunas de navegação

Temas que um desenvolvedor novo procuraria e não acharia em até dois cliques a partir do 00:

1. **Próxima tarefa por sistema.** A coluna "Tarefas" do 00 não mostra estado nem prioridade, e nenhuma ficha indica qual tarefa vem primeiro.
2. **Glossário.** Não há definição de turno × tick, FE, mB, divisão e aba da holding, Povoado/Vila/Cidade, "Cloud", "Company/Public/Shop" nem E1–E4/J fora do 00.
3. **Preparação do ambiente.**
   - Toolchain Rust e edição: só "edition 2024" no 04 §1.
   - Onde clonar o Horizon e como apontar o stub.
   - Como rodar no Windows.

   O 04 §9 lista os comandos, mas o 00 não leva até ele por esse tema.
4. **Índice reverso arquivo → sistema.** O 04 §3 lista módulos sem a coluna SIS; para saber de qual ficha é `stations.rs`, é preciso buscar no texto.
5. **Evidência efêmera.**
   - Os logs E1 (`SP/a/logs/…`) estão num `/tmp` de sessão.
   - Os relatórios A1–A7 e B1–B7 são citados como fonte em fichas e TAR, por exemplo "A2 MER F.1", "B2 VIL F.4" e "A7". Um leitor futuro não os alcança.
6. **Roteiro de teste em jogo.** Só existe como aceite da TAR-003 ("escrever o roteiro"). Não há tema "teste em jogo" no 00.
7. **"Estação de carga" ambígua no 00 Mapa por tema.** O termo leva tanto a SIS-MER quanto a SIS-TRA, sem desambiguação.
8. **PRs #53 e #54 por sistema.** Só aparecem em TAR-064 e TAR-065 e no 01 §8. As fichas OBR e VIL não os mencionam em §D ou §F.

---

## 5. Resposta ao supervisor

Ver a mensagem de retorno (≤ 350 palavras).
