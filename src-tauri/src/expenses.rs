use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ExpenseCategory {
    Ad,
    Storage,
    Logistics,
    Compensation,
    Other,
}

impl ExpenseCategory {
    fn all_variants() -> Vec<ExpenseCategory> {
        vec![
            ExpenseCategory::Ad,
            ExpenseCategory::Storage,
            ExpenseCategory::Logistics,
            ExpenseCategory::Compensation,
            ExpenseCategory::Other,
        ]
    }
}

const AD_KEYWORDS: &[&str] = &[
    "клик", "click", "продвижени", "реклам", "promotion", "advertis", "adv",
    "маркет", "market", "cost per click", "cost per order",
];
const STORAGE_KEYWORDS: &[&str] = &[
    "хранени", "storage", "place", "размещ", "излишк", "неполн", "surplus",
    "shortage", "бронирован",
];
const LOGISTICS_KEYWORDS: &[&str] = &[
    "логистик", "logistic", "доставк", "delivery", "кросс-док", "crossdock",
    "обратн", "return", "возврат",
];
const COMPENSATION_KEYWORDS: &[&str] = &[
    "компенсац", "compens", "утер", "поврежд", "loss", "damage",
];

fn categorize_expense(type_name: &str) -> ExpenseCategory {
    let lower = type_name.to_lowercase();

    let spans: &[(ExpenseCategory, &[&str])] = &[
        (ExpenseCategory::Ad, AD_KEYWORDS),
        (ExpenseCategory::Storage, STORAGE_KEYWORDS),
        (ExpenseCategory::Logistics, LOGISTICS_KEYWORDS),
        (ExpenseCategory::Compensation, COMPENSATION_KEYWORDS),
    ];

    for (cat, keywords) in spans {
        if keywords.iter().any(|k| lower.contains(k)) {
            return cat.clone();
        }
    }

    ExpenseCategory::Other
}

fn skip_types() -> &'static HashSet<String> {
    static SKIP: std::sync::LazyLock<HashSet<String>> = std::sync::LazyLock::new(|| {
        ["OperationAgentDeliveredToCustomer", "OperationItemReturn"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    });
    &SKIP
}

#[derive(Debug, Clone, Serialize)]
pub struct ExpenseCategories {
    pub cats: HashMap<String, f64>,
    pub details: HashMap<String, Vec<ExpenseDetail>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExpenseDetail {
    pub name: String,
    pub date: String,
    pub amount: f64,
    pub operation_id: Value,
}

pub fn build_expense_categories(operations: &[Value], sku: i64) -> ExpenseCategories {
    let skipped = skip_types();
    let mut cats: HashMap<ExpenseCategory, f64> = HashMap::new();
    let mut details: HashMap<ExpenseCategory, Vec<ExpenseDetail>> = HashMap::new();

    for op in operations {
        let op_type = op["operation_type"].as_str().unwrap_or("");
        if skipped.contains(op_type) {
            continue;
        }

        let items = op["items"].as_array();
        if items.map_or(true, |i| i.is_empty()) {
            continue;
        }

        let has_sku = items.unwrap().iter().any(|item| {
            item["sku"].as_i64().map_or(false, |s| s == sku)
        });
        if !has_sku {
            continue;
        }

        let amount = (op["amount"].as_f64().unwrap_or(0.0)).abs();
        if amount == 0.0 {
            continue;
        }

        let cat = categorize_expense(op["operation_type_name"].as_str().unwrap_or(""));
        *cats.entry(cat.clone()).or_insert(0.0) += amount;
        details.entry(cat.clone()).or_default().push(ExpenseDetail {
            name: op["operation_type_name"].as_str().unwrap_or("—").to_string(),
            date: op["operation_date"].as_str().unwrap_or("").to_string(),
            amount,
            operation_id: op["operation_id"].clone(),
        });
    }

    ExpenseCategories {
        cats: ExpenseCategory::all_variants()
            .into_iter()
            .map(|c| {
                let label = serde_json::to_string(&c).unwrap().trim_matches('"').to_string();
                let val = cats.get(&c).copied().unwrap_or(0.0);
                (label, val)
            })
            .collect(),
        details: ExpenseCategory::all_variants()
            .into_iter()
            .map(|c| {
                let label = serde_json::to_string(&c).unwrap().trim_matches('"').to_string();
                let dets = details.get(&c).cloned().unwrap_or_default();
                (label, dets)
            })
            .collect(),
    }
}

pub fn build_account_level_expenses(operations: &[Value]) -> ExpenseCategories {
    let skipped = skip_types();
    let mut cats: HashMap<ExpenseCategory, f64> = HashMap::new();
    let mut details: HashMap<ExpenseCategory, Vec<ExpenseDetail>> = HashMap::new();

    for op in operations {
        let op_type = op["operation_type"].as_str().unwrap_or("");
        if skipped.contains(op_type) {
            continue;
        }

        let items = op["items"].as_array();
        if items.map_or(false, |i| !i.is_empty()) {
            continue;
        }

        let amount = (op["amount"].as_f64().unwrap_or(0.0)).abs();
        if amount == 0.0 {
            continue;
        }

        let cat = categorize_expense(op["operation_type_name"].as_str().unwrap_or(""));
        *cats.entry(cat.clone()).or_insert(0.0) += amount;
        details.entry(cat.clone()).or_default().push(ExpenseDetail {
            name: op["operation_type_name"].as_str().unwrap_or("—").to_string(),
            date: op["operation_date"].as_str().unwrap_or("").to_string(),
            amount,
            operation_id: op["operation_id"].clone(),
        });
    }

    ExpenseCategories {
        cats: ExpenseCategory::all_variants()
            .into_iter()
            .map(|c| {
                let label = serde_json::to_string(&c).unwrap().trim_matches('"').to_string();
                let val = cats.get(&c).copied().unwrap_or(0.0);
                (label, val)
            })
            .collect(),
        details: ExpenseCategory::all_variants()
            .into_iter()
            .map(|c| {
                let label = serde_json::to_string(&c).unwrap().trim_matches('"').to_string();
                let dets = details.get(&c).cloned().unwrap_or_default();
                (label, dets)
            })
            .collect(),
    }
}
