# B7: Revisão cruzada do trabalho do OPUS A pelo OPUS B

Autor: OPUS B. Data: 2026-11-08, ~21h UTC. Insumos: `SP/a/A1`–`A6` (OPUS A) e `SP/b/B1`–`B6` (eu).

Regra que usei: quando A e B descrevem a mesma funcionalidade, **o ID canônico é o do A**. Os FUN do B que não têm par no A ganham número novo, depois do maior número do A no mesmo SIS. Nenhum ID é reaproveitado. ID fundido ou movido fica registrado como alias.

---

## 1. Lista canônica

### 1.1 Sistemas (25)

| ID | Nome | Origem |
|---|---|---|
| SIS-PLT | Plataforma, build e ferramentas | CONTEXTO |
| SIS-MUN | Mundo: jazidas, rios, limite do mapa e mapa finito | CONTEXTO |
| SIS-EXT | Extração e processamento de minério | CONTEXTO |
| SIS-PRO | Produção: máquinas, receitas, módulos, Mk.II, grande indústria | CONTEXTO |
| SIS-ENE | Energia | CONTEXTO |
| SIS-AGU | Água encanada | CONTEXTO |
| SIS-AGR | Agro e extrativismo | CONTEXTO |
| SIS-LOG | Logística local | CONTEXTO |
| SIS-TRA | Transporte entre zonas e produção virtual | CONTEXTO |
| SIS-TRB | Aldeões trabalhadores | CONTEXTO |
| SIS-VIL | Sociedade da vila | CONTEXTO |
| SIS-ECO | Economia e contabilidade | CONTEXTO |
| SIS-MER | Mercado e comércio exterior | CONTEXTO |
| SIS-OBR | Obras e plantas | CONTEXTO |
| SIS-LAR | Casa do fundador e Lar | CONTEXTO |
| SIS-CTL | Centro de controle executivo | CONTEXTO |
| SIS-PRG | Progressão: avanços, quadro, contratos, grande projeto | CONTEXTO |
| **SIS-PES** | Pesquisa: laboratório da fábrica e pesquisa da vila | **Novo.** A e B propuseram de forma independente; os dois sistemas colidem na chave `veio.research.done` |
| SIS-TUT | Tutorial e guias | CONTEXTO |
| SIS-DEF | Ameaças e defesa | CONTEXTO |
| SIS-MEG | Megafábricas | CONTEXTO |
| SIS-APR | Apresentação: painéis, formas, som, shader, realismo | CONTEXTO |
| SIS-DES | Desempenho | CONTEXTO |
| SIS-MP | Multijogador LAN | CONTEXTO |
| **SIS-PER** | Persistência e ciclo de vida do mundo | **Novo** (A). É transversal: 5 arquivos sem versão e estado em memória (A6 R-08/R-09) |

Rejeitado: **SIS-SOC**. O próprio A deixou opcional, e dividir SIS-VIL não traz regra nova.

### 1.2 Mudanças de ID em relação ao A3

| ID do A | Destino canônico | Motivo |
|---|---|---|
| FUN-PRO-02 (combustíveis) | **Fundido em FUN-ENE-02** | Duplicava o gerador e a biomassa |
| FUN-PRO-08 (bônus de refeição) | **Fundido em FUN-LAR-03** | Mesma regra (DEC-046); o caráter global vira nota e questão Q15 |
| FUN-PRG-05 (laboratório) | **FUN-PES-01** | Passa para SIS-PES |
| FUN-OBR-07 (pesquisa da vila) | **FUN-PES-02** | Passa para SIS-PES |

### 1.3 FUN novos, vindos do B (não há linha no A3)

| ID canônico | Nome | Por quê | Estado main · pr47 · disperso |
|---|---|---|---|
| FUN-MUN-06 | Jazidas maiores e espaçadas em terreno novo (Shapez, parte 2) | Proposta (DEC-011) | — · — · Concebido |
| FUN-PRO-09 | Painel de produção: itens/min, % ativo, motivo de espera | **Pedido do dono** (16:38, "razões visíveis") sem linha no A3 | Integrado · Integrado · — |
| FUN-AGU-04 | Lava nos canos | Item das lacunas aprovadas (16:32). O A registra o comportamento como ND | — · Indeterminado · — |
| FUN-TRB-13 | Contratação por esmeralda | Regra central do TRB | Integrado · Integrado · — |
| FUN-TRB-14 | Comida por tarefa (2 pontos; o pão paga 3 tarefas) | Regra central do TRB (Grok lote 3) | Integrado · Integrado · — |
| FUN-OBR-11 | Funcionários das obras (`staff.rs`: cozinheira, professor…) | Sem linha no A3 | — · Integrado · — |
| FUN-PRG-09 | Meta de produção sustentada / meta final | Lacuna aprovada; DEC-023; Q10 | — · — · Concebido |
| FUN-DEF-06 | Monstros caçam aldeões à noite | Sem linha no A3 | Integrado · Integrado · — |
| FUN-APR-09 | Realismo 2.1, clima e "estações do ano" (#51) | Trabalho novo, não auditado | — · — · Em desenvolvimento |
| FUN-DES-05 | Critério de desempenho em PC de 8 GB | **Pedido do dono** (20:09) sem critério mensurável no A3 | — · — · Aprovado (sem medida) |

**Total canônico:** 150 do A − 2 fundidos + 10 novos = **158 FUN**. As 4 linhas acima do §1.2 entram com o ID novo e ficam registradas como alias.

### 1.4 Correspondência FUN do B ↔ FUN canônico

Uma linha por FUN do B. "=" quer dizer equivalente; "⊂" quer dizer que a FUN do B foi absorvida pela canônica.

| FUN do B | → canônico | | FUN do B | → canônico |
|---|---|---|---|---|
| PLT-01 Launcher | = PLT-07 | | MER-01 Preço local | = MER-01 |
| PLT-02 Geração de conteúdo | = PLT-08 | | MER-02 Oferta/procura | ⊂ MER-01 |
| PLT-03 Verificações | ⊂ PLT-01/02/03 | | MER-03 Atacado | = MER-02 |
| PLT-04 Simulador | = PLT-09 | | MER-04 Bolsa | = MER-06 |
| PLT-05 Pacote | = PLT-06 | | MER-05 Saturação | ⊂ MER-06 |
| MUN-01 Pureza | = MUN-01 (main 3×3) + MUN-02 (pr47) | | MER-06 Frete | ⊂ MER-04/05 |
| MUN-02 Jazidas Shapez | (1) ⊂ MUN-02; (2) → **MUN-06** | | MER-07 Contratos | ⊂ MER-07 |
| MUN-03 Rios | = MUN-03 | | MER-08 Reputação | ⊂ MER-07 |
| MUN-04 Mapa finito | = MUN-04 + MUN-05 | | MER-09 Importação | = MER-03 |
| EXT-01 Extratoras | = EXT-01 | | MER-10 "Liquidação" (sucata) | ⊂ MER-06. Renomear: no A, "liquidação" é o acerto do atacado (MER-02) |
| EXT-02 Produtos | ⊂ EXT-01 | | MER-11 Escopo (34 produtos) | ⊂ MER-06 |
| EXT-03 Processamento | = EXT-02 | | OBR-01 Catálogo | ⊂ OBR-02 |
| PRO-01 Receitas | = PRO-01 + EXT-04 (tempos) | | OBR-02 Efeitos | ⊂ OBR-02 |
| PRO-02 Grande indústria | = PRO-05 | | OBR-03 Prestígio | = OBR-03 |
| PRO-03 Módulos | = PRO-03 | | OBR-04 Pedidos da vila | = OBR-04 |
| PRO-04 Mk.II | = PRO-04 | | OBR-05 Caixa e juros | = ECO-08 + CTL-01 |
| PRO-05 Cozinha/moinho | = AGR-05 | | OBR-06 Decretos | = OBR-05 |
| PRO-06 Painel por minuto | → **PRO-09** | | OBR-07 Canteiro | = OBR-01 |
| PRO-07 Monitor | = PRO-07 | | OBR-08 Plantas | = OBR-06 |
| ENE-01 Gerador | = ENE-02 | | OBR-09 Boletim | = OBR-08 |
| ENE-02 Geotérmico | ⊂ ENE-03 | | LAR-01..04 | = LAR-01..04 |
| ENE-03 Renováveis | ⊂ ENE-03 + ENE-04 (vapor) | | CTL-01 Slots | = CTL-01 |
| ENE-04 Hidrelétrica | = ENE-05 | | CTL-02 Raio | ⊂ CTL-01 |
| ENE-05 Rede | = ENE-01 (+ bateria em ENE-04) | | CTL-03 Páginas | = CTL-03 |
| ENE-06 Tesla | = DEF-04 | | CTL-04 Foco | = CTL-04 |
| AGU-01..03 | = AGU-01..03 | | CTL-05 Abas | = CTL-02 |
| AGU-04 Lava | → **AGU-04** | | PRG-01 Avanços | = PRG-01 |
| AGR-01 Colheitadeira | = AGR-01/02 | | PRG-02 Quadro e licenças | = PRG-02 |
| AGR-02 Semeadora/lâmpada/silvicultora | ⊂ AGR-02 + AGR-04 | | PRG-03 Pedido do quadro | = PRG-03 |
| AGR-03 Rancho | = AGR-03 | | PRG-04 Central | = PRG-06 |
| AGR-04 Irrigação | = AGR-07 | | PRG-05 Grande projeto | = PRG-07 |
| AGR-05 Zonas | = TRB-06 | | PRG-06/07 (remissões) | Removidas (são VIL/OBR/MER) |
| LOG-01 Esteiras | = LOG-01 + LOG-02 | | PRG-08 Meta sustentada | → **PRG-09** |
| LOG-02 Túnel | = LOG-03 | | PES-01 Laboratório | = PES-01 (ex-PRG-05) |
| LOG-03 Braços | = LOG-05 | | PES-02 Árvore | ⊂ PES-01 |
| LOG-04 Divisor | = LOG-04 | | PES-03 Pesquisas da vila | = PES-02 (ex-OBR-07) |
| LOG-05 Lote de esteiras | ⊂ LOG-01 | | TUT-01 Tutorial na tela | = TUT-06 |
| TRA-01 Estação | = TRA-04 + TRA-05 | | TUT-02 Livros | = TUT-02..05 |
| TRA-02 Âncora | = TRA-06 | | TUT-03 Limites | ⊂ TUT-02 |
| TRA-03 rail_terminal | = TRA-01 | | DEF-01 Poluição | = DEF-01 |
| TRA-04 Drones | = TRA-02 | | DEF-02 Aviso/gatilho | ⊂ DEF-01 |
| TRA-05 Caravanas | = TRA-03 | | DEF-03 Ondas | = DEF-02 |
| TRA-06 Elevador | = TRA-07 | | DEF-04 Mobs caçam aldeões | → **DEF-06** |
| TRB-01 Contratação | → **TRB-13** | | DEF-05 Quartel | = DEF-03 + DEF-05 |
| TRB-02 Comida | → **TRB-14** | | DEF-06 Reparo | = PRO-06 |
| TRB-03 Lavrador | ⊂ TRB-05 | | MEG-01 Copiadora | = MEG-01 |
| TRB-04 Carregador | = TRB-01 (+ TRB-04 incremental) | | MEG-02 Marco | = MEG-02 |
| TRB-05 Capataz | = TRB-02 (+ TRB-03 incremental) | | MEG-03 Verticalidade | = MEG-04 + TRA-07 |
| TRB-06 Pastor | ⊂ TRB-05 / AGR-03 | | MEG-04 Prancheta | = MEG-03 |
| TRB-07 Caminhada e descanso | = TRB-10 | | APR-01 Painéis | = APR-01 |
| TRB-08 Spawn | = TRB-09 | | APR-02 Formas | = APR-03 |
| TRB-09 Crescimento | = TRB-07 + TRB-08 | | APR-03 Sons | = APR-04/05/06 |
| TRB-10 Staff | → **OBR-11** | | APR-04 Realismo 1.1 | = APR-07 |
| TRB-11 Escadas | = MEG-04 | | APR-05 Realismo maior | → **APR-09** |
| TRB-12 Itens na mão | = TRB-11 | | DES-01 Orçamento | = DES-01 |
| VIL-01 Casa | = VIL-01 | | DES-02 Buscas incrementais | = DES-02 + TRB-03/04 |
| VIL-02 Alvará | ⊂ VIL-04 | | DES-03 Ganhos offline | = DES-03 + APR-08 + TRB-12 |
| VIL-03 Poço | ⊂ VIL-04 | | DES-04 Produção virtual | = TRA-05 (removida do DES) |
| (fases/indicadores do B) | = VIL-03 / VIL-05 | | MP-01/02 | = MP-01/02 |
| ECO-01 Moeda | ⊂ ECO-01 (renomear "Moeda e contas") | | — | DES-05 novo (8 GB) |
| ECO-02 Salário | = ECO-03 | | ECO-05 Recompensa pública | = ECO-07 |
| ECO-03 Imposto | = ECO-02 | | ECO-06 Holding | = ECO-05 |
| ECO-04 Livros | = ECO-01 | | ECO-07 Equilíbrio | = PLT-09 |

FUN do A sem par no B, que entram na lista canônica sem mudança:
- PLT-04, 05, 10, 11
- EXT-03
- AGR-06
- LOG-06
- VIL-02, 06
- ECO-04, 06, 09
- MER-08
- OBR-09, 10
- PRG-04, 08
- TUT-01
- DEF-05
- APR-02
- DES-04
- PER-01..04

**Renomeação para estabilidade:** FUN-ECO-09 passa a se chamar "Fontes externas de moeda (fora do livro)". Assim ela deixa de parecer funcionalidade desejada; ver §2.

---

## 2. Estados do A3 que divergem das decisões ou da intenção

| FUN | Estado no A3 | Divergência | Proposta |
|---|---|---|---|
| EXT-03 "veio normal pede duas prensas" | pr47 **Integrado** (E3) | Contradiz a **DEC-045** (dono, 19:16: tempos da 0.25). Com a prensa a 30t = 40/min e a Mk.I a 30/min, um veio normal pede menos de uma prensa | pr47 → **Substituído**. A evidência só existe em commit (E3) |
| ECO-09 Emissão fora do livro | **Integrado** main/pr47 | Apresentada como funcionalidade. Contraria o princípio da economia fechada (DEC-030) e o espírito da DEC-035 | Manter o estado, mas abrir CNF-001 / Q1. Não é "feito", é defeito de desenho |
| PRG-01 Avanços liberando receitas | **Integrado** | Sem `doLimitedCrafting`, o bloqueio pode ser só visual (A6 R-21). A progressão inteira (DEC-014/018) depende disso | O gate real é **Indeterminado** até um teste no Horizon. Q3 recebe uma pré-condição |
| PRG-02 Quadro e licenças | **Integrado** | O texto "Libera" promete máquinas que, no pr47, vêm de `research/*.json` (R-20). O jogador recebe a licença e nada é liberado | Integrado com defeito visível. Aumenta a urgência da Q3 |
| TRA-05 Produção virtual | **Integrado** | O dono pediu (17:01) que a fábrica distante "continue produzindo e entregando". O código **cria itens sem insumo nem energia**, inclusive depois de o posto ser desmontado, e os canais são globais entre dimensões | Integrado em relação ao código, **divergente da intenção**. Ver Q5 |
| PRO-03 Módulos | Integrado (10 % → 20 %) | Produtividade dobrada no pr47 (`upgrades.rs:35`, c5db4f6) **sem decisão do dono**. O README da main diz "a cada dez ciclos" | Rotular como decisão de agente. Q14 |
| LAR-03 (+ ex-PRO-08) | Integrado | O Lar tem dono por jogador, mas o bônus de refeição é **global** (atomics). Em LAN, a refeição de um acelera todos | Q15 |
| APR-08 / DES-03 / TRB-12 | pr47 — / Integrado / **Bloqueado** | O pr47 tem cerca de 40 % mais custo por tick, 329 alocações por quadro e o bench da vila grande em pânico. Contraria a **DEC-056** (dono: leveza num PC de 8 GB) | Condição de merge do #47 (Q11). FUN-DES-05 novo |
| OBR-01 Canteiro / PES-02 (ex-OBR-07) | **Integrado** | Menu com 1 slot × 3 no código; cerca de 25 traduções faltando (R-30). A HANDOFF_OBRAS lista isso como não feito | pr47 → **Em desenvolvimento** nas partes de tela |
| ECO-05 Holding / ECO-06 BORDERS / DEF-01 Poluição | Integrado ("só em memória") | A 0.21 promete que o "relatório confere a identidade contábil", mas os indicadores zeram ao reiniciar. Reiniciar o mundo também apaga a ameaça de poluição (exploração) | Persistir; é TAR técnica de SIS-PER. A DEC-059 (poluição em memória) deveria ser revogada pelo integrador como correção |
| PLT-06 Pacote | pr47 — | O README do pr47 manda usar `Veio-0.16.1.zip` (inexistente), e `Veio-Realismo-1.1.zip` é a 0.16.1 (R-23) | O jogador instalaria código antigo. Bloqueia a publicação (Q11) |
| MUN-01 Pureza | main Integrado (3×3) | **Correção do B:** eu tinha dado a medida 3D como main 0.9. O 3D conectado só existe no pr47 | B2 e B3 corrigidos |

**Pedidos do dono sem linha no A3:**
- painel por minuto: FUN-PRO-09 novo;
- lava nos canos: FUN-AGU-04 novo;
- critério de 8 GB: FUN-DES-05 novo;
- meta final: FUN-PRG-09 novo.

O "explicar no tutorial" (15:55) está **atendido pelo guia** (`veio.guide.page.15` Água e irrigação, `.16` Rios e represas), **não pelo tutorial na tela**. É uma divergência leve, a registrar em TUT-06.

---

## 3. Achados do A que mexem no design e o que muda em B1–B6

| Achado do A | Efeito em fichas e DEC | Efeito em B6 |
|---|---|---|
| Produção virtual sem insumo nem energia; canais globais entre dimensões (`stations.rs:117-140`) | **DEC-043:** o código acompanha só **em parte**. **B3 §3.6:** a reconciliação "fábrica parada × produção virtual" deixa de estar fechada. A chave (dimensão, canal) é correção técnica do integrador | **Q5 reescrita**: regra da produção virtual |
| Emissões fora do livro (EXPORT ilimitado também na main, contrato village, grande projeto, pedido do dia) e dupla tributação (varejo e atacado) | Tabela de fontes de moeda em B2 SIS-ECO confirmada (E2 do A). A dupla tributação entra em SIS-ECO e SIS-MER | **Q1** atualizada; **Q13 nova** (imposto uma ou duas vezes) |
| Licenças do Quadro não liberam o que prometem | DEC-016 e DEC-018 colidem no código, e não só na intenção | **Q3** sobe para a ordem 2 |
| Nenhum `doLimitedCrafting` | Toda a SIS-PRG pode ser cosmética. TAR de verificação no Horizon | **Q3** ganha a pré-condição |
| Produtividade 10 % → 20 % | **DEC-013** corrigida: main 10 %, pr47 20 %; decisão de agente | **Q14 nova** |
| Bônus de refeição global | **DEC-046** cumprida no código, mas global. Pesquisa e grande projeto também são `@a` | **Q15 nova** (o que é do jogador e o que é do mundo) |
| Regressão de desempenho no pr47 | Pesa contra a **DEC-056** e o pilar P6 (B1) | **Q11**: condição de desempenho antes do merge |
| Estado em `<mundo>/veio/` sem versão; poluição e holding perdidos | Adotar SIS-PER. DEC-059 a revogar como correção técnica (integrador) | Nenhuma Q nova: é técnica |
| O pr47 nunca foi compilado contra o Horizon real | Reforça DEC-005. O DoD de "Integrado" (B4 §2.2) deve exigir `native-build` com o Horizon fixado antes de ir para a main | **Q11**: condição de compilação |

---

## 4. PRs #51 e #52

Registro (E3/E4, via A4 e o coordenador; código **não auditado**):

| PR | Aberto | Origem | Base | Conteúdo |
|---|---|---|---|---|
| **#51** | 20:34Z | Thread "Aumentar o realismo sem pesar o PC" | pr47 | Veio **0.26.0**: Realismo 2.1, clima e estações do ano. 53 arquivos (+2683/−189); `ambience.rs`, `climate.rs`, 9 shaders, `tools/check_shaders.py`, `release/Veio-0.26.0.zip`, +48 linhas no stub |
| **#52** | 20:35Z | Thread "Definir limite do mapa e otimizar" | pr47 | Desempenho pós-consolidação: sons só perto do jogador, mural ocioso, medição de escala (6 arquivos) |

Consequências:

1. **Versão.** O #51 já ocupa a 0.26.0 e embute um zip com esse nome, repetindo o padrão das duas 0.16.0 e das duas 0.25.0. A Q8 foi refeita (ver B6).
2. **Alvo móvel.** O #47 continua mudando por baixo (A6 R-42). Recomendo **congelar o #47**. O #51 e o #52 voltam como entregas de frente sobre a integração, depois do merge.
3. **Nome.** "Estações" (do ano) colide em português com "estações" de carga e descarga (SIS-TRA). Proponho chamar sempre de **"estações do ano"**, ou "temporadas", no código e no guia.
4. **Escopo.** O dono pediu "realismo maior sem pesar o PC". Clima e estações do ano podem ter efeito de jogo (lavoura?) que ninguém decidiu: ND até auditar. Isso entra na Q7.
5. Registro: **DEC-062** (decisão de agente: Realismo 2.1, clima, estações do ano, versão 0.26.0) e **DEC-063** (decisão de agente: otimizações do #52), ambas "não confirmadas pelo dono".

---

## 5. Alterações feitas nos meus arquivos

- **B2:** nota de topo remetendo os IDs canônicos a este B7. Correções:
  - MUN-01 (main 3×3);
  - PRO-03 (10 %/20 %);
  - TRA (produção virtual sem custo; canais globais);
  - ECO (EXPORT na main, dupla tributação);
  - PRG (licenças, `doLimitedCrafting`);
  - LAR (bônus global);
  - DES (regressão).
- **B3:** correções em DEC-010, DEC-013, DEC-043 e §3.6; novas DEC-062 e DEC-063.
- **B6:** Q1, Q3, Q5, Q7, Q8 e Q11 revistas; novas Q13, Q14 e Q15; tabela de prioridade refeita.
