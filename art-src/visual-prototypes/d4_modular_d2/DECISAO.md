# Decisão (proposta para o proprietário)

## Recomendação

**Pipeline C — híbrido**, com look **D2 detalhado** como identidade visual
oficial (D3-B1/B2 ficam como prova de proporção/camadas, não como skin final).

## Por quê

- O jogo já tem sockets + `HeldLayer` (arma/escudo) e capa procedural/tint —
  base funcional existe.
- Overlays 2D puros (B) funcionam no idle do protótipo, mas o motion GIF
  mostra o custo de não parentar peças aos sockets.
- Blender puro (A) resolve pose/consistência, mas o look EEVEE/geométrico
  (D3) foi rejeitado como identidade; A precisa de acabamento D2.

## Próximo passo (só após autorização)

1. Congelar lista mínima de camadas: body, cape, chest light/heavy, headgear,
   glasses, weapon, shield.
2. Definir se cosméticos usam Head/Accessory existentes (sem novo slot).
3. Produzir 1 clip (idle) com overlays registrados nos sockets — prova no Bevy
   **em branch/lab**, não no main sem pedido.
4. Só então expandir para run/attack.

## Não avançar agora

Produção completa de animações, Save V3, commit/push desta pasta — aguardam
revisão do proprietário.
