# B5: Governança documental permanente

Autor: OPUS B. Data: 2026-11-08.

Esta é uma proposta de mecanismo, a ser criado em `docs/consolidacao-mestra/` no repositório. **Nada é apagado.** Os documentos antigos ganham um status, e um aviso de uma linha é opcional. Quem escreve na árvore do repositório é o supervisor; eu escrevo só em `SP/b/`.

O mecanismo responde a quatro problemas observados:

1. **Spec, implementação e verificação estavam misturadas.** Um mesmo texto dizia o que o jogo deve fazer e afirmava que já fazia.
   - Exemplo: o README do pr47 descreve fases e tempos que o código já não usa.
2. **Nenhum ID estável.** Entregas numeradas em duplicidade ("Sexta" ×2, "Sétima" ×2), versões repetidas, códigos de status reaproveitados.
3. **Decisões do dono e de agentes misturadas.** "A progressão por fases substitui os tempos" (CONSOLIDACAO) parecia regra; o dono decidiu o contrário (DEC-045).
4. **Documento de coordenação parado.** A COORDENACAO está na 0.9.1 enquanto o jogo foi até a 0.25.

---

## 1. Estrutura de `docs/consolidacao-mestra/`

```
docs/consolidacao-mestra/
  README.md            índice + quadro de estado por SIS × camada (main / integração / disperso) + commit de referência
  VISAO.md             (de B1) pilares, identidade, loops, restrições — muda só com DEC do dono
  sistemas/
    SIS-PLT.md … SIS-MP.md, SIS-PES.md   uma ficha por sistema (formato §3)
  DECISOES.md          registro DEC-NNN (de B3 §2), só acrescenta
  TAREFAS.md           TAR-NNN abertas/fechadas
  VERIFICACAO.md       matriz FUN × evidência × camada (§4)
  REGISTRO_IDS.md      status, slots, blocos, NBT, arquivos de mundo, avanços, prefixos de tradução (B4 §2.3)
  CONFLITOS.md         CNF-NNN (protocolo §6)
  QUESTOES_DONO.md     (de B6) fila de perguntas ao dono; resposta vira DEC
  CHANGELOG.md         por versão, com IDs (§5)
  STATUS_DOCS.md       status de todo .md legado (§7)
  HISTORICO.md         (de B3 §1) cronologia das versões e das linhas paralelas
```

**Responsáveis**

| Arquivo | Quem escreve | Quem aprova |
|---|---|---|
| VISAO | Integrador | Dono (sempre) |
| sistemas/* | Frente (a sua SIS) e integrador | Integrador |
| DECISOES | Integrador. A entrada do dono é copiada literalmente, com hora | Dono para o tipo "aprovada" |
| TAREFAS | Qualquer agente | Integrador |
| VERIFICACAO | Frente (E1/E2), auditor, testador (jogo) | Integrador |
| REGISTRO_IDS | Frente reserva | Integrador confirma |
| CONFLITOS | Qualquer agente abre | Integrador (técnico) ou dono (produto) fecha |
| QUESTOES_DONO | Integrador | Dono responde |
| CHANGELOG | Integrador | Dono, no merge |
| STATUS_DOCS | Integrador | — |

## 2. Identificadores

| Prefixo | Formato | Exemplo | Regras |
|---|---|---|---|
| SIS | `SIS-` + 2 a 3 letras | `SIS-MER` | Lista fechada (CONTEXTO §5 + `SIS-PES`). Fundir ou dividir só com DEC |
| FUN | `FUN-<SIS>-NN` | `FUN-MER-05` (saturação) | Uma regra jogável ou um comportamento verificável. NN nunca é reaproveitado. Uma FUN substituída fica na ficha marcada "Substituída por FUN-…" |
| DEC | `DEC-NNN` sequencial global | `DEC-035` | Campos fixos: data/hora UTC, quem, tipo, texto, fonte, FUN afetadas, estado do código, substitui/substituída por |
| TAR | `TAR-NNN` sequencial | `TAR-012` | Campos: FUN/DEC de origem, frente, branch, estado, evidência de fechamento |
| CNF | `CNF-NNN` | `CNF-003` (40/41) | Conflito aberto; fecha com uma DEC ou uma TAR |
| Q | `Q-NN` | `Q-01` | Pergunta ao dono; a resposta vira DEC |

Tipos de DEC, iguais ao B3: ideia · proposta · decisão aprovada (dono) · decisão de agente (não confirmada pelo dono) · implementada · substituída · revogada · abandonada.

**Semente:** B2 propõe cerca de 130 FUN, B3 registra DEC-001 a DEC-061 e B6 tem Q1 a Q12. As TAR nascem das seções F das fichas.

## 3. Separação entre spec, implementação e verificação

Cada FUN numa ficha `sistemas/SIS-*.md` tem **três blocos que não se misturam**:

| Bloco | Pergunta | Conteúdo | Quem muda |
|---|---|---|---|
| **Spec** | O que deve acontecer? | Regra com números e DEC de origem. Escrita no futuro do presente ou no imperativo, nunca "já faz" | Só com DEC (dono ou agente rotulado) |
| **Implementação** | Onde está, por camada? | `arquivo:linha@commit` para main, integração e disperso; estado do ciclo de vida por camada | Frente e integrador, a cada integração |
| **Verificação** | Como sabemos? | Lista de evidências: E1 (comando, resultado, data, commit), E2 (arquivo:linha), E3/E4 (fonte), **J** (teste em jogo: versão, quem, roteiro, resultado) | Frente, auditor, testador |

Formato de linha, em tabela, como em B2:

```
| FUN-MER-05 | Spec: spot = cheio×absorção/(absorção+pressão), mínimo 25% (DEC-031/032) |
| Impl: main trade.rs:… @53267af — Integrado; integração trade.rs:… @80980db — Integrado; disperso — |
| Verif: E2 (OPUS B); E1 balance_* em check_project @80980db (a registrar); J — nenhum |
```

Regras de leitura:
- O estado de uma FUN numa camada é o **mínimo** sustentado pela verificação.
- Sem J, nada passa de "Integrado".
- Spec sem implementação é "Concebido" ou "Aprovado", conforme a DEC.

## 4. Matriz de verificação (`VERIFICACAO.md`)

Uma linha por FUN, com as colunas:

`FUN | estado main | estado integração | E1 (último comando/data) | E2 (arquivo:linha) | J (último teste em jogo) | divergência doc×código`

A coluna de divergência existe para capturar casos como "README diz 64 blocos, `pollution.rs:47` diz 26".

## 5. Vínculo com commits e PRs, e changelog

**Commits.** Rodapé obrigatório (trailer) com os IDs afetados:

```
Refs: FUN-OBR-04, DEC-035, TAR-021
```

**Branches.**
- `frente/<sis>-<tema>` para trabalho de frente.
- `integracao/<versão>` para o integrador.
- Nada de branch com nome de sessão como linha principal.

**PR.** Corpo com quatro seções fixas:
1. IDs (FUN/DEC/TAR);
2. Estado pretendido por FUN;
3. Evidência E1 colada (comando e saída);
4. "Não testado": sempre inclui jogo real e Horizon real enquanto for verdade.

**Integrador.** Antes de fundir, confere:
- se todo FUN citado existe;
- se o REGISTRO_IDS não tem colisão;
- se `data.rs` foi regenerado;
- se o CHANGELOG foi atualizado.

**CHANGELOG.** Uma seção por versão, nunca reescrita. O README do jogo deixa de acumular o histórico no primeiro parágrafo (hoje o parágrafo de abertura tem cerca de 3 KB de 0.16 a 0.25).

```
## 0.26.0 — 2026-11-DD — integração #47 (main ← integracao/0.26.0)
Adicionado: FUN-AGU-01..04 (DEC-039), FUN-TRA-01 (DEC-041/043), FUN-OBR-01..09 (DEC-033/034) …
Mudado: FUN-PRG-01 liberações de aço/circuitos → FUN-PES-02 (DEC-018)
Removido: bônus +15% em casa (DEC-046)
Substituído: prensa 80t (DEC-012 → DEC-045)
Conhecido/aberto: CNF-001 emissão de moeda (Q-01); CNF-003 status 40/41
Verificação: E1 check_project/check_rust/check_performance @<commit>; J: <roteiro/resultado ou "nenhum">
```

## 6. Protocolo de conflito

1. **Detectar.** Qualquer agente que encontrar duas fontes contraditórias (doc × doc, doc × código, frente × frente) abre um `CNF-NNN` com:
   - as duas fontes, com nível de evidência;
   - as FUN afetadas.
2. **Classificar.**
   - **Técnico** (IDs, slots, nomes internos, ordem de merge, regeneração): resolve o **integrador** e registra uma DEC de agente.
   - **Produto** (regras de jogo, economia, progressão, o que o jogador vê): vira uma **Q** para o dono, num cartão de decisão com 2–4 opções e uma recomendada.
3. **Regra provisória enquanto não resolve.**
   - Para **descrever** o jogo, vale o código da camada (E2).
   - Para **direção**, vale a última DEC do dono.
   - Doc contrário ao código ganha a marca "divergente" na VERIFICACAO.
4. **Resolver.** Uma DEC (com "substitui DEC-…") e as TAR de ajuste de código e docs.
5. **Fechar.** O CNF fecha quando as TAR fecharem e a VERIFICACAO não mostrar divergência.

CNF iniciais sugeridos (de B3 §3 e §3.10):

| CNF | Assunto | Tipo | Encaminhamento |
|---|---|---|---|
| CNF-001 | Emissão de esmeraldas fora do livro | Produto | Q1 |
| CNF-002 | Duas progressões (fases × pesquisa) | Produto | Q3 |
| CNF-003 | Códigos 40/41 (pendência do centro × frete) | Técnico | Integrador + OPUS A |
| CNF-004 | Dois pedidos diários | Produto | Q2 |
| CNF-005 | Versões 0.25.0 ×2 e launcher 0.16.1 | Técnico + produto | Q8 |
| CNF-006 | README do pr47 desatualizado (4 pontos) | Técnico | TAR |
| CNF-007 | PIB no guia × código | Técnico | TAR |
| CNF-008 | Bônus das obras × teto de 150 | Produto | Q6 |
| CNF-009 | Rotas longas × produção virtual | Produto | Q5 |
| CNF-010 | "Prefeitura" homônima; escadas de nível | Produto | Q4 |
| CNF-011 | Itens da grande indústria fora da bolsa | Produto | Q1/Q3 |
| CNF-012 | 29 páginas do Guia da Vila acima de 14 linhas; duas páginas "8." | Técnico | TAR |

## 7. Status proposto dos documentos existentes (`STATUS_DOCS.md`)

Valores:
- **vigente**: descreve regra ou contrato atual; é lido como fonte.
- **histórico**: registro de entrega ou ordem passada; verdadeiro para a época e não normativo.
- **substituído**: contradito por DEC ou código posterior; indicar o substituto.

"Camada" indica onde o arquivo existe hoje. Proposta: cada arquivo histórico ou substituído ganha **uma linha no topo**:

```
> Status: histórico — ver docs/consolidacao-mestra/…
```

Essa linha é opcional e não altera o conteúdo.

### 7.1 Raiz

| Documento | Camada | Status proposto | Motivo / ação |
|---|---|---|---|
| README.md | main, pr47 | **vigente**, com correção | Porta de entrada. No pr47 corrigir: §Progressão (gates e pesquisa), linha 21 (+15%), linha 24 ("duas prensas"), §Poluição (ondas 0.17). Mover o histórico do primeiro parágrafo para o CHANGELOG |
| CONTRIBUTING.md | main, pr47 | **vigente**, com correção | Trocar "subir a versão a cada entrega" pela política aprovada (Q8) |
| COORDENACAO.md | main, pr47 | **substituído** | Parado na 0.9.1; numeração de status errada (28/29). Substituto: `consolidacao-mestra/README.md` e workflow (B4) |
| CONSOLIDACAO.md | pr47 | **histórico** | Registro da consolidação Codex. A frase "fases substituem os tempos" foi revogada (DEC-045) |
| HANDOFF_OBRAS.md | pr47 | **vigente** até virar TAR; depois histórico | Lista de pendências das obras |
| MAPA.md | main, pr47 | **vigente** (contrato), pendente Q9 | Restrição de projeto |
| TRABALHADORES.md | main, pr47 | **vigente**, com trechos substituídos | Referência dos postos. Marcar "sexta entrega" (+15%) como substituída (DEC-046). Numeração duplicada vira FUN |
| TRANSPORTES.md | pr47 | **vigente** | Acrescentar o limite das áreas simuladas × estações (Q5) |
| LOGISTICA.md | main, pr47 | **histórico** | Logística 0.9.1; o atual está em CODEX-LOGISTICA |
| PERFORMANCE.md | main, pr47 | **histórico** | Lote de otimização sobre a 0.15 |
| VISUAL-MAQUINAS.md | pr47 | **vigente** | 34 formas e 27 silhuetas |
| CREDITS.md | main, pr47 | **vigente** | Licenças (Kenney CC0, shader MIT) |
| work-in-progress/finite-world/CONTINUAR.md | pr47 | **vigente** (rascunho) | Dono atual: thread "limite do mapa" |
| docs/visual/*.png | pr47 | **vigente** | Catálogo de máquinas |

### 7.2 `frentes/`

| Documento | Camada | Status proposto | Motivo / substituto |
|---|---|---|---|
| AGUA-E-RIOS.md | pr47 | vigente | Título com "0.22.0" obsoleto (renumerações) |
| AMBIENTE-E-MOVIMENTO.md | pr47 | vigente | Sons e animação |
| AUDIO-MAQUINAS.md | pr47 | vigente | — |
| BALANCEAMENTO-ECONOMIA.md | pr47 | **substituído** | Economia de recompensas da 0.15 → ECONOMIA.md (0.25) |
| BALANCEAMENTO-MISSOES.md | pr47 | histórico | Missões da 0.15. Produtividade +20% segue no README |
| CAPATAZES-INCREMENTAIS.md | main, pr47 | vigente (técnico) | Mecanismo atual |
| CARREGADORES-INCREMENTAIS.md | main, pr47 | vigente (técnico) | — |
| CASA-FUNDADOR-INTEGRACAO.md | pr47 | **substituído** em parte | Comportamento de casa → DEC-046 |
| CHECKPOINT-INTEGRACAO.md | pr47 | histórico | Modelo de bom handoff (B4) |
| CLOUD.md | main, pr47 | histórico | Ordem de frente 0.9.1 |
| CODEX-BRACOS.md | main, pr47 | **substituído** | O próprio texto aponta CODEX-LOGISTICA |
| CODEX-LOGISTICA.md | main, pr47 | vigente | Braços, filtros, divisor, unificador |
| CODEX.md | main, pr47 | histórico | Ordem de frente |
| DESEMPENHO.md | main, pr47 | histórico | Lote 0.17.1 |
| ECONOMIA.md | main, pr47 | **vigente** | Números e regras de equilíbrio (fonte da SIS-ECO/MER) |
| ESCALA-E-ENERGIA.md | main, pr47 | histórico | Lote do Codex |
| EXPERIENCIA-JOGADOR.md | pr47 | vigente, com trechos substituídos | Quadro vigente. "Unidade de 30/min" substituída (DEC-045). Pedido diário pendente de Q2 |
| GRANDE-INDUSTRIA.md | pr47 | **vigente** | — |
| GROK-BALANCE.md | pr47 | **substituído** | Prensa 80t → DEC-045 |
| GROK-LOTE.md, GROK-LOTE3–8.md | main, pr47 | histórico | Registros de lote |
| GROK-LOTE2.md | pr47 | **substituído** | Conflitou; o lote 3 o substituiu (o próprio GROK-LOTE3 diz) |
| GROK.md | main, pr47 | histórico | Ordem de frente |
| INTERFACE_POSTOS.md | main, pr47 | vigente (técnico), a confirmar | Contrato de inspeção de postos (OPUS A confirma o uso atual) |
| JAZIDAS-CONECTADAS.md | pr47 | vigente como **proposta** | Parte (1) implementada (industrial ≥16); parte (2) Concebido |
| MEGAFABRICAS.md | pr47 | vigente | — |
| ORCAMENTO-GLOBAL.md | main, pr47 | **vigente** | Cotas por tick (normativo) |
| PROGRESSAO-FASES.md | pr47 | **substituído** (tempos e unidade de 30/min); gates vigentes até Q3 | DEC-045 |
| REDES-INCREMENTAIS.md | main, pr47 | vigente (técnico) | — |
| SOCIEDADE.md | main, pr47 | **vigente** | Desenho da vila. Item pendente "obras no IDH" vira TAR |
| VILA-CIDADE.md | pr47 | vigente como **proposta** | Pedido do usuário relatado; parte absorvida pela sociedade 0.18 |
| branches-consolidadas.json | pr47 | histórico | Auditoria de 29 branches |
| prototipos/jazidas_conectadas.py | pr47 | histórico | Protótipo da proposta |

### 7.3 Documentos fora do repositório

| Documento | Status | Ação |
|---|---|---|
| Artefato "Veio: lacunas em relação a jogos de fábrica" (Claude Docs `e04da5cb` / `UhUqLwEVPnRrKFzKbK6NMD`) | vigente como **origem da DEC-017** | Citar em DECISOES. As lacunas restantes (subproduto, fluido do Nether, meta sustentada) viram TAR ou Q10 |
| Memória do projeto (notas de agentes) | fonte secundária (E4) | Migrar os fatos para o REGISTRO_IDS e as fichas; a memória não é fonte de regra |

## 8. Manutenção mínima

| Gatilho | Atualizar |
|---|---|
| Resposta do dono no chat | DECISOES (texto literal e hora); QUESTOES_DONO (fechar Q); ficha afetada (Spec) |
| Merge na branch de integração | Implementação e VERIFICACAO das FUN tocadas; REGISTRO_IDS; CHANGELOG (rascunho) |
| Merge na main | CHANGELOG final; README da consolidação (quadro de estados) |
| Teste em jogo | Coluna J da VERIFICACAO; estado "Validado" só aqui |
| Doc novo de frente | Linha em STATUS_DOCS; a frente escreve na ficha SIS, não num doc solto, salvo registro de lote |

**Automação sugerida** (TAR, não decisão): estender `tools/check_project.py` para validar:
- (a) que todo `FUN-/DEC-/TAR-` citado em commits e docs existe;
- (b) que nenhum índice de `STATUS_KEYS` ou slot registrado se repete com outro sentido;
- (c) que todo documento listado tem status em STATUS_DOCS.
