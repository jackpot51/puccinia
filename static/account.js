function download(callback) {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            var response = JSON.parse(this.responseText);
            callback(response);
        }
    };
    xhttp.open("GET", "/json", true);
    xhttp.send();
}

function chart(element, type, title, data) {
    var context = element.getContext('2d');
    var chart = new Chart(context, {
        type: type,
        data: {
            datasets: [{
                label: title,
                data: data
            }]
        },
        options: {
            responsive: true,
            scales: {
                xAxes: [{
                    type: 'time',
                    display: true,
                    scaleLabel: {
                        display: true,
                        labelString: 'Date'
                    }
                }]
            },
            tooltips: {
                mode: 'index',
                callbacks: {
                    title: function(tooltipItems, data) {
                        var tooltipItem = tooltipItems[0];
                        var item = data.datasets[tooltipItem.datasetIndex].data[tooltipItem.index];
                        return item.transaction.time;
                    },
                    label: function(tooltipItem, data) {
                        var item = data.datasets[tooltipItem.datasetIndex].data[tooltipItem.index];
                        return item.transaction.name;
                    },
                    afterLabel: function(tooltipItem, data) {
                        var item = data.datasets[tooltipItem.datasetIndex].data[tooltipItem.index];
                        return item.y;
                    }
                },
                footerFontStyle: 'normal'
            }
        }
    });
}

function generate(response, wallet_id, account_id) {
    var wallets = response.wallets;
    var accounts = response.accounts;
    var transactions = response.transactions;

    var wallet = wallets.find(function(wallet) {
        return wallet.id == wallet_id;
    });
    var account = accounts.find(function(account) {
        return wallet.id == wallet_id && account.id == account_id;
    });

    var data = [];
    var integral = [];
    var total = 0.0;
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];
        if (transaction.wallet_id == wallet_id && transaction.account_id == account_id) {
            var date = new Date(transaction.time);
            var amount = parseFloat(transaction.amount);
            data.push({
                x: date,
                y: Math.round(amount * 100.0)/100.0,
                wallet: wallet,
                account: account,
                transaction: transaction
            });

            total += amount;
            integral.push({
                x: date,
                y: Math.round(total * 100.0)/100.0,
                wallet: wallet,
                account: account,
                transaction: transaction
            });
        }
    }

    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', data);
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', integral);
}

function onload(wallet_id, account_id) {
    download(function(response) {
        generate(response, wallet_id, account_id);
    });
}
