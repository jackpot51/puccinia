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

function share_price(prices) {
    var data = [];
    for (var i = 0; i < prices.length; i++) {
        var price = prices[i];

        var date = new Date(price.time);
        var value = parseFloat(price.price);
        data.push({
            x: date,
            y: Math.round(value * 100.0)/100.0,
            title: price.time,
            label: price.position_id
        });
    }
    return data;
}

function share_value(position, transactions, prices) {
    var data = [];
    var current_units = parseFloat(position.units);

    var add_price = function(price) {
        data.push({
            x: new Date(price.time),
            y: Math.round(parseFloat(price.price) * current_units * 100.0)/100.0,
            title: price.time + "Price",
            label: price.position_id + ": " + current_units + " @ " + price.price
        });
    };

    var j = prices.length - 1;
    for (var i = transactions.length - 1; i >= 0; i--) {
        var transaction = transactions[i];

        for(; j >= 0; j--) {
            var price = prices[j];

            if (price.time <= transaction.time) {
                break;
            }

            add_price(price);
        }

        data.push({
            x: new Date(transaction.time),
            y: Math.round(parseFloat(transaction.price) * current_units * 100.0)/100.0,
            title: transaction.time + " Transaction",
            label: transaction.position_id + ": " + current_units + " @ " + transaction.price
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

    return data.reverse();
}

function cash_flow(transactions, amount_key="amount") {
    var data = [];
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];

        var date = new Date(transaction.time);
        var amount = parseFloat(transaction[amount_key]);
        data.push({
            x: date,
            y: Math.round(amount * 100.0)/100.0,
            title: transaction.time,
            label: transaction.name
        });
    }

    return data;
}

function net_cash_flow(transactions, amount_key="amount") {
    var data = [];
    var total = 0.0;
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];

        var date = new Date(transaction.time);
        var amount = parseFloat(transaction[amount_key]);
        total += amount;
        data.push({
            x: date,
            y: Math.round(total * 100.0)/100.0,
            title: transaction.time,
            label: transaction.name
        });
    }

    return data;
}
