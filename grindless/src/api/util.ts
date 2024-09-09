export const CREDIT_PRICE = 0.03
export const TOKEN_WEIGHT = 4;

// In credits
export const SUMMARY_1000_TOKENS_COST = 1

export function CalculatePricing(cost_per_1000: number, text_length: number) {
    let tokens = text_length / TOKEN_WEIGHT;
    let total_cost = tokens / 1000 * cost_per_1000;

    total_cost = Math.ceil(total_cost);

    if (total_cost < 1) {
        total_cost = 1;
    }

    return total_cost;
}