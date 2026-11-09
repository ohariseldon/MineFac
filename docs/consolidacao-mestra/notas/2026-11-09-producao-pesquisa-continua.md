# Pesquisa contínua: destino sem fim para engrenagens, molduras, circuitos e processadores

- **Data / frente / autor:** 2026-11-09 · producao · thread "Dar uso aos itens produzidos"
- **Onde:** branch `claude/project-thread-0v2oym` (sobre `claude/teste-combinado@c32a10f`), PR a seguir
- **Sistemas:** SIS-PES (laboratório), SIS-PRO (máquinas), SIS-PRG · **FUN:** nova, sem ID
- **Tipo:** código · conteúdo

## Por quê
Pedido do dono (09/11 02:16): cada peça intermediária, como a engrenagem, precisa de um uso vantajoso que peça **fluxo grande e contínuo**, como no Factorio e no Satisfactory, e não um ou dois itens por receita de máquina. Antes, depois das 4 pesquisas e do grande projeto, a engrenagem só servia para receitas de 1 a 8 unidades e para a bolsa.

## O que mudou
- O laboratório (`research.rs`) não para depois de Escala: entra na **pesquisa contínua**, em 4 linhas sem fim. A peça da 1ª entrada escolhe a linha (vazia, vale a 2ª; sem nada, a última do bloco).

| Linha | Pacote do nível 1 | Bônus por nível (fábrica inteira) |
|---|---|---|
| Usinagem | 256 engrenagens + 64 aço | +5% de ritmo nas máquinas de receita |
| Perfuração | 128 molduras + 16 motores | +5% de ritmo nas extratoras |
| Rede elétrica | 128 circuitos + 256 fios | +5% de rendimento da energia (menos FE por tick) |
| Precisão | 32 processadores + 64 módulos | +2% de produto, até +50% |

- O nível *n* pede *n* vezes o pacote, e o bônus tem retorno decrescente (fator `100 / (100 + 5 × nível)`), então a procura cresce sem quebrar o jogo. Um laboratório gasta 2 de cada peça por segundo (120/min, 4 montadoras de engrenagem só para ele); vários somam.
- O bônus entra em `upgrades::effect_as` (receita × extratora) via `research::boost`, lido de atômicos (sem trava por tick). A poluição não muda. O painel (`ui/screens.rs`) mostra linha, nível e pacote, e as razões por minuto já contam o bônus.
- Save: `pesquisa.txt` ganhou 12 números no fim (nível e entregas de cada linha). Arquivos antigos abrem com tudo zerado. Slot 8 do painel do laboratório: na contínua, `4 + linha + 4 × nível`.
- Textos: `veio.research.line.*`, `veio.research.level_up`, `veio.ui.lab.level`; `veio.research.complete` agora fala da contínua. Duas páginas novas no Guia da Grande Indústria ("Pesquisa contínua" e "Por que fluxo", 31 páginas) e a seção na wiki.

## Evidência
- E1: `python3 tools/check_rust.py` → 404 passam (3 testes novos: custo e save, escolha da linha, bônus).
- E1: `python3 tools/check_project.py` → 100 páginas e traduções conferidas.
- Não testado em jogo.

## Toca no compilado
- NBT/arquivo: `<mundo>/veio/pesquisa.txt` (formato estendido, compatível); `ContractStage` do laboratório ≥ 4 codifica linha e nível.
- Decisão para o dono: números (pacotes, 5%/nível, teto da Precisão) são iniciais.
- Próximos sumidouros possíveis: grande indústria na bolsa (TAR-056); pedidos da vila em volume nas vilas grandes.
