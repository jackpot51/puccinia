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
                    beforeLabel: function(tooltipItem, data) {
                        var item = data.datasets[tooltipItem.datasetIndex].data[tooltipItem.index];
                        console.log(item);
                        return item.wallet.name + " > " + item.account.name;
                    },
                    label: function(tooltipItem, data) {
                        var item = data.datasets[tooltipItem.datasetIndex].data[tooltipItem.index];
                        console.log(item);
                        return item.transaction.name;
                    },
                    afterLabel: function(tooltipItem, data) {
                        var item = data.datasets[tooltipItem.datasetIndex].data[tooltipItem.index];
                        console.log(item);
                        return item.transaction.time + ": " + item.y;
                    }
                },
                footerFontStyle: 'normal'
            }
        }
    });
}

function generate(response, start_date, end_date) {
    var wallets = response.wallets;
    var accounts = response.accounts;
    var transactions = response.transactions;

    var data = [];
    var integral = [];
    var total = 0.0;
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];
        var account = accounts.find(function(account) {
            return account.id == transaction.account_id;
        });
        var wallet = wallets.find(function(wallet) {
            return wallet.id == transaction.wallet_id;
        });

        var date = new Date(transaction.time);
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
