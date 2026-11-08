# Interação com aldeões e máquinas: o jogo diz o que fazer

- **Data / frente / autor:** 2026-11-08 · interacao · Claude, thread "Consertar interação com aldeões e fábricas"
- **Onde:** branch `claude/project-thread-szmjbg`, PR sobre o #47 (`claude/project-thread-l94frq`)
- **Sistemas:** SIS-TRB, SIS-PRO · **FUN:** FUN-TRB-01, FUN-TRB-13 (mudam); escolha do aldeão e consulta ao aldeão são novas, sem ID
- **Tipo:** código · conteúdo

## O que mudou
- **Painel dos postos:** a 3ª linha virou o próximo passo conforme o estado (contratar, pôr comida, esvaziar a saída, pôr insumos, pôr esmeraldas, caminho bloqueado...). Fica a dica fixa quando está tudo bem. Função pura `workforce::next_step(block, data)`.
- **Contratar:** sem aldeão livre a 8 blocos, o posto espera 30 s e o jogador clica, ainda com a esmeralda, no aldeão que quiser (até 32 blocos). Fora dessa espera, esmeralda no aldeão continua sendo comércio vanilla. Mensagem de contratação certa por posto (antes todos diziam "vai até as plantações") e, sem comida, ela pede a comida.
- **Perguntar ao aldeão:** agachado e de mão vazia, clicar num aldeão mostra o posto onde ele trabalha e o problema dele, ou que está livre. Gancho novo `interact_entity` (API 29 do Horizon; acrescentado ao substituto).
- **Máquinas:** com "Faltam ingredientes", a máquina publica a receita mais próxima (`RecipeIndex`, as taxas acompanham) e a entrada que falta (`Info`); o painel diz "Falta: 2 Chapa de cobre (entrada 1)". Função pura `production::missing_line(block, data)`.
- **Carregador encadeia máquinas:** o que ele recolhe de uma máquina agora abastece outra máquina parada por falta daquilo (prensa > montador), como a ficha SIS-TRB C.2 já prometia. Nunca devolve um produto a máquina que o fabrica. Antes ele só guardava as chapas e o jogador tinha que mover à mão.

## Evidência
- E1: `python tools/check_rust.py` → 330 testes passando (5 novos: escolha do aldeão, regras da escolha, próximo passo do painel, carregador prensa > montador, entrada que falta). `python tools/check_project.py` ok (só os avisos antigos do Guia da Vila).
- J: não testado em jogo. A assinatura do gancho vem de `native_hooks.rs@66f3981` (`entity: i32, kind: &str, stack: Option<&ItemStack>`, contrato cancel → `Flow`); só a compilação pelo launcher confirma.

## Toca no compilado
- `Info` (data[12]) das máquinas de receita passa a significar "entrada que falta" (0 = nada a apontar).
- A thread de UI (kit `ui/`) vai chamar `next_step` e `missing_line` no rodapé das telas novas.
- Pendente: chaves `veio.site.no_staff` e `veio.center.project.idle` faltam no #47 e chegam com o #53.
