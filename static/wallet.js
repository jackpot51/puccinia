function generate(response, wallet_id) {
    var checkbox_filter = create_checkbox_filter("account_id");
    var filter = function(item) {
        return item.wallet_id == wallet_id
            && checkbox_filter(item);
    };
    var time_filter = create_time_filter("chart_since", filter);

    var positions = response.positions.filter(filter);
    var prices = response.position_prices.filter(time_filter);
    var position_transactions = response.position_transactions.filter(time_filter);
    var transactions = response.transactions.filter(time_filter);

    convert_transactions(transactions, position_transactions);

    chart(document.getElementById("chart_change_in_value"), 'line', 'Change in Value', change_in_value(positions, position_transactions, prices));
    chart(document.getElementById("chart_value"), 'line', 'Value', value(positions, position_transactions, prices));
    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', cash_flow(transactions));
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', net_cash_flow(transactions));
}

function refresh(wallet_id) {
    download(function(response) {
        generate(response, wallet_id);
    });
}

function onload(wallet_id) {
    chart_divs(document.getElementById("charts"), [
        "chart_change_in_value",
        "chart_value",
        "chart_cash_flow",
        "chart_net_cash_flow",
    ]);

    refresh(wallet_id);
}
