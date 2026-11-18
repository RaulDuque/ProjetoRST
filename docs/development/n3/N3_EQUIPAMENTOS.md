# N3 — Equipamentos

Uma folha de corpo leve e uma de corpo pesado, o mesmo grid (6×4 células de 96×128) e o mesmo pivô `(48, 120)`. A arma e a capa não estão nessas folhas.

| Peça | Arquivo | Notas |
|---|---|---|
| corpo leve | `body_light.png` | tabardo, pele, cabelo |
| corpo pesado | `body_heavy.png` | placa, ombreiras, elmo; bytes diferentes do leve |
| capa | `cape.png` | célula 88×100, gola no pivô `(44, 14)`, um quadro por quadro do corpo |
| espada | `sword.png` | 64×24, grip `(12, 12)`, aponta para +x |
| lança | `spear.png` | 88×18, grip `(26, 9)` |

Os sockets estão em `sockets.ron`. No ataque, a mão vai de x −12,9 (preparo, arma atrás) a x 47,1 (golpe, arma na frente). Trocar arma ou armadura no exemplo não zera o quadro. As quatro combinações são as duas folhas de corpo com um dos dois sprites de arma, não quatro folhas de corpo. A capa liga e desliga sem outra folha.

Uma quinta arma, se um dia for feita, seria outro PNG e outro grip. O corpo não precisa de folha nova. Isso não foi cronometrado com um artista.
