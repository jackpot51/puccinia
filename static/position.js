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

function generate(response, wallet_id, account_id, position_id) {
    var wallets = response.wallets;
    var accounts = response.accounts;
    var positions = response.positions;
    var transactions = response.position_transactions;

    var wallet = wallets.find(function(wallet) {
        return wallet.id == wallet_id;
    });
    var account = accounts.find(function(account) {
        return account.wallet_id == wallet_id
            && account.id == account_id;
    });
    var position = positions.find(function(position) {
        return position.wallet_id == wallet_id
            && position.account_id == account_id
            && position.id == position_id;
    });

    var data_units = [];
    var integral_units = [];
    var total_units = 0.0;
    var data_value = [];
    var integral_value = [];
    var total_value = 0.0;
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];
        if (
            transaction.wallet_id == wallet_id
            && transaction.account_id == account_id
            && transaction.position_id == position_id
        ) {
            var date = new Date(transaction.time);
            var units = parseFloat(transaction.units);
            data_units.push({
                x: date,
                y: units,
                wallet: wallet,
                account: account,
                transaction: transaction
            });

            total_units += units;
            integral_units.push({
                x: date,
                y: units,
                wallet: wallet,
                account: account,
                transaction: transaction
            });


            var value = parseFloat(transaction.value);
            data_value.push({
                x: date,
                y: Math.round(value * 100.0)/100.0,
                wallet: wallet,
                account: account,
                transaction: transaction
            });

            total_value += value;
            integral_value.push({
                x: date,
                y: Math.round(value * 100.0)/100.0,
                wallet: wallet,
                account: account,
                transaction: transaction
            });
        }
    }

    chart(document.getElementById("chart_change_in_shares"), 'scatter', 'Change in Shares', data_units);
    chart(document.getElementById("chart_total_shares"), 'line', 'Total Shares', integral_units);
    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', data_value);
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', integral_value);
}

function onload(wallet_id, account_id, position_id) {
    download(function(response) {
        generate(response, wallet_id, account_id, position_id);
    });
}
