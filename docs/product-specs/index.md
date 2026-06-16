# Product Specs

Router into the product contract. In this repo the living contract lives under
[`docs/product/`](../product/) — this folder exists to match the Advanced Pack
shape and to hold larger, spec-style write-ups when an initiative needs one
(create from `docs/templates/spec.md`).

Current contract:

- [`../product/PRODUCT.md`](../product/PRODUCT.md) — purpose, capabilities, rules.
- [`../product/api-contract.md`](../product/api-contract.md) — REST contract.
- [`../product/domain-model.md`](../product/domain-model.md) — core read models.

Do not duplicate the contract here. Add a spec file only for a net-new product
area, then decompose it into stories and `feature_list.json` entries.
