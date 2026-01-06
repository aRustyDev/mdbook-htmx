---
title: Visual Testing
description: Test documentation rendering with Playwright
---

# Visual Testing

Ensure documentation renders correctly across browsers.

## Playwright Setup

```bash
npm init -y
npm install -D @playwright/test
npx playwright install
```

## Screenshot Tests

```typescript
// tests/visual.spec.ts
import { test, expect } from '@playwright/test';

test('homepage renders correctly', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveScreenshot('homepage.png');
});

test('navigation works', async ({ page }) => {
  await page.goto('/');
  await page.click('text=Getting Started');
  await expect(page.locator('#content h1')).toContainText('Getting Started');
});

test('search works', async ({ page }) => {
  await page.goto('/');
  await page.fill('[type="search"]', 'configuration');
  await expect(page.locator('#search-results')).toBeVisible();
});
```

## CI Integration

```yaml
visual-tests:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4

    - name: Install dependencies
      run: npm ci

    - name: Install browsers
      run: npx playwright install --with-deps

    - name: Run tests
      run: npx playwright test

    - name: Upload report
      uses: actions/upload-artifact@v4
      if: failure()
      with:
        name: playwright-report
        path: playwright-report/
```

## Accessibility Testing

```typescript
import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('accessibility scan', async ({ page }) => {
  await page.goto('/');
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations).toEqual([]);
});
```

## See Also

- [CI/CD Pipelines](cicd.md)
- [Contributing: Development](../contributing/development.md)
