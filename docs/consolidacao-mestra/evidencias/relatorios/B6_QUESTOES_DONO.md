# B6 — Questões que só o dono (Luiz) pode decidir

Autor: OPUS B. Data: 2026-11-08.

Estas questões **não são decisões**. Para cada uma apresento:
- o contexto, com fonte;
- as alternativas e suas consequências;
- a minha recomendação justificada.

Nada aqui deve ser registrado como DEC aprovada antes da resposta do dono. As referências DEC/FUN/SIS estão em B2 e B3.

## Prioridade

Revista na Fase III, depois da auditoria do OPUS A (B7).

| Ordem | Questão | O que bloqueia | Urgência |
|---|---|---|---|
| 1 | Q11: condições para fundir o #47 (compilação, desempenho, teste) e o que fazer com #51/#52 | Tudo: o #47 é a candidata | Alta |
| 2 | Q3: espinha da progressão. As licenças do Quadro não liberam o que prometem; o gate por avanço pode ser só visual | O jogo de fábrica em si | Alta |
| 3 | Q8: número da versão (o #51 já ocupa a 0.26.0) | Pacote, launcher, saves | Alta |
| 4 | Q1: esmeraldas criadas fora da economia fechada | Balanço, comércio, progressão | Alta |
| 5 | Q5: produção virtual sem custo | Coerência com a decisão de 17:01 e com a economia | Alta |
| 6 | Q13: imposto uma ou duas vezes | Balanço da 0.25 | Média |
| 7 | Q2: dois pedidos diários | Q1 e Q3 | Média |
| 8 | Q9: mapa finito (tamanho e garantias) | Thread em andamento | Média |
| 9 | Q7: realismo, clima e estações do ano (#51) | Thread em andamento | Média |
| 10 | Q15: o que é de cada jogador e o que é do mundo (LAN) | Multijogador | Média |
| 11 | Q14: produtividade 10 % ou 20 % | Balanço | Baixa |
| 12 | Q6: teto de população e bônus das obras | Balanço da vila | Baixa |
| 13 | Q4: nomes das escadas de nível | Clareza | Baixa |
| 14 | Q10: fim de jogo | Direção de conteúdo | Estratégica |
| 15 | Q12: peso da sociedade frente à fábrica | Direção de produto | Estratégica |

---|---|---|---|
| 1 | Q11: o que fazer com o PR #47 e os PRs abertos | Tudo: o #47 é a candidata a build | Alta |
| 2 | Q8: número da versão consolidada e quem sobe versão | Pacote, launcher, saves | Alta |
| 3 | Q1: esmeraldas criadas fora da economia fechada | Balanço, comércio, progressão | Alta |
| 4 | Q3: qual é a espinha da progressão | Tutorial, guia, balanço | Alta |
| 5 | Q2: dois pedidos diários | Q1 e Q3 | Média |
| 6 | Q9: mapa finito (tamanho e garantias) | Thread "limite do mapa" em andamento | Média (há trabalho correndo) |
| 7 | Q7: realismo opcional ou padrão | Thread "realismo" em andamento | Média (há trabalho correndo) |
| 8 | Q5: transporte longo e produção virtual | Coerência com DEC-041 | Média |
| 9 | Q6: teto de população e bônus das obras | Balanço da vila | Baixa |
| 10 | Q4: nomes das escadas de nível | Clareza da interface | Baixa |
| 11 | Q10: fim de jogo | Direção de conteúdo | Baixa (estratégica) |
| 12 | Q12: peso da sociedade frente à fábrica | Direção de produto | Estratégica |

---

## Q1. Esmeraldas criadas fora da economia fechada

**Contexto**

A 0.21 declarou que "o dinheiro só circula" entre companhia, famílias, lojas e prefeitura, e que a moeda só entra ou sai por esmeraldas físicas (DEC-030, decisão de agente). Às 17:18 o dono decidiu que os pedidos da vila e os juros da obra Prefeitura saem do caixa público (DEC-035), contra a recomendação do agente de usar uma "entrada externa".

Apesar disso, várias fontes **criam esmeraldas** sem passar por nenhum livro:

| Fonte | Valor | Evidência |
|---|---|---|
| Pedido diário do Quadro | 1–6 esmeraldas por dia | `milestones/mod.rs:146-151` @pr47 (E2) |
| Exportação contínua da central | 4 por remessa, sem fim | E3 |
| Contrato Povoado | 8 | E3 |
| Armazém do povoado | 1 a cada 60 pontos de comida, +50% com 3 aldeões a 48 blocos | E3 |
| Grande projeto | 32, 48 e 64 por fase | E3 |
| Extratora Mk.II sobre jazida de esmeralda | 1 a cada 20 s | E3 |
| Trocas vanilla | — | — |

Algumas dessas fontes, como o armazém do povoado (até 64 por pilha de pão) e a exportação contínua, competem diretamente com a bolsa mundial e com as lojas. Na prática, o balanço da 0.25 (simulador com E1) não as considera.

**Alternativas**

| | Alternativa | Consequência |
|---|---|---|
| A | **Manter tudo como está** | O jogador tem atalhos que ignoram o comércio exterior e as lojas. O simulador e o balanço da 0.25 deixam de valer na prática. Zero trabalho |
| B | **Registrar todas como "entrada externa"** num livro "Exterior", visível no relatório, com teto diário | A identidade contábil continua correta. O jogador vê de onde veio o dinheiro. É preciso revisar os valores (ex.: a central pagar pelo preço da bolsa) |
| C | **Converter tudo ao circuito**: central e armazém viram vendas à bolsa ou à loja; quadro e grande projeto dão itens ou licenças, não esmeraldas; a Mk.II minerando esmeralda fica como única fonte "mineral" | Mantém a economia fechada pura. Mexe em sistemas de três agentes e muda o início de jogo, porque hoje a primeira esmeralda vem do armazém ou da central |
| D | **Híbrido**: B para as fontes de início de jogo (armazém do povoado, contrato Povoado) e C para as de meio e fim (quadro, central contínua, grande projeto) | O começo continua com uma porta de entrada de moeda. O meio e o fim passam pelo comércio exterior, que a ECONOMIA.md diz ser "o que sustenta uma indústria maior" |

**Atualização B7.** O OPUS A confirmou no código (E2):
- o EXPORT é ilimitado **também na main** (`contracts.rs:63-66`);
- o contrato village paga 8 esmeraldas;
- o grande projeto paga 32, 48 e 64;
- o pedido do dia do Quadro paga 1–6 esmeraldas (`milestones/mod.rs:147-150`).

Nenhuma dessas fontes aparece em `BORDERS` nem no `sim_economy.py`, de modo que o "ok" do simulador não cobre o jogo real.

**Recomendação: D.**
1. A escolha do dono às 17:18 (pagar pela prefeitura e não por "entrada externa") indica preferência pelo circuito fechado.
2. Mesmo assim, o início de jogo precisa de uma fonte de moeda antes de existir porto ou estação: a contratação custa 1 esmeralda e as zonas custam 8 a 32.
3. O balanço da 0.25 só vale se as fontes de meio de jogo passarem pela bolsa.

---

## Q2. Dois "pedidos diários"

**Contexto.** Há dois sistemas com o mesmo nome:
- **Pedido do Quadro da fábrica** (Codex/Claude MineFac): 16–64 itens, paga 1–6 esmeraldas criadas, uma vez por dia por dimensão. Fica **em memória**, então reabrir o mundo permite receber de novo (E3/E2).
- **Pedidos da vila** (obras): 3 por dia (4 com Correio), pagos pelo caixa público, rendem prestígio (E3/E2).

**Alternativas**

| | Alternativa | Consequência |
|---|---|---|
| A | Manter os dois e renomear (ex.: "Encomenda da fábrica" e "Pedido da vila") | Menos confusão; continuam duas recompensas diárias e a emissão da Q1 |
| B | **Fundir no pedido da vila**: o Quadro só mostra os pedidos da vila e o diagnóstico | Uma fonte só, já ligada à decisão de 17:18; o Quadro perde a recompensa própria |
| C | Remover o pedido do Quadro | Simples; perde o objetivo diário do início (antes de existir vila com prefeitura) |

**Recomendação: B, com persistência em disco.** O pedido da vila já segue a regra do dono (DEC-035). A duplicação cria duas regras de pagamento para a mesma ação. O problema de "receber de novo ao reabrir" é um defeito que a fusão resolve.

---

## Q3. Qual é a espinha da progressão

**Contexto.** No pr47 convivem sete mecanismos (B2 SIS-PRG):
1. avanços que liberam receitas;
2. fases e licenças do Quadro (32 engrenagens / 16 aço / 16 circuitos / 32 circuitos);
3. laboratório de pesquisa (#40: metallurgy, electronics, automation, scale);
4. central de contratos;
5. grande projeto;
6. fases da prefeitura;
7. prestígio, reputação e níveis de zona.

Os mecanismos 2 e 3 foram feitos ao mesmo tempo por agentes diferentes, ambos para corrigir a "progressão por um item", e o #40 tirou liberações dos avanços de aço e circuitos para a pesquisa. O dono aprovou as lacunas (16:32) e escolheu os tempos da 0.25 porque "a grande indústria depende deles" (19:16), mas não decidiu entre os mecanismos 2 e 3. O README e PROGRESSAO-FASES ainda descrevem as fases do Codex como espinha.

**Alternativas**

| | Alternativa | Consequência |
|---|---|---|
| A | **Laboratório como espinha tecnológica**: as fases do Quadro passam a ser marcos que **abrem** os níveis de pesquisa, e as licenças deixam de existir como item | Um caminho só ("produza X para abrir a pesquisa Y"). Exige refazer textos, tutorial e testes do Quadro |
| B | **Fases do Quadro como espinha**; o laboratório vira ramo opcional (bônus e itens tardios) | Devolve as liberações aos avanços. Desfaz parte do #40 e da cadeia funda |
| C | Manter os dois como estão | Zero trabalho. O jogador tem duas portas para coisas parecidas, e os docs ficam inconsistentes |

**Atualização B7** (A6 R-20 e R-21).
- **Defeito visível hoje:** no pr47, o Quadro anuncia que as licenças liberam vapor, bateria, geotermia, Mk.II, solar, Tesla e Mk.III. Isso vem do laboratório (`research/*.json`). O jogador recebe a licença e nada é liberado.
- **Pré-condição:** não existe `doLimitedCrafting` em nenhum ramo. Se o Horizon se comportar como o vanilla, "liberar receita" só tira a receita do livro, e a fabricação continua possível. Qualquer alternativa abaixo só funciona se o gate for real. Recomendo um teste no Horizon e, se confirmado, ligar a regra nos mundos do Veio. Efeito colateral: as receitas vanilla também passam a exigir desbloqueio, que no vanilla acontece ao obter o item.

**Recomendação: A.**
- É coerente com as duas decisões do dono (aprovar as lacunas e manter os tempos da grande indústria).
- Preserva o diagnóstico do Quadro, que é o melhor "HUB" que o jogo tem.
- Elimina a duplicação de gates.

As escadas da vila (6 e 7) devem continuar paralelas, mas visíveis numa tela só (ver Q4).

---

## Q4. Nomes das escadas de nível e a "Prefeitura" duplicada

**Contexto.** Hoje existem várias escadas com nomes que se cruzam:
- fases da prefeitura: Povoado / **Vila** / **Cidade** / Metrópole;
- prestígio das obras: Aldeia / **Vila** / Vila próspera / **Cidade** operária;
- fases do Quadro (1–5);
- reputação (1/3/6);
- níveis de zona.

Além disso, "Prefeitura" é ao mesmo tempo o bloco do prefeito e uma obra (`HALL`: prestígio +50%, juros).

**Alternativas**
- **A.** Renomear o prestígio (ex.: "Fama" com estrelas 0–3) e a obra (ex.: "Paço Municipal").
- **B.** Fundir prestígio e fases da prefeitura numa escada só.
- **C.** Manter.

**Recomendação: A.** É a de menor custo. A fusão (B) mexe em regras de dois sistemas que foram balanceados separadamente.

---

## Q5. Produção virtual e transporte longo

**Contexto.** O dono pediu (17:01) que a fábrica distante "continue produzindo e entregando sem carregar tudo". A auditoria do OPUS A (E2, `stations.rs:117-140`) mostra três coisas:
1. A estação descarregada **cria itens no ritmo medido, sem consumir insumos nem energia**, e continua mesmo se o posto foi desmontado enquanto estava descarregado.
2. Os canais são **globais entre dimensões**.
3. Trem, drones e caravanas (até 512 blocos) param fora da área simulada.

**Alternativas**

| | Alternativa | Consequência |
|---|---|---|
| A | **Produção virtual com custo:** o ritmo virtual só sai se houver insumo virtual (canal de entrada) e energia (gasto contabilizado); a produção é invalidada ao quebrar o posto | Fiel ao "continua produzindo". Mantém a economia e o balanço. Mais código e mais casos de borda |
| B | **Sem custo, com limites:** teto por canal, decaimento depois de N dias fora e invalidação ao quebrar | Simples e leve. A mina distante continua sendo quase uma fonte grátis |
| C | Manter como está | Exploração óbvia: montar, medir, desmontar e receber para sempre |

As caravanas e os drones ficam como estão (só na área simulada), documentados no guia de Transportes. A chave (dimensão, canal) é correção técnica do integrador, não decisão de produto.

**Recomendação: A, com B como passo intermediário se A atrasar o #47.**
- As palavras do dono descrevem uma fábrica que continua funcionando, não uma torneira de itens.
- A economia fechada (DEC-030) e o balanço da 0.25 deixam de valer com uma fonte grátis.

---|---|---|
| A | Integrar os terminais às estações: rota longa vira canal virtual com tempo de viagem | Comportamento uniforme. Trabalho em código de dois agentes |
| B | Manter e documentar: trem, drone e caravana servem para distâncias dentro da área simulada; distância longa é com estação | Zero código. Exige ajustar o guia de Transportes |
| C | Remover as caravanas de longa distância | Simplifica. Perde conteúdo pronto |

**Recomendação: B agora e A depois do primeiro teste em jogo.** As estações já cumprem a decisão do dono; o risco de mexer antes de testar é maior que o ganho.

---

## Q6. Teto de população e bônus das obras

**Contexto.** A população máxima é `min(teto dinâmico + extra das obras, 150)` (`village.rs:121`), e o nascimento também respeita o limite da fase (30/60/100/150) e a vaga em casa. Escola (+15), Estalagem (+20) e prestígio (+5 por nível) somam no teto dinâmico. Numa vila madura, esses bônus podem não ter efeito nenhum.

**Alternativas**
- **A.** Os bônus das obras furam o limite da fase até 150.
- **B.** Os bônus elevam o teto absoluto acima de 150 (ex.: até 200), com impacto em desempenho.
- **C.** Trocar o bônus de teto por outro efeito (natalidade, felicidade, IDH).

**Recomendação: C.** O 150 é limite de desempenho medido (o censo de 0.14/0.16). Os efeitos de serviço combinam com o IDH, cuja integração com as obras ficou pendente (SOCIEDADE).

---

## Q7. Realismo: opcional ou padrão, e até onde

**Contexto.** O visual base é o "estilo Minecraft" (0.7). O "Realismo leve 1.1" é opcional (shader MIT, perfis Realismo e Leve). Às 20:09 o dono pediu "realismo maior sem pesar o PC" num PC de 8 GB; uma thread está trabalhando nisso agora.

**Alternativas**
- **A.** Realismo continua opcional, com o visual vanilla como padrão.
- **B.** Realismo vira padrão, com fallback Leve.
- **C.** Dois produtos visuais mantidos em paralelo.

**Atualização B7.** A thread de realismo abriu o **PR #51** (20:34Z, base pr47): Realismo 2.1, **clima e estações do ano**, 9 shaders, versão 0.26.0. O código não foi auditado. O clima e as estações do ano podem afetar o jogo (por exemplo, a lavoura), e ninguém decidiu isso. A palavra "estações" colide com as estações de carga.

Pergunta adicional: o clima e as estações do ano devem ter efeito de jogo, ou só visual? Recomendação: **só visual na primeira entrega**, porque o pedido do dono foi "realismo sem pesar o PC" e não uma mecânica nova. Nome no jogo: "estações do ano".

**Recomendação: A, com critério de aceite numérico.** Por exemplo: no perfil Realismo, no PC de 8 GB, medir FPS e memória numa fábrica de referência antes de promover a padrão. As luzes de estado e as silhuetas não podem perder legibilidade. Hoje não há nenhuma medida no Horizon real.

---

## Q8. Número da versão consolidada e quem sobe versão

**Contexto.**
- A `main` e o pr47 dizem **0.25.0** com conteúdos diferentes, e há dois `release/Veio-0.25.0.zip` diferentes.
- O launcher do pr47 é 0.16.1, e `Veio-Realismo-1.1.zip` contém a 0.16.1.
- Já houve duas 0.16.0, e o #40 foi renumerado cinco vezes.
- **Novo:** o PR #51 (20:34Z, base pr47) já se declara **0.26.0** e traz `release/Veio-0.26.0.zip`.
- A COORDENACAO diz "não subir versão nas frentes"; o CONTRIBUTING diz "subir a cada entrega que mude `native/`".

**Alternativas**

| | Alternativa | Consequência |
|---|---|---|
| A | **Só o integrador sobe versão.** O #47 sai como **0.26.0**. O #51 perde o bump e o zip e recebe o número que o integrador der ao fundi-lo (0.27.0) | Fim das colisões. Exige retrabalho pequeno no #51 |
| B | Aceitar o 0.26.0 do #51: o #47 sai como 0.25.1 ou 0.26.0-rc e o #51 vira a 0.26.0 | Premia quem empacotou primeiro. A 0.25.1 mente sobre a escala da mudança (79 commits, `native/` alterado) |
| C | Fundir #47 + #51 + #52 numa 0.26.0 única | Empilha 53 arquivos não auditados numa candidata nunca compilada nem jogada (A6 R-42) |
| D | Cada frente sobe a versão (regra atual do CONTRIBUTING) | Mantém as colisões |

**Recomendação: A.**
- É a única que impede a próxima colisão.
- Segue o mesmo princípio de B4 §2.4: o número pertence a quem integra.
- Mantém o #47 congelado, avaliável e com a versão alinhada entre o launcher e o conteúdo.

---|---|---|
| A | **Só o integrador sobe versão**, na fusão com a main. As frentes usam o nome da branch | Fim das colisões. As frentes perdem o "pacote próprio" para teste |
| B | Cada frente sobe a versão (regra atual do CONTRIBUTING) | Mantém as colisões |
| C | Faixas por frente (ex.: 0.26.x obras, 0.27.x indústria) | Reduz as colisões. Complica a leitura |

**Recomendação: A, com o #47 publicado como 0.26.0** (nunca 0.25.0) e o launcher alinhado. Ver B4 e B5.

---

## Q9. Mapa finito: tamanho e garantias

**Contexto.** O MAPA.md registra como "decisão de produto" um Overworld de 15.000², com recursos garantidos por geração adaptada e Nether de 8:1, "autorizada pelo usuário" (E3). Não há registro dessa decisão no chat de 08/11. Às 20:09 o dono pediu "limite do mapa e otimizar". O rascunho do Codex está desligado, com a auditoria do Nether falhando. Uma thread está trabalhando nisso agora.

**Alternativas**
- **A.** Contrato completo: 15.000², muralha e todos os recursos garantidos.
- **B.** Só o limite: borda de 15.000 com muralha e Nether ajustado, **sem** garantia de recursos, porque o Horizon não tem gancho de worldgen.
- **C.** Outro tamanho (menor, para PCs de 8 GB).

**Recomendação: B como primeira entrega, e confirmar o número.**
- A garantia de recursos depende de API que o Horizon `66f3981` não expõe (`native_chunk_generate`).
- O limite sozinho já atende o "limite do mapa" pedido às 20:09.

---

## Q10. Fim de jogo

**Contexto.** Não há condição de fim. O grande projeto em 4 fases termina com 2 estrelas do Nether e 2 faróis. A "meta de produção sustentada" (20 circuitos/min por 3 min) está no README como planejamento (Concebido), e o doc de lacunas aprovado listava "falta meta final".

**Alternativas**
- **A.** O grande projeto é o fim e ganha uma cena e crédito final.
- **B.** Metas sustentadas e contínuas (estilo Space Elevator, Satisfactory).
- **C.** Sandbox sem fim, com a Metrópole como topo.

**Recomendação: B, apoiado em A.** As metas sustentadas exercitam exatamente a produção estável que o Veio ensina e cobrem a lacuna "entrega acumulada × sustentada" do doc de lacunas.

---

## Q11. O PR #47 e os PRs abertos

**Contexto.**
- O #47 tem 79 commits à frente da main e reúne obras, sons, formas, caravanas/drones/ferrovias, grande indústria/água/rios, a consolidação Codex, os PRs do Grok e o rascunho do mapa.
- Os PRs #50, #49, #31, #30, #28, #25, #23, #21, #20, #19, #7 e #40 continuam abertos, embora (segundo a thread de integração) o conteúdo de código esteja no #47.
- A main recebeu merges das contas "jefterpereira" e "bizi"; não sei qual corresponde ao dono (ND).
- Nada foi testado em partida desde a 0.11. A compilação nativa do pr47 não foi feita.

**Alternativas**

| | Alternativa | Consequência |
|---|---|---|
| A | Fundir o #47 na main agora | A main passa a ter tudo. O risco de regressão no jogo é desconhecido e os docs estão desatualizados em 4 pontos (B3 §3.10k) |
| B | **Compilar o pr47 pelo launcher e jogar uma sessão curta** (roteiro de 30 min do tutorial ao porto), corrigir os docs, publicar como 0.26.0 e só então fundir. Fechar os PRs absorvidos com referência ao #47 | Atraso curto, risco muito menor. Exige o dono ou alguém com Windows e Horizon |
| C | Manter o #47 aberto como "integração contínua" e não fundir | A main continua 0.25.0 sem casa, tutorial na tela nem água. Dois produtos divergem |

**Atualização B7** (auditoria do OPUS A, E1/E2):
- **Desempenho:** o pr47 está cerca de 40 % mais lento por tick na fábrica de referência, desenha um quadro com 329 alocações (zero na main), e o benchmark da vila grande entra em pânico. Isso contraria a prioridade de 20:09 (leveza num PC de 8 GB).
- **Compilação:** o pr47 nunca foi compilado contra o Horizon real, e a dependência do Horizon não tem revisão fixa.
- **Pacote:** o README manda instalar um zip 0.16.1.
- **Alvo móvel:** os PRs #51 e #52 foram abertos sobre o pr47 às 20:34–20:35Z.

Isso torna a alternativa B mais exigente. Antes do merge, B passa a incluir:
1. `native-build` com o Horizon numa revisão fixa;
2. desempenho de volta perto da main (proposta: `bench_factory` até +10 %, zero alocação por quadro, bench da vila grande rodando). O #52 pode ajudar e precisa de auditoria;
3. pacote próprio e README corrigido;
4. sessão curta em jogo;
5. **congelar o #47**: o #51 e o #52 voltam como frentes depois do merge.

**Recomendação: B (ampliada).** Também recomendo que o dono defina **quem pode fazer merge na main**, preferencialmente só ele (B4).

---

## Q12. Peso da sociedade frente à fábrica

**Contexto.** Da 0.18 à 0.25, em cerca de 1h30 (16:06–17:29 UTC de 08/11), o agente Cloud construiu sociedade, contas, holding, economia fechada e comércio exterior. As obras acrescentaram prestígio e decretos. Nada disso foi pedido explicitamente pelo dono no chat; ele disse que há "muito trabalho vindo em aldeões, vilas, sistemas e visualização" (16:27) e que a ideia central é o jogo de fábrica.

**Alternativas**
- **A.** Sociedade e economia são pilar do Veio, no mesmo nível da fábrica.
- **B.** São camada de suporte: a vila consome e dá mão de obra, mas a complexidade (IDH, Gini, holding) fica opcional ou é simplificada.
- **C.** Congelar a sociedade até o primeiro teste em jogo.

**Recomendação: C, e depois decidir entre A e B com base no teste.** O volume de regras sociais é grande e nunca foi jogado. Decidir o peso antes de ver em partida é especular.

---

## Q13. Imposto uma ou duas vezes sobre o mesmo bem

**Contexto.** O OPUS A encontrou (E2, `market.rs` `receive_sale` e `settle`) que o imposto da fase (4–16 %) é retido **no varejo** (família → loja) **e de novo no atacado** (loja → companhia). A ECONOMIA.md calcula o teto local "já contando o imposto da fase", como se fosse uma vez só. A regra de equilíbrio 3 ("no teto, o atacado local rende tanto quanto exportar") pode não valer na fase 4 (até 16 % + 16 %). Isso é risco, não fato medido: o `sim_economy.py` replica as fórmulas à mão.

**Alternativas**

| | Alternativa | Consequência |
|---|---|---|
| A | Tributar só o consumo final (varejo) | Coerente com "imposto na fonte em cada venda ao consumidor". A prefeitura arrecada menos e é preciso recalibrar as alíquotas |
| B | Tributar só o atacado (produção) | A prefeitura depende da companhia, e o comércio local fica livre |
| C | Manter os dois e recalibrar as alíquotas pelo simulador | Preserva o código. O simulador precisa modelar a dupla incidência |

**Recomendação: A**, recalibrando com o simulador depois de ele passar a ler as constantes do Rust. É a leitura mais direta do desenho da 0.21 e não pune a cadeia vila → companhia, que é o loop médio do jogo (B1 §6).

---

## Q14. Módulo de produtividade: 10 % ou 20 %

**Contexto.** Na main, a produtividade dá um produto extra a cada 10 ciclos (README da main; `upgrades.rs`). No pr47 dá um a cada 5 (`upgrades.rs:35`, commit c5db4f6, autor "Jefter Pereira", frente de balanço). Nenhuma decisão do dono. O módulo também deixa o ciclo 15 % mais lento e gasta 40 % mais energia. Com 10 %, a máquina **produz menos por minuto** (1,10/1,15 ≈ 0,96) e só economiza insumo. Com 20 %, produz cerca de 4 % mais por minuto e economiza mais insumo.

**Alternativas**
- **A.** 20 % (pr47).
- **B.** 10 % (main).
- **C.** 10 % com ciclo sem penalidade.

**Recomendação: A.** Com 10 %, o módulo é uma armadilha para quem não lê a tabela: parece "mais produção" e produz menos por minuto. Com 20 %, o nome bate com o efeito. Registrar como decisão do dono para encerrar a divergência entre main e pr47.

---

## Q15. O que é de cada jogador e o que é do mundo (LAN)

**Contexto** (E2 do A):
- O bônus de refeição e de banquete do Lar é **global** (atomics): a refeição de um jogador acelera as máquinas de todos.
- A pesquisa do laboratório (`pesquisa.txt`) e o grande projeto concedem os avanços a **todos** (`@a`).
- O Lar tem "dono" e o tutorial é por jogador (UUID).
- O cofre da companhia é um só por dimensão.

**Alternativas**
- **A.** "Uma empresa por mundo": tudo é compartilhado (cooperativo), e isso é documentado.
- **B.** Por jogador: Lar, bônus e pesquisa. Mais código e conflito com o cofre único.
- **C.** Misto: progresso tecnológico e cofre compartilhados; o bônus do Lar vale só nas máquinas a até N blocos daquele Lar.

**Recomendação: C.** O LAN do Veio já é cooperativo (o convidado vai à vila do anfitrião, FUN-MP-01), então pesquisa e cofre compartilhados combinam. O bônus global do Lar contradiz a ideia de "casa" e é a única regra que um jogador ausente sente sem ter feito nada.

