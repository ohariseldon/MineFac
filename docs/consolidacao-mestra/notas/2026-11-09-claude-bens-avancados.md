# Bens avançados na branch de teste

- Data: 2026-11-09
- O que: oito bens novos sobre as Cores (zip `bens.zip`, patch sobre `0299f63` aplicado como `c02b2e6` sobre as Cores já integradas): ventilador, rádio, máquina de costura, televisão, geladeira, motocicleta, carro elétrico e robô ajudante.
- Onde: receitas na Fabricadora e no Integrador (`tools/production.json`), avanço "Linha de bens avançados", página no Guia da Grande Indústria, artigo na wiki.
- Adaptação: `trade::PRODUCTS` passa a ter 58 produtos (Combinadora, depois Cores, depois os 8 bens no fim, índices antigos preservados). Línguas juntas chave a chave; guias, wiki e `data.rs` regerados.
- Testes: `check_rust.py` 424 ok; `check_project.py` ok. Falta o teste no jogo.
