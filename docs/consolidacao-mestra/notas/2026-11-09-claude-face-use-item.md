# Correção: face do clique ao pôr máquina grande

- Data: 2026-11-09
- Erro: a compilação no PC (Horizon 66f3981) falhou em `lib.rs` `use_item_on`: na API real `UseItemOnEvent.face` é a normal `[i32; 3]`, mas `frames::use_item` pedia `Direction` (o stub offline usa `Direction`).
- Correção: `frames::use_item` aceita qualquer `Face` (trait com `normal()`), implementada para `[i32; 3]` e `Direction`; serve à API real e aos testes.
- Testes: `check_rust.py` 424 ok. Falta recompilar no PC.
