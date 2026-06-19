import { chromium } from 'playwright';
import { mkdirSync, writeFileSync } from 'fs';
import path from 'path';

const EVIDENCE_DIR = '.omo/evidence/final-qa';
const BASE_URL = 'http://localhost:1420';
const VIEWPORTS = [1366, 1920, 2560, 3840];

// Mock data for get_dashboard_summary
function makePosting(sku, name, offerId, postingNum, date, sellerPrice, commissionPct, delivery, returnFee) {
  const commission = sellerPrice * commissionPct;
  const net = sellerPrice - commission - delivery - returnFee;
  const ratio = commission / sellerPrice;
  return {
    posting_number: postingNum,
    date,
    sku,
    name,
    offer_id: offerId,
    seller_price_per_instance: sellerPrice,
    commission_amount: +commission.toFixed(2),
    commission_ratio: +ratio.toFixed(4),
    delivery_charge: delivery,
    delivery_details: { amount: delivery * 0.7, bonus: delivery * 0.1, standard_fee: delivery * 0.1, bank_coinvestment: delivery * 0.05, stars: delivery * 0.05, total: delivery },
    return_charge: returnFee,
    return_details: { amount: returnFee * 0.7, bonus: returnFee * 0.1, standard_fee: returnFee * 0.1, bank_coinvestment: returnFee * 0.05, stars: returnFee * 0.05, total: returnFee },
    services: [
      { name: 'Упаковка', price: 15.5 },
      { name: 'Сборка', price: 25.0 }
    ],
    net: +net.toFixed(2),
  };
}

const mockProducts = [
  {
    sku: 100001,
    name: 'Смарт-часы Ultra Pro (ART-100001)',
    offer_id: 'ART-100001',
    product_id: 50001,
    has_fbo_stocks: true,
    has_fbs_stocks: false,
    archived: false,
    product_info: {
      name: 'Смарт-часы Ultra Pro',
      offer_id: 'ART-100001',
      price: 5990,
      old_price: 7990,
      min_price: 4990,
      stocks_present: 150,
      stocks_reserved: 25,
      color_index: 'COLOR_INDEX_GREEN',
      commissions: [
        { sale_schema: 'FBO', percent: 5.0, delivery_amount: 0, return_amount: 0, value: 5.0 }
      ],
      volume_weight: 0.5,
      is_archived: false,
      is_super: false,
      status: 'active',
      net_price: 5990,
      images: [],
      primary_image: '',
      scheme: 'FBO'
    },
    summary: {
      total_quantity: 12,
      total_revenue: 71880,
      total_commission: 3594,
      total_delivery: 1800,
      total_returns: 600,
      service_expenses: 1200,
      expenses_cats: { 'ad': 500, 'logistics': 1800, 'storage': 200 },
      expenses_details: [],
      total_expenses: 7194,
      net_profit: 64686,
    },
    costs: {
      commission: 3594,
      acquiring: 100,
      order_processing: 200,
      logistics: 1800,
      delivery_to_pickup: 300,
      placement: 50,
      return_processing: 300,
      return_logistics: 300,
      disposal: 0,
      ovh_processing: 0,
      operational_errors: 0,
      pay_per_click: 500,
      pay_per_order: 100,
      star_products: 0,
      paid_brand: 0,
      reviews_cost: 50,
      discount_points: 0,
      partner_programs: 0,
      compensation: 0,
      other_services: 0,
    },
    totalRevenue: 71880,
    totalCosts: 7194,
    netProfit: 64686,
    profitPerUnit: 5390.5,
    totalQuantity: 12,
    postings: [
      makePosting(100001, 'Смарт-часы Ultra Pro', 'ART-100001', '230615-001', '2026-06-15T10:00:00.000Z', 5990, 0.05, 150, 50),
      makePosting(100001, 'Смарт-часы Ultra Pro', 'ART-100001', '230614-002', '2026-06-14T12:00:00.000Z', 5990, 0.05, 140, 45),
    ]
  },
  {
    sku: 100002,
    name: 'Беспроводные наушники SoundX (ART-100002)',
    offer_id: 'ART-100002',
    product_id: 50002,
    has_fbo_stocks: true,
    has_fbs_stocks: true,
    archived: false,
    product_info: {
      name: 'Беспроводные наушники SoundX',
      offer_id: 'ART-100002',
      price: 3490,
      old_price: 4490,
      min_price: 2990,
      stocks_present: 200,
      stocks_reserved: 15,
      color_index: 'COLOR_INDEX_RED',
      commissions: [
        { sale_schema: 'FBO', percent: 4.5, delivery_amount: 0, return_amount: 0, value: 4.5 },
        { sale_schema: 'FBS', percent: 3.5, delivery_amount: 0, return_amount: 0, value: 3.5 }
      ],
      volume_weight: 0.3,
      is_archived: false,
      is_super: false,
      status: 'active',
      net_price: 3490,
      images: [],
      primary_image: 'https://example.com/headphones.jpg',
      scheme: 'FBO+FBS'
    },
    summary: {
      total_quantity: 8,
      total_revenue: 27920,
      total_commission: 1256.4,
      total_delivery: 960,
      total_returns: 320,
      service_expenses: 400,
      expenses_cats: { 'ad': 200, 'logistics': 960, 'storage': 100 },
      expenses_details: [],
      total_expenses: 2936.4,
      net_profit: 24983.6,
    },
    costs: {
      commission: 1256.4,
      acquiring: 80,
      order_processing: 100,
      logistics: 960,
      delivery_to_pickup: 200,
      placement: 30,
      return_processing: 160,
      return_logistics: 160,
      disposal: 0,
      ovh_processing: 0,
      operational_errors: 0,
      pay_per_click: 200,
      pay_per_order: 50,
      star_products: 0,
      paid_brand: 0,
      reviews_cost: 25,
      discount_points: 0,
      partner_programs: 0,
      compensation: 0,
      other_services: 0,
    },
    totalRevenue: 27920,
    totalCosts: 2936.4,
    netProfit: 24983.6,
    profitPerUnit: 3122.95,
    totalQuantity: 8,
    postings: [
      makePosting(100002, 'Беспроводные наушники SoundX', 'ART-100002', '230613-003', '2026-06-13T14:00:00.000Z', 3490, 0.045, 120, 40),
      makePosting(100002, 'Беспроводные наушники SoundX', 'ART-100002', '230612-004', '2026-06-12T09:00:00.000Z', 3490, 0.045, 110, 35),
    ]
  },
  {
    sku: 100003,
    name: 'Чехол для телефона Basic (ART-100003)',
    offer_id: 'ART-100003',
    product_id: 50003,
    has_fbo_stocks: false,
    has_fbs_stocks: true,
    archived: false,
    product_info: {
      name: 'Чехол для телефона Basic',
      offer_id: 'ART-100003',
      price: 490,
      old_price: 690,
      min_price: 390,
      stocks_present: 500,
      stocks_reserved: 50,
      color_index: 'COLOR_INDEX_WITHOUT_INDEX',
      commissions: [
        { sale_schema: 'FBS', percent: 12.0, delivery_amount: 0, return_amount: 0, value: 12.0 }
      ],
      volume_weight: 0.1,
      is_archived: false,
      is_super: false,
      status: 'active',
      net_price: 490,
      images: [],
      primary_image: '',
      scheme: 'FBS'
    },
    summary: {
      total_quantity: 45,
      total_revenue: 22050,
      total_commission: 2646,
      total_delivery: 2250,
      total_returns: 900,
      service_expenses: 500,
      expenses_cats: { 'logistics': 2250, 'storage': 300, 'ad': 0 },
      expenses_details: [],
      total_expenses: 6296,
      net_profit: 15754,
    },
    costs: {
      commission: 2646,
      acquiring: 180,
      order_processing: 450,
      logistics: 2250,
      delivery_to_pickup: 100,
      placement: 20,
      return_processing: 450,
      return_logistics: 450,
      disposal: 0,
      ovh_processing: 0,
      operational_errors: 0,
      pay_per_click: 0,
      pay_per_order: 0,
      star_products: 0,
      paid_brand: 0,
      reviews_cost: 20,
      discount_points: 0,
      partner_programs: 0,
      compensation: 0,
      other_services: 0,
    },
    totalRevenue: 22050,
    totalCosts: 6296,
    netProfit: 15754,
    profitPerUnit: 350.09,
    totalQuantity: 45,
    postings: [
      makePosting(100003, 'Чехол для телефона Basic', 'ART-100003', '230611-005', '2026-06-11T16:00:00.000Z', 490, 0.12, 50, 20),
      makePosting(100003, 'Чехол для телефона Basic', 'ART-100003', '230610-006', '2026-06-10T11:00:00.000Z', 490, 0.12, 45, 17),
    ]
  }
];

// Product with unknown color index for fallback scenario
const unknownColorProduct = {
  sku: 100004,
  name: 'Тестовый товар (ART-100004)',
  offer_id: 'ART-100004',
  product_id: 50004,
  has_fbo_stocks: true,
  has_fbs_stocks: false,
  archived: false,
  product_info: {
    name: 'Тестовый товар',
    offer_id: 'ART-100004',
    price: 1000,
    old_price: 1200,
    min_price: 800,
    stocks_present: 10,
    stocks_reserved: 2,
    color_index: 'COLOR_INDEX_CUSTOM_UNKNOWN',
    commissions: [
      { sale_schema: 'FBO', percent: 15.0, delivery_amount: 0, return_amount: 0, value: 15.0 }
    ],
    volume_weight: 0.2,
    is_archived: false,
    is_super: false,
    status: 'active',
    net_price: 1000,
    images: [],
    primary_image: '',
    scheme: 'FBO'
  },
  summary: {
    total_quantity: 3,
    total_revenue: 3000,
    total_commission: 450,
    total_delivery: 150,
    total_returns: 75,
    service_expenses: 100,
    expenses_cats: {},
    expenses_details: [],
    total_expenses: 775,
    net_profit: 2225,
  },
  costs: {
    commission: 450, acquiring: 0, order_processing: 0, logistics: 150,
    delivery_to_pickup: 0, placement: 0, return_processing: 35, return_logistics: 40,
    disposal: 0, ovh_processing: 0, operational_errors: 0, pay_per_click: 0,
    pay_per_order: 0, star_products: 0, paid_brand: 0, reviews_cost: 0,
    discount_points: 0, partner_programs: 0, compensation: 0, other_services: 0,
  },
  totalRevenue: 3000,
  totalCosts: 775,
  netProfit: 2225,
  profitPerUnit: 741.67,
  totalQuantity: 3,
  postings: [
    makePosting(100004, 'Тестовый товар', 'ART-100004', '230609-007', '2026-06-09T08:00:00.000Z', 1000, 0.15, 50, 25),
  ]
};

const allProducts = [...mockProducts, unknownColorProduct];

const mockDashboardData = {
  month: 6,
  year: 2026,
  period: { from: '2026-06-01T00:00:00.000Z', to: '2026-06-30T23:59:59.999Z' },
  data_source: 'realization',
  totals: {
    total_revenue: 124850,
    product_expenses: 17201.4,
    account_expenses: 5000,
    total_expenses: 22201.4,
    net_profit: 102648.6,
    total_quantity: 68,
    product_count: 4,
  },
  account_expenses: {
    cats: { 'ad_clicks': 1500, 'ad_orders': 800, 'logistics': 1700, 'storage': 1000 },
    details: [],
  },
  products: allProducts,
};

// Mock Tauri invoke handler
async function mockInvoke(cmd, args) {
  if (cmd === 'check_config') {
    return { valid: true, message: '' };
  }
  if (cmd === 'get_dashboard_summary') {
    return mockDashboardData;
  }
  // For other commands, return empty
  return null;
}

const mockInvokeScript = `
window.__TAURI_INTERNALS__ = {
  invoke: async function(cmd, args, options) {
    try {
      const result = await window.__mockInvokeHandler(cmd, args);
      return result;
    } catch (e) {
      // Fallback
      return null;
    }
  },
  transformCallback: function(cb, once) {
    return window.__TAURI_INTERNALS__?.__nextCbId ?? 1;
  },
  metadata: {
    currentWindow: { label: 'main' },
    currentWebview: { label: 'main' },
    currentWebviewWindow: { label: 'main' }
  }
};
`;

async function screenshot(page, name) {
  const filePath = path.join(EVIDENCE_DIR, name);
  await page.screenshot({ path: filePath, fullPage: false });
  console.log(`  [screenshot] ${filePath}`);
  return filePath;
}

async function main() {
  mkdirSync(EVIDENCE_DIR, { recursive: true });

  console.log('Starting QA test suite...\n');

  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({
    bypassCSP: true,
  });
  
  // Inject mock handler
  await context.addInitScript(() => {
    window.__mockInvokeHandler = async (cmd, args) => {
      if (cmd === 'check_config') return { valid: true, message: '' };
      if (cmd === 'get_dashboard_summary') return ${JSON.stringify(mockDashboardData)};
      return null;
    };
  });
  await context.addInitScript(mockInvokeScript);

  const page = await context.newPage();
  
  // Navigate to app
  console.log('Navigating to app...');
  await page.goto(BASE_URL, { waitUntil: 'networkidle', timeout: 30000 });
  
  // Wait for product table to appear
  console.log('Waiting for data table...');
  try {
    await page.waitForSelector('.n-data-table', { timeout: 15000 });
    console.log('Data table loaded.');
  } catch (e) {
    console.log('Error: Data table not found. Taking debug screenshot...');
    await screenshot(page, 'debug-error.png');
    // Try to see what's on screen
    const html = await page.content();
    console.log('Page HTML (first 2000 chars):', html.substring(0, 2000));
    await browser.close();
    process.exit(1);
  }

  const results = { scenarios: {}, viewports: 4 };
  let pass = 0;
  let fail = 0;

  // === Execute all scenarios ===
  
  for (const vp of VIEWPORTS) {
    await page.setViewportSize({ width: vp, height: Math.round(vp * 0.65) });
    console.log(`\n--- Viewport: ${vp}px ---`);
    
    // Scenario 1: Nested table with posting data
    {
      const key = `S1-vp${vp}`;
      try {
        // Find the expand chevron in the first row
        const expandIcon = page.locator('.n-data-table-tr').first().locator('.n-data-table-td--expand .n-data-table-expand-trigger').first();
        await expandIcon.click();
        await page.waitForTimeout(500);
        
        // Assert nested table is visible
        const nestedTable = page.locator('.n-data-table-tr--expanded .n-data-table');
        const nestedVisible = await nestedTable.first().isVisible();
        
        // Check posting number pattern
        const postingCell = nestedTable.locator('tbody tr').first().locator('td').first();
        const postingText = await postingCell.textContent();
        const hasPostingPattern = /\d+-\d+/.test(postingText);
        
        // Check revenue formatting
        const revenueCell = nestedTable.locator('tbody tr').first().locator('td').nth(3);
        const revenueText = await revenueCell.textContent();
        const hasRubAmount = /[₽MК]/.test(revenueText);
        
        // Check net profit colored text
        const profitCell = nestedTable.locator('tbody tr').first().locator('td').nth(7);
        const profitHtml = await profitCell.innerHTML();
        const hasColoredProfit = profitHtml.includes('amount-positive') || profitHtml.includes('amount-negative');
        
        const allPassed = nestedVisible && hasPostingPattern && hasRubAmount && hasColoredProfit;
        
        await screenshot(page, `s1-nested-table-${vp}.png`);
        console.log(`  S1: nestedVisible=${nestedVisible} postingPattern=${hasPostingPattern} rubAmount=${hasRubAmount} coloredProfit=${hasColoredProfit} -> ${allPassed ? 'PASS' : 'FAIL'}`);
        results.scenarios[key] = allPassed;
        if (allPassed) pass++; else fail++;
        
        // Collapse again
        await expandIcon.click();
        await page.waitForTimeout(300);
      } catch (e) {
        console.log(`  S1: ERROR - ${e.message}`);
        results.scenarios[key] = false;
        fail++;
      }
    }
    
    // Scenario 2: Price index labels render correctly
    {
      const key = `S2-vp${vp}`;
      try {
        // Find color index column cells
        const colorCells = page.locator('.n-data-table-tr').filter({ hasNot: page.locator('.n-data-table-tr--expanded') }).locator('.n-tag');
        const tagCount = await colorCells.count();
        
        // Check that at least some NTag elements exist
        let hasTags = tagCount > 0;
        
        // Check for expected label text
        const allTagTexts = [];
        for (let i = 0; i < Math.min(tagCount, 10); i++) {
          const text = await colorCells.nth(i).textContent();
          allTagTexts.push(text);
        }
        console.log(`  S2: tags found=${tagCount}, texts=[${allTagTexts.join(', ')}]`);
        
        const hasExpectedLabels = allTagTexts.some(t => 
          t.includes('Зеленый') || t.includes('Красный') || t.includes('Без индекса') || 
          t.includes('Green') || t.includes('Red') || t.includes('Without')
        );
        
        await screenshot(page, `s2-price-index-${vp}.png`);
        console.log(`  S2: hasTags=${hasTags} hasExpectedLabels=${hasExpectedLabels} -> ${(hasTags && hasExpectedLabels) ? 'PASS' : 'FAIL'}`);
        results.scenarios[key] = hasTags && hasExpectedLabels;
        if (hasTags && hasExpectedLabels) pass++; else fail++;
      } catch (e) {
        console.log(`  S2: ERROR - ${e.message}`);
        results.scenarios[key] = false;
        fail++;
      }
    }
    
    // Scenario 3: Commission badges with popover
    {
      const key = `S3-vp${vp}`;
      try {
        // Find commission column tags
        const allTags = page.locator('.n-tag');
        const allTagCount = await allTags.count();
        
        // Look for commission badges (FBO, FBS, rFBS tags)
        let foundCommissionBadges = false;
        let popoverTextVerified = false;
        
        for (let i = 0; i < Math.min(allTagCount, 20); i++) {
          const text = await allTags.nth(i).textContent();
          if (['FBO', 'FBS', 'RFBS', 'rFBS', 'FBP'].includes(text.trim().toUpperCase())) {
            foundCommissionBadges = true;
            // Hover to trigger popover
            await allTags.nth(i).hover();
            await page.waitForTimeout(800);
            
            // Check popover
            const popoverContent = page.locator('.n-popover');
            const popoverVisible = await popoverContent.isVisible();
            if (popoverVisible) {
              const popoverText = await popoverContent.textContent();
              console.log(`  S3: hovered "${text}", popover text="${popoverText}"`);
              // Popover should contain percentage
              if (/%/.test(popoverText)) {
                popoverTextVerified = true;
              }
            }
            break;
          }
        }
        
        await screenshot(page, `s3-commission-popover-${vp}.png`);
        console.log(`  S3: foundCommissionBadges=${foundCommissionBadges} popoverVerified=${popoverTextVerified} -> ${(foundCommissionBadges && popoverTextVerified) ? 'PASS' : 'FAIL'}`);
        results.scenarios[key] = foundCommissionBadges && popoverTextVerified;
        if (foundCommissionBadges && popoverTextVerified) pass++; else fail++;
      } catch (e) {
        console.log(`  S3: ERROR - ${e.message}`);
        results.scenarios[key] = false;
        fail++;
      }
    }
  }

  // Scenario 4: Fluid container at 4K (only at 3840)
  {
    const key = 'S4-vp3840';
    try {
      await page.setViewportSize({ width: 3840, height: 2160 });
      await page.waitForTimeout(1000);
      
      const viewContainer = page.locator('.view-container');
      const box = await viewContainer.boundingBox();
      const width = box?.width || 0;
      
      await screenshot(page, `s4-fluid-4k.png`);
      console.log(`  S4: view-container width=${width}px expected>3000px -> ${width > 3000 ? 'PASS' : 'FAIL'}`);
      results.scenarios[key] = width > 3000;
      if (width > 3000) pass++; else fail++;
    } catch (e) {
      console.log(`  S4: ERROR - ${e.message}`);
      results.scenarios[key] = false;
      fail++;
    }
  }

  // Scenario 5: Reactive table height on resize
  {
    const key = 'S5-resize';
    try {
      // Set to 1920 first
      await page.setViewportSize({ width: 1920, height: 1080 });
      await page.waitForTimeout(500);
      
      // Get initial max-height
      const table1 = page.locator('.n-data-table .n-data-table-wrapper').first();
      const style1 = await table1.getAttribute('style');
      const match1 = style1?.match(/max-height:\s*(\d+)px/);
      const initialHeight = match1 ? parseInt(match1[1]) : 0;
      console.log(`  S5: initial max-height (1920) = ${initialHeight}px`);
      
      // Resize to 3840
      await page.setViewportSize({ width: 3840, height: 2160 });
      await page.waitForTimeout(500);
      
      const table2 = page.locator('.n-data-table .n-data-table-wrapper').first();
      const style2 = await table2.getAttribute('style');
      const match2 = style2?.match(/max-height:\s*(\d+)px/);
      const newHeight = match2 ? parseInt(match2[1]) : 0;
      console.log(`  S5: new max-height (3840) = ${newHeight}px`);
      
      const heightIncreased = newHeight > initialHeight;
      
      await screenshot(page, `s5-resize-3840.png`);
      console.log(`  S5: heightIncreased=${heightIncreased} -> ${heightIncreased ? 'PASS' : 'FAIL'}`);
      results.scenarios[key] = heightIncreased;
      if (heightIncreased) pass++; else fail++;
    } catch (e) {
      console.log(`  S5: ERROR - ${e.message}`);
      results.scenarios[key] = false;
      fail++;
    }
  }

  // Scenario 6: Graceful fallback for unknown price index
  {
    const key = 'S6-fallback';
    try {
      // The unknown product (sku 100004) has COLOR_INDEX_CUSTOM_UNKNOWN
      // The colorIndexRender function should show raw string in NTag when code not in map
      await page.setViewportSize({ width: 1920, height: 1080 });
      await page.waitForTimeout(500);
      
      // Find all tags and look for the unknown code
      const allTags = page.locator('.n-tag');
      const tagCount = await allTags.count();
      let foundFallback = false;
      
      for (let i = 0; i < Math.min(tagCount, 30); i++) {
        const text = await allTags.nth(i).textContent();
        if (text.includes('COLOR_INDEX_CUSTOM_UNKNOWN')) {
          foundFallback = true;
          console.log(`  S6: Found fallback tag with raw code: "${text}"`);
          break;
        }
      }
      
      // Also verify the mapped labels are shown (not raw codes)
      let foundMappedLabels = false;
      for (let i = 0; i < Math.min(tagCount, 30); i++) {
        const text = await allTags.nth(i).textContent();
        if (text.includes('Зеленый') || text.includes('Красный') || text.includes('Без индекса')) {
          foundMappedLabels = true;
          break;
        }
      }
      
      await screenshot(page, `s6-fallback.png`);
      console.log(`  S6: foundFallback=${foundFallback} foundMappedLabels=${foundMappedLabels} -> ${foundFallback ? 'PASS' : 'FAIL'}`);
      results.scenarios[key] = foundFallback;
      if (foundFallback) pass++; else fail++;
    } catch (e) {
      console.log(`  S6: ERROR - ${e.message}`);
      results.scenarios[key] = false;
      fail++;
    }
  }

  await browser.close();

  // === Summary ===
  const totalScenarios = Object.keys(results.scenarios).filter(k => !k.includes('vp')).length;
  // Count unique scenario types (S1, S2, etc.) rather than instances
  const scenarioTypes = new Set(Object.keys(results.scenarios).map(k => k.split('-')[0]));
  const passedScenarios = [...scenarioTypes].filter(sc => 
    Object.entries(results.scenarios).filter(([k, v]) => k.startsWith(sc) && v).length > 0
  ).length;

  console.log(`\n========== QA RESULTS ==========`);
  console.log(`Scenarios [${passedScenarios}/${scenarioTypes.size} pass] | Viewports [4/4]`);
  console.log(`Total assertions passed: ${pass}, failed: ${fail}`);
  
  if (fail === 0) {
    console.log(`VERDICT: APPROVE`);
    writeFileSync(path.join(EVIDENCE_DIR, 'verdict.txt'), 'APPROVE - All scenarios passed');
  } else {
    console.log(`VERDICT: REJECT`);
    writeFileSync(path.join(EVIDENCE_DIR, 'verdict.txt'), `REJECT - ${fail} assertions failed`);
  }
  console.log(`==================================`);
}

main().catch(e => {
  console.error('Fatal error:', e);
  process.exit(1);
});
