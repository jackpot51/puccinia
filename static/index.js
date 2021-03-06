function generate(response) {
    var filter = create_checkbox_filter("wallet_id");
    var time_filter = create_time_filter("chart_since", "chart_until", filter);

    var positions = response.positions.filter(filter);
    var prices = response.position_prices.filter(time_filter);
    var position_transactions = response.position_transactions.filter(time_filter);
    var transactions = response.transactions.filter(time_filter);

    convert_transactions(transactions, position_transactions);

    chart(document.getElementById("chart_change_in_value"), 'line', 'Change in Value', change_in_value(positions, position_transactions, prices));
    chart(document.getElementById("chart_value"), 'line', 'Value', value(positions, position_transactions, prices));
    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', cash_flow(transactions));
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', net_cash_flow(transactions));
    chart(document.getElementById("chart_expenses"), "scatter", "Expenses", expenses(transactions, response.transfers));
    chart(document.getElementById("chart_net_expenses"), "line", "Net Expenses", net_expenses(transactions, response.transfers));
    chart(document.getElementById("chart_income"), "scatter", "Income", income(transactions, response.transfers));
    chart(document.getElementById("chart_net_income"), "line", "Net Income", net_income(transactions, response.transfers));
}

function refresh() {
    download(function(response) {
        generate(response);
    });
}

function onload() {
    chart_divs(document.getElementById("charts"), [
        "chart_change_in_value",
        "chart_value",
        "chart_cash_flow",
        "chart_net_cash_flow",
        "chart_expenses",
        "chart_net_expenses",
        "chart_income",
        "chart_net_income",
    ]);

    refresh();
}
