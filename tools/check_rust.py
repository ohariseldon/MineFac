"""Compila e testa o módulo nativo sem o launcher, contra um substituto da API do Horizon.

Requer Rust 1.90 ou superior. Funciona sem internet: o substituto fica em
`native/offline-check/horizon-client` e não baixa o Horizon.

Isto confere tipos, regras de jogo e testes. Não substitui a compilação pelo launcher:
se o Horizon mudar uma assinatura, só a compilação real acusa. Ao usar um método novo do
Host ou um gancho novo, copie a assinatura do Horizon para o substituto.

`python tools/check_rust.py --preview` também grava `native/offline-check/target/screens.jsonl`
com tudo o que os painéis desenham, para gerar imagens de conferência.
"""
import subprocess
import sys
from pathlib import Path

WORKSPACE = Path(__file__).resolve().parents[1] / "native/offline-check"


def run(*args):
    print("$", " ".join(args))
    subprocess.run(args, cwd=WORKSPACE, check=True)


def main():
    run("cargo", "test", "--offline", "-p", "veio-check")
    if "--preview" in sys.argv:
        run("cargo", "test", "--offline", "-p", "veio-check", "preview", "--", "--ignored")


if __name__ == "__main__":
    main()
