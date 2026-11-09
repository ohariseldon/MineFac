# SIS-TUT: Tutorial e guias

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. Linhas em `native/api/src/` no pr47. Contagens de página por `check_project` (E1).

## A. Identidade

Ensinar fazendo. "Você conhece Minecraft; a vila ensina a transformar uma oficina operada à mão numa linha que trabalha sozinha" (README do pr47). Um objetivo por vez acima da barra; os livros são consulta.

- **Depende de:** [SIS-PRO](SIS-PRO.md), [SIS-LOG](SIS-LOG.md), [SIS-PRG](SIS-PRG.md) (central de contratos), [SIS-PER](SIS-PER.md) (progresso por jogador).
- **Alimenta:** a primeira hora do jogador.

## B. Funcionamento

### B.1 Tutorial na tela (pr47)

O servidor decide (`onboarding.rs:1-3`): o tutorial observa produção real e transferências aceitas. Conteúdo do inventário e cliques nunca completam um objetivo. Itens transferidos levam uma origem curta e limitada pela linha (até 128 rastros, por 6.000 ticks: `MAX_TRACE`, `TRACE_TTL`).

| Passo | Objetivo |
|---|---|
| 1 | Pôr um gerador |
| 2 | Energizar uma prensa |
| 3 | Fazer 4 chapas |
| 4 | Fazer uma engrenagem |
| 5 | Extrair cobre de verdade |
| 6 | Abastecer a prensa automaticamente (≥ 2 cobres) |
| 7 | Guardar 4 chapas automaticamente |
| 8 | Cumprir a 1ª encomenda da central (24 chapas + 8 engrenagens) |

Fonte: README do pr47 (E3); 8 objetivos e slots 0–4 (gerador, prensa, montador, broca, central) em `onboarding.rs:9-17` (E2).

- Aceita esteiras, braços, funis e carregadores como automação. Os funis anunciam tentativas; o crédito espera até o inventário mudar de fato (`onboarding/transport.rs:2`).
- Comandos: `/veiotutorial ocultar | mostrar | reiniciar`.
- Progresso por jogador em `<mundo>/veio/tutorial/<UUID>.txt`, gravado a cada 100 ticks (`onboarding.rs:403`) por arquivo temporário.
- Dicas contextuais com chaves dinâmicas em todas as camadas (`tutorial.rs:52`).

**O tutorial termina na central.** Não apresenta água, sociedade, comércio exterior nem pesquisa. A água pedida pelo dono ([DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039)) está só no guia, páginas 15 ("Água e irrigação") e 16 ("Rios e represas").

### B.2 Livros

| Livro | Receita | Páginas (pr47) | Situação |
|---|---|---|---|
| Guia do Veio | livro + papel | 100 (teto do `check_project`) | main: 93 |
| Primeira oficina | livro + pedregulho | — | — |
| Guia da Vila Operária | livro + trigo | 29 páginas acima de 14 linhas (contadas por idioma: 17 em pt_br e 12 em en_us); duas páginas "8."; página do PIB errada | Aviso aberto |
| Transportes | — | Terminais só aqui ([DEC-058](../06_HISTORICO_E_DECISOES.md#dec-058)) | pr47 |
| Grande Indústria | — | 26 páginas; fora do `check_page_fit` | pr47 |

**Regras de tamanho** ([DEC-058](../06_HISTORICO_E_DECISOES.md#dec-058)): guia principal até 100 páginas; página até 14 linhas.

**Textos errados conhecidos:** PIB ([CNF-007](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)); página "Comércio" ainda descreve a exportação pela central e não o porto ou a estação; páginas de progressão que falam em fases e tempos substituídos.

## C. Experiência do jogador

1. Ao entrar, a pessoa vê "Ponha um gerador" acima da barra.
2. Segue os 8 passos até cumprir a primeira encomenda.
3. Depois, abre os livros quando precisa.
4. Em LAN, cada jogador tem o seu progresso.

## D. Implementação

**Arquivos:** `onboarding.rs`, `onboarding/transport.rs`, `tutorial.rs`; livros e páginas em `content/assets/veio/lang/pt_br.json` (`veio.guide.*`, `veio.village.page.*`, transportes, indústria). Testes: onboarding 19 + 3 ignorados, que exigem `--test-threads=1` (E1). `onboarding::session` chama `world_dir` e quebra o harness da vila grande ([SIS-TRB](SIS-TRB.md) FUN-TRB-12).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-TUT-01 Dicas contextuais | Chaves dinâmicas | `tutorial.rs` · `tutorial.rs:52` · — | E2 |
| FUN-TUT-02 Guia principal | ≤ 100 páginas | 93 páginas · 100 (teto) · — | E1 |
| FUN-TUT-03 Guia da Vila Operária | ≤ 14 linhas por página | Integrado · 29 páginas acima (contadas por idioma: 17 em pt_br e 12 em en_us) (aviso) · — | E1 |
| FUN-TUT-04 Guia Transportes | — | — · Integrado · — | E1 |
| FUN-TUT-05 Guia Grande Indústria | 26 páginas | — · Integrado · #40 (contido) | E1; fora do `check_page_fit` |
| FUN-TUT-06 Tutorial interativo por jogador | 8 passos; servidor decide; arquivo por UUID | — · `onboarding.rs:1-17,403` · Substituído (zip 0.15.1, `TRACE_TTL 200`) | E1 (19 + 3 testes) |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 18:24 (0.6) | `5eaea2c` | Guia do Veio |
| 07/11 22:33 (0.14) | `a691802` | Guia da Vila |
| 08/11 madrugada | Codex local 0.15.1–0.15.3 | Tutorial na tela; versão do zip 0.15.1 substituída |
| 08/11 ~17h | #40 | Guia Grande Indústria |
| Integração | CHECKPOINT | 30 páginas cortadas corrigidas; Transportes separado |

## F. Pendências

1. Reduzir as 29 páginas longas (contadas por idioma: 17 em pt_br e 12 em en_us) e corrigir as duas páginas "8." do Guia da Vila ([CNF-012](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
2. Corrigir PIB, Comércio e progressão nos guias ([CNF-007](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
3. Estender o tutorial: água e irrigação (pedido do dono), vila, comércio, pesquisa.
4. Pôr o guia de indústria no `check_page_fit`.
5. Fazer os 3 testes ignorados rodarem em paralelo.
6. Isolar `world_dir` em `onboarding::session` para o harness.
7. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-015](../08_BACKLOG_E_PRIORIDADES.md#tar-015): Corrigir o README e os docs do pr47 que contradizem o código
- [TAR-023](../08_BACKLOG_E_PRIORIDADES.md#tar-023): Guia e tutorial cobrindo o que o pr47 acrescentou
- [TAR-030](../08_BACKLOG_E_PRIORIDADES.md#tar-030): Guias dentro dos limites
- [TAR-048](../08_BACKLOG_E_PRIORIDADES.md#tar-048): Textos do guia desatualizados (PIB, Comércio)
- [TAR-052](../08_BACKLOG_E_PRIORIDADES.md#tar-052): Testes de onboarding em paralelo

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01–03) · pr47 **Integrado** (01–06) |
| Confiança | Alta para o código do tutorial (E1); média para os textos (avisos) |
| Prioridade (sugestão) | Média: os textos errados enganam o jogador |
| Criticidade (sugestão) | Média: é a primeira hora de jogo |
| Responsável conhecido | Codex local (tutorial, guias); Claude nuvem (Guia da Vila); Claude do #40 (indústria) |
| DEC | [DEC-039](../06_HISTORICO_E_DECISOES.md#dec-039), [DEC-058](../06_HISTORICO_E_DECISOES.md#dec-058) |
| Q | — |
| CNF | [CNF-007](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (PIB), [CNF-012](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (guia) |
| Referências | README do pr47 §Tutorial; `CHECKPOINT-INTEGRACAO.md`; A2 TUT |
