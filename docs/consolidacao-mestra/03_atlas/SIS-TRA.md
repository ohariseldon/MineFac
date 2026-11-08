# SIS-TRA: Transporte entre zonas

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `pr47@80980db`. Na main o sistema não existe. O bloco `veio:freight_station` da main é do comércio exterior ([SIS-MER](SIS-MER.md)).
- **Linhas citadas:** salvo indicação, valem para `native/api/src/` no pr47.

## A. Identidade

O sistema liga pontos distantes da fábrica: uma mina a 300 blocos, um posto noutro andar, uma zona agrícola do outro lado do rio. É a resposta à lacuna "a fábrica longe para", que o dono mandou consertar em 16:32 ([DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017)). Em 17:01 ele especificou o comportamento: a fábrica distante continua produzindo e entregando "sem carregar tudo", como no Satisfactory ([DEC-041](../06_HISTORICO_E_DECISOES.md#dec-041)).

**Cinco meios, com papéis diferentes:**

| Meio | Bloco | Alcance | Precisa da área carregada? | Para que serve |
|---|---|---|---|---|
| Estação de carga e descarga | `veio:cargo_station`, `veio:unloading_station` | Sem limite (canal global) | **Não**: produção virtual | Fábrica distante (canônico, [DEC-041](../06_HISTORICO_E_DECISOES.md#dec-041)/043) |
| Terminal ferroviário | `veio:rail_terminal` | O dos trilhos | Sim | Trem vanilla com vagão de baú |
| Porto de drone | `veio:drone_port` | 64 blocos | Sim (`level_is_loaded`) | Saltos curtos sobre obstáculos |
| Estação de caravana | `veio:caravan_station` | 512 blocos | Sim | Rota terrestre com animal de verdade |
| Elevador de carga | `veio:cargo_lift` | Coluna vertical | Sim | Andares de uma megafábrica |
| Âncora de chunk | `veio:chunk_anchor` | 1 chunk | É ela que carrega | Simulação real de uma área (opcional, [DEC-040](../06_HISTORICO_E_DECISOES.md#dec-040)) |

- **Depende de:**
  - [SIS-ENE](SIS-ENE.md): estações gastam 10 FE por item; drones, 800 + 32×d FE; âncora, 32 FE/t;
  - [SIS-LOG](SIS-LOG.md): esteiras e braços alimentam e esvaziam as estações;
  - [SIS-PER](SIS-PER.md): `estacoes.txt` e `ancoras.txt`;
  - [SIS-DES](SIS-DES.md): custo por tick das estações virtuais.
- **Alimenta:** [SIS-PRO](SIS-PRO.md) (insumos na base), [SIS-TRB](SIS-TRB.md) (o capataz abastece terminais pela fila) e [SIS-MEG](SIS-MEG.md) (elevador).

## B. Funcionamento

### B.1 Estação de carga e descarga (o mecanismo principal)

A comparação é com o Satisfactory: o jogo supõe que uma fábrica estável continua estável quando ninguém olha.

1. **Canal.** O jogador põe um corante no slot da estação. As 16 cores são 16 canais (`stations.rs:30-31`). A carga de um canal vai para qualquer descarga do mesmo canal, **em qualquer lugar do mundo**.
2. **Envio real.** A estação de carga recebe itens por esteira, braço ou funil. A cada 8 ticks ela manda até 8 itens ao canal (1 item por tick, 1.200 por minuto). Cada item custa 10 FE (`stations.rs:20-24`).
3. **Trânsito.** O canal guarda até 4.096 itens (`CAPACITY`). Com a descarga cheia ou ausente, o canal enche e a carga para, como uma esteira travada.
4. **Medição.** Enquanto a área está carregada, a carga mede quantos itens de cada tipo manda numa janela de 1.200 ticks (1 minuto). Esse ritmo fica em `rates`.
5. **Produção virtual.** Se a carga não roda há mais de 100 ticks (`UNLOADED_AFTER`), o jogo considera a área descarregada. A cada passo, `simulate` soma `ritmo × tempo decorrido` por item, guarda as frações e põe os inteiros no canal, até o limite de 4.096 (`stations.rs:117-140`).
6. **Volta.** Quando a área recarrega, a carga retoma o envio real de onde parou, sem recuperar duas vezes. A recuperação por máquina foi descartada justamente por contar em dobro ([DEC-042](../06_HISTORICO_E_DECISOES.md#dec-042)).
7. **Persistência.** Canais, cargas e ritmos ficam em `<mundo>/veio/estacoes.txt`, com linhas `S dim x y z canal visto item ritmo` e `P canal item qtd` (`stations.rs:207`).

**O que a produção virtual não faz** (E2, `stations.rs:117-140`):

| Esperado pela intenção do dono | Código |
|---|---|
| Consumir os insumos da fábrica distante | **Não consome.** Nenhuma máquina roda. O ritmo medido vira itens do nada |
| Gastar energia | **Não gasta.** Os 10 FE/item só são cobrados no envio real |
| Parar se a fábrica foi desmontada | **Não para.** Só `forget_idle` apaga cargas que nunca mediram nada. Uma carga com ritmo medido segue mandando até o canal encher, mesmo com as máquinas quebradas |
| Respeitar dimensões | **Não respeita.** O canal é global: carga no Nether e descarga no Overworld trocam itens |

Consequência de design: uma mina medida por um minuto vira uma fonte infinita e grátis. Isso enfraquece a economia fechada ([SIS-ECO](SIS-ECO.md)) e o custo de energia ([SIS-ENE](SIS-ENE.md)). A decisão sobre o custo está na [Q-05](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo).

**Custo de processamento** (E1, `evidencias/logs/pr47_stations_cost.log:25`): 194,4 µs por 1.000 estações a cada passo de 20 ticks. A estimativa de 80 µs da `frentes/GRANDE-INDUSTRIA.md` está errada.

### B.2 Terminal ferroviário

- O terminal é um ponto de troca. O bloco só define status e o item importado (`FreightImport`). **Quem move a carga são o vagão de baú e os funis vanilla** (A2 TRA).
- Cada zona é identificada por uma cor. Cada terminal declara o produto que exporta e o que importa.
- Até 0.16.1 do Codex, este bloco se chamava `veio:freight_station`. O dono mandou renomear em 17:41 ([DEC-044](../06_HISTORICO_E_DECISOES.md#dec-044), `a79f326`).

### B.3 Drones

1. O porto recebe um cartão de rota (`veio:route_card`, `freight.rs:11`) que aponta para outro porto a até 64 blocos (`RANGE`).
2. A cada voo o drone leva até 16 itens de um único tipo, do slot de carga (`CARGO = 6`).
3. **Custo:** `800 + 32 × distância` FE, pago uma vez pela ida e pela volta (`freight.rs:236`). O porto guarda até 8.000 FE. Sem energia, status 5.
4. **Duração:** 20 + 2 × distância ticks.
5. **Validação:** antes de entregar, o porto confere se o destino ainda é um porto do mesmo canal e se importa aquele item (`freight.rs:65-69`). Produtos diferentes em cada sentido evitam o ciclo A→B→A (`freight.rs:230`).
6. **Limite:** os dois portos precisam estar em área carregada (`level_is_loaded`). O registro de portos (`PORTS`) fica em memória e se refaz a cada segundo.

### B.4 Caravanas

Usam **um animal real, designado pelo jogador** (`freight/caravan.rs:1-2`). A carga fica retida no terminal até o animal chegar de fato ao terminal de destino.

| Animal | Capacidade | Velocidade |
|---|---|---|
| Cavalo | 16 | 1,1 |
| Burro ou mula | 64 | 0,75 |
| Porco | 8 | 0,5 |

- Rotas de até 512 blocos (`RANGE`). A ração vai no slot 7.
- **Status:** 40 sem rota, 41 sem animal, 42 sem ração, 43 animal ocupado (`freight.rs:15-18`).
- Os códigos 40 e 41 também são usados pelo centro de controle para "zona pronta" e "armazém cheio". Hoje isso não tem efeito, porque `waiting()` do centro só aceita códigos de espera (`center.rs:111-113,193-205`). É um risco latente ([CNF-013](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
- O animal anda pela navegação real. Fora da área carregada, a caravana **para** ([DEC-051](../06_HISTORICO_E_DECISOES.md#dec-051)). Isso diverge da [DEC-041](../06_HISTORICO_E_DECISOES.md#dec-041), que pede continuidade.

### B.5 Elevador de carga

- Os elevadores se empilham numa coluna. Cada bloco passa 1 item a cada 4 ticks (`lift.rs` `STEP 4`, 5 itens/s) ao vizinho de cima, ou de baixo em descida.
- O último da coluna entrega pela frente: numa esteira, máquina, baú ou outro elevador.
- Não gasta energia. Redstone pausa o bloco. Com a chave e o jogador agachado, a coluna alterna subida e descida (`lift.rs:1-9`).

### B.6 Âncora de chunk

- Mantém o próprio chunk carregado pelo comando `forceload` (`industry.rs:285-291`). Custa 32 FE/t (`ANCHOR_POWER`, `industry.rs:21`).
- Sem energia, o chunk é solto. As âncoras ficam em `ancoras.txt` (`industry.rs:260`), para que o chunk de uma âncora quebrada também seja solto antes de fechar o mundo.
- `forceload` não é API do Horizon, é comando de console. O Horizon tem `native_chunk_tickets.rs`, que a âncora não usa (A7).
- **Papel:** opcional. O dono escolheu a âncora em 16:38 ([DEC-040](../06_HISTORICO_E_DECISOES.md#dec-040)) e complementou em 17:01 com a continuidade sem carregar ([DEC-041](../06_HISTORICO_E_DECISOES.md#dec-041)). A âncora serve para quem quer simulação real de uma área pequena.

### B.7 Qual meio usar (regra de projeto)

| Situação | Meio | Por quê |
|---|---|---|
| Mina ou fábrica fora da área do jogador | Estação de carga/descarga | Único meio que segue produzindo fora da tela |
| Duas áreas sempre carregadas, a menos de 64 blocos | Drone | Atravessa obstáculos |
| Linha longa com trilhos | Terminal ferroviário | Visual e vanilla, mas para fora da área carregada |
| Rota longa no começo do jogo, sem energia | Caravana | Não usa FE; para fora da área carregada |
| Andares | Elevador ou escadas ([SIS-MEG](SIS-MEG.md)) | Vertical |
| Área que precisa rodar de verdade (rios, pastagem) | Âncora | Carrega o chunk |

## C. Experiência do jogador

1. **Primeira rota (caravana).** A pessoa monta uma pedreira a 200 blocos. Põe duas estações de caravana, sela um burro e designa o animal. O burro vai e volta com 64 itens. Quando ela se afasta muito, o burro para.
2. **Fábrica distante (estação).** Ela põe uma estação de carga na saída da mina, com corante azul. Na base, uma descarga azul. Por um minuto a mina roda com ela por perto, enquanto a estação mede o ritmo. Depois a pessoa volta para casa e os minérios continuam chegando.
3. **Megafábrica.** Elevadores levam placas do térreo ao terceiro andar.
4. **Surpresa negativa (hoje).** Se ela desmontar a mina, os minérios continuam chegando. Num save da 0.16.1 do Codex, o antigo terminal ferroviário vira uma estação de exportação e vende a carga ([CNF-014](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

Nada disso foi visto em jogo (coluna J vazia).

## D. Implementação

**Arquivos (pr47)**
- `stations.rs` (384 linhas): estações, canais, simulação, `estacoes.txt`.
- `freight.rs` (355): terminal ferroviário, drones, cartão de rota, status 40–43.
- `freight/caravan.rs` (248): caravanas e objetivo de IA do animal.
- `lift.rs` (150): elevador.
- `industry.rs` (399): âncora (com monitor de itens/min de [SIS-PRO](SIS-PRO.md)).
- Testes: `freight` 21 (14 em `freight/tests.rs` + 7 em `freight/caravan/tests.rs`), stations 2, lift 2 (E1, `check_rust` pr47 325/0/9).
- Slots: a estação tem 4 (`lib.rs:550-551`).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-TRA-01 Terminal ferroviário `veio:rail_terminal` | Ponto de troca; carga movida por vagão e funil vanilla; ID por [DEC-044](../06_HISTORICO_E_DECISOES.md#dec-044) | — · `freight.rs:7` (`RAIL`), `a79f326` · #28 como `freight_station` (contido) | E2; E1 testes de frete; J nenhum |
| FUN-TRA-02 Drones e porto | 64 blocos; 16 itens; 800 + 32×d FE; 20 + 2×d ticks; buffer 8.000; área carregada | — · `freight.rs:8,12`, `:225-247` · #28 (contido) | E1 + E2 |
| FUN-TRA-03 Caravanas | Cavalo 16/1,1; burro 64/0,75; porco 8/0,5; 512 blocos; ração slot 7; status 40–43 | — · `freight/caravan.rs:6-25`; `freight.rs:15-18` · #28 (contido) | E1 + E2 |
| FUN-TRA-04 Estações de carga/descarga | 16 canais × 4.096; 8 itens/8 t; 10 FE/item; janela 1.200 t | — · `stations.rs:17-31` · #40 (contido) | E1 + E2 |
| FUN-TRA-05 Produção virtual ([DEC-041](../06_HISTORICO_E_DECISOES.md#dec-041)/043) | Depois de 100 t sem rodar, soma ritmo × tempo **sem insumo nem energia**, canal global | — · `stations.rs:117-140` · #40 (contido) | E1 (194,4 µs/1.000 estações); E2. **Diverge da intenção** ([Q-05](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo)) |
| FUN-TRA-06 Âncora com custo de energia ([DEC-040](../06_HISTORICO_E_DECISOES.md#dec-040)) | 32 FE/t; `forceload`; `ancoras.txt` | — · `industry.rs:19-21`, `:253-291` · #40 (contido) | E2; comando `forceload` no Horizon real ND |
| FUN-TRA-07 Elevador de carga | 1 item/4 t; sem energia; redstone pausa | — · `lift.rs:1-12` · import-megafactories (contido) | E1 (2 testes) |

Estados: main ausente; pr47 **Integrado** em todas. Ver [05](../05_ESTADO_REAL_DA_BUILD.md).

## E. Histórico

| Quando (UTC) | Commit / PR | Mudança |
|---|---|---|
| 07/11 23:33 → 08/11 00:00 | `codex/interzone-logistics`, PR #28 (aberto, contido no pr47) | Codex nuvem cria terminais, drones e caravanas. O terminal usa o ID `veio:freight_station` |
| 08/11 13:16 | megafábricas (Codex local) | Elevador de carga |
| 08/11 16:38 | [DEC-040](../06_HISTORICO_E_DECISOES.md#dec-040) | Dono escolhe a âncora (opção 0) |
| 08/11 17:01–17:08 | [DEC-041](../06_HISTORICO_E_DECISOES.md#dec-041), [DEC-042](../06_HISTORICO_E_DECISOES.md#dec-042) | Dono pede continuidade "sem carregar tudo"; recuperação por máquina proposta e abandonada (`e36ee2e`) |
| 08/11 ~17:06 | #40 | Estações com produção virtual ([DEC-043](../06_HISTORICO_E_DECISOES.md#dec-043)) |
| 08/11 17:41 | `a79f326`, [DEC-044](../06_HISTORICO_E_DECISOES.md#dec-044) | Terminal renomeado para `veio:rail_terminal` |
| 08/11 19:06 | `eedfd47` | #40 fundido no pr47 |

## F. Pendências

1. Decidir o custo da produção virtual ([Q-05](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo)): consumir insumo e energia, ter teto, ou ser invalidada quando o posto é quebrado.
2. Tornar os canais por dimensão, ou registrar a decisão de deixá-los globais.
3. Caravanas, drones e trem param fora da área simulada. Decidir se seguem a [DEC-041](../06_HISTORICO_E_DECISOES.md#dec-041) ou ficam como meios "locais" declarados ([Q-05](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo)).
4. Renumerar os status de frete 40–43 ou criar um registro central de status ([CNF-013](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [04 §10.2](../04_ARQUITETURA_E_DEPENDENCIAS.md)).
5. Escrever nota de migração para saves 0.16.1 e Realismo 1.1 com o antigo `freight_station` ([CNF-014](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
6. Trocar `forceload` por tickets do Horizon (`native_chunk_tickets.rs`), ou confirmar o comando no Horizon real.
7. Gravar `estacoes.txt` de forma atômica e com versão de formato ([SIS-PER](SIS-PER.md)).
8. Incluir as estações virtuais em `check_performance` com um cenário de escala.
9. Corrigir "80 µs" em `frentes/GRANDE-INDUSTRIA.md` para 194 µs.
10. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-007](../08_BACKLOG_E_PRIORIDADES.md#tar-007): Proteger saves 0.16.1 e Realismo 1.1 da troca de sentido de `veio:freight_station`
- [TAR-027](../08_BACKLOG_E_PRIORIDADES.md#tar-027): Produção virtual com canal por dimensão, invalidação e custo
- [TAR-029](../08_BACKLOG_E_PRIORIDADES.md#tar-029): Persistir o que some ao reiniciar
- [TAR-044](../08_BACKLOG_E_PRIORIDADES.md#tar-044): Âncora pela API de tickets

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main — · pr47 **Integrado** (7 de 7) · disperso: #28 e #40 contidos no pr47 |
| Confiança | Média: E2 completo e E1 nos testes; APIs `forceload` e `level_is_loaded` sem compilação real; J nenhum |
| Prioridade (sugestão) | Alta: é o sistema que cumpre uma decisão explícita do dono ([DEC-041](../06_HISTORICO_E_DECISOES.md#dec-041)) |
| Criticidade (sugestão) | Alta para o desenho (fonte grátis); média para o save (dois arquivos sem versão) |
| Responsável conhecido | Codex nuvem (terminais, drones, caravanas); Claude da thread "agricultura" (#40: estações, âncora); Codex local (elevador); integrador do #47 (renomeação) |
| DEC | [DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017), [DEC-040](../06_HISTORICO_E_DECISOES.md#dec-040), [DEC-041](../06_HISTORICO_E_DECISOES.md#dec-041), [DEC-042](../06_HISTORICO_E_DECISOES.md#dec-042), [DEC-043](../06_HISTORICO_E_DECISOES.md#dec-043), [DEC-044](../06_HISTORICO_E_DECISOES.md#dec-044), [DEC-051](../06_HISTORICO_E_DECISOES.md#dec-051) |
| Q | [Q-05](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-05-produção-virtual-e-transporte-longo), [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem) ("estações" de carga × "estações do ano" do #51) |
| CNF | [CNF-009](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (fábrica distante × produção virtual), [CNF-013](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (status 40/41), [CNF-014](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (`freight_station` sem migração), [CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (nomes) |
| Referências | `frentes/TRANSPORTES.md`, `frentes/GRANDE-INDUSTRIA.md`, `frentes/MEGAFABRICAS.md`; A2 TRA; A7 §2; `evidencias/logs/pr47_stations_cost.log` |
