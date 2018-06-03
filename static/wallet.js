function generate(response, wallet_id) {
    var transactions = response.transactions.filter(function(transaction) {
        return transaction.wallet_id == wallet_id;
    });

    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', cash_flow(transactions));
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', net_cash_flow(transactions));
}

function onload(wallet_id) {
    download(function(response) {
        generate(response, wallet_id);
    });
}
