# B2: Fichas funcionais dos sistemas do Veio

Autor: OPUS B. Data: 2026-11-08. Bases: `main@53267af` (0.25.0) e `pr47@80980db`. Decisões citadas como DEC-NNN estão em B3.

## Como ler esta ficha

**Níveis de evidência**
- **E1**: executei a verificação.
- **E2**: li no código (`arquivo:linha@ramo`; quando não há ramo, vale `@pr47`).
- **E3**: está num doc, commit ou PR.
- **E4**: foi dito no chat ou está na memória.

**Seções de cada ficha:** A Identidade, B Funcionamento, C Experiência, E Histórico, F Pendências.

**Estado por camada:** main, pr47 e disperso, com os estados do CONTEXTO (Concebido … Indeterminado). Neste documento:
- **"ausente"** quer dizer que a camada não contém o item. É uma constatação, não um estado do ciclo de vida.
- **Nenhum item está "Validado".** Nada foi testado numa partida real nem compilado contra o Horizon real nesta rodada. A compilação nativa completa mais recente é da consolidação 0.16.1 do Codex (CHECKPOINT-INTEGRACAO, E3).

**IDs de funcionalidade:** `FUN-<SIS>-NN` são propostos aqui como semente do registro (ver B5).

> **Atualização (Fase III):** os IDs FUN desta ficha são do B. A lista canônica, com 25 SIS (inclui SIS-PER) e 158 FUN, e a correspondência B ↔ A estão em `B7_REVISAO_CRUZADA.md` §1. Na dúvida, vale o ID do B7. As correções que vieram da auditoria do OPUS A estão marcadas com "[B7]".

**IDs novos propostos**
- **SIS-PES (Pesquisa).** Sai de SIS-PRG porque há dois mecanismos de pesquisa com regras e persistência próprias: o laboratório da fábrica, que consome continuamente e grava em `<mundo>/veio/pesquisa.txt` (#40), e as pesquisas da vila das obras (`projects.rs:60-63`, `research_bit`). Nenhum dos dois é marco de avanço.
- **Quadro da fábrica e spawn.** Não proponho SIS-QDR. O quadro fica em SIS-PRG como FUN, e o spawn fica em SIS-TRB/SIS-LAR. Separá-los multiplicaria fichas sem regra própria.

---

## Quadro-resumo de estados

| SIS | Sistema | main | pr47 | disperso |
|---|---|---|---|---|
| PLT | Plataforma e build | Integrado | Integrado (launcher 0.16.1 × conteúdo 0.25.0) | zips 0.15.x históricos |
| MUN | Mundo (rios, jazidas, mapa finito) | Integrado (só pureza 3D) | Integrado (rios); mapa finito **Em desenvolvimento**, desligado | — |
| EXT | Extração | Integrado | Integrado | — |
| PRO | Produção e grande indústria | Integrado | Integrado (+ grande indústria) | Grok 80t: Substituído |
| ENE | Energia | Integrado | Integrado (+ hidrelétrica) | — |
| AGU | Água encanada | ausente | Integrado | — |
| AGR | Agro e extrativismo | Integrado | Integrado (+ irrigação) | — |
| LOG | Logística local | Integrado | Integrado (+ expressa e túnel) | — |
| TRA | Transporte entre zonas | ausente | Integrado | PR #28 aberto (conteúdo já no pr47) |
| TRB | Aldeões trabalhadores | Integrado | Integrado (+ staff e escadas) | — |
| VIL | Sociedade da vila | Integrado | Integrado | VILA-CIDADE: Concebido |
| ECO | Economia e contabilidade | Integrado | Integrado (+ `center::reward`) | — |
| MER | Mercado e comércio exterior | Integrado | Integrado | — |
| OBR | Obras e plantas | ausente | Integrado com lacunas (Em desenvolvimento nas partes que faltam) | `cloud/obras-planta` só em zip |
| LAR | Casa do fundador e Lar | ausente | Integrado | — |
| CTL | Centro de controle | Integrado | Integrado (8 slots, páginas, foco) | — |
| PRG | Progressão | Integrado (avanços de 1 item, central) | Integrado (fases, quadro, grande projeto) | — |
| PES | Pesquisa (proposto) | ausente | Integrado | — |
| TUT | Tutorial e guias | Integrado (livros) | Integrado (tutorial na tela + 4 livros) | — |
| DEF | Ameaças e defesa | Integrado | Integrado | — |
| MEG | Megafábricas | ausente | Integrado | — |
| APR | Apresentação | Integrado | Integrado (formas, sons, Realismo 1.1) | realismo maior: Em desenvolvimento (thread) |
| DES | Desempenho | Integrado | Integrado | otimização e limite: Em desenvolvimento (thread) |
| MP | Multijogador LAN | Indeterminado (LAN vanilla) | Integrado (convidado vai à vila do anfitrião) | PR #25 aberto |

---

## SIS-PLT: Plataforma, build e ferramentas

**A. Identidade.** Este sistema leva o jogo até o PC do jogador.

| Item | Detalhe |
|---|---|
| Base do jogo | Horizon (Minecraft 26.3 em Rust) |
| Código | Mod nativo em `native/api/src` |
| Conteúdo | `content/` |
| Geradores Python | `tools/build_content.py`, `build_textures.py`, `build_tutorial.py`, `build_production.py` (gera `production/data.rs`), `machine_forms.py`, `blueprints`, `build_house.py` |
| Launcher | `launcher.exe` (Windows) |
| Pacote | `release/Veio-<versão>.zip` |

Fontes: CONTRIBUTING e README (E3).

**B. Funcionamento**

| FUN | Regra | Fonte |
|---|---|---|
| FUN-PLT-01 Launcher | Recompila o nativo pelo Horizon Launcher quando o stamp muda, instala o conteúdo na instância **Veio** e cria a instância se ela não existir (`horizon-launcher instance create Veio`) | README pr47 (E3) |
| FUN-PLT-01 (stamp) | Stamp na main: `"{version}"` (`launcher/src/main.rs:52@main`). Stamp no pr47: `"{version}\nprofile=performance"` (`main.rs:56-57@pr47`) | E2 |
| FUN-PLT-02 Geração de conteúdo | `data.rs` é gerado. Depois de um merge, é preciso rodar `build_production.py` (um auto-merge já pegou tempos do Codex por engano) | Memória (E4) |
| FUN-PLT-03 Verificações offline | `check_project.py` (JSON, traduções, páginas ≤ 100 no guia principal, regras de equilíbrio `balance_*`); `check_rust.py` (testes contra o stub `native/offline-check/horizon-client`); `check_performance.py` | CONTRIBUTING, ECONOMIA (E3) |
| FUN-PLT-03 (testes) | Contagem: 51 testes → 176 na 0.25.0 (TRABALHADORES). Consolidação Codex: 248 testes | E3 |
| FUN-PLT-04 Simulador econômico | `tools/sim_economy.py --check` → "Invariantes de equilíbrio: ok" no pr47@80980db. **Isto é um modelo Python, não o jogo** | **E1** |
| FUN-PLT-05 Pacote | `python tools/package.py` gera o zip único; `launcher.exe` e `release/` ficam versionados por pedido do usuário | DEC-004 (E3) |

Não há CI (`.github` inexistente). Não há compilação contra o Horizon real nesta sessão.

**C. Experiência.** O jogador abre o `launcher.exe`. Na primeira vez, faz o Prepare do Horizon e ativa o Veio em Opções > Mods. Depois de cada versão que muda `native/`, espera a recompilação (a 0.16.1 levou 7 min 14 s; CONSOLIDACAO, E3).

**E. Histórico**
- O launcher.exe entrou na 0.10.0.
- O Codex consolidou a 0.16.1 com compilação completa (Horizon `66f3981`) e encontrou o cache do Horizon incoerente (`horizon_client` × `horizon_core`).
- O pr47 ficou com o launcher 0.16.1 e o conteúdo 0.25.0 (CHECKPOINT, E3).

**F. Pendências**
1. Alinhar a versão do launcher e do conteúdo e escolher o número da build consolidada (B6-Q8).
2. Fazer a compilação nativa completa do pr47.
3. Os dois `Veio-0.25.0.zip` são diferentes entre as camadas.
4. Criar CI mínima.
5. O launcher só existe para Windows; o `.exe` não compila na nuvem (E4).

---

## SIS-MUN: Mundo (rios, jazidas, mapa finito)

**A. Identidade.** O mundo vanilla é o tabuleiro da fábrica. As jazidas do Minecraft sustentam a extração, os rios dão água e energia, e o mapa finito limita o crescimento. Decisões: DEC-006, DEC-009, DEC-010, DEC-011, DEC-039.

**B. Funcionamento**

| FUN | Regra e números | Fonte | main | pr47 |
|---|---|---|---|---|
| FUN-MUN-01 Pureza da jazida | Medida pelo veio conectado em 3D a partir do bloco abaixo da extratora: 1–2 blocos impura (×0,5); 3–6 normal; 7–15 pura (×2); ≥16 **industrial** (×4 com o dobro de potência, só no pr47). Diamante, esmeralda e detritos param na pura. Jazidas infinitas (pedra, areia etc.) leem o 3x3 | README pr47 "Produção (0.9)" (E3); [B7] A2: main conta só o **3×3** abaixo da extratora (`extraction.rs`, E2 do A); o 3D conectado até 16 blocos é do pr47 (`extraction.rs:34-57`) | Integrado (**3×3**, sem conexão) | Integrado (3D, industrial) |
| FUN-MUN-02 Jazidas conectadas e espaçadas (Shapez) | Depósitos maiores e mais raros em terreno novo | JAZIDAS-CONECTADAS, PR #23 (E3, "é uma proposta") | ausente | ausente (Concebido) |
| FUN-MUN-03 Rios com nascente | Nascente acima de y 100; chance de 20–50%; a pelo menos 40 blocos de outra nascente (`SPRING_SPACING = 40`, `rivers.rs:30`, E2); degraus de ≥3; só chunks com menos de 6.000 ticks; estado em `<mundo>/veio/rios.txt`; traçado em tempo de execução sobre chunks novos | AGUA-E-RIOS (E3) | ausente | Integrado (APIs não verificadas no Horizon: OPUS A) |
| FUN-MUN-04 Mapa finito | Overworld de 15.000² de -7.500 a 7.500 exclusive; muralha de bedrock; todos os biomas e estruturas garantidos; Nether de 1.875 (8:1); portais seguros. 15.000 blocos ≈ 44 min 39 s a 5,6 blocos/s | MAPA.md (E3) | ausente | **Em desenvolvimento, desligado** (`work-in-progress/finite-world`: 22 de 24 injeções, auditoria do Nether falha, testes de portal ausentes; CONTINUAR.md, E3) |

**C. Experiência.** Achar uma jazida grande vale montar uma operação maior. Os rios descem das montanhas e servem para a hidrelétrica, a roda d'água e as bombas. O mapa finito daria um mundo com todos os recursos ao alcance, a menos de 45 minutos a pé de borda a borda.

**E. Histórico.**
- A pureza veio na 0.9.
- A classe industrial veio pela consolidação Codex/fases.
- Os rios vieram no #40 (pedido do dono às 15:55).
- O mapa finito é de 07/11 (MAPA.md); `native_chunk_generate` não existe no Horizon `66f3981`.
- A thread "Definir limite do mapa e otimizar" está em andamento.

**F. Pendências**
1. Confirmar com o dono o tamanho de 15.000 e se ele quer "geração garantida" ou só "limite" (B6-Q9).
2. Verificar as APIs de chunk.
3. Testar rios no jogo.
4. Implementar o FUN-MUN-02.

---

## SIS-EXT: Extração e processamento de minério

**A. Identidade.** É a primeira etapa de toda cadeia: extratoras sobre jazidas vanilla, que nunca se esgotam ("as jazidas permanecem disponíveis").

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-EXT-01 Extratora Mk.I | 1 unidade a cada 2 s; 6 FE/t; 240 FE por unidade | README (E3) |
| FUN-EXT-01 Extratora Mk.II | 1/s; 12 FE/t; diamante e esmeralda só com ela, 20× mais devagar (1 a cada 20 s; 4.800 FE/unidade); obsidiana e ametista | README (E3) |
| FUN-EXT-01 Extratora Mk.III | 4× a Mk.I; detritos ancestrais. No pr47 é liberada pela pesquisa *automation* (`drill_mk3`) | README; avanços pr47 (E2) |
| FUN-EXT-02 Produtos | Cobre, ferro, ouro, carvão, redstone, lápis, quartzo e ouro do Nether; jazidas infinitas de pedra, ardósia, areia, cascalho, argila, netherrack, areia das almas, pedra-negra e pedra do End | README (E3) |
| FUN-EXT-03 Processamento | Prensa: bruto → chapa/lingote. Fornalha elétrica: 2,5× a vanilla. Triturador: quartzo → silício; osso → 4 farinhas; pedregulho → cascalho → areia | README (E3) |

**C. Experiência.** O tutorial manda "extrair cobre de verdade" (passo 5). Um veio puro alimenta mais máquinas. A Mk.II traz a decisão de minerar esmeralda gastando energia.

**E. Histórico.** Existe desde a 0.6. A pureza entrou na 0.9. O tempo da prensa oscilou (30t → 40t no Codex → 80t no Grok → 30t, DEC-045).

**F. Pendências**
1. A Mk.II minerando esmeralda emite moeda fora do livro (B6-Q1).
2. Falta subproduto (escória), sugerido no doc de lacunas e não feito.

---

## SIS-PRO: Produção, máquinas e grande indústria

**A. Identidade.** O coração do jogo de fábrica: receitas em `tools/production.json`, módulos e a cadeia profunda.

O documento de lacunas aprovado (DEC-017) mediu a main em **9 intermediários e 53 receitas** (cadeia rasa). O #40 aprofundou a cadeia.

**B. Funcionamento**

| FUN | Regra e números | Fonte | main | pr47 |
|---|---|---|---|---|
| FUN-PRO-01 Receitas básicas | Prensa 30t (8 FE/t) = 40 chapas/min. Montador 40t: 2 chapas de cobre → 1 engrenagem. Cortadora: ferro → moldura; cobre → 2 fiações. Integrador: moldura + engrenagem → módulo mecânico. Forno de ligas: 2 ferro + carvão → aço. Montador de circuitos: 2 fiações + silício → circuito | `production.json` @main/@pr47 (E2, tempos); README (E3) | Integrado | Integrado |
| FUN-PRO-01 (razão) | O guia de indústria do pr47 diz "Prensa 40/min, Montador 60 chapas → 30 engr.; 3 prensas = 2 montadores", coerente com os tempos da 0.25 | lang pt_br @pr47 (E2) | — | Integrado |
| FUN-PRO-02 Grande indústria | Motor 100t; processador 160t; núcleo de automação 300t; kit Mk.II 240t | `production.json` @pr47 (E2) | ausente | Integrado |
| FUN-PRO-03 Módulos | Velocidade: ciclo 60%, energia 180%, poluição 150%. Eficiência: 100% / 60% / 50%. Produtividade: 115% / 140% / 120%, +1 lote a cada 5 ciclos (+20%). Um módulo por máquina | README (E3); [B7] código: **main 10 %** ("um produto extra a cada dez ciclos"), **pr47 20 %** (`upgrades.rs:35`, c5db4f6): decisão de agente, não confirmada (B6-Q14) | Integrado (10 %) | Integrado (20 %) |
| FUN-PRO-04 Mk.II de máquinas | Ciclo ×0,5, energia ×2, poluição ×1,25 (kit) | GRANDE-INDUSTRIA (E3) | ausente | Integrado |
| FUN-PRO-05 Cozinha, moinho, fornalha | Pão, biscoito, bolo etc.; trigo → farinha | README (E3) | Integrado | Integrado |
| FUN-PRO-06 Painel por minuto | Itens/min e % ativo, com leitura fechada por minuto; motivo de espera (material, energia, saída, redstone) | README "Projetos" (E3) | Integrado | Integrado |
| FUN-PRO-07 Monitor | Registro a cada 20 ticks; 9 itens; alerta a cada 600 ticks | GRANDE-INDUSTRIA (E3) | ausente | Integrado |

**C. Experiência.** A pessoa monta linhas com razões inteiras (3 prensas : 2 montadores). Gargalo após gargalo: extração, energia, fio, poluição. O painel mostra por que a máquina espera.

**E. Histórico.** Receitas por tabela desde a 0.9. Grok 80t (#19) substituído. Fases do Codex com 40t revogadas (DEC-045). #40 com a cadeia funda (DEC-019).

**F. Pendências**
1. Confirmar que o `data.rs` do pr47 foi regenerado com 30t (OPUS A).
2. Corrigir PROGRESSAO-FASES, GROK-BALANCE e README linha 24.
3. Os itens da grande indústria estão fora da bolsa (`trade.rs` PRODUCTS = 34; E2).
4. Faltam subproduto e fluido industrial.

---

## SIS-ENE: Energia

**A. Identidade.** É o segundo gargalo da progressão. Liga combustíveis vanilla, renováveis e a hidrelétrica dos rios.

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-ENE-01 Gerador | Até 40 FE/t. Carvão: 16.000 FE. Tronco: 3.000. Blaze: 24.000. Bloco de alga: 40.000. Bloco de carvão: 160.000 | README (E3) |
| FUN-ENE-02 Geotérmico | 80 FE/t; balde de lava = 64.000 FE | README (E3) |
| FUN-ENE-03 Renováveis | Solar: 16 FE/t ao meio-dia. Roda d'água: 8 por lado, até 32. Cata-vento: 8 ao nível do mar, até 32 (16 em y 110). Biomassa: 40. Caldeira: 120 FE/t, com o dobro de energia por combustível | README (E3) |
| FUN-ENE-04 Hidrelétrica | `6 × queda × min(espelho, 40) / 40 × (1 + min(vazão, 12) / 4)`, teto de 320 FE/t | AGUA-E-RIOS (E3) |
| FUN-ENE-05 Rede | Poste de madeira: 8 blocos, raio 2, 120 FE/t. Poste de aço: 16 blocos, raio 3, 400 FE/t. Até 6 vizinhos. Acumulador: 30.000 FE. Máquina: 2.000 FE | README (E3) |
| FUN-ENE-06 Tesla | Raio 10; 2.400 FE por disparo; 2 s; guarda 20.000 | README (E3) |

**C. Experiência.** A pessoa sai do carvão para a caldeira, depois para o solar com acumuladores para atravessar a noite (meta energética, Concebido). A hidrelétrica recompensa achar uma queda d'água.

**E. Histórico.** 0.6 gerador/solar/acumulador. 0.8 postes. 0.9 renováveis. Grok lote 1 reforçou os renováveis. #40 trouxe a hidrelétrica.

**F. Pendências.** Testar a hidrelétrica em rios reais. Pela fórmula, o multiplicador de vazão chega no máximo a 4 (vazão ≥ 12), e com espelho ≥ 40 a usina rende 24 × queda. O teto de 320 FE/t só é atingido com queda ≥ 14 blocos. Uma queda de 10 rende 240, seis vezes o gerador a carvão. O balanço contra a caldeira (120 FE/t) não foi testado em jogo.

---

## SIS-AGU: Água encanada

**A. Identidade.** Leva água real dos rios e do mar à lavoura e à indústria, sem os tanques do Horizon (NBT "Water" em mB; DEC-050).

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-AGU-01 Bomba | 25 mB/t por fonte (até 4); 6 FE/t; tanque de 4.000 mB | AGUA-E-RIOS (E3) |
| FUN-AGU-02 Cano | 1.000 mB; até 200 mB/t | E3 |
| FUN-AGU-03 Aspersor | Área 9x9 | E3 |
| FUN-AGU-04 Lava nos canos | 5 mB/t | GRANDE-INDUSTRIA (E3) |

Estado: main ausente; pr47 Integrado.

**C. Experiência.** A pessoa bombeia do rio, encana até o aspersor e vê a lavoura crescer mais rápido. O tutorial deveria explicar isso (pedido do dono).

**E. Histórico.** Veio do #40, renumerado várias vezes, e foi integrado no #47 (`eedfd47`).

**F. Pendências.** Confirmar que o tutorial na tela ou o livro cobre a água (o dono pediu "explicar no tutorial"). Teste em jogo.

---

## SIS-AGR: Agro e extrativismo

**A. Identidade.** Comida para aldeões e famílias, e biomassa, couro e lã para a indústria. Absorve a poluição (cada colheita absorve 3).

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-AGR-01 Colheitadeira | Área 9x9 em 3 camadas; 120 FE por colheita; adubo adianta 2 estágios; volta de cerca de 4 s | README (E3) |
| FUN-AGR-02 Semeadora, lâmpada de cultivo, silvicultora | Lâmpada: 1 estágio a cada 4 ticks, 50 FE. Silvicultora: grade de passo 3 | README (E3) |
| FUN-AGR-03 Rancho | Até 10 adultos por espécie; tosquia, ordenha; abate o excedente (animais com nome nunca) | README (E3) |
| FUN-AGR-04 Irrigação e necessidades (pr47) | mB por planta e solo: cenoura 45/4; trigo 60/6; batata 70/6; abóbora 80/7; melancia 90/7; beterraba 150/14; flor-tocha 60/5. Chance por estágio: 1/10 com adubo, 1/30 só com água | AGUA-E-RIOS (E3) |
| FUN-AGR-05 Zonas de lavoura e cercado | Níveis 9/13/17/21; custo 8/16/32 esmeraldas; 32/128/384 tarefas; 6/12/20 aldeões; celeiro e silo | TRABALHADORES (E3) |

**C. Experiência.** No começo, o lavrador cuida de 9x9. A zona cresce com tarefas e esmeraldas. Com água encanada, a lavoura deixa de depender do acaso.

**E. Histórico.** 0.7 colheitadeira. 0.9 lavoura e rancho. 0.13 zonas. #40 necessidades e irrigação.

**F. Pendências**
1. A melhoria de zona custa esmeraldas: confirmar se esse pagamento passa pelo livro (OPUS A).
2. A interação entre irrigação e o posto do lavrador não foi descrita.

---

## SIS-LOG: Logística local

**A. Identidade.** Move os itens dentro da fábrica: esteiras, braços, divisor, unificador, filtros e redstone.

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-LOG-01 Esteiras | Comum 2,5/s; rápida 5/s; **expressa 10/s** (pr47); curvas e rampas; mostra "andando" enquanto espera a vez (0.17.2) | README; GRANDE-INDUSTRIA (E3) |
| FUN-LOG-02 Túnel | Até 6 blocos (pr47) | GRANDE-INDUSTRIA (E3) |
| FUN-LOG-03 Braços | Básico: 1 item/s, 40 FE/item. Rápido: 2,5/s, 64 FE. Longo: 2 blocos, 1/s, 60 FE. Filtro por tipo | CODEX-LOGISTICA (E3) |
| FUN-LOG-04 Divisor e unificador | Equilibrado ou com prioridade; modo filtrado; pausa por redstone reavaliada em até 8 ticks | CODEX-LOGISTICA (E3) |
| FUN-LOG-05 Lote de esteiras | Receita inicial: 16. Com aço: 64 comuns | README (E3) |

**C. Experiência.** A rede de Factorio dentro do Minecraft. O tutorial aceita esteiras, braços, funis e carregadores como prova de automação.

**E. Histórico.** 0.8.1 unificador. 0.9.1 curvas e rampas. PR #1/#2 braços (0.15). #40 expressa e túnel.

**F. Pendências.** "Logística compacta" é lacuna menor no doc de lacunas; Concebido.

---

## SIS-TRA: Transporte entre zonas

**A. Identidade.** Liga zonas distantes da fábrica e é a resposta à lacuna "fábrica longe para" (DEC-041).

**B. Funcionamento**

| FUN | Regra e números | Fonte | pr47 |
|---|---|---|---|
| FUN-TRA-01 Estação de carga/descarga | 4 slots; 8 itens a cada 8 ticks (1.200/min); 10 FE por item; canal por corante com 4.096 itens; **produção virtual depois de 100 ticks sem rodar**; `estacoes.txt`; 1.000 estações ≈ 80 µs a cada 20 ticks. [B7] **A simulação soma ritmo × tempo sem consumir insumos nem energia**, continua mesmo com o posto desmontado, e os **canais são globais entre dimensões** (`stations.rs:117-140`, E2 do A); 194 µs por 1.000 estações (E1 do A) | GRANDE-INDUSTRIA (E3); A2/A6 R-13 | Integrado (diverge da intenção: B6-Q5) |
| FUN-TRA-02 Âncora de chunk | 32 FE/t (`industry.rs:21`, E2); forceload; `ancoras.txt` | E3/E2 | Integrado (opcional) |
| FUN-TRA-03 Terminal ferroviário `veio:rail_terminal` | Vagão de baú com funis; zonas por cor; produtos de ida e volta | TRANSPORTES (E3); DEC-044 | Integrado |
| FUN-TRA-04 Drones | Até 64 blocos; 16 itens; 800 + 32×d FE | TRANSPORTES (E3) | Integrado |
| FUN-TRA-05 Caravanas | Cavalo 16, burro/mula 64, porco 8; rotas de até 512 blocos **que dependem de áreas simuladas**; status 40–43 (`freight.rs:15-18`, E2) | TRANSPORTES (E3) | Integrado |
| FUN-TRA-06 Elevador de carga | 5/s entre andares | MEGAFABRICAS (E3) | Integrado |

Main: ausente. A `freight_station` da main é do comércio exterior (SIS-MER).

**C. Experiência.** A pessoa monta uma mina distante, põe uma estação de carga num canal de cor, e a estação de descarga na base recebe mesmo com a mina fora da tela. Os aldeões abastecem os terminais pela fila do capataz.

**E. Histórico.** Codex nuvem em 07/11 23:33 (interzone). #40 estações (0.23). DEC-044 renomeou o terminal.

**F. Pendências**
1. Caravanas e drones não usam produção virtual, então rotas longas param fora da simulação (B6-Q5).
2. Possível ambiguidade dos códigos 40/41 (B3 §3.10b).
3. Mundo com `freight_station` antiga do Codex: migração ND.

---

## SIS-TRB: Aldeões trabalhadores

**A. Identidade.** Os aldeões são a mão de obra da fábrica e a ponte para a sociedade. Distinguem o Veio de Factorio e Satisfactory (B1).

**B. Funcionamento: ciclo, comida e contratação**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-TRB-01 Contratação | 1 esmeralda; preferência por aldeão sem profissão ou nitwit | TRABALHADORES (E3) |
| FUN-TRB-02 Comida | 2 pontos de comida por tarefa; o pão paga 3 tarefas; trigo cru não fecha o ciclo | Grok lote 3 (E3) |
| FUN-TRB-03 Lavrador | Raio 8; área 9x9x3; desiste depois de 200 ticks; ausência máxima de 6.000 ticks | TRABALHADORES (E3) |
| FUN-TRB-04 Carregador | 16 itens por viagem; recolhe 32; raio 6; até 12 máquinas. Com a obra Ferraria: 24 e 48 | E3 |
| FUN-TRB-05 Capataz | Raio 12; 6 postos; fila de 8 tarefas a cada 40 ticks; rede de 24 blocos; a marca vence em 200 ticks | E3 |
| FUN-TRB-06 Pastor | Raio 6; 10 animais por espécie | E3 |
| FUN-TRB-07 Caminhada e descanso | Cérebro vanilla (0.16.0). Descanso noturno dos civis com status `RESTING = 28`, a partir de 12.000 ticks (`workforce/mod.rs:83-85`, E2) | E2 |
| FUN-TRB-08 Spawn na vila | O jogador nasce no centro da vila; gera `village_plains` depois de 30 s se não houver vila a 96 blocos | TRABALHADORES (E3) |
| FUN-TRB-09 Crescimento | Teto dinâmico: 30 + 3/posto com trabalhador + 5/nível de zona + 2/armazém + 20/centro, até 150 (`village.rs:121`, E2). Natalidade 1/6 a cada 5.999 ticks (1/3 com Escola). **Exige cama livre** (Grok #21) e vaga em casa com prefeito (`society::birth_allowed`) | E3/E2 |
| FUN-TRB-10 Staff das obras | Cozinheira, professor, ferreiro, bibliotecário, carteiro, estalajadeiro, cientista (`staff.rs`) | HANDOFF_OBRAS (E3); arquivo existe @pr47 (E2) |
| FUN-TRB-11 Escadas | Os carregadores sobem e descem escadas de mão entre andares | MEGAFABRICAS (E3) |
| FUN-TRB-12 Itens na mão | O trabalhador mostra o que carrega (0.16.1) | E3 |

**C. Experiência**
1. A pessoa contrata um aldeão com uma esmeralda e vê-o arar.
2. Abastece o posto com pão.
3. Põe um capataz para distribuir tarefas e carregadores para alimentar máquinas.
4. À noite, os civis dormem e os monstros caçam aldeões.

**E. Histórico**

| Versão | Mudança |
|---|---|
| 0.10 | Lavrador |
| 0.11 | Carregador e capataz |
| 0.12 | Fila de tarefas e pastor |
| 0.13 | Zonas e teto de 30 |
| 0.14 | Teto de 150 |
| 0.16 | Cérebro vanilla |
| 0.16.1 | Itens na mão |
| Obras | Staff |

**F. Pendências**
1. Sem teste em jogo desde a 0.11.
2. Confirmar que o "cama livre" do Grok está ativo no pr47 (OPUS A).
3. Unificar o número do teto com o limite da fase (B6-Q6).

---

## SIS-VIL: Sociedade da vila

**A. Identidade.** A vila deixa de ser só mão de obra. Famílias moram, recebem salário e consomem, e a prefeitura mede e libera as fases. Decisão de agente DEC-029.

**B. Funcionamento**

*Casas e famílias*

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-VIL-01 Casa | Até 4 aldeões a 10 blocos formam uma família. Fica apertada com mais de 6 casas vizinhas a 16 blocos. Bairro = 24 blocos. Despensa de 3 espaços; 1 ponto de comida por pessoa por turno | SOCIEDADE (E3) |
| FUN-VIL-02 Alvará | Fase 2: precisa de poço a 24 blocos. Fase 3: também poste a 12. Fase 4: também loja a 40 | README pr47 (E3) |
| FUN-VIL-03 Poço | Saneamento; enche baldes | README (E3) |

*Fases da prefeitura* (tabela `PHASES` em `mayor.rs`, E2)

| Fase | Requisitos | O que libera |
|---|---|---|
| 1 Povoado | Sem prefeitura | Zonas até nível 2, 30 aldeões, 4 alvarás, imposto 4% |
| 2 Vila | 12 moradores, 3 famílias, IDH 0,40 | Nível 3, 60 aldeões, 10 alvarás, imposto 8% |
| 3 Cidade | 36 moradores, 9 famílias, IDH 0,55, PIB 400/dia, emprego 60% | Nível 4, 100 aldeões, 24 alvarás, imposto 12% |
| 4 Metrópole | 80 moradores, 20 famílias, IDH 0,70, Gini ≤ 0,45, 3 bairros | 150 aldeões, 48 alvarás, imposto 16% |

*Indicadores do prefeito* (raio de 128 blocos)

| Indicador | Definição | Efeito |
|---|---|---|
| População | Adultos, crianças, famílias, sem-teto | Requisito de fase |
| Emprego | % dos adultos com posto | Requisito da fase 3 |
| PIB | **Consumo final no varejo do último dia** (README/SOCIEDADE). O guia, na pág. 33, diz "tarefas feitas e vendas" (conflito) | Requisito da fase 3 |
| IDH | Média geométrica de 4 dimensões por família: comida, moradia, serviços, renda (0–1) | Requisito de fase. **Se cair muito, o prefeito suspende novos níveis de zona e os nascimentos** |
| Gini | Desigualdade de renda | Requisito da fase 4 |

**C. Experiência**
1. A pessoa espalha casas para evitar o "apertado".
2. Põe um poço perto.
3. Contrata o prefeito e lê o livro da prefeitura com bairros, problemas e a meta da próxima fase.
4. A vila sobe de fase e libera zonas e população.

**E. Histórico.** 0.18 sociedade. 0.19 contas. 0.21 economia fechada. Na integração obras + sociedade ficou pendente: "serviços entram no IDH; prefeitura = obra" (SOCIEDADE, E3).

**F. Pendências**
1. Definição do PIB no guia × código.
2. Integrar os serviços das obras no IDH (OPUS A: está feito?).
3. Homonímia "Prefeitura" (B6-Q4).
4. VILA-CIDADE (casas variadas, praça, vida) Concebido.

---

## SIS-ECO: Economia e contabilidade

**A. Identidade.** Uma **economia fechada** em que o dinheiro circula entre 4 livros, para que a indústria tenha consumidores e não só "missões" (DEC-030). Princípio declarado pelo agente Cloud. O dono decidiu sobre ele só num ponto (DEC-035).

**B. Funcionamento: o circuito do dinheiro**

```
Companhia (cofre do centro) --salário 4/turno--> Famílias --compras--> Lojas (comerciante)
   ^                                                                   |
   +------------- atacado 75% do preço local <---- estoque entregue ---+
Cada venda --imposto na fonte 4/8/12/16%--> Prefeitura --> salário do prefeito, poços, assistência (reserva 200)
Lojas --repasse 10%/turno acima da reserva 600--> família do comerciante
Famílias com > 2 dias de custo de vida --1/4 do excedente/turno--> bens de conforto
Bordas: esmeraldas físicas (depósito/saque), compras do jogador, exportação (entra), importação (sai; a tarifa fica na prefeitura)
```

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-ECO-01 Moeda | 1 esmeralda = 60 moedas (`COINS_PER_EMERALD`) | ECONOMIA (E3) |
| FUN-ECO-02 Custo de vida e salário | 40 por pessoa por dia; `WAGE_PER_TURN = 2 × LIVING_COST / TURNS_PER_DAY` = 4 (80/dia); turno = 1.200 ticks, 20 turnos por dia | `society/mod.rs` (E2) |
| FUN-ECO-03 Imposto na fonte | 4/8/12/16% pela fase (`mayor::tax_rate`) | E2 |
| FUN-ECO-04 Livros | Companhia, Lojas, Prefeitura (livro `Public`), Famílias (`society/ledger.rs`). O relatório da prefeitura confere a identidade contábil e mostra a moeda em circulação por conta | README 0.21 (E3). Nomes exatos dos livros e identidade: OPUS A |
| FUN-ECO-05 Recompensa pública | `center::reward()` debita o livro Public (`ASSIST`) e credita a Company com o rótulo RECEIVED (`center.rs:609`, E2). Sem saldo, só prestígio | DEC-035 |
| FUN-ECO-06 Holding | Abas por empresa: mineração, manufatura, energia, logística, agro, extrativismo, urbanização, comércio, gestão e segurança. Receita, gastos, lucro, folha estimada | README 0.20 (E3) |
| FUN-ECO-07 Equilíbrio | `balance_*` nos testes; `sim_economy.py --check` passa (E1). Cenários do modelo: Povoado sem conexão −83/dia; Vila com porto +1.120; Cidade industrial +3.062; Vila sem comida −15 com 86% alimentada | E1 (modelo) |

**Fontes de moeda fora do livro (conflito; B6-Q1)**

| Fonte | Valor | Evidência |
|---|---|---|
| Armazém do povoado | 1 esm / 60 pts de comida, +50% com 3 aldeões | README (E3) |
| Exportação da central (EXPORT) | 4 esm por remessa, **sem fim, também na main** | README (E3); `contracts.rs:63-66` (E2 do A) |
| Contrato Povoado | 8 esm | README (E3) |
| Pedido diário do quadro | 1–6 esm, uma vez por dia por dimensão; em memória | `milestones/mod.rs:146-151` (E2) |
| Grande projeto | 32 / 48 / 64 esm | GRANDE-INDUSTRIA (E3) |
| Mk.II em jazida de esmeralda | 1 a cada 20 s | README (E3) |
| Trocas vanilla | — | — |

**C. Experiência.** O cofre do centro de controle é a conta da empresa. O jogador vê salários saindo, atacado entrando e lucro por empresa. A esmeralda física é a ponte com o mundo vanilla.

**E. Histórico.** 0.19 contas separadas. 0.21 economia fechada. 0.25 balanço com simulador. Obras e `reward` em 17:18.

**F. Pendências**
1. Decidir o destino das fontes externas de moeda (B6-Q1).
   [B7] O mesmo bem é **tributado duas vezes**: no varejo (família → loja) e no atacado (loja → companhia) (`market.rs` `settle`/`receive_sale`, E2 do A). Ver B6-Q13. O livro da holding e `BORDERS` ficam só em memória e zeram ao reiniciar.
2. "Caixa da vila" das obras × livros (OPUS A).
3. Pedido diário em memória, que se pode receber de novo ao reabrir (B6-Q2).
4. Nada disso foi testado em jogo.

---

## SIS-MER: Mercado e comércio exterior

**A. Identidade.** Dá destino à produção e é o motor que permite uma indústria maior que a vila ("O comércio exterior é o que sustenta uma indústria maior", ECONOMIA, E3). Inspiração declarada pelo agente: conexões externas de Cities: Skylines (0.23).

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-MER-01 Preço local (formação) | Fator de 50% a 200% do preço de referência; alvo = 100/√cobertura; anda 1/4 por turno; parado cai 5 por turno até 70% (`IDLE_FLOOR 70`); falta de comida soma 25. `FLOOR 50`, `CEILING 200` | `market.rs` (E2); SOCIEDADE (E3) |
| FUN-MER-01 (referência) | Valor = base × complexidade (matéria-prima 1×, processado 1,5×, componente 2×, eletrônico 3×, módulo 4×). Mesma âncora dentro e fora: pão = 9 | ECONOMIA (E3) |
| FUN-MER-01 (multiplicador) | `tier_factor` 100/150/200/300/400 (`trade.rs`, E2) | E2 |
| FUN-MER-02 Oferta e procura locais | Sobe quando a procura passa o estoque; cada vila tem seu mercado | README (E3) |
| FUN-MER-03 Atacado | O comerciante compra o que a companhia entrega por 75% do preço local (`WHOLESALE 75`); paga com as vendas (fiado se faltar caixa) | E2/E3 |
| FUN-MER-04 Bolsa mundial | Cotação = valor × complexidade × índice do dia (`index = 80 + hash % 41`, ou seja, 80–120%) | `trade.rs` (E2) |
| FUN-MER-05 Saturação | `spot = cheio × absorção / (absorção + pressão)`, mínimo de 25% do cheio (sucata). Absorção 256/512/512/256/128 por camada. Pressão = metade da média recente + vendido hoje | `trade.rs` (E2); ECONOMIA (E3) |
| FUN-MER-06 Frete | Porto: 9 pilhas a cada 2 min, frete 4 + 6 por pilha, precisa de 20 blocos de água. Estação: 6 pilhas por minuto, frete 2 + 12 por pilha, na fronteira a 96 blocos da sede. Status: sem companhia 32, sem água 33, fora da fronteira 34, mercado fechado 35 (`trade.rs:37-40`, E2) | E2/E3 |
| FUN-MER-07 Contratos | +50% sobre o preço cheio do dia, sem saturação (comprador à parte); até 3 contratos com prazo de 3–4 dias; lote 256/128/80/64/48 por camada; sem produto repetido | ECONOMIA; README 0.24 (E3) |
| FUN-MER-08 Reputação | Abre camadas aos níveis 1, 3 e 6 (`TIER_AT`); **camada aberta nunca fecha**; salva em `Trade`/`TradeRecent` (formato v3) | E3 |
| FUN-MER-09 Importação | Quando falta comida, a loja importa: mundial + 20% de frete + 10% de tarifa (a tarifa fica na prefeitura); margem de 10%. **Teto local = paridade de importação**; importa a comida que mais vende (`STAPLE` = pão) | `market.rs` `import_ceiling` (E2); E3 |
| FUN-MER-10 Liquidação | O piso de sucata de 25% garante que inundar nunca zera a receita. A carga para mercado fechado espera no porto | ECONOMIA; README 0.24 (E3) |
| FUN-MER-11 Escopo | Bolsa e reputação por dimensão; `PRODUCTS` = 34 itens, **sem os da grande indústria** | `trade.rs` (E2) |

**Papel estratégico.** Valor agregado manda: 256 engrenagens/dia rendem 2.418, contra 263 de 256 minérios. O frete pesa no minério (cerca de 10% na estação) e não na engrenagem (cerca de 1%). Os contratos puxam a reputação, que abre eletrônicos e módulos. Isso empurra a pessoa para cadeias longas (ECONOMIA, E3; números do modelo, E1).

**C. Experiência**
1. A pessoa constrói um porto no mar ou uma estação na fronteira.
2. Escolhe o que exportar e acompanha a bolsa do dia no relatório e na aba Comércio exterior do centro.
3. Recebe avisos no chat de contratos e mercados abertos; partidas com buzina e fumaça.

**E. Histórico.** 0.22 cais. 0.23 conexões externas. 0.24 acabamento. 0.25 balanço. Antes da 0.25, o pão valia 6 na loja e 9 fora, e a vila passava fome (ECONOMIA, E3).

**F. Pendências**
1. A grande indústria não exporta (B6-Q3).
2. A página "Comércio" do guia principal ainda fala da central.
3. Bolsa por dimensão × vários povoados: OPUS A.
4. Teste em jogo.

---

## SIS-OBR: Obras e plantas

**A. Identidade.** É a vila como projeto de construção. O prédio desenhado precisa existir para o efeito valer (DEC-033). Escopo: prestígio, pedidos, decretos e boletim.

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-OBR-01 Catálogo | Refeitório: 24 tijolos, 8 chapas de ferro, cozinha. Escola: 24 papéis, 6 livros, 16 tijolos. Ferraria: 8 aço, 8 engrenagens, bigorna. Mercado: 16 vidros, 16 chapas de cobre, 8 esmeraldas. Festa (repetível): bolo, 16 biscoitos, 16 pães. Biblioteca (nível 1). Correio (1). Estalagem (2). Prefeitura/`HALL` (3): 4 blocos de ouro, sino, 8 circuitos. Laboratório (1): 4 circuitos, 16 vidros, 8 aço | `projects.rs:98-140` (E2) |
| FUN-OBR-02 Efeitos | Refeitório: 4ª tarefa grátis. Escola: natalidade 1/3, teto +15. Ferraria: carregador 24/48. Praça/Mercado: zonas 25% mais baratas. Estalagem: teto +20. Prefeitura: prestígio +50% e juros | TRABALHADORES (E3) |
| FUN-OBR-03 Prestígio | Pedido +10, obra +25, festa +10, banquete +5. Níveis Aldeia 0 / Vila 60 / Vila próspera 200 / Cidade operária 500 (`TIERS`, `projects.rs:75`, E2); +5 no teto por nível | E2/E3 |
| FUN-OBR-04 Pedidos diários da vila | 3 por dia, 4 com Correio (+50%). Recompensa pelo caixa público (DEC-035) | E3/E2 |
| FUN-OBR-05 Caixa e juros | Saque de até 16; juros da Prefeitura de 1/16 por dia, até 8 | TRABALHADORES (E3) |
| FUN-OBR-06 Decretos | Sino, Papel, Feno (`CALL`, `RATION`, `PLENTY`, `projects.rs:53-55`, E2); slot 7 do centro | E2 |
| FUN-OBR-07 Canteiro e mestre de obras | O mestre assenta 10 blocos por tarefa; status 36–39 (`site.rs:42-45`, E2); conferência de 48 blocos por ciclo | E2/E3 |
| FUN-OBR-08 Plantas | Cerca de 25% maiores que a versão anterior (o guia diz Refeitório 11x9x12; o TRABALHADORES dizia 9x7x9) | HANDOFF (E3) |
| FUN-OBR-09 Boletim da manhã | Juros, viajante da estalagem, prestígio, pedidos novos (`center.rs:297`) | E2 |

**C. Experiência.**
1. No centro de controle, a pessoa escolhe a obra (slot 6).
2. Coloca o canteiro, entrega os materiais e vê o mestre erguer o prédio.
3. Ganha prestígio com pedidos e festas.
4. Todo amanhecer, lê o boletim.

**E. Histórico.** 7ª entrega: obras. 8ª: prestígio, pedidos, caixa, decretos. 9ª: plantas e canteiro. Feito pelo agente das obras, que ficou sem tokens. Chegou em zip pelo dono e entrou no #47 (`f1452f5`).

**F. Pendências** (HANDOFF_OBRAS, E3)
1. Traduções.
2. Ruínas num anel de 48–80 blocos.
3. Proteção de território (ganchos `destroy_block`).
4. Menu do canteiro com 3 slots.
5. Integração com o IDH.
6. Os bônus de teto podem ser anulados pelo limite de 150 e da fase (B6-Q6).

---

## SIS-LAR: Casa do fundador e Lar do jogador

**A. Identidade.** É a casa do jogador no centro da vila: âncora emocional e mecânica de ritmo. Canônico: DEC-046.

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-LAR-01 Casa do fundador | Gerada uma vez por mundo, com marca persistente e escolha do terreno; centro de controle, lareira, sofá, cama, mural com 4 mapas | CASA-FUNDADOR-INTEGRACAO (E3) |
| FUN-LAR-02 Lar `veio:home` | É a lareira da casa (`build_house.py:177-178`, E2); despensa, refeições, conforto de 0 a 100, melhorias de nível 2–4 com itens da fábrica, dono, renascimento | TRABALHADORES (E3) |
| FUN-LAR-03 Ritmo | Só por refeição e banquete do Lar (`home/mod.rs:101`: `time_factor = food::time_factor`, E2). Valores herdados da casa do fundador: refeição 0,95 por 5 min, banquete 0,91 por 8 min (OPUS A confirmar que valem no Lar). **O +15% por estar em casa foi removido**. [B7] Confirmado: ×0,95 por 6.000 t e ×0,91 por 9.600 t (`home/food.rs:22-29`); o bônus é **global** (atomics), não por jogador nem por Lar (B6-Q15) | E2/E3 |
| FUN-LAR-04 Camas | A cama da casa e mais uma externa; o renascimento respeita a cama marcada | E3 |

**C. Experiência.** O jogador nasce numa casa pronta, volta para comer e acelera a fábrica, e melhora a casa com produtos da própria indústria.

**E. Histórico.** Claude local (sexta entrega, +15%). Codex integrou como 0.15.3. Obras criaram o Lar. O dono fundiu os dois às 19:16.

**F. Pendências.** README linha 21 e TRABALHADORES ainda dizem "+15%". Confirmar os fatores 0,95 e 0,91 e o banquete (OPUS A).

---

## SIS-CTL: Centro de controle executivo

**A. Identidade.** É a sede da holding e o painel de comando da vila (Lacuna "visão global" do doc de lacunas).

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-CTL-01 Slots | 0 comida; 1–3 metas (`GOAL_SLOTS = 1..4`; `META_UNIT = 16`); 4 cofre; 5 saque; 6 obra; 7 decreto | `center.rs:28-29`, `projects.rs:68-70` (E2) |
| FUN-CTL-02 Raio | 64 blocos (`RADIUS`) | `center.rs:27` (E2) |
| FUN-CTL-03 Páginas | 4 páginas (Resumo, Atenção, Obras, …) + abas da holding (`VIEWS = PAGES + TABS - 1`); até 10 pendências | `center.rs:30,37,527` (E2) |
| FUN-CTL-04 Foco e bússola | Leva o jogador até cada pendência (`focus.rs`) | README pr47 (E3); E2 |
| FUN-CTL-05 Abas | Holding por empresa; Comércio exterior; relatório (livro) | E3 |

**C. Experiência.** Um único bloco responde "o que está parado e onde".

**E. Histórico.** 0.13 centro com metas. 0.20 holding. Obras: 8 slots, páginas e foco.

**F. Pendências.** Ainda não há "visão global" de toda a fábrica (mapa ou fluxo), só listas. O doc de lacunas pedia mais.

---

## SIS-PRG: Progressão

**A. Identidade.** Diz como o jogador avança. **É o sistema mais fragmentado**: há 7 mecanismos sobrepostos no pr47, nenhum deles desenhado em conjunto (achado principal; B6-Q3).

**B. Funcionamento**

| FUN | Mecanismo | Regra | Fonte | main | pr47 |
|---|---|---|---|---|---|
| FUN-PRG-01 | Avanços (receitas) | **Main:** engineering, steel e circuits pedem 1 item. Steel libera bateria, geotérmico, eletrônica, lote, poste de aço, vapor, silvicultura, rancho. Circuits libera solar, Mk.II, Tesla, quartel, módulos, lâmpada. **pr47:** engineering pede 32 engrenagens **ou** licença, e libera também o `research_lab`. Steel pede 16 ou licença e libera só eletrônica e elevador de carga. Circuits pede 16 ou licença e libera quartel, copiadora e marco de obra | `content` avanços (E2) | Integrado | Integrado |
| FUN-PRG-02 | Fases e licenças do Quadro da fábrica | 5 fases: 16 chapas → 32 engrenagens (`permit_workshop`) → 16 aço (`permit_steel`) → 16 circuitos (`permit_electronics`) → 32 circuitos (2 diamantes) | `milestones/mod.rs:37-51` (E2) | ausente | Integrado |
| FUN-PRG-03 | Pedido diário do quadro | 16–64 itens, paga 1–6 esmeraldas, uma vez por dia por dimensão, **em memória** | `milestones/mod.rs:146-151` (E2); EXPERIENCIA-JOGADOR (E3) | ausente | Integrado |
| FUN-PRG-04 | Central de contratos | Oficina (24 chapas + 8 engr. → 64 esteiras + 32 cabos); Entrepostos (24 aço + 12 módulos → trilhos); Povoado (24 circ + 32 molduras → 8 funis + 8 esm); Farol (estrela + 48 circ); Exportação contínua (4 circ + 4 aço → 4 esm + 2 frascos) | README (E3) | Integrado | Integrado |
| FUN-PRG-05 | Grande projeto (4 fases) | 200 aço + 100 módulos → 32 esm + 4 velocidades; 300 circ + 100 motores → 48 esm + 4 produtividades; 150 proc + 50 núcleos → 64 esm + 8 kits; 200 núcleos + 50 kits → 2 estrelas + 2 faróis | GRANDE-INDUSTRIA (E3) | ausente | Integrado |
| FUN-PRG-06 | Prestígio e fases da vila | Ver SIS-OBR e SIS-VIL | — | parcial | Integrado |
| FUN-PRG-07 | Reputação e níveis de zona | Ver SIS-MER e SIS-AGR | — | Integrado | Integrado |
| FUN-PRG-08 | Meta sustentada (20 circ/min por 3 min) | Concebido | README, lacunas (E3) | ausente | ausente |

**Como se avança (síntese do pr47):** extrair cobre → chapas → 32 engrenagens (oficina) → pesquisa *metallurgy* no laboratório (bateria, geotérmico, vapor, poste de aço, rancho, monitor, túnel) → 16 aço → pesquisa *electronics* (solar, Mk.II, Tesla, módulos, hidrelétrica, âncora, estações) → 16 circuitos → *automation* (fabricador, Mk.III) → *scale* (expressa) → grande projeto. Em paralelo, a vila sobe de fase (prefeitura), de prestígio (obras) e de reputação (comércio).

**C. Experiência.** O jogador vê pelo menos 4 "barras de nível" diferentes: quadro, prefeitura, prestígio e reputação. Nenhuma tela as mostra juntas.

**E. Histórico.** 0.5 central. "Claude (MineFac)" fases e quadro. #40 laboratório e grande projeto, que desloca liberações. Dono: tempos da 0.25 (DEC-045).

**F. Pendências**
1. Unificar ou hierarquizar os mecanismos (B6-Q3). [B7] **No pr47, o texto "Libera" do Quadro (`milestones/mod.rs:246`) promete máquinas que vêm de `research/*.json`, não das licenças.** Não existe `doLimitedCrafting` em nenhum ramo: o gate por avanço pode só esconder a receita do livro (comportamento no Horizon ND). Pesquisa e grande projeto são concedidos a `@a`.
2. README §Progressão e PROGRESSAO-FASES desatualizados.
3. Emissão de esmeraldas (B6-Q1).
4. Pedido diário em memória.

---

## SIS-PES: Pesquisa (novo ID proposto)

**A. Identidade.** Responde à lacuna "progressão por item" (DEC-017/018). Pede consumo contínuo de peças, não uma unidade.

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-PES-01 Laboratório | 4 pesquisas de 128–256 peças; consome 1 de cada entrada a cada 10 ticks; 24 FE/t; progresso do mundo em `<mundo>/veio/pesquisa.txt`; avanços com gatilho `impossible` concedidos pelo código | GRANDE-INDUSTRIA (E3); avanços @pr47 (E2) |
| FUN-PES-02 Árvore | metallurgy, electronics, automation, scale (itens em SIS-PRG) | E2 |
| FUN-PES-03 Pesquisas da vila | Agronomia, Logística, Nutrição, Pedagogia, Engenharia (obras; `research_bit`, `projects.rs:60-63`) | HANDOFF (E3); E2 |

**C. Experiência.** A pessoa alimenta o laboratório por esteira. Não há pressa, mas é preciso uma linha estável. É "entrega acumulada", não "produção sustentada" (doc de lacunas).

**E. Histórico.** #40, 08/11 ~17h. Obras (staff e pesquisas) em paralelo, sem coordenação entre as duas.

**F. Pendências**
1. Os dois sistemas de pesquisa usam o mesmo nome com regras diferentes.
2. Relação com as licenças do quadro (B6-Q3).

---

## SIS-TUT: Tutorial e guias

**A. Identidade.** Ensinar fazendo: "Você conhece Minecraft; a vila ensina a transformar uma oficina operada à mão numa linha que trabalha sozinha" (README pr47).

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-TUT-01 Tutorial na tela (pr47) | 8 passos: gerador → prensa energizada → 4 chapas → engrenagem → cobre extraído → prensa abastecida automaticamente (≥ 2 cobres) → 4 chapas guardadas automaticamente → 1ª encomenda da central (24 chapas + 8 engr.). Só produção nova conta; aceita esteiras, braços, funis e carregadores; `/veiotutorial ocultar\|mostrar\|reiniciar`; progresso individual em `<mundo>/veio/tutorial/<UUID>.txt`, salvo a cada 5 s | README pr47 (E3) |
| FUN-TUT-02 Livros | Guia do Veio (livro + papel); Primeira oficina (livro + pedregulho); Guia da Vila Operária (livro + trigo); Transportes; Grande Indústria. Chaves no pr47: guide 34, village 44, transport 11, industry 14 | README; lang (E2) |
| FUN-TUT-03 Limites | Guia principal ≤ 100 páginas; página ≤ 14 linhas | check_project (E3/E4) |

**C. Experiência.** Um objetivo por vez acima da barra. Os livros são consulta.

**E. Histórico.** 0.6 Guia do Veio. 0.14 Guia da Vila. Codex 0.15.x tutorial na tela. #40 livro da indústria.

**F. Pendências**
1. 29 páginas do Guia da Vila com mais de 14 linhas.
2. Duas páginas "8.".
3. Textos desatualizados (PIB, Comércio).
4. O tutorial termina na central e não apresenta água, sociedade, comércio exterior nem pesquisa.

---

## SIS-DEF: Ameaças e defesa

**A. Identidade.** É o custo de crescer: a poluição atrai ataques (o quarto gargalo, "Gargalos e melhorias").

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-DEF-01 Poluição | Gerador 6/s; forno de ligas e Mk.II 4; extratora, triturador e geotérmico 2; demais 1. Espalha a cada 3,2 s; dissipa 2%; colheita absorve 3. Fica em memória e recomeça ao reabrir | README (E3) |
| FUN-DEF-02 Aviso e gatilho | Chunk acima de 400: aviso. Ataque só um dia depois, à noite, com chunk acima de 1.200 | README (E3) |
| FUN-DEF-03 Ondas (0.17) | Chegam por um lado, num arco a 26 blocos da máquina mais poluída; 4 a 12 Saqueadores; arqueiros a partir da 3ª noite; Bruto (40 de vida, bate 6) a partir da 5ª; no máximo uma onda por noite | README main (E3); `pollution.rs:44,47` @pr47 (E2) |
| FUN-DEF-04 Monstros caçam aldeões | À noite; um aldeão se recupera depois de 30 s sem apanhar | README (E3) |
| FUN-DEF-05 Quartel | Milícia, guarda, arqueiro, mago; troca de classe agachando com uma esmeralda; plantão noturno | README (E3) |
| FUN-DEF-06 Reparo | Integridade zerada para a máquina; o kit de reparo recupera 50% | README (E3) |

**C. Experiência.** A fábrica suja traz noites de cerco. Muros, Tesla e quartel são a resposta, e a lavoura e a eficiência limpam.

**E. Histórico.** 0.7 Tesla. 0.8 poluição. Grok "noite" e quartel. 0.17 ondas.

**F. Pendências.** O README do pr47 voltou ao texto antigo das ondas (regressão só de doc). A poluição não persiste.

---

## SIS-MEG: Megafábricas

**A. Identidade.** É a "copiadora" do Satisfactory, pedida pelo usuário segundo MEGAFABRICAS (E3).

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-MEG-01 Copiadora `veio:blueprint_designer` | Prancheta de 5/9/15/21 por 24 de altura; até 6.000 blocos; cobra os materiais do inventário; preserva orientação e ajustes; não copia inventários | MEGAFABRICAS (E3) |
| FUN-MEG-02 Marco de obra | Até 256 blocos | E3 |
| FUN-MEG-03 Verticalidade | Elevador de carga 5/s; escadas para aldeões | E3 |
| FUN-MEG-04 Prancheta (#40) | Ferramenta de planejamento | GRANDE-INDUSTRIA (E3) |

**C. Experiência.** Projetar um módulo de fábrica e colar.

**E. Histórico.** Codex 08/11. Liberado pelo avanço circuits no pr47.

**F. Pendências.** Sem teste em jogo. "Projetos em memória não passam para outro mundo" (CHECKPOINT).

---

## SIS-APR: Apresentação

**A. Identidade.** Legibilidade no estilo Minecraft mais realismo opcional e leve (DEC-054, DEC-055, DEC-056).

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-APR-01 Painéis | No estilo de baú e fornalha; luz de estado na tela e nas 4 faces | README (E3) |
| FUN-APR-02 Formas | 34 modelos, 27 silhuetas; colisão acompanha a forma | VISUAL-MAQUINAS (E3) |
| FUN-APR-03 Sons | Kenney CC0; ambiente limitado; esteiras suaves; avaria; animação mecânica | AUDIO-MAQUINAS, AMBIENTE (E3) |
| FUN-APR-04 Realismo leve 1.1 | Shader WGSL MIT; perfis Realismo (8 chunks, simulação 5, 85%) e Leve (6 chunks, 75%); alternativa sem shader | CREDITS; README (E3) |
| FUN-APR-05 Realismo maior | Thread em andamento (Opus 5.5) | E4 |

**F. Pendências.** Até onde vai o realismo e se vira padrão (B6-Q7). PRs #49 e #50 continuam abertos apesar de o conteúdo estar no pr47.

---

## SIS-DES: Desempenho

**A. Identidade.** O jogo tem de rodar num PC de 8 GB (DEC-056).

**B. Funcionamento**

| FUN | Regra e números | Fonte |
|---|---|---|
| FUN-DES-01 Orçamento global | Por tick: elétrica 512; agricultura 384; fios 4.096; coordenação 2.048; alvos 512; carregadores 1.024; FIFO por categoria | ORCAMENTO-GLOBAL (E3) |
| FUN-DES-02 Buscas incrementais | Capatazes, carregadores e redes | CAPATAZES, CARREGADORES, REDES (E3) |
| FUN-DES-03 Ganhos medidos offline | 0.16.0: vilas grandes ~4× mais baratas. 0.17.1: −36% de trabalho por tick | README (E3) |
| FUN-DES-04 Produção virtual | Estações substituem simular a fábrica distante (SIS-TRA) | E3 |

Benchmarks do OPUS A em `SP/a/logs/*bench*`: a cargo dele.

**F. Pendências.** Não existe medida de FPS no Horizon real ("não alegam FPS no Horizon" em todos os lotes do Grok). A thread de limite e otimização está em andamento. [B7] **O pr47 regrediu** (E1 do A):
- `bench_factory` ~272 → ~385 µs/tick (+40 %);
- `bench_frame` de 0 para 329 alocações por quadro;
- `bench_large_village` entra em pânico (`world_dir`).

Isso contraria a DEC-056 (8 GB). O PR #52 (20:35Z) ataca parte do problema e não foi auditado. Criar FUN-DES-05 (critério de 8 GB).

---

## SIS-MP: Multijogador LAN

**A. Identidade.** Jogar com amigos no mesmo Wi-Fi, no mundo do anfitrião.

**B. Funcionamento**

| FUN | Regra | Fonte |
|---|---|---|
| FUN-MP-01 | Convidado novo vai para a vila do anfitrião | README pr47 (E3); Grok #25 |
| FUN-MP-02 | Tutorial individual por UUID; retorno da copiadora a quem clicou | README; CHECKPOINT (E3) |

**F. Pendências**
1. Pedido diário e bolsa por dimensão, não por jogador.
2. O caixa da companhia é único: falta uma regra para vários jogadores.
3. Sem teste com dois jogadores.
