import { test, expect } from "@playwright/test";

test("homepage has title and heading text", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  await expect(page).toHaveTitle("cool articles [here!]");
  await expect(page.locator("h1")).toHaveText("Articles Collect");

  await page.goto("http://localhost:3000/edit");

  await expect(page.locator("h1")).toHaveText("Articles Collect");
  await expect(page.getByText("Unauthorized!")).toBeVisible();
});
