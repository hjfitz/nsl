# NSL - Node Script List

Usage: `nsl`

```bash
~/code/project $ nsl
audit:         npm audit --production --audit-level=high
build:         turbo run build
ci:            turbo run ci
clean:         find . -name '.turbo' -type d -exec rm -rf {} + && rm -rf node_modules/ && find . -name 'dist' -type d -exec rm -rf {} + && find  . -name 'coverage' -type d -exec rm -rf {} +
lint:fix:      npm run lint -- --fix
prepare:       node -e "try { require('husky').install() } catch (e) {if (e.code !== 'MODULE_NOT_FOUND') throw e}"
publish:pkg:   node shared/cli/dist/index.js
start:dev:     turbo run start:dev
test:          turbo run test
test:cov:      turbo run test:cov
```
