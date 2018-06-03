function generate(response, wallet_id) {
    var positions = response.positions.filter(function(position) {
        return position.wallet_id == wallet_id;
    });
    var position_transactions = response.position_transactions.filter(function(transaction) {
        return transaction.wallet_id == wallet_id;
    });
    var prices = response.position_prices.filter(function(price) {
        return price.wallet_id == wallet_id;
    });
    var transactions = response.transactions.filter(function(transaction) {
        return transaction.wallet_id == wallet_id;
    });

    convert_transactions(transactions, position_transactions);

    chart(document.getElementById("chart_change_in_value"), 'line', 'Change in Value', change_in_value(positions, position_transactions, prices));
    chart(document.getElementById("chart_value"), 'line', 'Value', value(positions, position_transactions, prices));
    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', cash_flow(transactions));
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', net_cash_flow(transactions));
}

function onload(wallet_id) {
    download(function(response) {
        generate(response, wallet_id);
    });
}
