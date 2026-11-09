# SIS-APR: Apresentação (visual, som e realismo)

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af`, `pr47@80980db` e, para o realismo 2.1, `#51@a0b0d72` (não auditado linha a linha). Linhas em `native/api/src/` no pr47.

## A. Identidade

Legibilidade no estilo Minecraft, com realismo opcional e leve.
- A base vanilla é obrigatória e jogável sem shader: painéis como baú e fornalha, luzes de estado, texturas 16 × 16 ([DEC-054](../06_HISTORICO_E_DECISOES.md#dec-054)).
- O realismo é uma camada escolhida no launcher ([DEC-055](../06_HISTORICO_E_DECISOES.md#dec-055)).
- O dono pediu "realismo maior sem pesar o PC", com referência de 8 GB (20:09, [DEC-056](../06_HISTORICO_E_DECISOES.md#dec-056)).

- **Depende de:** [SIS-PLT](SIS-PLT.md) (geradores de textura, formas e som; launcher), [SIS-DES](SIS-DES.md) (custo por quadro).
- **Alimenta:** a leitura do estado de todas as máquinas.

## B. Funcionamento

### B.1 Painéis e estados

- Cada máquina tem tela no estilo de baú ou fornalha e uma luz de estado na tela e nas 4 faces.
- **44 status** numa tabela global (`STATUS_KEYS`, `presentation.rs:19-34`): produção 14–17, trabalhadores 18–28, casa 29–31, conexões externas 32–35, canteiro 36–39, frete 40–43 ([DEC-052](../06_HISTORICO_E_DECISOES.md#dec-052); [04 §10.1](../04_ARQUITETURA_E_DEPENDENCIAS.md)).
- 13 slots de dados por painel (`DATA_SLOTS`).

### B.2 Formas, animação e som (pr47)

| Peça | Regra | Fonte |
|---|---|---|
| Formas | 34 modelos e 27 silhuetas, gerados por `machine_forms.py`; a colisão acompanha a forma; indicadores e peças móveis usam as mesmas âncoras dos modelos | `appearance.rs:1-2`; `check_machine_forms` (E1) |
| Animação | 8 poses-chave cíclicas, interpoladas no relógio de render, nunca no tick do servidor; peças que giram usam rotação na GPU | `motion.rs:1-2` |
| Som posicional | 30 amostras `.ogg` (Kenney, CC0); curtas, sem laço persistente, sem timer no NBT; alcance 12, cauda 48 | `soundscape.rs:1-2`; `check_machine_audio` (E1) |
| Ambiente | Ambiente sonoro limitado; esteiras suaves; som de avaria | `frentes/AMBIENTE.md` (E3) |

### B.3 Realismo

| Versão | Onde | O que é |
|---|---|---|
| Realismo leve 1.1 | pr47: `visuals/veio-industrial`, `Escolher-Visual.cmd`, `Jogar-Leve.cmd` | Shader WGSL (MIT), api 0.1 aceita pelo Horizon `5f7cadf` (`manifest.rs:611-626`). O WGSL nunca foi validado |
| Perfis do launcher | `launcher/src/visuals.rs:118-146` | **Realismo:** distância 8 chunks, simulação 5, escala de resolução 85%, nuvens rápidas. **Leve:** 6 chunks, 75%, sem nuvens. Ambos 60 FPS, sem LOD, TAA nem FXAA, 256 MiB de texturas |
| Realismo 2.1, clima e estações do ano | PR #51 | `ambience.rs` (238 linhas), `climate.rs` (160), 16 ganchos novos em `lib.rs` (`visuals`, `shader_params`), stub com `fn player()`, `NativePlayer`, `NativeBlockRim`, 9 shaders, `tools/check_shaders.py`. Declara a versão 0.26.0 ([DEC-062](../06_HISTORICO_E_DECISOES.md#dec-062)) |

- O pacote `release/Veio-Realismo-1.1.zip` entrega o Veio 0.16.1, não a candidata ([CNF-005](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
- Se o clima do #51 tem efeito de jogo ou é só visual é ND ([Q-07](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-07-realismo-clima-e-estações-do-ano)).
- "Estações" do ano colide com estações de carga ([CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf), [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem)).

### B.4 Custo

- 0.17.1 (main): o quadro desenhado não aloca memória (`bench_frame` = 0 alocações, E1).
- pr47: **329 alocações por quadro** e `bench_frame` de 45,2 para 71,4 µs (E1). O #51 mede 68,7 µs e o #52, 66,0 µs, ambos com 329 alocações ([SIS-DES](SIS-DES.md) B.3).
- Suspeitos da bisseção: `appearance`, `motion`, `soundscape` (09 [R-14](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)/15).

## C. Experiência do jogador

1. A pessoa reconhece cada máquina pela silhueta e ouve onde ela está.
2. A luz de cada face diz se a máquina trabalha, espera ou está quebrada.
3. Antes de jogar, escolhe "Realismo" ou "Leve" no launcher.

## D. Implementação

**Arquivos:** `presentation.rs`, `appearance.rs` e `appearance/`, `motion.rs`, `soundscape.rs` e `soundscape/`, `home/mural.rs` (mapas); `tools/build_textures.py`, `machine_forms.py`; `visuals/veio-industrial`; `launcher/src/visuals.rs`. Testes: soundscape 7, motion 2, appearance 2 (E1).

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-APR-01 Painéis e telas | 44 status; 13 dados | `presentation.rs` · `presentation.rs:19-34` · — | E2 |
| FUN-APR-02 Texturas 16 × 16 | Geradas por texto | 65 · 89 · — | E1 |
| FUN-APR-03 Silhuetas e colisão | 34 formas; 27 silhuetas | — · `appearance.rs` · #31 (contido) | E1 |
| FUN-APR-04 Animação de peças | 8 poses no relógio de render | — · `motion.rs` · #50 (contido) | E1 |
| FUN-APR-05 Som posicional | 30 `.ogg`; alcance 12 | — · `soundscape.rs` · #49 (contido); #52 restringe a perto do jogador | E1 |
| FUN-APR-06 Ambiente sonoro limitado | — | — · Integrado · #50 (contido) | E1 |
| FUN-APR-07 Shader "realismo leve" 1.1 | WGSL; perfis Realismo e Leve | — · `visuals/veio-industrial`; `visuals.rs:118-146` · zip Realismo 1.1 (= 0.16.1) | E2; WGSL ND |
| FUN-APR-08 Quadro sem alocação | 0 alocações por quadro | Integrado · — (329) · — | E1 |
| FUN-APR-09 Realismo 2.1, clima, estações do ano | Ver B.3 | — · — · Em desenvolvimento (#51) | E3 (não auditado) |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 19:10 (0.7) | `819ba92` | Visual no estilo Minecraft ([DEC-054](../06_HISTORICO_E_DECISOES.md#dec-054)) |
| 08/11 00:25 | `codex/machine-identity`, PR #31 | 34 formas, 27 silhuetas |
| 08/11 madrugada | Codex local | Realismo leve 1.1 ([DEC-055](../06_HISTORICO_E_DECISOES.md#dec-055)) |
| 08/11 07:17 (0.17.1) | `0472750` | Quadro sem alocação |
| 08/11 16:42 / 17:11 | Codex nuvem, PRs #49 e #50 | Som posicional, ambiente, animação |
| 08/11 20:09 | Dono | "Realismo maior sem pesar o PC" ([DEC-056](../06_HISTORICO_E_DECISOES.md#dec-056)) |
| 08/11 20:34 | PR #51 | Realismo 2.1, clima, estações do ano ([DEC-062](../06_HISTORICO_E_DECISOES.md#dec-062)) |

## F. Pendências

1. Achar e corrigir as 329 alocações por quadro do pr47 por bisseção ([R-14](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)/15).
2. Validar o WGSL no Horizon real e definir um critério de FPS e memória para o realismo ([Q-07](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-07-realismo-clima-e-estações-do-ano)).
3. Decidir se o realismo é opcional ou padrão e se o clima tem efeito de jogo ([Q-07](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-07-realismo-clima-e-estações-do-ano)).
4. Renumerar o #51 quando entrar ([Q-08](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera)) e auditar seus 53 arquivos.
5. Fechar os PRs #31, #49 e #50, já contidos no pr47.
6. Teste em jogo: nenhum.

### Tarefas no backlog

- [TAR-009](../08_BACKLOG_E_PRIORIDADES.md#tar-009): Quadro sem alocação de novo
- [TAR-013](../08_BACKLOG_E_PRIORIDADES.md#tar-013): Auditar e reintegrar o #52
- [TAR-014](../08_BACKLOG_E_PRIORIDADES.md#tar-014): Auditar o #51 (Realismo 2.1, clima, 0.26.0)
- [TAR-034](../08_BACKLOG_E_PRIORIDADES.md#tar-034): Registro central de status, slots e reservas
- [TAR-047](../08_BACKLOG_E_PRIORIDADES.md#tar-047): Nomes que colidem
- [TAR-055](../08_BACKLOG_E_PRIORIDADES.md#tar-055): Validar o WGSL do shader
- [TAR-063](../08_BACKLOG_E_PRIORIDADES.md#tar-063): Clima e estações do ano com efeito de jogo

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01, 02, 08) · pr47 **Integrado** (01–07), regressão em 08 · disperso: 09 **Em desenvolvimento** |
| Confiança | Média: E1 offline; nada visto no Horizon |
| Prioridade (sugestão) | Alta para a regressão de alocação; média para o realismo |
| Criticidade (sugestão) | Alta: a regressão contraria a meta de 8 GB |
| Responsável conhecido | Claude local (0.7); Codex nuvem (formas, sons); Codex local (shader 1.1); thread "Aumentar o realismo sem pesar o PC" (#51) |
| DEC | [DEC-052](../06_HISTORICO_E_DECISOES.md#dec-052), [DEC-054](../06_HISTORICO_E_DECISOES.md#dec-054), [DEC-055](../06_HISTORICO_E_DECISOES.md#dec-055), [DEC-056](../06_HISTORICO_E_DECISOES.md#dec-056), [DEC-062](../06_HISTORICO_E_DECISOES.md#dec-062) |
| Q | [Q-07](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-07-realismo-clima-e-estações-do-ano), [Q-04](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-04-nomes-que-colidem), [Q-08](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera) |
| CNF | [CNF-005](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (pacote Realismo 1.1), [CNF-010](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (estações), [CNF-020](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (#52 afirma 0 alocações) |
| Referências | `CREDITS.md`, `frentes/VISUAL-MAQUINAS.md`, `frentes/AUDIO-MAQUINAS.md`, `frentes/AMBIENTE.md`; A2 APR; A4 (#51) |
