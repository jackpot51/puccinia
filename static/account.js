function generate(response, wallet_id, account_id) {
    var checkbox_filter = create_checkbox_filter("position_id");
    var filter = function(item) {
        return item.wallet_id == wallet_id
            && item.account_id == account_id
            && checkbox_filter(item);
    };
    var time_filter = create_time_filter("chart_since", filter);

    var positions = response.positions.filter(function(position) {
        position.position_id = position.id;
        return filter(position);
    });
    var prices = response.position_prices.filter(time_filter);
    var position_transactions = response.position_transactions.filter(time_filter);
    var transactions = response.transactions.filter(function(transaction) {
        transaction.position_id = "balance";
        return time_filter(transaction);
    });

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

function refresh(wallet_id, account_id) {
    download(function(response) {
        generate(response, wallet_id, account_id);
    });
}

function onload(wallet_id, account_id) {
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

    refresh(wallet_id, account_id);
}
