# N3.1 — Pixelorama

Foi baixado o tarball oficial Linux 64-bit da versão 1.2.4 (publicada em 2026-11-03, MIT), de `https://github.com/Orama-Interactive/Pixelorama/releases/download/v1.2.4/Pixelorama-Linux-64bit.tar.gz`.

SHA-256 local: `692473e44fe90ea02de49048343d6b3cb86749b7beb166286dbc3a308c568e0b`. Não havia checksum publicado conferido contra o GitHub.

O binário extraído respondeu a `--help` com o texto do Godot Engine 4.7.1. Não há uma linha de comando de pintura. Não houve sessão gráfica de abrir, editar e exportar um PNG.

O tarball e a pasta extraída ficam em `art-src/n3-proof/tools/`, ignorados pelo git. Não entram no commit.

Nenhum pixel desta prova foi pintado no Pixelorama. O caminho B é um script: redução BOX, quantização para 16 cores, contorno de 1 px e ampliação vizinha 2×. Isso não é acabamento manual. O pipeline híbrido não fica aprovado.
