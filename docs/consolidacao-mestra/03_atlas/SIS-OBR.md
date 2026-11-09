# SIS-OBR: Obras, prestígio e pedidos da vila

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `pr47@80980db`. O sistema não existe na main. Linhas em `native/api/src/workforce/` no pr47.
- **Origem:** branch `cloud/obras-planta` (`4a096e9`), nunca enviada, porque o agente ficou sem tokens. O dono mandou o zip e pediu para integrar em 17:13 ([DEC-034](../06_HISTORICO_E_DECISOES.md#dec-034)). Entrou como `f1452f5`.

## A. Identidade

O centro de controle decide e constrói melhorias que **mudam a vila de verdade**. Cada obra é um prédio desenhado, erguido bloco a bloco por um mestre de obras, e o efeito só vale com o prédio de pé e funcionando ([DEC-033](../06_HISTORICO_E_DECISOES.md#dec-033); `projects.rs:1-60`).

- **Depende de:** [SIS-CTL](SIS-CTL.md) (slots Obra e Decreto, raio 64), [SIS-TRB](SIS-TRB.md) (mestre de obras e funcionários), [SIS-ECO](SIS-ECO.md) (caixa e `reward`), [SIS-VIL](SIS-VIL.md) (prefeitura, lojas).
- **Alimenta:** [SIS-TRB](SIS-TRB.md) (comida, lote, contratação, crescimento), [SIS-VIL](SIS-VIL.md) (teto), [SIS-PES](SIS-PES.md) (centro de pesquisa).

## B. Funcionamento

### B.1 Pagar e construir

1. **Pagamento.** O jogador põe os materiais no slot **Obra** (6) do centro. Com o administrador contratado, cada material vai primeiro para os pedidos do dia e depois para a primeira obra liberada que ainda precisa dele. A obra fica paga com os três materiais completos (`projects.rs:4-8`).
2. **Esmeraldas que sobram** no slot Obra vão para o caixa, que é o livro Company (`center.rs:256-264`). As obras pagam esmeraldas **destruindo moeda** do caixa ([SIS-ECO](SIS-ECO.md) B.4).
3. **Canteiro** (`site.rs:1-19`):
   1. o jogador põe o canteiro onde quer o prédio; a borda da planta aparece no chão e a chave troca a planta;
   2. o canteiro confere o terreno, 48 blocos por ciclo (`SURVEY`): terra, pedra, grama, árvores e água saem; baús, camas, portas e máquinas param a obra;
   3. um mestre de obras contratado assenta 10 blocos por vez (`BATCH`), de baixo para cima; enfeites e móveis por último;
   4. pronto, o mestre é dispensado e o efeito passa a valer.
4. **O canteiro é o coração da obra.** Quebrá-lo desfaz o efeito.
5. **Status** 36 planta, 37 bloqueado, 38 construindo, 39 pronto (`site.rs:42-45`).

### B.2 As obras

| Obra (bit) | Prestígio mínimo | Materiais | Efeito | Funcionário |
|---|---|---|---|---|
| Refeitório (1) | Aldeia | 24 tijolos, 8 chapas de ferro, 1 cozinha | Toda 4ª tarefa dos postos não gasta comida | Cozinheira |
| Escola (2) | Aldeia | 24 papéis, 6 livros, 16 tijolos | Natalidade 1/3 e teto +15 | Professor |
| Ferraria (4) | Aldeia | 8 chapas de aço, 8 engrenagens, 1 bigorna | Carregadores levam 24 itens | Ferreiro (2 chapas de ferro → 2 kits de reparo) |
| Praça do mercado (8) | Aldeia | 16 vidros, 16 chapas de cobre, 8 esmeraldas | Expandir zonas custa 25% menos | Lojas da sociedade nos lugares reservados |
| Festa da colheita (16) | Aldeia | 1 bolo, 16 biscoitos, 16 pães | Repetível: hoje e amanhã, tarefas sem comida e natalidade 1/2 | — |
| Biblioteca (32) | Vila | 8 estantes, 16 livros, 4 lanternas | Zonas ganham o dobro de trabalho por tarefa | Bibliotecário (3 papéis → 1 livro) |
| Correio (64) | Vila | 16 papéis, 4 baús, 8 chapas de cobre | 4 pedidos por dia e +50% de esmeraldas | Carteiro |
| Estalagem (128) | Vila próspera | 8 camas brancas, 32 tábuas, 4 chapas de aço | Teto +20; um viajante chega toda manhã se houver lugar | Estalajadeiro |
| Prefeitura (256) | Cidade operária | 4 blocos de ouro, 1 sino, 8 circuitos | Prestígio +50%; o caixa rende 1/16 até 8 por dia | Prefeito da sociedade |
| Centro de pesquisa (512) | Vila | 4 circuitos, 16 vidros, 8 chapas de aço | Pesquisa da vila ([SIS-PES](SIS-PES.md)) | Cientista |

Fonte: tabela em `projects.rs:9-24`; bits `:42-76`; catálogo `:98-140`; funcionários em `staff.rs:6-14`.

- Os funcionários andam entre os móveis da planta. No horário da planta, aldeões livres visitam os prédios: almoço, aula, leitura, conselho, compras, jantar (`staff.rs:20-23`).
- Os rendimentos da Prefeitura são pagos pelo livro Public via `reward` (`projects.rs:475-476`; [DEC-035](../06_HISTORICO_E_DECISOES.md#dec-035)).
- A obra Prefeitura tem o mesmo nome do bloco do prefeito ([CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

### B.3 Prestígio

- Pontos no centro vindos de pedidos (+10), obras (+25), festas (+10), banquetes no Lar (+5), dias em ordem no Lar (+2) e manhãs sem pendências (+3) (`projects.rs:29-33`).
- Níveis: Aldeia (0), Vila (60), Vila próspera (200), Cidade operária (500) (`TIERS`).
- Cada nível soma 5 ao teto da vila e libera obras e pedidos mais ricos.
- "Vila" e "Cidade" também são fases da prefeitura ([CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem)). O bônus de teto pode não ter efeito por causa do teto de 150 ([CNF-008](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-06](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-06-teto-de-população-e-bônus-das-obras)).

### B.4 Pedidos da vila

- A cada manhã, 3 pedidos abertos, 4 com o Correio, de uma lista de 27 itens (`requests.rs:1-13,61-64`).
- O jogador entrega pelo slot Obra. Nada entregue se perde. Um pedido parado 3 dias sem entrega é trocado.
- Cada pedido atendido paga esmeraldas ao caixa e 10 de prestígio.
- **Quem paga:** o caixa público, via `reward` (Public → Company). Sem prefeitura ou sem saldo, só prestígio ([DEC-035](../06_HISTORICO_E_DECISOES.md#dec-035), decisão do dono das 17:18, contra a recomendação do agente).
- Coexiste com o pedido do dia do Quadro, que cria esmeraldas ([CNF-004](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-02](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-02-dois-pedidos-diários)).

### B.5 Decretos

Um item no slot **Decreto** (7) muda a regra da área sem ser gasto (`projects.rs:35-40`):

| Item | Decreto | Efeito |
|---|---|---|
| Sino | Convocação | Postos contratam a até 16 blocos (em vez de 8) |
| Papel | Racionamento | Tarefas custam 1 ponto de comida; ninguém nasce |
| Fardo de feno | Fartura | Tarefas custam 3; zonas ganham o dobro de trabalho; natalidade 1/4 |

### B.6 Boletim e efeitos

- O centro manda um boletim da manhã (`center.rs:15`).
- Os efeitos são publicados num registro em memória (`PERKS`) que os postos consultam uma vez por ciclo, sem varrer blocos (`projects.rs:42-44`).

## C. Experiência do jogador

1. A pessoa põe 24 tijolos, 8 chapas e uma cozinha no centro e escolhe onde fica o Refeitório.
2. Contrata um mestre de obras e o vê erguer o prédio.
3. Contrata uma cozinheira. Os aldeões almoçam lá e os postos gastam menos comida.
4. Atende pedidos da manhã e ganha prestígio.
5. Põe um sino no slot Decreto para contratar de mais longe.

Nunca jogado.

## D. Implementação

**Arquivos:** `projects.rs`, `site.rs`, `staff.rs`, `requests.rs`, `blueprints.rs`, `blueprint_data.rs` (9 plantas, 7.542 blocos), `center.rs`. Menu `content/data/veio/horizon/menu/site.json`.

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-OBR-01 Canteiro | Planta, terreno, mestre, 10 blocos por vez; status 36–39 | — · `site.rs:26-45` · zip das obras (contido) | E2; **menu com 1 slot × código com 3** (`site.json` × `mod.rs:150`) |
| FUN-OBR-02 10 obras com efeitos | Bits 1…512; efeito só de pé | — · `projects.rs:42-140,416-476` · idem | E2 |
| FUN-OBR-03 Prestígio | 0/60/200/500; +5 de teto por nível | — · `projects.rs:29-33,75` · idem | E2 |
| FUN-OBR-04 Pedidos da vila | 27 itens; 3 ou 4; 3 dias; +10; pagos via `reward` | — · `requests.rs:61-64`; `center.rs:609-620` · idem | E2 |
| FUN-OBR-05 Decretos | Sino, papel, feno | — · `projects.rs:35-40,70` · idem | E2 |
| FUN-OBR-06 9 plantas prontas | 7.542 blocos | — · `blueprint_data.rs` · idem | E1 |
| FUN-OBR-08 Boletim da manhã | — | — · `center.rs:15` · idem | E2 |
| FUN-OBR-09 Ruínas | — | — · — · Concebido (`HANDOFF_OBRAS.md`; `ruins.rs` inexistente) | E3 |
| FUN-OBR-10 Proteção das obras | Ganchos `destroy_block`, `left_click_block`, `use_item_on`, `explosion_detonate` | — · — · Concebido | E3 |
| FUN-OBR-11 Funcionários e visitantes | 7 papéis; horário da planta | — · `staff.rs:1-40` · idem | E2 |

FUN-OBR-07 (pesquisa da vila) passou a FUN-PES-02.

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 08/11, hora ND | `cloud/obras-planta 4a096e9` | 7ª a 9ª entregas: plantas, canteiro, prestígio, pedidos, decretos, staff, pesquisa ([DEC-033](../06_HISTORICO_E_DECISOES.md#dec-033)) |
| 08/11 | `HANDOFF_OBRAS.md` | Canteiro renumerado 32–35 → 33–36 → 36–39; centro de 5 para 8 slots ([DEC-053](../06_HISTORICO_E_DECISOES.md#dec-053)) |
| 08/11 17:13 | Dono | Integrar a build parcial das obras ([DEC-034](../06_HISTORICO_E_DECISOES.md#dec-034)) |
| 08/11 17:18 | Dono | Recompensas pagas pelo caixa público ([DEC-035](../06_HISTORICO_E_DECISOES.md#dec-035)) |
| 08/11 17:51 | Dono | Obras, sons e caravanas no mesmo PR ([DEC-047](../06_HISTORICO_E_DECISOES.md#dec-047)) |

## F. Pendências

1. Corrigir o menu do canteiro para 3 slots ([R-30](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
2. Criar as traduções que faltam: `veio.site.no_staff` (`mod.rs:917`), `veio.center.project.idle` (`center.rs:690`), `veio.project.lab.name/.effect`, `veio.site.next.<papel>` (9) e as 10 de pesquisa. São cerca de 25 chaves.
3. Implementar ou descartar as ruínas e a proteção das obras (pendências do handoff).
4. Decidir se os serviços das obras entram no IDH ([SIS-VIL](SIS-VIL.md) B.4).
5. Resolver nomes: obra Prefeitura × bloco; escadas de prestígio × fases ([Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem)).
6. Remover `site.rs:142 sites_near`, nunca chamada.
7. Teste em jogo: nenhum.

**Depois do corte:** o PR #53 (`claude/project-thread-6fbc15@5c9ffdc`, 20:56) diz cobrir os itens 1 a 3: menu com 3 slots, traduções, ruínas a 50–62 blocos e proteção do território. Também diz corrigir os rótulos de obra e decreto no centro. Nada foi auditado ([06, DEC-064](../06_HISTORICO_E_DECISOES.md#dec-064)).

O PR #54 (`2c49d41`) também diz acrescentar "obras cívicas" à agenda da vila, sem auditoria ([TAR-065](../08_BACKLOG_E_PRIORIDADES.md#tar-065), [CNF-022](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

### Tarefas no backlog

- [TAR-006](../08_BACKLOG_E_PRIORIDADES.md#tar-006): Menu do canteiro com 3 slots
- [TAR-017](../08_BACKLOG_E_PRIORIDADES.md#tar-017): Serviços das obras no IDH
- [TAR-020](../08_BACKLOG_E_PRIORIDADES.md#tar-020): Traduções das obras e da pesquisa da vila
- [TAR-021](../08_BACKLOG_E_PRIORIDADES.md#tar-021): Ruínas
- [TAR-022](../08_BACKLOG_E_PRIORIDADES.md#tar-022): Proteção de território das obras
- [TAR-028](../08_BACKLOG_E_PRIORIDADES.md#tar-028): Pedido do dia persistente ou fundido
- [TAR-046](../08_BACKLOG_E_PRIORIDADES.md#tar-046): Teto de população e bônus das obras
- [TAR-047](../08_BACKLOG_E_PRIORIDADES.md#tar-047): Nomes que colidem

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main — · pr47 **Integrado** (01–06, 08, 11), **Concebido** (09, 10) |
| Confiança | Média: E2 completo; menu e traduções quebrados; J nenhum |
| Prioridade (sugestão) | Média: corrigir menu e traduções antes do merge |
| Criticidade (sugestão) | Média: o canteiro com 1 slot pode impedir a obra em jogo (ND) |
| Responsável conhecido | Claude nuvem, agente das obras (sem tokens); integrador do #47 |
| DEC | [DEC-033](../06_HISTORICO_E_DECISOES.md#dec-033), [DEC-034](../06_HISTORICO_E_DECISOES.md#dec-034), [DEC-035](../06_HISTORICO_E_DECISOES.md#dec-035), [DEC-047](../06_HISTORICO_E_DECISOES.md#dec-047), [DEC-053](../06_HISTORICO_E_DECISOES.md#dec-053) |
| Q | [Q-02](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-02-dois-pedidos-diários), [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem), [Q-06](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-06-teto-de-população-e-bônus-das-obras), [Q-12](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-12-peso-da-sociedade-frente-à-fábrica) |
| CNF | [CNF-004](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (dois pedidos), [CNF-008](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (teto), [CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (nomes) |
| Referências | `HANDOFF_OBRAS.md`, `TRABALHADORES.md` (7ª–9ª entregas); A2 OBR; A7 (caixa, staff) |
