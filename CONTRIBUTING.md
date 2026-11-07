# Trabalhar no MineFac / Veio

Este repositório contém o Veio, um jogo de automação industrial executado como mod nativo do Horizon. O estado consolidado inicial é a versão **0.6.0**, com energia, transporte, processamento, contratos e tutorial dentro do jogo.

## Começar na nuvem

1. Clone este repositório e crie uma branch a partir da `main` atualizada.
2. Leia o [README](README.md) para conhecer o comportamento implementado.
3. Execute `python tools/check_project.py`. Requer Python 3.11 ou superior e nenhuma biblioteca adicional.
4. Faça as alterações e envie um pull request para `main`. Atualize sua branch antes de integrar trabalhos simultâneos.

O código e o conteúdo podem ser editados na nuvem sem instalar o launcher, baixar Minecraft ou abrir um mundo. A verificação leve confere JSON, versões, traduções e navegação do guia. Ela não compila Rust e não comprova funcionamento no jogo.

## Onde alterar

- `native/api/src/lib.rs`: simulação, energia, inventários, transporte e regras de interação.
- `native/api/src/presentation.rs`: painéis e desenho animado das máquinas e esteiras.
- `native/api/src/contracts.rs`: pedidos e recompensas da central de projetos.
- `native/api/src/tutorial.rs`: ajuda no inventário, rótulos e dicas dos painéis.
- `content/data/veio`: registros, receitas, avanços e tabelas de saque.
- `content/assets/veio`: modelos, texturas e traduções `pt_br` e `en_us`.
- `tools/build_tutorial.py`: fonte das páginas do guia, ajuda traduzida e roteiro inicial. Rode este script ao alterar o tutorial e inclua os JSON gerados no mesmo commit.
- `third_party/kenney` e `CREDITS.md`: licença e origem das imagens reutilizadas.

As receitas do guia são extraídas das receitas de fabricação reais. Ao mudar ingredientes de máquinas, gere novamente o guia. Ao mudar regras de processamento, atualize também os textos explicativos do gerador do tutorial.

## Compilar e instalar para jogar

A versão 0.6 foi compilada com Horizon `66f3981f522b24c250b21523a70080965b940424`, base `nightly-2026-11-05-66f3981`, Rust 1.99.1 e perfil `dev`. As APIs do Horizon evoluem; conserve essa referência ao investigar incompatibilidades. O launcher resolve e prepara o ambiente nativo. Um `cargo build` isolado nesta pasta não substitui esse processo.

Em uma máquina Windows com Horizon Launcher instalado e uma instância chamada Veio, execute, ajustando os caminhos:

```powershell
& "$env:LOCALAPPDATA\Programs\Horizon Launcher\horizon-launcher.exe" native-build "E:\GPTmodMine\native" --instance Veio --trust-native-code --profile dev
```

Com o jogo fechado, copie o **conteúdo** de `content/` para a pasta `horizon-mods/veio/` da instância. Ela deve conter diretamente `horizon-mod.toml`, `assets/` e `data/`. Selecione Veio no launcher para jogar.

No ambiente local original, launcher, instâncias e cache estão no E:, configurados por `HORIZON_LAUNCHER_LOCATION_FILE=E:\GPTmodMine\.build\location.json`. Esse arquivo pertence à instalação local e não é distribuído. Uma máquina nova deve configurar seus próprios caminhos no launcher. Os diretórios `.build/`, `.launcher/` e `.game/` são recriados localmente e não precisam ser baixados do GitHub.

## Combinar o trabalho

- Concentre cada branch em uma mudança e descreva o efeito para o jogador no pull request.
- Faça a verificação leve ao concluir uma entrega. Evite abrir o jogo e repetir testes a cada ajuste; o usuário prefere testar os lotes concluídos.
- Separe comportamento implementado de propostas futuras. O README registra as limitações atuais dos contratos e da progressão.
- Preserve mundos existentes e dados salvos ao mudar máquinas. Explique quando uma alteração exigir migração.
- Mantenha downloads, compiladores, executáveis, mundos, contas e configurações pessoais fora do Git. Não distribua arquivos do Minecraft. As referências de textura são resolvidas pela instalação do jogador.
