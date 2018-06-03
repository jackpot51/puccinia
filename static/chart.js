function generate(response, start_date, end_date) {
    var wallets = response.wallets;
    var accounts = response.accounts;
    var transactions = response.transactions;

    var data = [];
    var integral = [];
    var total = 0.0;
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];
        var wallet = wallets.find(function(wallet) {
            return wallet.id == transaction.wallet_id;
        });
        var account = accounts.find(function(account) {
            return account.wallet_id == transaction.wallet_id
                && account.id == transaction.account_id;
        });

        var date = new Date(transaction.time);
        if ((!start_date || date >= new Date(start_date)) && (!end_date || date <= new Date(end_date))) {
            var amount = parseFloat(transaction.amount);
            data.push({
                x: date,
                y: amount,
                wallet: wallet,
                account: account,
                transaction: transaction
            });

            total += amount;
            integral.push({
                x: date,
                y: total,
                wallet: wallet,
                account: account,
                transaction: transaction
            });
        }
    }

    chart(document.getElementById("scatter"), 'scatter', 'Transactions', data);
    chart(document.getElementById("line"), 'line', 'Total', integral);
}

function onload() {
    var params = new URLSearchParams(location.search.slice(1));
    var start_date = params.get("start_date");
    var end_date = params.get("end_date");
    if (start_date) {
        document.getElementById("start_date").value = start_date;
    }
    if (end_date) {
        document.getElementById("end_date").value = end_date;
    }

    download(function(response) {
        generate(response, start_date, end_date);
    });
}
