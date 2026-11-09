# Combinadora (zip "Fabricante") adaptada e integrada na branch de teste

- **Data / frente / autor:** 2026-11-09 · claude · thread "Testar a build combinada no PC"
- **Onde:** zip `MineFac-fabricante.zip` enviado pelo dono (patch sobre `53267af`, 0.25.0), juntado na `claude/teste-combinado`
- **Sistemas:** produção, painel, mercado, guia · **FUN:** nova, sem ID
- **Tipo:** código · conteúdo

## O que mudou
- Entra a máquina `veio:manufacturer`, com 4 entradas, uma saída e um slot de sobra. A receita é escolhida pela mistura (a mais completa vence) e pode ser fixada no painel com `< >`. Há subprodutos (escória vira cascalho, os trilhos devolvem pepitas). Entram também a placa reforçada e o rotor. Detalhes em `frentes/FABRICANTE.md`.
- Pedido do dono: a máquina foi **renomeada para Combinadora** ("Combiner"), porque a branch já tinha a **Fabricadora** (`veio:fabricator`, 3 entradas).
- O **motor** já existia (`veio:motor`, "Motor elétrico", feito no integrador). Virou um item só: ficam as duas receitas do zip na Combinadora e a do integrador. A pilha continua de 64. A receita alternativa passou a se chamar "Motor com circuito".
- **`Info` no painel:** nas máquinas de receita, `Info % 64` continua sendo a entrada que falta, como na versão atual. Só na Combinadora, `Info / 64` guarda a receita fixada. A receita atual vem de `RecipeIndex` (slot 8). Com a receita fixada e faltando ingrediente, o painel mostra o que falta nela.
- Ela é liberada pela pesquisa de Eletrônica (`research/electronics.json`), porque a branch não libera mais máquinas pelo avanço "circuitos".
- Ganhou estrutura multibloco 3x3 em `tools/machine_frames.py` (quatro tremonhas, mesa, saída e tanque de sobra), na linha do #60/#62.
- O guia principal estava no limite de 100 páginas, então as receitas da Combinadora não entram nele: elas ficam no Guia da Grande Indústria (ao lado da Fabricadora) e na wiki.
- `build_production.py` passou a aceitar 3 entradas na Fabricadora (`INPUT_SLOTS`).

## Evidência
- E1: `python3 tools/check_rust.py` → 398 passam, entre eles os 4 testes do zip (mistura, sobra, receita fixada e slots).
- E1: `python3 tools/check_project.py` → 100 páginas conferidas.
- Ainda não testado em jogo.

## Toca no compilado
- Conflito resolvido: dois `veio:motor` independentes (Grande Indústria × Fabricante).
- Uma decisão para o dono depois: os números das receitas ainda são os iniciais do zip.
