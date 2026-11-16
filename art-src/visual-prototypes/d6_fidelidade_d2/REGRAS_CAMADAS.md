# Regras de sobreposição (proposta)

Ordem típica (atrás → frente), lado direito (câmera):

1. Capa (back) — pode vir à frente em frames extremos (`front` por socket)
2. Body / cabelo atrás
3. Calças / botas
4. Casaco leve **ou** peitoral pesado
5. Ombreiras / braçadeiras
6. Franja / rosto
7. Elmo (oculta cabelo se fechado)
8. Escudo se `hand_l.front == false` atrás; senão à frente
9. Arma conforme `weapon.front`
10. FX

No Bevy (não implementado): reutilizar `front` dos sockets + Z relativos;
separar capas/coat em back/front meshes se a animação exigir.

Ocultações: elmo fechado → hide hair+glasses; casaco longo → mask upper pants;
cuirass pesado → hide light coat torso.
