function generate(response) {
    var filter = checkbox_filter("wallet_id");
    var positions = response.positions.filter(filter);
    var position_transactions = response.position_transactions.filter(filter);
    var prices = response.position_prices.filter(filter);
    var transactions = response.transactions.filter(filter);

    convert_transactions(transactions, position_transactions);

    chart(document.getElementById("chart_change_in_value"), 'line', 'Change in Value', change_in_value(positions, position_transactions, prices));
    chart(document.getElementById("chart_value"), 'line', 'Value', value(positions, position_transactions, prices));
    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', cash_flow(transactions));
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', net_cash_flow(transactions));
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
    ]);

    refresh();
}
