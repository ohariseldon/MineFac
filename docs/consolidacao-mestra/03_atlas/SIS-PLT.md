# SIS-PLT: Plataforma, build e ferramentas

[Índice do atlas](../03_ATLAS_DE_SISTEMAS.md).

- **Bases:** `main@53267af` e `pr47@80980db`. Medições de `check_*` executadas pelo OPUS A (E1, `evidencias/logs/`).
- **Escopo:** tudo o que leva o jogo do repositório ao PC do jogador. A arquitetura de módulos está em [04](../04_ARQUITETURA_E_DEPENDENCIAS.md).

## A. Identidade

| Peça | O que é | Fonte |
|---|---|---|
| Motor | Horizon: Minecraft 26.3 reescrito em Rust ([DEC-002](../06_HISTORICO_E_DECISOES.md#dec-002)) | README; CONTRIBUTING |
| Mod nativo | Crate `mod-veio` 0.25.0 em `native/api/src`, compilado pelo Horizon Launcher | `native/api/Cargo.toml` |
| Conteúdo | `content/` (blocos, itens, receitas, avanços, lang, texturas, sons) | — |
| Geradores | `tools/*.py`: `production.json` → `production/data.rs`; texturas, tutorial, formas (`machine_forms.py`), plantas, casa (`build_house.py`) ([DEC-003](../06_HISTORICO_E_DECISOES.md#dec-003)) | CONTRIBUTING |
| Launcher | `launcher.exe` (Windows), versionado no Git junto com `release/` ([DEC-004](../06_HISTORICO_E_DECISOES.md#dec-004)) | CONTRIBUTING |
| Pacote | `release/Veio-<versão>.zip` | — |

- **Depende de:** Horizon (dependência Git **sem `rev` e sem Cargo.lock**: [R-02](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
- **Alimenta:** todos os sistemas. Sem compilação real, nenhuma FUN passa de "Integrado".

## B. Funcionamento

### B.1 Do repositório ao jogo

1. O agente edita `tools/production.json` ou outro gerador e roda o script. O `data.rs` gerado entra no commit. Depois de um merge, é preciso regenerar: um auto-merge já trouxe tempos do Codex por engano (E4, memória).
2. `python tools/package.py` monta o zip.
3. O jogador abre o `launcher.exe`. O launcher compara um **carimbo** gravado com o esperado. Se diferir, roda `horizon-launcher native-build`, instala o conteúdo na instância **Veio** e a cria se não existir.
   - main: recompila se `built.trim() != version` (Cargo do launcher 0.10.0).
   - pr47: o carimbo é `"{version}\nprofile={NATIVE_PROFILE}"`, com perfil `performance` (main: `dev`). Cargo do launcher 0.16.1.
4. A última compilação nativa alegada é a 0.16.1 do Codex em `4ffb605`, contra o Horizon `66f3981`, em 7 min 14 s (`CONSOLIDACAO.md`, E3). No commit de referência, o pr47 nunca tinha sido compilado contra o Horizon real ([R-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
   - **Depois do corte (E4 e E3, não verificado):** às 21:18 a thread do #47 compilou o pacote combinado no PC do dono, contra o Horizon `66f3981` do cache do launcher. Parou em 4 erros no código do Veio. O commit `3f7cb28` (21:40) corrigiu a visibilidade de `with_nbt_key`, `FastHasher` e `FastHash` e o uso de `Mob::on_climbable`, que no Horizon é método e no substituto não era. O E1 offline, portanto, não garante a compilação ([06, DEC-065](../06_HISTORICO_E_DECISOES.md#dec-065)).

### B.2 Verificações offline

| Comando | O que valida | main | pr47 |
|---|---|---|---|
| `check_project.py` | JSON, traduções, páginas, texturas, regras `balance_*` | 529 JSON, 93 páginas, 65 texturas | 742 JSON, 100 páginas (teto), 89 texturas; aviso: 29 páginas do Guia da Vila acima de 14 linhas (contadas por idioma: 17 em pt_br e 12 em en_us) |
| `check_rust.py` | Testes Rust contra o stub `native/offline-check/horizon-client` (Horizon `66f3981`) | 176 ok / 0 falhas / 5 ignorados | 325 / 0 / 9 |
| `check_performance.py` | 7 cenários de custo por tick | ok | ok (os limites não pegam a regressão: [SIS-DES](SIS-DES.md)) |
| `check_native_api.py` | Assinaturas contra o `.rmeta` real do Horizon | não roda sem `.rmeta` | idem |
| `check_machine_audio`, `check_machine_forms` | 30 sons; 34 formas e 27 silhuetas | — | ok |
| `sim_economy.py --check` | 7 invariantes do modelo econômico | ok | ok (não modela as emissões: [SIS-ECO](SIS-ECO.md)) |
| Testes do launcher | — | 2 | 6 |

- O stub reproduz 44 (main) e 52 (pr47) métodos do `Host`. O que o stub aceita, o Horizon real pode recusar.
- O guia `industry_guide` está fora do `check_page_fit`.
- Os geradores são reprodutíveis: só PNG (zlib) e `.ogg` diferem em bytes ao regenerar (E1).
- **Não há CI** (`.github` inexistente, [R-31](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).

### B.3 Pacotes no repositório

| Arquivo | Camada | Conteúdo |
|---|---|---|
| `release/Veio-0.25.0.zip` | main e pr47 | **O mesmo arquivo** nas duas (blob `e01ef740`, sha256 `7dc57f01…`, E1). O pr47 **não tem pacote próprio** |
| `release/Veio-Realismo-1.1.zip` | pr47 | Veio 0.16.1 do Codex com o shader |
| `release/Veio-0.26.0.zip` | #51 | Pacote do PR #51 (não auditado) |

O README do pr47 manda usar `Veio-0.16.1.zip`, que não existe ([R-23](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43), [CNF-006](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).

## C. Experiência do jogador

1. Abre o `launcher.exe`. Na primeira vez, faz o Prepare do Horizon e ativa o Veio em Opções > Mods.
2. A cada versão que muda `native/`, espera a recompilação (minutos).
3. Hoje, quem baixar o pr47 e abrir o zip 0.25.0 recebe a main.

## D. Implementação

**Arquivos:** `launcher/src/main.rs` (main `:52`; pr47 `:56-57`); `tools/check_*.py`; `tools/package.py`; `native/offline-check/horizon-client/`; `native/api/Cargo.toml`.

| FUN | Spec | Implementação (main · pr47 · disperso) | Verificação |
|---|---|---|---|
| FUN-PLT-01 `check_project` | JSON, lang, páginas ≤ 100, ≤ 14 linhas (aviso) | ok · ok + aviso · — | E1 |
| FUN-PLT-02 Testes Rust contra o stub | Stub do Horizon `66f3981` | 176/0/5 · 325/0/9 · — | E1 |
| FUN-PLT-03 `check_performance` | 7 cenários | ok · ok · — | E1 |
| FUN-PLT-04 Compilação contra o Horizon real | `native-build` | Indeterminado · Indeterminado; primeira tentativa em 08/11 21:18 falhou com 4 erros, corrigidos em `3f7cb28`; recompilação ND · 0.16.1 em `4ffb605` (E3) | ND ([R-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)) |
| FUN-PLT-05 `check_native_api.py` | Exige `.rmeta` real | Implementado isoladamente · idem · — | E2 |
| FUN-PLT-06 Pacote jogável | `release/Veio-<v>.zip` | Integrado · — (zip da main) · Substituído (zips 0.15.1/0.15.2) | E1 (hash) |
| FUN-PLT-07 Launcher instala e compila | Carimbo; perfil | 0.10.0, `dev` · 0.16.1, `performance` · — | E1 (2 × 6 testes) |
| FUN-PLT-08 Geradores reprodutíveis | `tools/*.py` | Integrado · Integrado · — | E1 |
| FUN-PLT-09 `sim_economy.py` | 7 invariantes | Integrado · Integrado · — | E1 (modelo) |
| FUN-PLT-10 CI | — | — · — · — | E2 (ausente) |
| FUN-PLT-11 Horizon fixado | `rev` ou lock | — · — · — | E2 (ausente) |

## E. Histórico

| Quando | Fonte | Mudança |
|---|---|---|
| 07/11 21:04 (0.10.0) | `9421883` | Launcher no repositório |
| 07/11 20:30 | COORDENACAO | Três frentes; "não subir versão nas frentes" × CONTRIBUTING "subir a cada entrega" ([DEC-007](../06_HISTORICO_E_DECISOES.md#dec-007), [DEC-008](../06_HISTORICO_E_DECISOES.md#dec-008)) |
| 08/11 madrugada | `4ffb605` | 0.16.1 compilada contra o Horizon real; cache incoerente `horizon_client` × `horizon_core` |
| 08/11 antes de 18:11 | CHECKPOINT | Usuário interrompe a consolidação do Codex ([DEC-049](../06_HISTORICO_E_DECISOES.md#dec-049)) |
| pr47 | `bcfb9d5` | `read_columns`/`chunk_load` entram depois da última compilação |

## F. Pendências

1. Compilar o pr47 com `horizon-launcher native-build` numa máquina com Horizon ([R-01](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43), [Q-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-11-condições-para-fundir-o-47)).
2. Fixar o Horizon por `rev` e versionar o Cargo.lock ([R-02](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
3. Criar CI mínima com as três verificações ([R-31](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43)).
4. Gerar um pacote próprio do pr47 e corrigir o README ([R-23](../09_CONFLITOS_LACUNAS_E_RISCOS.md#apêndice-a-registro-completo-de-riscos-técnicos-r-01r-43), [CNF-006](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
5. Escolher o número de versão e quem numera ([Q-08](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera), [CNF-005](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf)).
6. Pôr o `industry_guide` no `check_page_fit`.
7. O launcher só existe para Windows.

### Tarefas no backlog

- [TAR-001](../08_BACKLOG_E_PRIORIDADES.md#tar-001): Compilar o pr47 contra o Horizon real
- [TAR-002](../08_BACKLOG_E_PRIORIDADES.md#tar-002): Fixar a revisão do Horizon
- [TAR-004](../08_BACKLOG_E_PRIORIDADES.md#tar-004): Pacote próprio do pr47 e instruções de instalação
- [TAR-010](../08_BACKLOG_E_PRIORIDADES.md#tar-010): `world_dir` no harness de teste
- [TAR-015](../08_BACKLOG_E_PRIORIDADES.md#tar-015): Corrigir o README e os docs do pr47 que contradizem o código
- [TAR-019](../08_BACKLOG_E_PRIORIDADES.md#tar-019): CI mínima
- [TAR-036](../08_BACKLOG_E_PRIORIDADES.md#tar-036): Simulador econômico ligado ao código
- [TAR-040](../08_BACKLOG_E_PRIORIDADES.md#tar-040): Política de versão
- [TAR-050](../08_BACKLOG_E_PRIORIDADES.md#tar-050): Despacho e acoplamentos
- [TAR-051](../08_BACKLOG_E_PRIORIDADES.md#tar-051): Código morto e avisos
- [TAR-054](../08_BACKLOG_E_PRIORIDADES.md#tar-054): Launcher versionado e conferido

## G. Classificação

| Campo | Valor |
|---|---|
| Estado | main **Integrado** (01–03, 06–09) · pr47 **Integrado** (01–03, 07–09), sem pacote · compilação real **Indeterminado** · CI e `rev` ausentes |
| Confiança | Alta para o offline (E1); nula para o Horizon real |
| Prioridade (sugestão) | **Muito alta**: condição do merge ([Q-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-11-condições-para-fundir-o-47)) |
| Criticidade (sugestão) | Crítica: sem compilação real, nada chega ao jogador |
| Responsável conhecido | Codex local (geradores, verificações, launcher); integrador do #47 |
| DEC | [DEC-002](../06_HISTORICO_E_DECISOES.md#dec-002), [DEC-003](../06_HISTORICO_E_DECISOES.md#dec-003), [DEC-004](../06_HISTORICO_E_DECISOES.md#dec-004), [DEC-005](../06_HISTORICO_E_DECISOES.md#dec-005), [DEC-008](../06_HISTORICO_E_DECISOES.md#dec-008), [DEC-049](../06_HISTORICO_E_DECISOES.md#dec-049) |
| Q | [Q-08](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-08-número-de-versão-e-quem-numera), [Q-11](../09_CONFLITOS_LACUNAS_E_RISCOS.md#q-11-condições-para-fundir-o-47) |
| CNF | [CNF-005](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (versões), [CNF-006](../09_CONFLITOS_LACUNAS_E_RISCOS.md#2-conflitos-cnf) (README do pr47) |
| Referências | `CONTRIBUTING.md`, `CONSOLIDACAO.md`, `CHECKPOINT-INTEGRACAO.md`; A1 §2; A5; [04 §8–9](../04_ARQUITETURA_E_DEPENDENCIAS.md) |
