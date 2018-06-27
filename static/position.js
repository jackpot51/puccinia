function generate(response, wallet_id, account_id, position_id) {
    var wallet = response.wallets.find(function(wallet) {
        return wallet.id == wallet_id;
    });
    var account = response.accounts.find(function(account) {
        return account.wallet_id == wallet_id
            && account.id == account_id;
    });
    var position = response.positions.find(function(position) {
        return position.wallet_id == wallet_id
            && position.account_id == account_id
            && position.id == position_id;
    });
    var prices = response.position_prices.filter(function(price) {
        return price.wallet_id == wallet_id
            && price.account_id == account_id
            && price.position_id == position_id;
    });
    var transactions = response.position_transactions.filter(function(transaction) {
        return transaction.wallet_id == wallet_id
            && transaction.account_id == account_id
            && transaction.position_id == position_id;
    });

    if (position_id === "balance") {
        var balance_transactions = response.transactions.filter(function(transaction) {
            return transaction.wallet_id == wallet_id
                && transaction.account_id == account_id;
        });
        convert_transactions(balance_transactions, transactions);
    }

    chart(document.getElementById("chart_price"), 'line', 'Price', share_price(prices));
    chart(document.getElementById("chart_value"), 'line', 'Value', share_value(position, transactions, prices));
    chart(document.getElementById("chart_change_in_shares"), 'scatter', 'Change in Shares', cash_flow(transactions, "units"));
    chart(document.getElementById("chart_total_shares"), 'line', 'Total Shares', net_cash_flow(transactions, "units"));
    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', cash_flow(transactions, "value"));
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', net_cash_flow(transactions, "value"));
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

    download(function(response) {
        generate(response, wallet_id, account_id, position_id);
    });
}
