function generate(response, wallet_id, account_id, position_id) {
    var filter = function(item) {
        return item.wallet_id == wallet_id
            && item.account_id == account_id
            && item.position_id == position_id;
    };
    var time_filter = create_time_filter("chart_since", filter);

    var position = response.positions.find(function(position) {
        position.position_id = position.id;
        return filter(position);
    });
    var prices = response.position_prices.filter(time_filter);
    var transactions = response.position_transactions.filter(time_filter);
    var balance_transactions = response.transactions.filter(function(transaction) {
        transaction.position_id = "balance";
        return time_filter(transaction);
    });

    convert_transactions(balance_transactions, transactions);

    chart(document.getElementById("chart_price"), 'line', 'Price', share_price(prices));
    chart(document.getElementById("chart_value"), 'line', 'Value', share_value(position, transactions, prices));
    chart(document.getElementById("chart_change_in_shares"), 'scatter', 'Change in Shares', cash_flow(transactions, "units"));
    chart(document.getElementById("chart_total_shares"), 'line', 'Total Shares', net_cash_flow(transactions, "units"));
    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', cash_flow(transactions, "value"));
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', net_cash_flow(transactions, "value"));
}

function refresh(wallet_id, account_id, position_id) {
    download(function(response) {
        generate(response, wallet_id, account_id, position_id);
    });
}

function onload(wallet_id, account_id, position_id) {
    chart_divs(document.getElementById("charts"), [
        "chart_price",
        "chart_value",
        "chart_change_in_shares",
        "chart_total_shares",
        "chart_cash_flow",
        "chart_net_cash_flow",
    ]);

    refresh(wallet_id, account_id, position_id);
}
