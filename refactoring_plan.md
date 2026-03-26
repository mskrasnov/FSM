# refactoring plan

## [ ] Step 1

- [ ] Move all custom widgets code from the `ferrix-app` to the `ferrix-widgets`;
- [ ] Refactoring `ferrix-widgets`;

## [ ] Step 2

- [ ] Move data model from the `ferrix-app` to the `ferrix-data`;
- [ ] Remove all `ToPlainText` implementations from the `ferrix-lib`;

## [ ] Step 3

- [ ] Create `ferrix-export` module with `ToJson` and `ToPlainText` traits;
- [ ] Implement functions to convert rust structs to the `*.txt` format;

## [ ] Step 4

- [ ] Total refactoring the `ferrix-app` crate: change `message`, `subscription`, `ferrix` modules;
- [ ] Reimplement pages model;
- [ ] Automated creating pages!!!

## [ ] Step 5

- [ ] Dashboard redesign;
