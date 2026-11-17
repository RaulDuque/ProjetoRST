# Padrão de equipamento independente

Cada item vive sob `assets-user/equip/<categoria>/<id_item>/`.

## Categorias

| Pasta | Conteúdo |
|---|---|
| `arma/` | Espadas, pesadas, adagas, lanças, cajados… |
| `cabeça/` | Capacetes, capuzes, coroas, máscaras, chapéus |
| `corpo/` | Armaduras, casacos, peitorais, túnicas |
| `calça/` | Calças e proteções de pernas |
| `capa/` | Capas, mantos, tecidos independentes |
| `sapatos/` | Botas, sapatos, proteções de pés |

## Contrato por item

```text
equip/<categoria>/<id_item>/
├── referencia/     # PNG/JPG autorizados
├── modelo/         # GLB (canônico e/ou experimental)
├── blender/        # .blend de lab + inspect JSON
├── renders/        # ortográficas isoladas
└── metadata.json   # contrato (ver schema abaixo)
```

## Schema `metadata.json` (campos)

- `id` — único (`equip.arma.espada_inicial`)
- `name`, `category`, `description`
- `version`
- `reference_image`
- `glb_canonical` / `glb_hunyuan_experimental` (opcionais)
- `blender_*`
- `scale` — bbox e unidade
- `orientation`
- `attach_point` — socket + osso
- `required_bones`
- `deform_type` — `rigid` | `skinned` | `cloth`
- `material`
- `validation_status` — `draft` | `pilot_partial` | `fit_ok` | `anim_ok` | `ready`
- `known_issues`

**Nunca** marcar `ready` sem encaixe + pelo menos um ciclo de animação ok.

## Primeiro item

`arma/espada_inicial/` — piloto D15 (canônico v007 + Hunyuan experimental).
