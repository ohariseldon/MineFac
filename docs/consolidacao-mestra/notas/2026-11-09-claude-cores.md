# Cores (zip) e pesquisa contínua (#63) na branch de teste

- **Data / frente / autor:** 2026-11-09 · claude · thread "Testar a build combinada no PC"
- **Onde:** zip `cores.zip` do dono (patch `0299f63` sobre `50f1bd4`) e PR #63 (`96e620d`), juntados na `claude/teste-combinado`
- **Sistemas:** produção, mercado, conforto, guias, máquinas grandes · **FUN:** nova, sem ID
- **Tipo:** código · conteúdo

## O que mudou
- Entraram o #63 (pesquisa contínua no laboratório) e o zip das Cores, com:
  - corantes no moinho e no triturador;
  - o Misturador de tintas e a Cabine de pintura;
  - peças pintadas e bens finais (relógio, placa de sinalização, bicicleta);
  - 13 produtos no mercado, relógio e bicicleta em `GOODS`, e os avanços "Fábrica de cores", "Pintado na linha" e "Bens de consumo".
  Detalhes na descrição do zip.
- A cadeia funda agora pede peças pintadas: o kit Mk.II usa engrenagens pretas e o núcleo de automação usa chapas de aço azuis. Com isso, a pesquisa de Escala e o grande projeto, que pedem núcleos, passam pela linha de pintura. A pesquisa contínua do #63 continua consumindo engrenagens e aço comuns, sem conflito.
- O que foi adaptado na junção com a Combinadora:
  - as tabelas de máquinas (`is_machine`, `consumes`, `has_inputs`, `emission`) têm as três máquinas;
  - `PRODUCTS` tem 50 entradas: as da Combinadora primeiro e as cores depois, sempre no fim;
  - o campo `"book": "industry"` do zip substituiu o `GUIDE_SKIPS_RECIPES` da Combinadora, e as 12 receitas dela também levam esse campo;
  - os testes do zip usam `stocked(id, ...)`, e os da Combinadora foram ajustados para isso.
- O Misturador (2x2, cobre) e a Cabine (2x3, ferro) ganharam estrutura multibloco em `tools/machine_frames.py`, como as outras máquinas do #60/#62.

## Evidência
- E1: `python3 tools/check_rust.py` → 417 passam. `check_project.py` → 100 páginas e 113 texturas.
- Ainda não testado em jogo.

## Toca no compilado
- Os números (tempo, energia, preço, conforto) são os iniciais do zip, para a frente de balanceamento.
