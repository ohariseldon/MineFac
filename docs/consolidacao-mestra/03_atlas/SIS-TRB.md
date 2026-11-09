# SIS-TRB: Aldeões trabalhadores

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. O núcleo (`workforce/mod.rs`, postos, capataz, zonas) é o mesmo nas duas. O pr47 acrescenta a regra da cama livre, os efeitos das obras e o pagamento de zonas pelo caixa.
- **Linhas citadas:** salvo indicação, valem para `native/api/src/workforce/` no pr47.

## A. Identidade

Os aldeões são a mão de obra da fábrica. É o que distingue o Veio de Factorio e Satisfactory ([02 P3](../02_VISAO_E_CONCEPCAO_DO_JOGO.md)). O trabalho é **físico**: o aldeão anda até o alvo e só age ao lado dele. Os itens viajam com ele (`Carried` no NBT do posto) (`mod.rs:1-18`).

**Conceitos:**

| Termo | Definição |
|---|---|
| Trabalhador | Aldeão adulto, identificado pelo UUID salvo no posto. O id de entidade só vale na sessão |
| Posto | Bloco que guarda a atribuição, a comida do trabalhador e os estoques do setor |
| Tarefa | `Visit` (ir e agir) ou `Bring` (levar itens do posto a outro bloco), escolhida pelo adaptador do setor |
| Atribuição | Exclusiva: um aldeão pertence a no máximo um posto (`CLAIMS`) |
| Reserva | Um alvo atendido por um posto não é escolhido por outro enquanto a reserva for renovada |

Aldeões sem ordem recente seguem o comportamento vanilla, inclusive o comércio.

- **Depende de:** [SIS-AGR](SIS-AGR.md) (comida), [SIS-PRO](SIS-PRO.md) (máquinas atendidas), [SIS-OBR](SIS-OBR.md) (efeitos das obras), [SIS-VIL](SIS-VIL.md) (alvará e fase para nascer).
- **Alimenta:** [SIS-PRO](SIS-PRO.md), [SIS-AGR](SIS-AGR.md), [SIS-TRA](SIS-TRA.md) (terminais abastecidos pela fila), [SIS-VIL](SIS-VIL.md) (empregos, população) e [SIS-CTL](SIS-CTL.md) (pendências).

## B. Funcionamento

### B.1 Contratação e comida (o custo de um trabalhador)

1. **Contratar** custa 1 esmeralda, paga só quando a contratação dá certo (`mod.rs:65-66`). O posto contrata o aldeão adulto livre mais próximo a até 8 blocos (`HIRE_RADIUS`). Com o decreto Convocação o raio dobra para 16 (`projects.rs:470-471`). A preferência por aldeão sem profissão ou nitwit vem de `TRABALHADORES.md` (E3).
2. **Comida.** Cada tarefa concluída (colheita, viagem, recontratação) gasta 2 pontos (`TASK_COST`, `mod.rs:96`). O trigo cru vale 1 ponto e não paga uma tarefa. O pão vale 6 e paga três ([DEC-025](../06_HISTORICO_E_DECISOES.md#dec-025)). Sem comida, o posto mostra o status 21 e para.
3. **Modificadores** (`projects.rs:416-431`), aplicados nesta ordem:

| Condição | Custo por tarefa |
|---|---|
| Festa da colheita ativa | 0 |
| Refeitório de pé | 0 a cada 4ª tarefa; 2 nas demais |
| Decreto Racionamento | 1 (e ninguém nasce) |
| Decreto Fartura | 3 |
| Padrão | 2 |
| Pesquisa Nutrição concluída | −1 sobre o resultado, se ele for maior que 1 |

4. **Ritmo.** O posto age a cada 10 ticks, com a fase espalhada pela posição (`EVERY`). Uma ordem desiste depois de 200 tentativas de caminhada (`WALK_LIMIT`). O trabalhador ausente por 6.000 ticks é liberado (`ABSENT_LIMIT`).

### B.2 Os quatro postos

| Posto | Área | O que faz | Números |
|---|---|---|---|
| Lavrador (`farm_post`) | 9×9 em volta; ou a zona de lavoura inteira | Colhe e replanta. Com zona, também **planta** sementes da saída e **ara** terra com água a até 4 blocos. Plantar e arar não gastam comida (`farmhand.rs:1-10`) | O marco da zona pode fixar a cultura |
| Pastor (`shepherd_post`) | Raio 6; ou o cercado inteiro | Recolhe ovos, penas, couro e lã; tosquia; alimenta para reproduzir até 10 por espécie. Não ordenha nem abate (`shepherd.rs:1-14`) | No cercado, a ração escolhe as espécies: trigo (vaca, ovelha, cabra), cenoura (porco, coelho), sementes (galinha) |
| Carregador (`hauler_post`) | 13×13 em 3 camadas (`RADIUS 6`) | Abastece até 12 máquinas com os insumos do posto e recolhe os produtos | 16 itens por viagem (`BATCH`); completa cada entrada até 16 (`TOP_UP`); ×1,5 com a Ferraria e +8 com a pesquisa Carga (`projects.rs:434-437`) |
| Capataz (`foreman_desk`) | 25×25 (`RADIUS 12`) | Coordena até 6 postos: leva comida a quem está ficando sem, recontrata postos vazios com a esmeralda do slot 3 e mantém a fila de tarefas (`foreman.rs:1-17`) | Ver B.3 |

**Prioridade do carregador** (`hauler.rs:9-11`):
1. tarefa da fila de uma mesa de capataz próxima;
2. máquina parada por falta de insumo ou combustível;
3. produto pronto;
4. completar entradas até 16.

O carregador só atende estações do tipo `Workstation`. Nunca atende esteiras, divisores, postes, cabos ou outros postos (`hauler.rs:4-7`).

### B.3 A fila do capataz

A fila é o que transforma postos isolados numa rede. Constantes em `foreman.rs:18-45`:

| Regra | Valor |
|---|---|
| Rede da mesa (postos e máquinas) | até 24 blocos de rede, a até 12 blocos |
| Fila | até 8 tarefas, refeita a cada 4 ciclos (40 ticks) |
| Quem pega tarefas | Carregadores a até 16 blocos da mesa (`DESK_RANGE`) |
| Alcance de uma tarefa | Origem e destino a até 12 blocos do posto do carregador (`HAUL_RANGE`) |
| Marca de uma tarefa | Vence em 200 ticks sem renovação e a tarefa volta a ficar livre |
| Comida da mesa | Pede comida pela fila quando guarda menos de 16; reabastece postos com menos de 4 |
| Varredura | Uma fileira do 25×25 por ciclo; a volta completa leva 75 ciclos. Depois, 3 postos inspecionados por ciclo |

- **O que entra na fila:** produto pronto de um bloco para a entrada de outro que o aceite, inclusive comida e esmeraldas para a própria mesa, ração para o pastor e armazéns como último destino das sobras e como origem.
- **Planejamento e busca incrementais** (Codex `13758bd`, `604369d`): a preparação da fila (`foreman/planning.rs`) e a busca espacial (`foreman/search.rs`) avançam por vários ticks e nunca tocam inventários. O planejamento é retomável.
- **Recusa de entrega:** destino que não aceita o item é ignorado (`operations::container_accepts`, [SIS-LOG](SIS-LOG.md) FUN-LOG-06).

### B.4 Zonas de trabalho

Um marco de lavoura (`field_marker`) ou de cercado (`pen_marker`) define uma área quadrada em 3 camadas (`zones.rs:1-14`). Postos do tipo certo a até 3 blocos da borda trabalham nela inteira.

| Nível | Lado | Custo para abrir | Trabalho feito na zona | Aldeões na vila (raio 48) |
|---|---|---|---|---|
| 1 | 9 | — | — | — |
| 2 | 13 | 8 esmeraldas | 32 | 6 |
| 3 | 17 | 16 | 128 | 12 |
| 4 | 21 | 32 | 384 | 20 |

- **Pagamento:** na main, só esmeraldas físicas (`zones.rs:249@main`). No pr47, primeiro o caixa da companhia (que **destrói** moeda) e depois esmeraldas físicas; em caso de falha, `return_treasury` devolve (`zones.rs:256-268`). A Praça do mercado tira 25% (`projects.rs:460`).
- **Crédito de trabalho** por tarefa: 1, +1 com a Biblioteca, +1 com Fartura, +1 com a pesquisa Agronomia (`projects.rs:465-467`).
- **Teto por fase:** a fase da prefeitura limita o nível de zona ([SIS-VIL](SIS-VIL.md) B.2).

### B.5 Crescimento da vila

Dois tetos e um sorteio (`village.rs:1-14,21-33,103-154`):

1. **Teto de desenvolvimento**, a até 64 blocos: `30 + 3 por posto com trabalhador + 5 por nível de zona + 2 por armazém + 20 por centro com administrador + extra das obras`, limitado a 150. O extra das obras é Escola +15, Estalagem +20 e +5 por nível de prestígio (`projects.rs:455-457`).
2. **Teto de camas:** camas livres num raio de 32, em três camadas. A cabeceira conta, o pé não. A primeira volta leva cerca de 40 s. Até ela terminar, vale só o desenvolvimento. Depois, **teto = mínimo entre desenvolvimento e camas**.
3. **Sorteio:** a cada 5.999 ticks (cerca de 5 min), cada adulto tem 1 chance em 6 de ter um filho, se houver pelo menos 2 adultos a até 48 blocos.

| Efeito | Chance |
|---|---|
| Padrão | 1/6 |
| Escola | 1/3 |
| Festa da colheita | 1/2 |
| Decreto Fartura | 1/4 |
| Decreto Racionamento | ninguém nasce |

4. **Sociedade:** no pr47, o nascimento também exige que a fase permita, que a prefeitura não esteja suspensa e que exista vaga em casa com alvará (`society/mod.rs:328-341`).

Numa vila madura, os bônus de teto das obras podem não ter efeito: o teto absoluto de 150 e o limite da fase vêm antes ([CNF-008](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-06](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-06-teto-de-população-e-bônus-das-obras)).

### B.6 Spawn, noite e andares

- **Spawn** (`spawn.rs:1-13`): no primeiro login, o Veio procura uma vila a até 96 blocos e põe o jogador no centro dela. Sem vila em 600 ticks, gera uma `village_plains` ali e procura de novo. Com um centro de controle com administrador, o spawn do mundo e de todos passa para cima dele. A tag `veio_village` impede que a chegada se repita. Só no Overworld.
- **Noite:** a partir de 12.000 o trabalhador dorme, como todo aldeão vanilla. O posto espera em vez de disputar a cama (status 28, `RESTING`, `mod.rs:83-90`). A navegação usa o cérebro vanilla desde a 0.16.0 ([DEC-028](../06_HISTORICO_E_DECISOES.md#dec-028)).
- **Andares:** o posto planeja a rota pelas escadas de mão, porque a navegação vanilla não planeja escadas (`ladders.rs:1-9`; [SIS-MEG](SIS-MEG.md) FUN-MEG-04).

### B.7 Status no painel

| Código | Sentido |
|---|---|
| 18 | Sem trabalhador |
| 19 | Trabalhador ausente |
| 20 | Sem tarefa |
| 21 | Sem comida |
| 22 | Sem caminho |
| 23 | Andando |
| 24 | Carregador ou capataz ocioso |
| 25 | Capataz sem esmeralda para recontratar |
| 26 | Marco de zona ativo |
| 27 | Armazenando |
| 28 | Descansando (noite) |

Fonte: `mod.rs:68-83`. Tabela global em [04 §10.1](../04_ARQUITETURA_E_DEPENDENCIAS.md).

## C. Experiência do jogador

1. **Primeiro trabalhador.** A pessoa põe um posto de lavrador ao lado da horta da vila, coloca pão e uma esmeralda, e vê o aldeão mais próximo vir colher.
2. **Indústria.** Um carregador entre a prensa e o montador mantém as entradas cheias. O painel mostra "sem comida" quando o pão acaba.
3. **Rede.** A mesa do capataz faz os carregadores levarem chapas de uma máquina para outra e manda comida para o lavrador.
4. **Zona.** Ela abre a lavoura nível 2 (13×13) com 8 esmeraldas depois de 32 tarefas e com 6 aldeões na vila.
5. **Crescimento.** Põe camas, contrata mais gente, e a vila passa de 30.
6. **Noite.** Os trabalhadores dormem. Os monstros caçam aldeões ([SIS-DEF](SIS-DEF.md) FUN-DEF-06).

Nenhum desses passos foi visto em jogo desde a 0.11 (README pr47 l.31).

## D. Implementação

**Arquivos (pr47)**
- `mod.rs` (1.581 linhas): conceitos, ciclo do posto, status, contratação, comida.
- `farmhand.rs`, `shepherd.rs`, `hauler.rs`, `foreman.rs` com `foreman/planning.rs` e `foreman/search.rs`.
- `zones.rs`, `village.rs`, `spawn.rs`, `ladders.rs`, `storage.rs` (celeiro 27 slots, silo 18).
- Testes: workforce 82 (main) e 109 (pr47) (E1).
- Desempenho: `foreman_95` 747 → 625 µs (E1). `bench_large_village` (150 aldeões + 300 animais): main 3,4 µs/tick; pr47 **em pânico** porque `onboarding::session` chama `world_dir`, que o harness não implementa (E1, `evidencias/logs/bench_interleaved.log`).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-TRB-01 Posto de carregador | 13×13×3; 12 máquinas; lote 16; prioridade da fila | `hauler.rs` · `hauler.rs:12-40` · — | E1 |
| FUN-TRB-02 Mesa do capataz e fila | Raio 12; 6 postos; rede 24; fila 8; marca 200 t | `foreman.rs` · `foreman.rs:18-45` · — | E1 (`foreman_95`) |
| FUN-TRB-03 Planejamento incremental | Fila retomável, sem tocar inventários | `13758bd` · `foreman/planning.rs:1` · — | E2 |
| FUN-TRB-04 Busca incremental | Varredura espacial retomável | `604369d` · `foreman/search.rs:1` · — | E2 |
| FUN-TRB-05 Lavrador e pastor | 9×9 ou zona; raio 6 ou cercado; cap 10 | `farmhand.rs`, `shepherd.rs` · idem · — | E1 |
| FUN-TRB-06 Zonas em níveis | 9/13/17/21; 8/16/32 esm; 32/128/384; 6/12/20 aldeões | `zones.rs:249` (só esmeralda) · `zones.rs:23-27,256-268` (caixa primeiro) · — | E1 (`zones::unlocked`) |
| FUN-TRB-07 Crescimento 30 → 150 | Desenvolvimento ≤ 150; 1/6 a cada 5.999 t | `village.rs` · `village.rs:21-28,121-129` · — | E1 |
| FUN-TRB-08 Nascimento exige cama livre | Cabeceira; raio 32; teto = min(desenvolvimento, camas) | — · `village.rs:30-33,96-108,153-154` · #21 (contido) | E2 (A7) |
| FUN-TRB-09 Spawn no centro da vila | Busca 96; gera vila em 600 t; tag `veio_village` | `spawn.rs` · idem · — | E2 |
| FUN-TRB-10 Cérebro vanilla e descanso | `RESTING 28` a partir de 12.000 | `mod.rs` · `mod.rs:83-90` · — | E3 (0.16.0); nunca visto em jogo |
| FUN-TRB-11 Carga visível nas mãos | Item carregado aparece na mão | 0.16.1 · idem · — | E3 |
| FUN-TRB-12 Vila grande medida | 150 aldeões + 300 animais | `perf` · **Bloqueado** (pânico `world_dir`) · — | E1 |
| FUN-TRB-13 Contratação | 1 esmeralda, só se der certo; raio 8 (16 com Convocação) | `mod.rs:65-66,100` · idem + `projects.rs:470` · — | E2 |
| FUN-TRB-14 Comida por tarefa | 2 pontos; pão = 3 tarefas; modificadores das obras e decretos | `mod.rs:96` · `mod.rs:96`; `projects.rs:416-431` · — | E2 |

Estados: TRB-08 só no pr47; TRB-12 Bloqueado no pr47; as demais **Integrado** nas duas camadas. Ver [05](../05_ESTADO_REAL_DA_BUILD.md).

## E. Histórico

| Versão | Commit | Mudança |
|---|---|---|
| 0.10.0 | `9421883` | Lavrador (frente Cloud) |
| 0.11.0 | `903c4ec` | Carregador e capataz |
| 0.12.0 | `6ac0c62` | Fila do capataz, pastor; Grok lote 3 (comida que não fecha o ciclo, [DEC-025](../06_HISTORICO_E_DECISOES.md#dec-025)) |
| 0.13.0 | `36e43b6` | Zonas, celeiro, silo; vila até 30 |
| 0.14.0 | `a691802` | Teto dinâmico até 150; spawn no centro da vila ([DEC-026](../06_HISTORICO_E_DECISOES.md#dec-026)) |
| #32, #38 | `5d76390`, `5217b62` | Capataz e carregador incrementais (Codex) |
| 0.16.0 | `939c552` | Cérebro vanilla e descanso noturno ([DEC-028](../06_HISTORICO_E_DECISOES.md#dec-028)) |
| 0.16.1 | `7208324` | Carga visível |
| pr47 | #21 (Grok), obras (`f1452f5`) | Cama livre ([DEC-027](../06_HISTORICO_E_DECISOES.md#dec-027)); efeitos das obras e decretos; zona paga pelo caixa |

## F. Pendências

1. Fazer o harness implementar `world_dir`, ou isolar `onboarding::session`, para medir de novo a vila grande no pr47.
2. Decidir o papel dos bônus de teto das obras diante do teto de 150 e do limite da fase ([Q-06](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-06-teto-de-população-e-bônus-das-obras)).
3. Registrar como decisão, ou rever, o pagamento de zonas pelo caixa da companhia, que destrói moeda ([SIS-ECO](SIS-ECO.md) B.4).
4. A preferência por aldeão sem profissão está só em `TRABALHADORES.md`; confirmar no código.
5. Atualizar `TRABALHADORES.md:589`, que fala em "26 estados" (o código tem 44 na tabela global).
6. Remover a função morta `foreman.rs:149 distance` (aviso E1).
7. Teste em jogo: nenhum desde a 0.11.

### Tarefas no backlog

- [TAR-010](../08_BACKLOG_E_PRIORIDADES.md#tar-010): `world_dir` no harness de teste
- [TAR-046](../08_BACKLOG_E_PRIORIDADES.md#tar-046): Teto de população e bônus das obras

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (13 de 14; TRB-08 ausente) · pr47 **Integrado** (13) e **Bloqueado** (TRB-12) · disperso: #21 contido |
| Confiança | Média: E1 nos testes; comportamento de IA nunca visto em jogo |
| Prioridade (sugestão) | Alta: pilar P3 e base de vários sistemas |
| Criticidade (sugestão) | Alta: falha aqui para fábrica, vila e obras |
| Responsável conhecido | Claude nuvem ("Cloud", frente de trabalhadores); Codex (incrementais); Grok (#21, lote 3); agente das obras (efeitos) |
| DEC | [DEC-024](../06_HISTORICO_E_DECISOES.md#dec-024), [DEC-025](../06_HISTORICO_E_DECISOES.md#dec-025), [DEC-026](../06_HISTORICO_E_DECISOES.md#dec-026), [DEC-027](../06_HISTORICO_E_DECISOES.md#dec-027), [DEC-028](../06_HISTORICO_E_DECISOES.md#dec-028) |
| Q | [Q-06](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-06-teto-de-população-e-bônus-das-obras), [Q-12](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-12-peso-da-sociedade-frente-à-fábrica) |
| CNF | [CNF-008](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (bônus das obras × teto) |
| Referências | `TRABALHADORES.md`, `frentes/CLOUD.md`, `frentes/VILA-CIDADE.md`; A2 TRB; A7 (cama livre, zonas) |
