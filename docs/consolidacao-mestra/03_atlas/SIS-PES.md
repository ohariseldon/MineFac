# SIS-PES: Pesquisa

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `pr47@80980db`. O sistema não existe na main.
- **Linhas citadas:** `native/api/src/` no pr47, salvo indicação.
- **Por que é um sistema próprio:** há duas pesquisas no pr47, criadas por agentes diferentes, com o mesmo nome e a mesma chave de mensagem. Separá-las de [SIS-PRG](SIS-PRG.md) e [SIS-OBR](SIS-OBR.md) deixa a sobreposição visível (B7).

## A. Identidade

A pesquisa responde à lacuna "progressão por item" que o dono mandou consertar ([DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017)): antes, 1 chapa de aço liberava a eletrônica inteira. A pesquisa pede **centenas de peças entregues aos poucos**, ou seja, uma linha de produção estável, não um item.

| | Laboratório (FUN-PES-01) | Pesquisa da vila (FUN-PES-02) |
|---|---|---|
| Bloco | `veio:research_lab` | Canteiro do Centro de pesquisa ([SIS-OBR](SIS-OBR.md)), com cientista |
| Criado por | Claude do #40 ([DEC-018](../06_HISTORICO_E_DECISOES.md#dec-018)) | Agente das obras |
| O que pede | 2 insumos industriais em centenas, consumidos a 2/s | 3 materiais em dezenas + 24 tarefas do cientista |
| O que dá | Avanços que liberam as receitas centrais da fábrica | Efeitos sobre os aldeões e a vila |
| Escopo | O mundo inteiro (um progresso só) | A área do centro de controle |
| Onde fica | `<mundo>/veio/pesquisa.txt` | NBT do canteiro e registro `PERKS` |

- **Depende de:** [SIS-PRO](SIS-PRO.md) (aço, engrenagens, circuitos, estruturas, motores, processadores, núcleos), [SIS-ENE](SIS-ENE.md) (24 FE/t), [SIS-TRB](SIS-TRB.md) (cientista) e [SIS-PER](SIS-PER.md).
- **Alimenta:** [SIS-PRG](SIS-PRG.md) (avanços `veio:research/*`), [SIS-TRB](SIS-TRB.md) e [SIS-OBR](SIS-OBR.md) (efeitos).

## B. Funcionamento

### B.1 Laboratório

1. **Liberação.** O laboratório é liberado pelo avanço `engineering` (32 engrenagens ou `permit_workshop`).
2. **Árvore linear.** Quatro pesquisas, em ordem fixa. O estágio é global (`RESEARCH`, `research.rs:22-26`):

| # | Pesquisa | Entradas | Tempo mínimo de consumo | Libera (`research/*.json`) |
|---|---|---|---|---|
| 1 | Metalurgia | 128 aço + 128 engrenagens | 1.280 ticks (64 s) | bateria, geotérmica, lotes de esteira e cabo, poste de aço, vapor, silvicultura, rancho, monitor, túnel |
| 2 | Eletrônica | 128 circuitos + 128 estruturas | 64 s | solar, Mk.II, Tesla, módulos de velocidade e eficiência, lâmpada, hidrelétrica, âncora, estações de carga e descarga |
| 3 | Automação | 64 motores + 64 processadores | 32 s | fabricador, Mk.III |
| 4 | Escala | 32 núcleos de automação + 256 aço | 128 s | esteira expressa |

3. **Consumo.** A cada 10 ticks, o laboratório consome 1 item de cada entrada, ou seja 2 por segundo por entrada. Gasta 24 FE/t (`POWER`, `EVERY`, `research.rs:13-15`). O tempo mínimo é o da entrada maior.
4. **Progresso do mundo.** Vários laboratórios somam na mesma pesquisa. Quebrar um não perde nada (`research.rs:1-6`).
5. **Gravação.** `pesquisa.txt` guarda `estágio entregue0 entregue1`. Grava a cada 600 ticks e ao concluir (`research.rs:150-162`). A troca de mundo é conferida a cada 100 ticks.
6. **Conclusão.** O código roda `advancement grant @a only veio:research/<id>` (`research.rs:166`) e avisa todos os jogadores. Os avanços de pesquisa têm gatilho `impossible`: só o código os concede.

**Pontos abertos:**
- **Quem não estava online** no momento da conclusão não recebe o avanço. Não há reconcessão no login (E2, ND em jogo). [Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan).
- As receitas liberadas dependem da trava de receitas, que pode não funcionar sem `doLimitedCrafting` ([CNF-003](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [SIS-PRG](SIS-PRG.md) B.1).
- Isso é **"entrega acumulada"**, não **"produção sustentada"**: a linha pode parar depois de entregar. A meta sustentada continua Concebida (FUN-PRG-09).

### B.2 Pesquisa da vila

1. **Onde.** Depois que o Centro de pesquisa é construído ([SIS-OBR](SIS-OBR.md)), o canteiro vira o posto do prédio. Um cientista contratado trabalha ali (`staff.rs`).
2. **Custo.** Cada pesquisa pede 3 materiais e, depois de entregues, 24 tarefas do cientista (`RESEARCH_WORK`). As tarefas gastam comida ([SIS-TRB](SIS-TRB.md) B.1).

| Pesquisa (chave) | Bit | Materiais | Efeito no código |
|---|---|---|---|
| Agronomia (`agronomy`) | 1 | 16 farinha de osso, 32 sementes de trigo, 2 composteiras | +1 de crédito de trabalho nas zonas (`zone_credit`, `projects.rs:465-467`) |
| Carga (`cargo`, nome provisório) | 2 | 8 engrenagens, 8 chapas de ferro, 4 baús | +8 itens por viagem do carregador (`haul_batch`, `projects.rs:434-437`) |
| Nutrição (`nutrition`) | 4 | 16 pães, 8 carnes assadas, 8 batatas assadas | −1 ponto de comida por tarefa, se o custo passar de 1 (`food_cost`, `projects.rs:416-419`) |
| Pedagogia (`pedagogy`) | 8 | 16 livros, 16 papéis, 4 lanternas | Dobra o crescimento dos alunos na escola (1.200 → 2.400 ticks por aula, `staff.rs:228-231`) |
| Engenharia (`engineering`) | 16 | 4 circuitos, 2 módulos, 16 redstone | O cientista instala módulos do slot 1 em máquinas a até 12 blocos (`staff.rs:271-277`) |

Fonte: `TECHS`, `staff.rs:44-71`.

3. **Escopo.** Os efeitos valem na área do centro de controle, publicados em `PERKS` (memória), como os das obras.
4. **Mensagem.** Ao concluir, `announce_tech` manda `veio.research.done` com **2 argumentos**: nome e efeito (`staff.rs:388-392`). O laboratório usa a mesma chave com **1 argumento** (`research.rs:167-168`). Uma das duas mensagens sai errada ([CNF-018](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
5. **Traduções.** As 10 chaves `veio.research.{key}.name` e `.effect` não existem no lang. O jogador vê a chave crua.

### B.3 Relação entre as duas

Não há relação no código. A "Engenharia" da vila e a "Eletrônica" do laboratório são independentes. Nenhuma pesquisa da vila libera receita, e nenhuma do laboratório dá efeito na vila. O nome "pesquisa" e a chave de mensagem são as únicas coisas em comum.

## C. Experiência do jogador

1. A pessoa constrói o laboratório depois das 32 engrenagens e liga uma esteira de aço e outra de engrenagens. O painel mostra o que falta de cada entrada e o que a pesquisa libera.
2. Depois de 128 de cada, todos os jogadores recebem a mensagem e as receitas de vapor, bateria e geotérmica.
3. Na vila, depois de construir o Centro de pesquisa, ela entrega pães, carne e batata e espera o cientista. Recebe uma mensagem com chaves cruas e passa a gastar menos comida.

Nunca jogado.

## D. Implementação

**Arquivos (pr47)**
- `research.rs`: laboratório, estado global, `pesquisa.txt`, concessão.
- `content/data/veio/advancement/research/{metallurgy,electronics,automation,scale}.json`.
- `workforce/staff.rs`: pesquisa da vila e cientista. `workforce/projects.rs`: efeitos por bit (`research_bit`).
- Testes: research 2 (E1). Pesquisa da vila sem teste específico.
- Código morto: `staff.rs:124 research_percent` (aviso E1).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-PES-01 Laboratório (ex-PRG-05) | 4 pesquisas lineares; 2/s por entrada; 24 FE/t; global; concede a `@a` | — · `research.rs:1-26,150-168`; `research/*.json` · #40 (contido) | E1 (2 testes) + E2; J nenhum |
| FUN-PES-02 Pesquisa da vila (ex-OBR-07) | 5 pesquisas; 3 materiais + 24 tarefas; efeitos por bit | — · `staff.rs:28-71,228-231,271-277,388-392`; `projects.rs:416-467` · zip das obras (contido) | E2; traduções faltando |

Estados: pr47 **Integrado** nas duas. Ver [05](../05_ESTADO_REAL_DA_BUILD.md).

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 08/11 16:32 | Dono | "Conserte essas lacunas" ([DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017)) |
| 08/11 ~16:40–17:06 | #40 | Laboratório e `research/*.json`; os avanços `steel` e `circuits` perdem receitas para a pesquisa ([DEC-018](../06_HISTORICO_E_DECISOES.md#dec-018)) |
| 08/11, hora ND | `cloud/obras-planta 4a096e9` (nunca enviado) → `f1452f5` | Centro de pesquisa da vila e 5 pesquisas |
| 08/11 19:06 | `eedfd47` | #40 entra no pr47 junto com as obras, sem coordenação entre as duas pesquisas |

## F. Pendências

1. Decidir se o laboratório é a espinha da progressão ([Q-03](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão)). Se for, revisar o que as licenças do Quadro concedem.
2. Dar chaves de mensagem diferentes às duas pesquisas, ou unificar a aridade ([CNF-018](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
3. Criar as 10 traduções `veio.research.*.name/.effect` e fixar o nome definitivo de `cargo`.
4. Reconceder os avanços de pesquisa no login, ou decidir que a tecnologia é por jogador ([Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan)).
5. Gravar `pesquisa.txt` de forma atômica e com versão de formato ([SIS-PER](SIS-PER.md)).
6. Decidir se a pesquisa deve exigir produção sustentada ([Q-10](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-10-fim-de-jogo)).
7. Remover `research_percent`, que nunca é chamada.
8. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-016](../08_BACKLOG_E_PRIORIDADES.md#tar-016): Alinhar o Quadro e a pesquisa ("Libera")
- [TAR-020](../08_BACKLOG_E_PRIORIDADES.md#tar-020): Traduções das obras e da pesquisa da vila
- [TAR-047](../08_BACKLOG_E_PRIORIDADES.md#tar-047): Nomes que colidem

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main — · pr47 **Integrado** (2 de 2) · disperso: #40 e zip das obras contidos |
| Confiança | Média para o laboratório (E1 + E2); baixa para a pesquisa da vila (E2 sem teste, mensagens quebradas) |
| Prioridade (sugestão) | Muito alta para o laboratório (depende da [Q-03](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão)); baixa para a pesquisa da vila |
| Criticidade (sugestão) | Alta: no pr47, as receitas centrais da fábrica só vêm daqui |
| Responsável conhecido | Claude da thread "agricultura" (#40); agente das obras (pesquisa da vila) |
| DEC | [DEC-017](../06_HISTORICO_E_DECISOES.md#dec-017), [DEC-018](../06_HISTORICO_E_DECISOES.md#dec-018), [DEC-033](../06_HISTORICO_E_DECISOES.md#dec-033) |
| Q | [Q-03](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-03-espinha-da-progressão), [Q-10](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-10-fim-de-jogo), [Q-15](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-15-o-que-é-de-cada-jogador-em-lan) |
| CNF | [CNF-002](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (duas espinhas), [CNF-003](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (trava de receitas), [CNF-017](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (concessão a `@a`), [CNF-018](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (duas pesquisas, mesma chave) |
| Referências | `frentes/GRANDE-INDUSTRIA.md`, `HANDOFF_OBRAS.md`; A2 PRG; A7 (pesquisas da vila) |
