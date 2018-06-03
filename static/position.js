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
                        return item.title;
                    },
                    label: function(tooltipItem, data) {
                        var item = data.datasets[tooltipItem.datasetIndex].data[tooltipItem.index];
                        return item.label;
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

    var data_price = [];
    for (var i = 0; i < prices.length; i++) {
        var price = prices[i];

        var date = new Date(price.time);
        var value = parseFloat(price.price);
        data_price.push({
            x: date,
            y: Math.round(value * 100.0)/100.0,
            title: price.time,
            label: position_id
        });
    }

    var data_value = [];
    var current_units = parseFloat(position.units);
    var j = prices.length - 1;
    var add_price = function(price) {
        data_value.push({
            x: new Date(price.time),
            y: Math.round(parseFloat(price.price) * current_units * 100.0)/100.0,
            title: price.time + "Price",
            label: position_id + ": " + current_units + " @ " + price.price
        });
    };
    for (var i = transactions.length - 1; i >= 0; i--) {
        var transaction = transactions[i];

        for(; j >= 0; j--) {
            var price = prices[j];

            if (price.time <= transaction.time) {
                break;
            }

            add_price(price);
        }

        data_value.push({
            x: new Date(transaction.time),
            y: Math.round(parseFloat(transaction.price) * current_units * 100.0)/100.0,
            title: transaction.time + " Transaction",
            label: position_id + ": " + current_units + " @ " + transaction.price
        });

        for(; j >= 0; j--) {
            var price = prices[j];

            if (price.time < transaction.time) {
                break;
            }

            add_price(price);
        }

        current_units -= parseFloat(transaction.units);
    }
    for (; j >= 0; j--) {
        var price = prices[j];

        add_price(price);
    }
    data_value = data_value.reverse();

    var data_units = [];
    var integral_units = [];
    var total_units = 0.0;
    var data_flow = [];
    var integral_flow = [];
    var total_flow = 0.0;
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];

        var date = new Date(transaction.time);
        var units = parseFloat(transaction.units);
        data_units.push({
            x: date,
            y: units,
            title: transaction.time,
            label: transaction.name
        });

        total_units += units;
        integral_units.push({
            x: date,
            y: units,
            title: transaction.time,
            label: transaction.name
        });

        var value = parseFloat(transaction.value);
        data_flow.push({
            x: date,
            y: Math.round(value * 100.0)/100.0,
            title: transaction.time,
            label: transaction.name
        });

        total_flow += value;
        integral_flow.push({
            x: date,
            y: Math.round(value * 100.0)/100.0,
            title: transaction.time,
            label: transaction.name
        });
    }

    chart(document.getElementById("chart_price"), 'line', 'Price', data_price);
    chart(document.getElementById("chart_value"), 'line', 'Value', data_value);
    chart(document.getElementById("chart_change_in_shares"), 'scatter', 'Change in Shares', data_units);
    chart(document.getElementById("chart_total_shares"), 'line', 'Total Shares', integral_units);
    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', data_flow);
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', integral_flow);
}

function onload(wallet_id, account_id, position_id) {
    download(function(response) {
        generate(response, wallet_id, account_id, position_id);
    });
}
